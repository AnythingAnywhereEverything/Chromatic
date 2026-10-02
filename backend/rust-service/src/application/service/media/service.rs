use std::{path::PathBuf, sync::Arc};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rs_vips::{
    VipsImage,
    voption::{Setter, VOption},
};
use sqlx::{Pool, Postgres};
use tokio::task::JoinHandle;

use crate::application::{
    repository::media::{
        self, row::{
            MediaKind, MediaObjectMetadataRow, MediaObjectsRow, MediaRow, MediaType,
            PostProcessingState, ProcessingState,
        },
    }, service::{
        errors::MediaServiceError,
        media::{
            inspector,
            model::{
                File, FileContainer,
                container::{ContainerConfig, NamingStrategy},
            },
            processor::{
                image::ImageProcessor, types::VideoPostProcessorType, video::VideoProcessor,
            },
            storage::{PersistentStore, TempStore},
        },
        snowflake_service::SnowflakeGenerator,
    }, state::AppState,
};

pub struct MediaService {
    snowflake: SnowflakeGenerator,
    temporary_store: Arc<dyn TempStore>,
    persistent_store: Arc<dyn PersistentStore>,
}

pub struct SaveOptions {
    pub container: Arc<FileContainer>,
    pub temp_store: Arc<dyn TempStore>,
}

#[derive(Debug)]
pub struct ProcessResponse {
    pub processed_file: File,
    pub file_to_insert: Vec<File>,
    pub post_container: Option<FileContainer>,
}

fn generate_thumbhash_from_file(path: PathBuf) -> String {
    let image = VipsImage::new_from_file(&path).unwrap();
    generate_thumbhash_from_image(image)
}

fn generate_thumbhash_from_image(image: VipsImage) -> String {
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

impl MediaService {
    pub fn new(
        snowflake: SnowflakeGenerator,
        temporary_store: Arc<dyn TempStore>,
        persistent_store: Arc<dyn PersistentStore>,
    ) -> Self {
        Self {
            snowflake,
            temporary_store,
            persistent_store,
        }
    }

    pub async fn save_media(
        &self,
        state: &AppState,
        container: &mut FileContainer,
    ) -> Result<(), MediaServiceError> {
        // Check if the container is empty
        if container.is_empty() {
            return Err(MediaServiceError::NoFilesInContainer);
        }

        // Extract uploader ID and target path from the container
        let uploader = container
            .uploader_id()
            .ok_or(MediaServiceError::UploaderIdNotSet)?;
        // Load all config from the container
        let config = container.config();
        let target_path = container.target_path()?.clone();
        if let Some(config) = &config {
            if config.is_file_id_contained() {
                container.file_id_contained()?;
            }
        }

        // Take all files out of the container for processing
        let files = container.take_files();
        let mut tasks = Vec::with_capacity(files.len());

        // Process each file in parallel using asynchronous tasks
        for file in files {
            let config = config.clone();
            let ts = self.temporary_store.clone();
            tasks.push(self.process_file(file, config, ts).await?);
        }

        // Create vectors to hold new files and post-processing containers
        let mut new_files = Vec::new();
        let mut ppc = Vec::new();

        // Await all processing tasks and collect results
        for task in tasks {
            // wait for each task to finish
            let res = task.await??;
            let file = res.processed_file;
            let file_to_insert = res.file_to_insert;
            let post_container = res.post_container;

            new_files.push(file);
            for file in file_to_insert {
                new_files.push(file);
            }
            if let Some(mut post_container) = post_container {
                post_container.set_target_path(target_path.clone());
                ppc.push(post_container);
            }
        }
        // Replace the files in the container with the newly processed files
        // ensure that the container now holds the newly processed files
        container.replace_files(new_files);

        // At this point, the container has been updated with the newly processed files
        // Now resolve the files from the container to prepare for database insertion
        let resolved = container.resolve_files();
        let mut tx = state.db_pool.begin().await?;

        // Loop through each resolved file and save it to the database
        for mut resolved_files in resolved {
            // check if media file id is within the post-processing container
            let is_post_processing = ppc.iter().any(|c| {
                c.files(0)
                    .map_or(false, |f| f.id() == Some(resolved_files.id))
            });

            // check if name exists and not duplicate
            for file in &resolved_files.files {
                // * This use in a very specific case where naming can dedupe itself such as hash-based file names
                if let Some(check_conflict_type) = &resolved_files.check_conflict_type {
                    if let Some(conflict_id) = media::check::conflict_recent_media(&mut tx, &uploader, &file.file_name_with_extension(), check_conflict_type.clone()).await? {
                        // update related id in container
                        for file in container.files_mut() {
                            if file.id() == Some(resolved_files.id) {
                                file.set_id(conflict_id);
                            }
                        }
                        
                        resolved_files.id = conflict_id;
                        tracing::info!("Conflict detected for resolved file: {:#?}", resolved_files);
                        container.abort_retain(self.temporary_store.clone()).await?;

                        // ! hardcode specific usecase.
                        return Ok(());
                    }
                }
            }

            // Append the resolved file information to the media table
            // Media Row
            let media_row = MediaRow {
                id: resolved_files.id,
                uploader_id: uploader,
                flags: resolved_files.flags,
                file_type: resolved_files.file_type,
                processing_state: ProcessingState::Pending,
                post_processing_state: if is_post_processing {
                    PostProcessingState::Idle
                } else {
                    PostProcessingState::Completed
                },
                original_name: resolved_files.original_name,
                original_content_type: resolved_files.original_content_type,
                ..Default::default()
            };
            media::create::media_create(&mut tx, &media_row).await?;

            // Media Object Metadata Row
            let meta = resolved_files.meta.as_ref().unwrap();
            let media_metadata_row = MediaObjectMetadataRow {
                width: meta.width.map(|w| w as i32),
                height: meta.height.map(|h| h as i32),
                duration: meta.duration.map(|d| d as f64),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            };
            media::create::media_object_metadata_create(&mut tx, media_row.id, &media_metadata_row).await?;

            // Loop through each resolved file and create media object rows
            for file in resolved_files.files {
                // Skip deleted files
                if file.is_deleted() {
                    continue;
                }
                // Generate the storage key for the file based on its directory and the target path
                let storage_key = file
                    .file_directory()
                    .replace(&container.relative_path(), &target_path);

                let media_objects_row = MediaObjectsRow {
                    kind: file.kind().clone(),
                    storage_key: storage_key,
                    content_type: file.content_type().to_string(),
                    size: file.size() as i64,
                    name: file.file_name_with_extension(),
                    thumbhash: file.placeholder().map(|t| t.to_string()),
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: chrono::Utc::now().naive_utc(),
                    deleted_at: None,
                };
                media::create::media_object_create(&mut tx, media_row.id, &media_objects_row).await?;
            }
        }

        // Commit early to ensure no conflict with post-processing tasks
        tx.commit().await?;

        // Finalize the container after all media objects have been created
        // And upload to the destination.
        container
            .finalize(self.persistent_store.clone(), self.temporary_store.clone())
            .await?;

        // Spawn post-processing tasks for each post-processing container
        let mut tx = state.db_pool.begin().await?;
        for mut post_container in ppc {
            post_container.set_target_path(target_path.clone());

            let ts = self.temporary_store.clone();
            let ps = self.persistent_store.clone();
            let db = state.db_pool.clone();

            // ! Fire and forget the post-processing task for the container
            // ! This requires further implementation on queuing for it to be completely safe
            // ! But for now, we will stick with it.
            tokio::spawn(async move {
                let _ = MediaService::post_process_containment(db, post_container, ts, ps).await;
            });
        }

        // Set the remaining files in the container as ready for further processing or usage
        for file in container.files_mut() {
            media::update::processing_state(&mut tx, &file.id().unwrap(), &ProcessingState::Ready)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    // Everything was already ensure that is a post processing container
    async fn post_process_containment(
        pool: Pool<Postgres>,
        mut container: FileContainer,
        temp_store: Arc<dyn TempStore>,
        persistent: Arc<dyn PersistentStore>,
    ) -> Result<(), MediaServiceError> {
        let config = container.config().clone();
        let target_path = container.target_path()?;
        let container_relative_path = container.relative_path().to_string();
        for file in container.files_mut() {
            let result: Result<(), MediaServiceError> = async {
                match file.category() {
                    MediaType::Video => {
                        let processing_options = config
                            .clone()
                            .unwrap_or(ContainerConfig::new())
                            .get_processing_options()
                            .cloned()
                            .unwrap_or_default();
                        let pvpo = processing_options.get_post_video_processors().cloned();
                        let fflages = processing_options.get_fflags().cloned().unwrap_or_default();

                        let video_processor = VideoProcessor::new(fflages.video_gpu_accel);

                        // make file target path
                        let ftp = file
                            .file_directory()
                            .replace(&container_relative_path, &target_path);

                        if let Some(video_processes) = pvpo {
                            tracing::info!(
                                "Starting post processing for video file: {} with processes: {:?}",
                                file.file_full_path().display(),
                                video_processes
                            );
                            for process in video_processes {
                                video_processor
                                    .run_post(&file, &ftp, &persistent, process, &pool)
                                    .await?;
                            }
                        }

                        tracing::info!(
                            "Post processing completed for video file: {}",
                            file.file_full_path().display()
                        );

                        // remove job source afterward as a cleanup
                        file.delete()?;
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            .await;

            let mut tx = pool.begin().await?;
            if let Err(e) = result {
                media::update::post_processing_state(
                    &mut tx,
                    &file.id().unwrap(),
                    &PostProcessingState::Failed,
                )
                .await?;
                container.abort(temp_store.clone()).await?;
                tx.commit().await?;
                return Err(e);
            }

            media::update::post_processing_state(
                &mut tx,
                &file.id().unwrap(),
                &PostProcessingState::Completed,
            )
            .await?;
            tx.commit().await?;
        }

        container
            .finalize(persistent.clone(), temp_store.clone())
            .await?;
        Ok(())
    }

    async fn process_file(
        &self,
        mut file: File,
        config: Option<ContainerConfig>,
        temp_store: Arc<dyn TempStore>,
    ) -> Result<JoinHandle<Result<ProcessResponse, MediaServiceError>>, MediaServiceError> {
        // if no ID was set, generate a new ID for the file using the snowflake generator
        if !file.id().is_some() {
            let file_id = self.snowflake.generate_id()?;
            file.set_id(file_id);
        }

        // Load configuration for processing, falling back to default if none is provided
        let config = config.unwrap_or(ContainerConfig::new());
        let processing_options = config.get_processing_options().cloned().unwrap_or_default();

        // Spawn a new asynchronous task to process the file, returning a JoinHandle for the task
        // TDLR: Multithread.
        let result: JoinHandle<Result<ProcessResponse, MediaServiceError>> = tokio::spawn(
            async move {
                // Init a vector to hold newly created files such as thumbnails or static images for animated files
                let mut extra_files: Vec<File> = Vec::new();

                // Filter processing based on the media type of the file, handling images and videos differently
                match file.category() {
                    MediaType::Image => {
                        // Extract process options for image processing, if any are specified in the configuration
                        let ipo = processing_options.get_image_processors().cloned();

                        // * No processing Early Exit
                        // * ------------------------
                        if ipo.is_none() {
                            if config.is_generate_thumbhash() {
                                let thumbhash = generate_thumbhash_from_file(file.file_full_path());
                                file.set_placeholder(thumbhash);
                            }

                            // set name strategy
                            match config.get_naming_strategy() {
                                NamingStrategy::FinalHash => {
                                    let hash = file.hash()?;
                                    file.rename(&hash)?;
                                }
                                NamingStrategy::OriginalName => {
                                    if let Some(original_name) = file.original_name() {
                                        file.rename_with_extension(&original_name.to_string())?;
                                    } else {
                                        tracing::warn!(
                                            "File with ID {} does not have an original name.",
                                            file.id().unwrap_or(-1)
                                        );
                                        file.rename(&file.id().unwrap().to_string())?;
                                        file.set_current_extension(
                                            file.detected_extension().to_string(),
                                        )?;
                                    }
                                }
                                NamingStrategy::FileId => {
                                    file.rename(&file.id().unwrap().to_string())?;
                                }
                            }

                            if config.is_animated_image_indicator()
                                && config.get_naming_strategy() != &NamingStrategy::OriginalName
                            {
                                let image = VipsImage::new_from_file(file.file_full_path())?;
                                if image.get_n_pages() > 1 {
                                    file.set_animated(true);
                                }

                                // generate a static image for the animated image
                                if file.is_animated() {
                                    let static_image = VipsImage::new_from_file_with_opts(
                                        file.file_full_path(),
                                        VOption::new().set("n", 1),
                                    )?;
                                    // build path for the static image
                                    let animated_name = format!("a_{}", file.file_name());

                                    // leave this without changing the file. the container will handle it.
                                    let static_image_path = file
                                        .file_full_directory()
                                        .join(&format!("{}.png", animated_name));
                                    static_image.write_to_file(&static_image_path)?;

                                    drop(static_image);
                                    // convert original file to webp
                                    file.rename(&animated_name)?;
                                }
                                let original_file = VipsImage::new_from_file_with_opts(
                                    file.file_full_path(),
                                    VOption::new().set("n", -1),
                                )?;

                                let webp_path = file.prepare_new_extension(".webp");
                                original_file.write_to_file_with_opts(
                                    &webp_path,
                                    VOption::new().set("strip", true),
                                )?;
                                file.replace_from_file(&PathBuf::from(&webp_path))?;
                                file.set_current_extension(".webp".to_string())?;
                            } else {
                                // no image indicator, serve the file extension from original, if doesnt have, fallback to detected extension
                                // dont use original name, because it might have been renamed already
                                if let Some(original_name) = file.original_name() {
                                    // get extension from original name
                                    // split the original name by '.' and get the last part
                                    if let Some(ext) = original_name.split('.').last() {
                                        file.set_current_extension(ext.to_string())?;
                                    } else {
                                        file.set_current_extension(
                                            file.detected_extension().to_string(),
                                        )?;
                                    }
                                } else {
                                    file.set_current_extension(
                                        file.detected_extension().to_string(),
                                    )?;
                                }
                            }
                            let res = ProcessResponse {
                                processed_file: file,
                                file_to_insert: extra_files,
                                post_container: None,
                            };
                            return Ok(res);
                        }

                        // * Processing Layer
                        // * ------------------------

                        // load image from file
                        let image = {
                            let mut processor = ImageProcessor::new();
                            let (image, is_animated) = {
                                let image = VipsImage::new_from_file(file.file_full_path())?;

                                let n_pages = image.get_n_pages();

                                if n_pages > 1 {
                                    let image = VipsImage::new_from_file_with_opts(
                                        file.file_full_path(),
                                        VOption::new().set("n", -1),
                                    )?;

                                    (image, true)
                                } else {
                                    (image, false)
                                }
                            };
                            file.set_animated(is_animated);
                            let processed_file = processor.transform(image, ipo, is_animated)?;
                            processed_file
                        };

                        let opts = VOption::new().set("strip", true);

                        //construct the new file
                        let new_path = file.prepare_new_extension(".webp");
                        image.write_to_file_with_opts(&new_path, opts)?;
                        file.replace_from_file(&PathBuf::from(&new_path))?;
                        file.set_current_extension(".webp".to_string())?;

                        match config.get_naming_strategy() {
                            NamingStrategy::FinalHash => {
                                let hash = file.hash()?;
                                file.rename(&hash)?;
                            }
                            NamingStrategy::OriginalName => {
                                if let Some(original_name) = file.original_name() {
                                    file.rename_with_extension(&original_name.to_string())?;
                                } else {
                                    tracing::warn!(
                                        "File with ID {} does not have an original name.",
                                        file.id().unwrap_or(-1)
                                    );
                                    file.rename(&file.id().unwrap().to_string())?;
                                    file.set_current_extension(
                                        file.detected_extension().to_string(),
                                    )?;
                                }
                            }
                            NamingStrategy::FileId => {
                                file.rename(&file.id().unwrap().to_string())?;
                            }
                        }

                        // generate thumbhash if requested
                        if config.is_generate_thumbhash() {
                            let thumbhash = generate_thumbhash_from_file(file.file_full_path());
                            file.set_placeholder(thumbhash);
                        }

                        if config.is_animated_image_indicator() && file.is_animated() {
                            let static_image = VipsImage::new_from_file_with_opts(
                                file.file_full_path(),
                                VOption::new().set("n", 1),
                            )?;
                            // build path for the static image
                            let animated_name = format!("a_{}", file.file_name());

                            // leave this without changing the file. the container will handle it.
                            let static_image_path = file
                                .file_full_directory()
                                .join(&format!("{}.png", animated_name));
                            static_image.write_to_file(&static_image_path)?;

                            file.rename(&animated_name)?;

                            let mut static_file = file.clone();
                            static_file
                                .set_content_type("image/png".to_string())
                                .set_extension_unsafe(".png".to_string())
                                .set_kind(MediaKind::Preview);
                            extra_files.push(static_file);
                        }

                        if let Some((w, h)) = inspector::probe_image(&file.file_full_path())? {
                            // Handle image dimensions if needed
                            file.set_width(w as u32);
                            file.set_height(h as u32);
                        }
                    }
                    MediaType::Video => {
                        let vpo = processing_options.get_video_processors().cloned();
                        let pvpo = processing_options.get_post_video_processors().cloned();
                        // we may generate a thumbnail, or just save it as is.
                        // optionally create a new container for video that does have post-processing, like generating a thumbnail or transcoding to a different format.
                        let fflages = processing_options.get_fflags().cloned().unwrap_or_default();

                        let video_processor = VideoProcessor::new(false);

                        // naming video file comes first
                        match config.get_naming_strategy() {
                            NamingStrategy::OriginalName => {
                                if let Some(original_name) = file.original_name() {
                                    file.rename_with_extension(&original_name.to_string())?;
                                } else {
                                    tracing::warn!(
                                        "File with ID {} does not have an original name.",
                                        file.id().unwrap_or(-1)
                                    );
                                    file.rename(&file.id().unwrap().to_string())?;
                                    file.set_current_extension(
                                        file.detected_extension().to_string(),
                                    )?;
                                }
                            }
                            // due to hash unable on video type
                            // this for good to prevent large video file.
                            _ => {
                                file.rename(&file.id().unwrap().to_string())?;
                                file.set_current_extension(file.detected_extension().to_string())?;
                            }
                        }

                        let thumbnail_byte = {
                            if fflages.video_thumbnail || config.is_generate_thumbhash() {
                                Some(video_processor.get_thumbnail(&file, 1, "png").await?)
                            } else {
                                None
                            }
                        };
                        
                        let (width, height, duration) =
                            inspector::probe_video(&file.file_full_path()).await?;
                        file.set_width(width.unwrap_or(0) as u32);
                        file.set_height(height.unwrap_or(0) as u32);
                        file.set_duration(duration.unwrap_or(0.0) as f64);

                        let is_file_id_contained = config.is_file_id_contained();
                        if fflages.video_thumbnail
                            && !config
                                .get_naming_strategy()
                                .eq(&NamingStrategy::OriginalName)
                        {

                            let (put_image_path, put_image_relative) =
                                if is_file_id_contained || !pvpo.is_some() {
                                    let put_image_path = file.file_full_directory().to_path_buf();
                                    let put_image_relative = file.file_directory();
                                    (put_image_path, put_image_relative.to_string())
                                } else {
                                    let put_image_path = file
                                        .file_full_directory()
                                        .join(format!("{}/", file.id().unwrap_or(-1)));
                                    let put_image_relative = format!(
                                        "{}/{}",
                                        file.file_directory(),
                                        file.id().unwrap_or(-1)
                                    );
                                    (put_image_path, put_image_relative)
                                };

                            let file_name = format!("t_{}", file.id().unwrap_or(-1));

                            let thumbnail_path = put_image_path.join(format!("{}.png", &file_name));

                            let mut thumbnail_file = file.clone();
                            // write bytes to a file
                            if let Some(thumbnail_byte) = thumbnail_byte.clone() {
                                std::fs::create_dir_all(&put_image_path)?;
                                std::fs::write(&thumbnail_path, &thumbnail_byte)?;
                                let thumbnail_size = thumbnail_byte.len() as u64;
                                thumbnail_file.set_size(thumbnail_size);
                            }

                            thumbnail_file
                                .set_file_name(file_name)
                                .set_extension_unsafe(".png".to_string())
                                .set_kind(MediaKind::Preview)
                                .set_file_directory(put_image_relative)
                                .set_content_type("image/png".to_string())
                                .set_full_file_directory(put_image_path);

                            extra_files.push(thumbnail_file);
                        }

                        if config.is_generate_thumbhash() {
                            if let Some(thumbnail_byte) = thumbnail_byte {
                                let image = VipsImage::new_from_buffer(&thumbnail_byte, "")?;
                                let thumbhash = generate_thumbhash_from_image(image);
                                file.set_placeholder(thumbhash.clone());

                                // additionally edit thumbhash on extra files if needed
                                for extra_file in &mut extra_files {
                                    extra_file.set_placeholder(thumbhash.clone());
                                }
                            }
                        }

                        // early exit if no processing is required
                        // original should not be processed, only post processing is required.
                        if (vpo.is_none() && pvpo.is_none()) || fflages.video_post_orig {
                            let res = ProcessResponse {
                                processed_file: file,
                                file_to_insert: extra_files,
                                post_container: None,
                            };
                            return Ok(res);
                        }

                        // default name to file id, and extension to detected extension
                        file.rename(&file.id().unwrap().to_string())?;
                        file.set_current_extension(file.detected_extension().to_string())?;

                        // * video processing doesnt support yet, only post processing that got supported.

                        if let Some(pvpo) = pvpo {
                            // create returning file for post processing.
                            let mut post_container =
                                FileContainer::new("video_processing_job".to_string(), &temp_store)
                                    .await?;
                            // set up the post container with the same config as the original container
                            post_container.set_config(config.clone());

                            let container_path: &str = post_container.path();
                            let container_full_path = post_container.full_path();

                            let new_file = file
                                .copy_to_directory(container_full_path.clone(), container_path)?;

                            if !fflages.video_post_orig {
                                file.delete()?;
                            }

                            post_container.add_file(new_file)?;
                            post_container.file_id_contained()?;

                            // set and check HLS EARLY
                            if pvpo
                                .iter()
                                .any(|p| matches!(p, VideoPostProcessorType::HLS { .. }))
                            {
                                file.set_category(MediaType::Hls);
                            }

                            let res = ProcessResponse {
                                processed_file: file,
                                file_to_insert: extra_files,
                                post_container: Some(post_container),
                            };
                            return Ok(res);
                        }
                    }
                    _ => {
                        let res = ProcessResponse {
                            processed_file: file,
                            file_to_insert: extra_files,
                            post_container: None,
                        };
                        return Ok(res);
                    }
                }
                let res = ProcessResponse {
                    processed_file: file,
                    file_to_insert: extra_files,
                    post_container: None,
                };
                Ok(res)
            },
        );
        Ok(result)
    }
}
