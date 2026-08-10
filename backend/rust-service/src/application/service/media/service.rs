use std::{path::Path, sync::Arc};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use content_inspector::{ContentType, inspect};
use infer;
use libvips::{
    VipsImage,
    ops::{self, Access},
};
use tokio::{fs::File, io::AsyncReadExt, task::JoinHandle};

use crate::{
    application::{
        repository::media::{self, row::{MediaDataRow, MediaStatus}}, service::{
            errors::MediaServiceError, media::{
                processor, transformer::Transformer, types::{
                    MediaCategory, MediaMeta, MediaOptions,
                    OnProcessingType, PostProcessingType, ProcessObject, ProcessedMedia,
                    ResizeStyle, TempUpload,
                },
            }, snowflake_service::SnowflakeGenerator,
        },
    }, constant::LOCKED_UPLOADS_DIR,
};

use super::storage::{MediaStorage};

pub struct MediaService {
    storage: Arc<dyn MediaStorage>,
    snowflake: SnowflakeGenerator,
    connection: sqlx::PgPool,
}

impl MediaService {
    pub fn new(snowflake: SnowflakeGenerator, storage: Arc<dyn MediaStorage>, connection: sqlx::PgPool) -> Self {
        Self {
            storage,
            snowflake,
            connection,
        }
    }

    /// Hashes the given file bytes using SHA-256 and returns the hexadecimal representation of the hash.
    /// This will be use as a name for the file to avoid duplicates and ensure uniqueness.
    fn hash_file(path: &std::path::Path) -> std::io::Result<String> {
        use sha2::{Digest, Sha256};
        use std::io::Read;

        let mut file = std::fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 64 * 1024]; // 64 KiB buffer

        loop {
            let read = file.read(&mut buffer)?;

            if read == 0 {
                break;
            }

            hasher.update(&buffer[..read]);
        }

        Ok(hex::encode(hasher.finalize()))
    }

    fn generate_thumbhash_from_image(image: &VipsImage) -> String {
        let mut rgba = image.image_write_to_memory();

        if image.get_bands() < 4 {
            let mut new_rgba =
                Vec::with_capacity((image.get_width() * image.get_height() * 4) as usize);
            for pixel in rgba.chunks_exact(3) {
                new_rgba.extend_from_slice(pixel);
                new_rgba.push(255); // Opaque alpha
            }
            rgba = new_rgba;
        }

        let thumbhash = thumbhash::rgba_to_thumb_hash(
            image.get_width() as usize,
            image.get_height() as usize,
            &rgba,
        );

        URL_SAFE_NO_PAD.encode(&thumbhash)
    }
    // PUBLICS

    pub async fn save_media(
        &self,
        uploader_id: i64,
        temp_uploaded_file: TempUpload,
        media_options: MediaOptions,
    ) -> Result<ProcessedMedia, MediaServiceError> {
        let upload_job = self.storage.new_temp_relative_path("upload_job")?;

        let processed_media = self
            .process_media(
                temp_uploaded_file.into(),
                upload_job.clone(),
                media_options.clone(),
            )
            .await?
            .await??;

        let final_destination = {
            if media_options.processing_order.options.locked {
                format!("{}/{}", LOCKED_UPLOADS_DIR, media_options.folder)
            } else {
                media_options.folder.clone()
            }
        };

        let upload_job_full_path = self.storage.temp_full_path(&upload_job)?;
        let final_destination_full_path = self.storage.full_path(&final_destination)?;

        // move file within storage to final destination
        self.storage.move_all_to_directory(&upload_job_full_path, &final_destination_full_path).await?;

        let mut tx = self.connection.begin().await?;

        let media_status = if media_options.processing_order.options.locked {
            MediaStatus::Locked
        } else {
            if let Some(transform) = media_options.processing_order.post_processing.clone() {
                Self::post_process_media(
                    &self,
                    transform,
                    final_destination,
                    processed_media.clone(),
                    media_options
                ).await?
            } else {
                MediaStatus::Ready
            }
        };

        let media_data_row = MediaDataRow {
            id: processed_media.file_id,
            uploader_id: uploader_id,
            name: processed_media.file_name.clone(),
            status: media_status,
            path: processed_media.path.clone(),
            thumbhash: processed_media.thumbhash.clone(),
            ..Default::default()
        };
        media::create::media_data(&mut tx, &media_data_row).await?;

        let media_metadata_row = media::row::MediaMetadataRow {
            media_id: processed_media.file_id,
            file_size: processed_media.meta.size as i64,
            mime_type: processed_media.meta.mime.clone(),
            width: processed_media.meta.width,
            height: processed_media.meta.height,
            duration: processed_media.meta.duration,
        };
        media::create::media_metadata(&mut tx, &media_metadata_row).await?;

        tx.commit().await?;

        Ok(processed_media)
    }

    /// Saved media as a group process
    /// This function will process a group of media files, applying the specified options and saving them to the storage.
    pub async fn save_media_group(
        &self,
        uploader_id: i64,
        temp_uploaded_files: Vec<TempUpload>,
        media_options: MediaOptions,
    ) -> Result<Vec<ProcessedMedia>, MediaServiceError> {
        // working space for processing files
        let upload_group_job = self.storage.new_temp_relative_path("upload_group_job")?;

        let mut threads = Vec::new();

        // on processing
        let mut saved_medias = Vec::new();
        for files in temp_uploaded_files {
            let thread = self
                .process_media(
                    files.into(),
                    upload_group_job.clone(),
                    media_options.clone(),
                )
                .await?;
            threads.push(thread);
        }

        for thread in threads {
            let media = thread.await??;
            saved_medias.push(media);
        }

        let final_destination = {
            if media_options.processing_order.options.locked {
                format!("{}/{}", LOCKED_UPLOADS_DIR, media_options.folder)
            } else {
                media_options.folder.clone()
            }
        };

        let mut tx = self.connection.begin().await?;

        // save to database
        for media in &saved_medias {
            let media_status = if media_options.processing_order.options.locked {
                MediaStatus::Locked
            } else {
                MediaStatus::Ready
            };

            let media_data_row = MediaDataRow {
                id: media.file_id,
                uploader_id: uploader_id,
                name: media.file_name.clone(),
                status: media_status,
                path: media.path.clone(),
                thumbhash: media.thumbhash.clone(),
                ..Default::default()
            };
            media::create::media_data(&mut tx, &media_data_row).await?;
            
            let media_metadata_row = media::row::MediaMetadataRow {
                media_id: media.file_id,
                file_size: media.meta.size as i64,
                mime_type: media.meta.mime.clone(),
                width: media.meta.width,
                height: media.meta.height,
                duration: media.meta.duration,
            };
            media::create::media_metadata(&mut tx, &media_metadata_row).await?;
        }

        tx.commit().await?;

        tracing::debug!("Saved medias: {:#?}", saved_medias);

        let mut tx = self.connection.begin().await?;

        let post_processing = media_options.processing_order.post_processing.clone();
        for media in saved_medias.iter() {
            if let Some(transforms) = &post_processing {
                let status = self.post_process_media(
                    transforms.clone(),
                    final_destination.clone(),
                    media.clone(),
                    media_options.clone(),
                )
                .await?;

                media::update::media_status(&mut tx, &media.file_id, &status).await?;
            };
        }

        tx.commit().await?;

        self.storage
            .move_file(
                &self.storage.temp_full_path(&upload_group_job)?,
                &self.storage.full_path(&final_destination)?,
            )
            .await?;

        Ok(saved_medias)
    }

    async fn post_process_media(
        &self,
        transforms: Vec<PostProcessingType>,
        final_destination: String,
        media: ProcessedMedia,
        options: MediaOptions,
    ) -> Result<MediaStatus, MediaServiceError> {
        let process_options = options.processing_order.options.clone();

        let job_dir_path = if let Some(post_job_dir) = &media.post_job_dir {
            post_job_dir.clone()
        } else {
            self.storage.new_temp_relative_path("post_processing_job")?
        };

        // match categorize
        match media.category {
            MediaCategory::Video => {
                if process_options.use_video_transcoding
                    && !process_options.locked
                    && !transforms.is_empty()
                {
                    let storage = self.storage.clone();
                    tokio::spawn(async move {
                        if let Err(e) = (async {
                            let source_path = &format!("{}/{}", job_dir_path, media.file_name);
                            for transform in transforms {
                                Transformer::transform_video_post(
                                    job_dir_path.clone(),
                                    transform.clone(),
                                    source_path.clone(),
                                    storage.clone(),
                                )
                                .await?;
                            }
                            // remove source video after processing
                            let _ = storage.delete_temp(&source_path).await;
                            let job_dir_full_path = storage.temp_full_path(&job_dir_path)?;
                            let destination = storage
                                .full_path(&format!("{}/{}", final_destination, media.file_id))?;
                            // move everything from job_dir to final output path
                            storage.move_file(&job_dir_full_path, &destination).await?;
                            // remove the job directory after moving
                            let _ = storage.delete_temp(&job_dir_path).await;
                            Ok::<(), MediaServiceError>(())
                        })
                        .await
                        {
                            // erase the job directory if processing failed
                            let _ = storage.delete_temp(&job_dir_path).await;
                            // log the error but don't crash the service
                            tracing::error!(
                                "Video processing failed for video job at {}. Moving to {}. {:?}",
                                job_dir_path,
                                format!("{}/{}", final_destination, media.file_id),
                                e
                            );
                        }
                    });
                    return Ok(MediaStatus::Processing);
                }
                return Ok(MediaStatus::Ready);
            }
            _ => {
                return Ok(MediaStatus::Ready);
            }
        }
    }

    async fn process_media(
        &self,
        temp_media_object: ProcessObject,
        target_relative_path: String,
        options: MediaOptions,
    ) -> Result<JoinHandle<Result<ProcessedMedia, MediaServiceError>>, MediaServiceError> {
        let object_full_path = self.storage.temp_full_path(&temp_media_object.path)?;

        // ----------------------------
        // * Validation Layer
        // ----------------------------
        // note: validate comes first before processing or generate id to avoid wasting resources on invalid files
        let mut file = File::open(&object_full_path).await?;
        let mut header = [0u8; 8192]; // Read the first 8kib for mime type detection

        let n = file.read(&mut header).await?;
        let (mime, extension) = if let Some(kind) = infer::get(&header[..n]) {
            (kind.mime_type(), kind.extension())
        } else {
            match inspect(&header[..n]) {
                ContentType::UTF_8
                | ContentType::UTF_8_BOM
                | ContentType::UTF_16LE
                | ContentType::UTF_16BE
                | ContentType::UTF_32LE
                | ContentType::UTF_32BE => ("text/plain", "txt"),

                ContentType::BINARY => {
                    return Err(MediaServiceError::InvalidMediaType);
                }
            }
        };

        tracing::debug!(
            "Detected mime type: {} and extension: {} for file: {}",
            mime,
            extension,
            object_full_path.to_string_lossy()
        );

        // strict filtering
        if let Err(e) = super::utils::validate_media_type(mime, &options.validation) {
            tracing::error!("Media validation failed: {:?}", e);
            return Err(e);
        }

        // ----------------------------------
        // * Processing Configuration Layer
        // ----------------------------------
        // note: we can add more processing options here in the future, like GPU acceleration, etc.

        let process_options = options.processing_order.options.clone();
        let file_id = self.snowflake.generate_id()?;

        // Determine the final destination path for the processed file
        let mut file_target_destination = target_relative_path.clone();
        if process_options.use_generated_id_as_container {
            file_target_destination = format!("{}/{}", target_relative_path, file_id);
        }
        self.storage
            .prepare_directory(&self.storage.temp_full_path(&file_target_destination)?)
            .await?;

        let true_final_destination = {
            if process_options.locked {
                if process_options.use_generated_id_as_container {
                    format!("{}/{}/{}", LOCKED_UPLOADS_DIR, options.folder, file_id)
                } else {
                    format!("{}/{}", LOCKED_UPLOADS_DIR, options.folder)
                }
            } else if process_options.use_generated_id_as_container {
                format!("{}/{}", options.folder, file_id)
            } else {
                options.folder
            }
        };

        // We can add more processing options here in the future, like GPU acceleration, etc.
        // important notice, later check if the file use raw name or not to prevent .png.webp

        let category = super::utils::categorize(mime);

        // ----------------------------------
        // * Processing Layer
        // ----------------------------------

        let storage = self.storage.clone();
        let result = tokio::spawn(async move {
            match category {
                MediaCategory::Image => {
                    let (file_name, output_extension, width, height) = {
                        let mut image = if mime == "image/gif" || mime == "image/webp" {
                            VipsImage::new_from_file_access(
                                &format!("{}[n=-1]", &object_full_path.to_string_lossy()),
                                Access::Sequential,
                                false,
                            )?
                        } else {
                            VipsImage::new_from_file_access(
                                &object_full_path.to_string_lossy(),
                                Access::Sequential,
                                false,
                            )?
                        };

                        if let Some(transforms) = &options.processing_order.on_processing {
                            if image.get_n_pages() <= 1 {
                                for transform in transforms {
                                    image = Transformer::transform_image(image, transform.clone())?;
                                }

                                // declare final extension for the processed image, we will use webp as the final format for all images
                                let final_ext: &str = ".webp";
                                let mut file_name = if let Some(raw_name) = &temp_media_object.data
                                {
                                    if process_options.use_raw_name_with_extension
                                        || process_options.use_raw_name
                                    {
                                        raw_name.name.to_string()
                                    } else {
                                        file_id.to_string()
                                    }
                                } else {
                                    file_id.to_string()
                                };
                                // * hash name done later due to libvips memory issue on multi-threads

                                let file_opts = "[strip]";

                                let original_full_path =
                                    storage.temp_full_path(&file_target_destination)?;
                                image.image_write_to_file(&format!(
                                    "{}/{}{}{}",
                                    &original_full_path.to_string_lossy(),
                                    &file_name,
                                    final_ext,
                                    &file_opts
                                ))?;

                                let original_file_full_path = format!(
                                    "{}/{}{}",
                                    &original_full_path.to_string_lossy(),
                                    &file_name,
                                    final_ext
                                );

                                // replace file name with hash if use_hash_as_name is true
                                // this will be use after out of the block to prevent await not send
                                if process_options.use_hash_as_name {
                                    file_name = Self::hash_file(Path::new(&original_file_full_path))?;
                                }

                                (
                                    file_name,
                                    final_ext.to_string(),
                                    image.get_width(),
                                    image.get_height(),
                                )
                            } else {
                                // * Animated Image Processing
                                // * -------------------------
                                let page_height = image.get_page_height();
                                let n_pages = image.get_n_pages();

                                let mut frames = Vec::with_capacity(n_pages as usize);
                                for page in 0..n_pages {
                                    let mut frame = ops::extract_area(
                                        &image,
                                        0,
                                        page * page_height,
                                        image.get_width(),
                                        page_height,
                                    )?;

                                    for transform in transforms {
                                        let new_frame =
                                            Transformer::transform_image(frame, transform.clone())?;
                                        frame = new_frame;
                                    }
                                    frames.push(frame);
                                }

                                let options = ops::ArrayjoinOptions {
                                    across: 1,
                                    shim: 0,
                                    background: vec![0.0],
                                    halign: ops::Align::Low,
                                    valign: ops::Align::Low,
                                    hspacing: frames[0].get_width(),
                                    vspacing: frames[0].get_height(),
                                };

                                let joined = ops::arrayjoin_with_opts(&mut frames, &options)?;
                                let first_frame = &frames[0];

                                // declare final extension
                                let final_ext = ".webp";

                                // generate file name
                                let mut file_name = if let Some(data) = &temp_media_object.data {
                                    if process_options.use_raw_name_with_extension
                                        || process_options.use_raw_name
                                    {
                                        data.name.to_string()
                                    } else {
                                        file_id.to_string()
                                    }
                                } else {
                                    file_id.to_string()
                                };
                                if process_options.use_animated_image_indicator {
                                    file_name = format!("a_{}", file_name);
                                }

                                // generate file options for webp, we will use page-height to preserve the animation
                                let file_opts =
                                    format!("[page-height={},strip]", first_frame.get_height());

                                let full_path = storage.temp_full_path(&file_target_destination)?;

                                // save as .webp with page-height option to preserve the animation
                                joined.image_write_to_file(&format!(
                                    "{}{}",
                                    full_path
                                        .join(format!("{}{}", file_name, final_ext))
                                        .to_string_lossy(),
                                    file_opts
                                ))?;

                                // generate hash name if use_hash_as_name is true
                                // also pass final filename to file_target_destination for storage
                                if process_options.use_hash_as_name {
                                    file_name = Self::hash_file(Path::new(&format!(
                                        "{}/{}{}",
                                        full_path.to_string_lossy(),
                                        file_name,
                                        final_ext
                                    )))?;
                                    if process_options.use_animated_image_indicator {
                                        file_name = format!("a_{}", file_name);
                                    }
                                }

                                if process_options.use_animated_image_indicator {
                                    let preview_full_path = storage.temp_full_path(&format!(
                                        "{}/{}{}",
                                        file_target_destination, file_name, ".png"
                                    ))?;
                                    let preview_path = Path::new(&preview_full_path);
                                    first_frame
                                        .image_write_to_file(&preview_path.to_string_lossy())?;
                                }

                                (
                                    file_name,
                                    final_ext.to_string(),
                                    first_frame.get_width(),
                                    first_frame.get_height(),
                                )
                            }
                        } else {
                            // ----------------------------------
                            // * Fallback for Raw Image Uploads
                            // ----------------------------------

                            // no processing, just save the original image
                            let final_ext = format!(".{}", extension);
                            let file_name = if let Some(raw_name) = &temp_media_object.data {
                                if process_options.use_raw_name_with_extension
                                    || process_options.use_raw_name
                                {
                                    raw_name.name.to_string()
                                } else {
                                    file_id.to_string()
                                }
                            } else if process_options.use_hash_as_name {
                                Self::hash_file(&object_full_path)?
                            } else {
                                file_id.to_string()
                            };

                            let original_file_path =
                                format!("{}/{}{}", file_target_destination, file_name, final_ext);
                            let original_file_full_path =
                                storage.temp_full_path(&original_file_path)?;
                            image
                                .image_write_to_file(&original_file_full_path.to_string_lossy())?;

                            (file_name, final_ext, image.get_width(), image.get_height())
                        }
                    };

                    // re validate file name
                    let original_processed_file_name =
                        if let Some(raw_name) = &temp_media_object.data {
                            if process_options.use_raw_name_with_extension
                                || process_options.use_raw_name
                            {
                                raw_name.name.to_string()
                            } else {
                                file_id.to_string()
                            }
                        } else {
                            file_id.to_string()
                        };

                    // post process for original file
                    // file name will return hash name if this option turns on, otherwise it will return the original generated file name
                    let final_file_name = if process_options.use_hash_as_name {
                        let animated_file_name = if process_options.use_animated_image_indicator
                            && file_name.starts_with("a_")
                        {
                            format!("a_{}", original_processed_file_name)
                        } else {
                            original_processed_file_name
                        };
                        let original_file_path = format!(
                            "{}/{}{}",
                            file_target_destination, animated_file_name, output_extension
                        );

                        let original_file_full_path =
                            storage.temp_full_path(&original_file_path)?;
                        let hashed_name = file_name.clone(); // file_name is already hashed if use_hash_as_name is true
                        let new_file_path = format!(
                            "{}/{}{}",
                            file_target_destination, hashed_name, output_extension
                        );
                        let new_file_full_path = storage.temp_full_path(&new_file_path)?;

                        storage
                            .move_file(&original_file_full_path, &new_file_full_path)
                            .await?;

                        //update a complete final destination path for the processed file with extension
                        file_target_destination = format!(
                            "{}/{}{}",
                            file_target_destination, file_name, output_extension
                        );

                        hashed_name
                    } else {
                        // update a complete final destination path for the processed file with extension
                        file_target_destination = format!(
                            "{}/{}{}",
                            file_target_destination, original_processed_file_name, output_extension
                        );
                        file_name
                    };

                    // thumbhash generation moved here to prevent libvips memory issue on multi-threads
                    let thumbhash = if process_options.use_thumbhash_generation {
                        let processed_file_full_path =
                            storage.temp_full_path(&file_target_destination)?;
                        let mut image = if mime == "image/gif" || mime == "image/webp" {
                            VipsImage::new_from_file(&format!(
                                "{}[n=-1]",
                                &processed_file_full_path.to_string_lossy()
                            ))?
                        } else {
                            VipsImage::new_from_file(&processed_file_full_path.to_string_lossy())?
                        };

                        // check if image is animated, if so, extract the first frame for thumbhash generation
                        if image.get_n_pages() > 1 {
                            image = ops::extract_area(
                                &image,
                                0,
                                0,
                                image.get_width(),
                                image.get_page_height(),
                            )?;
                        }

                        let downscaled = Transformer::transform_image(
                            image,
                            OnProcessingType::ImageResize {
                                style: ResizeStyle::AbsoluteKeepsRatio {
                                    width: 100,
                                    height: 100,
                                },
                                upscale: false,
                            },
                        )?;
                        Some(Self::generate_thumbhash_from_image(&downscaled))
                    } else {
                        None
                    };

                    let _ = storage.delete_temp(&temp_media_object.path).await;

                    tracing::debug!(
                        "Processed image saved to: {}, thumbhash: {:?}, width: {}, height: {}",
                        file_target_destination,
                        thumbhash,
                        width,
                        height
                    );

                    let file_full_path = storage.temp_full_path(&file_target_destination)?;
                    let mut header = [0u8; 8192];
                    let mut file = File::open(&file_full_path).await?;
                    let metadata = file.metadata().await?;
                    let final_size = metadata.len() as usize;
                    let n = file.read(&mut header).await?;
                    let mime = if let Some(kind) = infer::get(&header[..n]) {
                        kind.mime_type().to_string()
                    } else {
                        "image/webp".to_string() // fallback to webp if unable to detect
                    };

                    // re read the file to get the final size after processing
                    let processed_media = ProcessedMedia {
                        file_id,
                        path: format!("{}/{}{}", true_final_destination, final_file_name, output_extension),
                        file_name: final_file_name,
                        category,
                        thumbhash,
                        meta: MediaMeta {
                            size: final_size,
                            mime: mime,
                            width: Some(width),
                            height: Some(height),
                            duration: None,
                        },
                        post_job_dir: None,
                    };

                    return Ok(processed_media);
                }
                MediaCategory::Video => {
                    // create job directory for video processing
                    let job_dir_path = storage.new_temp_relative_path("video_processing_job")?;

                    let duration =
                        processor::video::get_video_duration(&object_full_path.to_string_lossy())
                            .await?;

                    let thumbnail_raw = processor::video::generate_video_thumbnail(
                        &temp_media_object.path,
                        "png",
                        1, // generate thumbnail at 1 second
                        storage.clone(),
                    )
                    .await?;

                    let thumbhash = if process_options.use_thumbhash_generation {
                        let thumbnail = VipsImage::new_from_buffer(&thumbnail_raw, "")?;
                        // image retain aspect ratio and fit within 100x100
                        let downscaled = Transformer::transform_image(
                            thumbnail,
                            OnProcessingType::ImageResize {
                                style: ResizeStyle::AbsoluteKeepsRatio {
                                    width: 100,
                                    height: 100,
                                },
                                upscale: false,
                            },
                        )?;
                        Some(Self::generate_thumbhash_from_image(&downscaled))
                    } else {
                        None
                    };

                    tracing::debug!(
                        "Generated video thumbhash: {}",
                        thumbhash.clone().unwrap_or("None".to_string())
                    );

                    // prepare for video post processing if needed
                    if process_options.use_video_transcoding
                        && options.processing_order.on_processing.is_some()
                        && !process_options.locked
                    {
                        // copy the original video to the job directory
                        let source_path = format!("{}/source.{}", job_dir_path, extension);
                        let source_full_path = storage.temp_full_path(&source_path)?;

                        storage
                            .move_file(&Path::new(&object_full_path), &Path::new(&source_full_path))
                            .await?;

                        // save the thumbnail to the job directory
                        // this help serving the thumbnail faster without having to generate it again
                        storage
                            .save_temp(
                                &format!("{}/t_{}.png", file_target_destination, file_id),
                                &thumbnail_raw,
                            )
                            .await?;

                        let processed_video = ProcessedMedia {
                            file_id,
                            path: format!("{}/{}/", true_final_destination, file_id),
                            file_name: format!("source.{}", extension), // use source as file name for post processing purposes.
                            category,
                            thumbhash: thumbhash,
                            meta: MediaMeta {
                                size: temp_media_object.size,
                                mime: "application/vnd.apple.mpegurl".to_string(), // m3u8 is the final format for video processing
                                width: None,
                                height: None,
                                duration: Some(duration.max(0.0).min(f32::MAX).round()),
                            },
                            post_job_dir: Some(job_dir_path),
                        };

                        return Ok(processed_video);
                    }

                    // ----------------------------------
                    // * Fallback for Raw Video Uploads
                    // ----------------------------------

                    let extension = if extension.is_empty() {
                        "mp4"
                    } else if process_options.use_raw_name_with_extension {
                        if let Some(raw_name) = &temp_media_object.data {
                            raw_name.extension.as_str()
                        } else {
                            extension
                        }
                    } else {
                        extension
                    };

                    // perserve the original video file if no processing is needed
                    // prioritize raw name and extension if available
                    // no hashing for video due to the file size can be very large and hashing can be expensive                    
                    let file_name = if process_options.use_raw_name_with_extension
                        || process_options.use_raw_name
                    {
                        if let Some(raw_name) = &temp_media_object.data {
                            format!("{}{}", raw_name.name, format!(".{}", raw_name.extension))
                        } else {
                            format!("{}{}", file_id, format!(".{}", extension))
                        }
                    } else {
                        format!("{}{}", file_id, format!(".{}", extension))
                    };

                    let relative_path = format!("{}/{}", file_target_destination, file_name);

                    let final_full_path = storage.temp_full_path(&relative_path)?;
                    storage
                        .move_file(&object_full_path, &final_full_path)
                        .await?;

                    let processed_video = ProcessedMedia {
                        file_id,
                        path: format!("{}/{}", true_final_destination, file_name),
                        file_name: file_name,
                        category,
                        thumbhash: thumbhash,
                        meta: MediaMeta {
                            size: temp_media_object.size,
                            mime: mime.to_string(),
                            width: None,
                            height: None,
                            duration: Some(duration.max(0.0).min(f32::MAX).round()),
                        },
                        post_job_dir: None,
                    };

                    return Ok(processed_video);
                }

                _ => {
                    // * fallback for audio, document, archive, etc.

                    // pioritize raw name if available, otherwise use hash as name, otherwise use generated id
                    // no hashing for audio, document, archive, etc. due to the file size can be very large and hashing can be expensive
                    let filename = if process_options.use_raw_name_with_extension
                        || process_options.use_raw_name
                    {
                        if let Some(raw_name) = &temp_media_object.data {
                            format!("{}{}", raw_name.name, format!(".{}", raw_name.extension))
                        } else {
                            format!("{}{}", file_id, format!(".{}", extension))
                        }
                    } else {
                        format!("{}{}", file_id, format!(".{}", extension))
                    };

                    let relative_path = format!("{}/{}", file_target_destination, filename);
                    let final_full_path = storage.temp_full_path(&relative_path)?;

                    storage
                        .move_file(&object_full_path, &final_full_path)
                        .await?;

                    let processed_media = ProcessedMedia {
                        file_id,
                        path: format!("{}/{}", true_final_destination, filename),
                        file_name: filename,
                        thumbhash: None,
                        category,
                        meta: MediaMeta {
                            size: temp_media_object.size,
                            mime: mime.to_string(),
                            width: None,
                            height: None,
                            duration: None,
                        },
                        post_job_dir: None,
                    };

                    return Ok(processed_media);
                }
            }
        });

        Ok(result)
    }
}
