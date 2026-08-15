use std::sync::Arc;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rs_vips::{
    VipsImage,
    voption::{Setter, VOption},
};
use sqlx::{Pool, Postgres};
use tokio::task::JoinHandle;

use crate::application::{
    repository::media::{
        self,
        row::{MediaDataRow, MediaStatus},
    }, service::{
        errors::MediaServiceError, media::{
            processor::{
                image::ImageProcessor, types::{MediaProcessorFFlags, PostProcessingType, VideoPostProcessorType}, video::VideoProcessor,
            }, service_type::{Container, MediaServiceOptions}, storage::MediaStorage, types::{file::MultipartFile, media_options::MediaCategory},
        },
    }, state::AppState,
};

pub struct MediaService;

impl MediaService {
    pub fn new() -> Self {
        Self {}
    }

    fn generate_thumbhash_from_image(image: &VipsImage) -> String {
        // resize
        let thumb = image.thumbnail_image(100).unwrap();
        let rgba_thumb = if thumb.get_bands() == 3 {
            thumb.bandjoin_const(&[255.0]).unwrap()
        } else {
            thumb
        };

        let width = rgba_thumb.get_width();
        let height = rgba_thumb.get_height();
        let raw_bytes = rgba_thumb.write_to_memory();

        let thumbhash = thumbhash::rgba_to_thumb_hash(width as usize, height as usize, &raw_bytes);
        URL_SAFE_NO_PAD.encode(&thumbhash)
    }
    // PUBLICS

    pub async fn save_media(
        self,
        state: &AppState,
        file: MultipartFile,
        options: MediaServiceOptions,
    ) -> Result<MultipartFile, MediaServiceError> {
        let storage = state.storage.clone();
        let upload_job = storage.new_temp_relative_path("upload_job");

        let thread = self
            .process_media(state, file.clone(), upload_job.clone(), options.clone())
            .await?
            .await?;

        // try catch the result of the thread
        let processed_media = match thread {
            Ok(media) => media,
            Err(e) => {
                storage.delete_temp(&upload_job).await;
                storage.delete_temp(&file.get_relative_path()).await;
                tracing::error!("Error processing media: {:?}", e);
                return Err(e);
            }
        };

        // move file within storage to final destination
        state
            .storage
            .upload(&upload_job, &processed_media.get_destination())
            .await?;

        let mut tx = state.db_pool.begin().await?;

        let processor_options = options.processor.unwrap_or_default();
        let media_status = {
            if let Some(post_processors) = processor_options.post_processors.clone() {
                // use old source path
                Self::post_process_media(
                    &self,
                    state,
                    processed_media.clone(),
                    post_processors.clone(),
                    processor_options.fflags,
                )
                .await?
            } else {
                MediaStatus::Ready
            }
        };

        let media_data_row = MediaDataRow {
            id: processed_media.get_id(),
            uploader_id: options.uploader_id,
            name: processed_media.get_full_name().clone(),
            status: media_status,
            path: processed_media.get_relative_destination(),
            thumbhash: processed_media.get_thumbhash().cloned(),
            ..Default::default()
        };
        media::create::media_data(&mut tx, &media_data_row).await?;

        let meta = processed_media.get_extra_meta()?;

        let media_metadata_row = media::row::MediaMetadataRow {
            media_id: processed_media.get_id(),
            file_size: processed_media.get_size() as i64,
            mime_type: processed_media.get_mime().to_string(),
            width: meta.width,
            height: meta.height,
            duration: meta.duration,
        };
        media::create::media_metadata(&mut tx, &media_metadata_row).await?;

        tx.commit().await?;

        Ok(processed_media)
    }

    /// Saved media as a group process
    /// This function will process a group of media files, applying the specified options and saving them to the storage.
    pub async fn save_media_group(
        &self,
        state: &AppState,
        files: Vec<MultipartFile>,
        options: MediaServiceOptions,
    ) -> Result<Vec<MultipartFile>, MediaServiceError> {
        // working space for processing files
        let upload_group_job = state.storage.new_temp_relative_path("upload_group_job");

        let mut threads = Vec::new();

        // on processing
        let mut saved_medias = Vec::new();
        for file in files {
            let thread = self
                .process_media(&state, file, upload_group_job.clone(), options.clone())
                .await?;
            threads.push(thread);
        }

        for thread in threads {
            let file = thread.await??;
            saved_medias.push(file);
        }

        // Early moving files to final
        state
            .storage
            .move_file(
                &state.storage.temp_full_path(&upload_group_job),
                &state.storage.full_path(&options.upload_route)?,
            )
            .await?;

        tracing::debug!("Processed media files: {:#?}", saved_medias);

        let mut tx = state.db_pool.begin().await?;

        let mut final_medias = Vec::new();
        // save to database
        for media in &saved_medias {
            let media_data_row = MediaDataRow {
                id: media.get_id(),
                uploader_id: options.uploader_id,
                name: media.get_full_name(),
                status: MediaStatus::Pending,
                path: media.get_relative_destination(),
                thumbhash: media.get_thumbhash().cloned(),
                ..Default::default()
            };
            media::create::media_data(&mut tx, &media_data_row).await?;

            // not move yet
            let meta = media.get_extra_meta()?;

            let media_metadata_row = media::row::MediaMetadataRow {
                media_id: media.get_id(),
                file_size: media.get_size() as i64,
                mime_type: media.get_mime().to_string(),
                width: meta.width,
                height: meta.height,
                duration: meta.duration,
            };
            media::create::media_metadata(&mut tx, &media_metadata_row).await?;
            final_medias.push(media.clone());
        }

        tx.commit().await?;

        tracing::debug!("Saved medias: {:#?}", final_medias);

        let mut tx = state.db_pool.begin().await?;

        let processor_options = options.processor.unwrap_or_default();

        for media in saved_medias.iter() {
            if let Some(post_processors) = &processor_options.post_processors {
                let status = self
                    .post_process_media(
                        state,
                        media.clone(),
                        post_processors.clone(),
                        processor_options.fflags.clone(),
                    )
                    .await?;
                    media::update::media_status(&mut tx, &media.get_id(), &status).await?;
            };
        }

        tx.commit().await?;

        Ok(final_medias)
    }

    async fn post_process_media(
        &self,
        state: &AppState,
        proc_file: MultipartFile,
        post_processors: PostProcessingType,
        fflags: Option<MediaProcessorFFlags>,
    ) -> Result<MediaStatus, MediaServiceError> {
        tracing::debug!(
            "Post-processing media: {:#?} with post_processors: {:#?} and fflags: {:#?}",
            &proc_file,
            post_processors,
            fflags
        );

        let process_job = proc_file.get_job().clone();

        if process_job.get_dir().is_none() {
            return Ok(MediaStatus::Ready);
        }

        let fflags = fflags.unwrap_or_default();

        // * Keep the category filter exactly as-is.
        if proc_file.get_category() != MediaCategory::Video {
            return Ok(MediaStatus::Ready);
        }

        let Some(video_processes) = 
            post_processors
            .get_video_post_processors()
            .cloned()
        else {
            return Ok(MediaStatus::Ready);
        };

        if video_processes.is_empty() || !fflags.video_transcode {
            return Ok(MediaStatus::Ready);
        }

        let job_dir_relative = process_job
            .get_relative_dir();
        if job_dir_relative.is_empty() {
            return Err(MediaServiceError::InternalServer);
        }

        let source_relative = process_job
            .get_source_relative_path();
        if source_relative.is_empty() {
            return Err(MediaServiceError::InternalServer);
        }

        let storage = state.storage.clone();
        let connection = state.db_pool.clone();

        tokio::spawn(async move {
            Self::run_video_post_processing(
                storage,
                connection,
                proc_file,
                video_processes.clone(),
                fflags,
                job_dir_relative,
                source_relative,
            )
            .await;
        });

        Ok(MediaStatus::Processing)
    }

    async fn run_video_post_processing(
        storage: Arc<dyn MediaStorage>,
        connection: Pool<Postgres>,
        file: MultipartFile,
        video_processes: Vec<VideoPostProcessorType>,
        fflags: MediaProcessorFFlags,
        job_dir_relative: String,
        source_relative: String,
    ) {
        if let Err(e) = async {
            let video_processor = VideoProcessor::new(fflags.video_gpu_accel);
            tracing::debug!(
                "Started video post-processing for media ID: {} with processes: {:#?}",
                file.get_id(),
                video_processes
            );

            for process in video_processes {
                video_processor.run_post(&file, process).await?;
            }

            tracing::debug!(
                "Completed video post-processing for media ID: {}",
                file.get_id()
            );

            // remove source video after processing
            storage.delete_temp(&source_relative).await;

            // move everything from job_dir to final output path
            storage
                .upload(&job_dir_relative, &file.get_destination())
                .await?;

            storage.delete_temp(&job_dir_relative).await;

            let mut tx = connection.begin().await?;

            media::update::media_status(&mut tx, &file.get_id(), &MediaStatus::Completed).await?;

            let _ = tx.commit().await;

            Ok::<(), MediaServiceError>(())
        }
        .await
        {
            // erase the job directory if processing failed
            let _ = storage.delete_temp(&job_dir_relative).await;

            tracing::error!(
                "Video processing failed for video job at {}. Moving to {}. {:?}",
                storage
                    .temp_full_path(&job_dir_relative)
                    .to_string_lossy()
                    .to_string(),
                format!("{}/{}", file.get_destination(), file.get_id()),
                e
            );

            if let Ok(mut tx) = connection.begin().await {
                let _ = media::update::media_status(&mut tx, &file.get_id(), &MediaStatus::Failed)
                    .await;

                let _ = tx.commit().await;
            }
        }
    }

    async fn process_media(
        &self,
        state: &AppState,
        uploaded_file: MultipartFile,
        container_path: String,
        options: MediaServiceOptions,
    ) -> Result<JoinHandle<Result<MultipartFile, MediaServiceError>>, MediaServiceError> {
        let file_id = state.snowflake_generator.generate_id()?;
        let has_process = options.processor.is_some();
        let container_conf = if let Some(container) = options.container.clone() {
            container
        } else {
            Container::default()
        };

        let mut uploaded_file = uploaded_file;

        uploaded_file.set_id(file_id);

        // Determine the file name based on the container configuration and processing options
        if container_conf.use_hash_names && has_process {
            let hash = uploaded_file.get_hash()?;
            uploaded_file.rename(&hash.to_string())?;
        } else if container_conf.use_raw_names && !has_process {
            let name = uploaded_file.get_name().clone();
            uploaded_file.rename(&name)?;
        } else {
            uploaded_file.rename(&file_id.to_string())?;
        }

        tracing::debug!(
            "Renamed uploaded file: {:#?} with file_id: {} and container_path: {}",
            uploaded_file, file_id, container_path
        );

        // Determine the final destination path for the processed file
        let container_path = if container_conf.use_file_id_sub_container {
            format!("{}/{}", container_path, file_id)
        } else {
            container_path
        };

        let true_path = if container_conf.use_file_id_sub_container {
            format!("{}/{}/", options.upload_route, file_id)
        } else {
            format!("{}", options.upload_route)
        };

        uploaded_file.set_destination(true_path.clone());

        state
            .storage
            .prepare_directory(&state.storage.temp_full_path(&container_path))
            .await?;

        tracing::debug!(
            "Processing media file: {:#?} to destination: {}",
            uploaded_file,
            true_path
        );

        // ----------------------------------
        // * Processing Layer
        // ----------------------------------

        let storage = state.storage.clone();
        let result = tokio::spawn(async move {
            let processed_file = match uploaded_file.get_category() {
                MediaCategory::Image => {
                    let processor_options = options.processor.unwrap_or_default();

                    if processor_options.image_processors.is_none() {
                        let name = uploaded_file.get_full_name();
                        let relative_path = format!("{}/{}", container_path, name);

                        let full_path = storage.temp_full_path(&relative_path);

                        uploaded_file.move_to_path(&full_path, relative_path)?;

                        let dst_rel_path = uploaded_file.build_relative_file_destination();

                        // replace path
                        uploaded_file.replace(
                            storage.full_path(&dst_rel_path)?,
                            dst_rel_path,
                        ).await?;

                        uploaded_file
                    } else {
                        let image = {
                            let image = {
                                if uploaded_file.get_mime() == "image/gif"
                                    || uploaded_file.get_mime() == "image/webp"
                                {
                                    let opts = VOption::new().set("n", -1);
                                    VipsImage::new_from_file_with_opts(
                                        &uploaded_file.get_full_path(),
                                        opts,
                                    )?
                                } else {
                                    VipsImage::new_from_file(&uploaded_file.get_full_path())?
                                }
                            };

                            let is_animated = image.get_n_pages() > 1;

                            let mut processor = ImageProcessor::new();

                            let image = ImageProcessor::transform(
                                &mut processor,
                                image,
                                processor_options.image_processors,
                                is_animated,
                            )?;

                            image
                        };

                        let opts = VOption::new().set("strip", true);

                        uploaded_file.set_extension("webp".to_string());
                        let name = uploaded_file.get_full_name();
                        let relative_path = format!("{}/{}", container_path, name);
                        let full_path = storage
                            .temp_full_path(&relative_path)
                            .to_string_lossy()
                            .to_string();

                        tracing::debug!(
                            "Writing processed image to {} with option {}",
                            full_path,
                            ""
                        );

                        image.write_to_file_with_opts(&full_path, opts)?;

                        tracing::debug!(
                            "Processed image written to {} with option {}",
                            full_path,
                            ""
                        );

                        uploaded_file
                            .del_replace(
                                &storage,
                                storage.temp_full_path(&relative_path),
                                relative_path,
                            )
                            .await?;

                        uploaded_file.revalidate(&storage)?;
                        uploaded_file.set_extension("webp".to_string());

                        tracing::debug!(
                            "Processed image revalidated: {:#?}",
                            uploaded_file
                        );

                        let use_thumbhash = container_conf.generate_thumbhash;
                        if use_thumbhash {
                            let image = VipsImage::new_from_file(&uploaded_file.get_full_path())?;

                            let thumbnail = image.thumbnail_image(100)?;

                            let thumbhash = Self::generate_thumbhash_from_image(&thumbnail);
                            uploaded_file.set_thumbhash(thumbhash);
                        }

                        let dst_rel_path = uploaded_file.build_relative_file_destination();

                        // replace path
                        uploaded_file.replace(
                            storage.full_path(&dst_rel_path)?,
                            dst_rel_path,
                        ).await?;

                        uploaded_file
                    }
                }

                MediaCategory::Video => {
                    let process_options = options.processor.unwrap_or_default();

                    tracing::debug!("Processing video file with options: {:#?}", process_options);

                    if process_options.video_processors.is_none()
                        && process_options.post_processors.is_none()
                    {
                        uploaded_file
                    } else {
                        // TODO: implement fast video processing here

                        let video_post_processes = process_options
                            .post_processors
                            .as_ref()
                            .and_then(|p| p.get_video_post_processors());

                        tracing::debug!(
                            "Video has post-processing tasks"
                        );

                        let video_processor = VideoProcessor::new(false);
                        let thumbnail_raw = video_processor
                            .get_thumbnail(&uploaded_file, 1, "png")
                            .await?;

                        tracing::debug!(
                            "Generated thumbnail for video id: {} with size: {} bytes",
                            file_id,
                            thumbnail_raw.len()
                        );

                        let image = VipsImage::new_from_buffer(&thumbnail_raw, "")?;

                        let fflags = process_options.fflags.unwrap();


                        let use_thumbhash = container_conf.generate_thumbhash;

                        if use_thumbhash {
                            let thumbnail = image.thumbnail_image(100)?;

                            let thumbhash = Self::generate_thumbhash_from_image(&thumbnail);
                            uploaded_file.set_thumbhash(thumbhash);
                        }

                        uploaded_file.revalidate(&storage)?;

                        if video_post_processes.is_some() && fflags.video_transcode {
                            let job_dir_path =
                                storage.new_temp_relative_path("video_processing_job");

                            // update the final destination to grup using id
                            uploaded_file.set_destination(format!(
                                "{}/{}/",
                                uploaded_file.get_destination(),
                                file_id
                            ));

                            // update container path to grup using id
                            let container_path = format!("{}/{}", container_path, file_id);
                            storage.prepare_directory(&storage.temp_full_path(&container_path)).await?;

                            uploaded_file.set_job_dir(
                                &storage.temp_full_path(&job_dir_path),
                                &job_dir_path
                            );

                            if fflags.video_thumbnail {
                                let thumbnail = image.thumbnail_image(1920)?;

                                thumbnail.write_to_file(&format!(
                                    "{}.png",
                                    storage
                                        .temp_full_path(&format!(
                                            "{}/t_{}",
                                            container_path,
                                            uploaded_file.get_name()
                                        ))
                                        .to_string_lossy()
                                ))?;
                            }

                            let source_path =
                                format!("{}/{}", job_dir_path, uploaded_file.get_full_name());

                            let source_full_path = storage.temp_full_path(&source_path);

                            tracing::debug!(
                                "Moving video file to job directory: {}",
                                source_full_path.to_string_lossy()
                            );

                            storage
                                .move_file(&uploaded_file.get_path(), &source_full_path)
                                .await?;

                            // set source path to the uploaded file
                            uploaded_file.set_job_source_file(&source_full_path, &source_path);

                            tracing::debug!(
                                "Video file moved to job directory: {:#?}",
                                uploaded_file.get_job()
                            );

                            // change the path of the uploaded file to the destination as folder
                            uploaded_file.replace(
                                storage.full_path(&uploaded_file.get_destination())?,
                                uploaded_file.get_destination().to_string(),
                            ).await?;

                            uploaded_file
                                .set_job_dir(&storage.temp_full_path(&job_dir_path), &job_dir_path);
                        }

                        uploaded_file
                    }
                }

                _ => {
                    let name = uploaded_file.get_full_name();
                    let relative_path = format!("{}/{}", container_path, name);

                    let full_path = storage.temp_full_path(&relative_path);

                    uploaded_file.move_to_path(&full_path, relative_path)?;

                    let dst_rel_path = uploaded_file.build_relative_file_destination();

                    // replace path
                    uploaded_file.replace(
                        storage.full_path(&dst_rel_path)?,
                        dst_rel_path,
                    ).await?;

                    uploaded_file
                },
            };

            return Ok(processed_file);
        });

        Ok(result)
    }
}
