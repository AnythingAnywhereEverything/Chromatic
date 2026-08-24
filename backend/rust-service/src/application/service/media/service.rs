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
        self,
        row::{MediaDataRow, MediaMetadataRow, MediaStatus},
    },
    service::{
        errors::MediaServiceError,
        media::{
            inspector::{self, MediaKind},
            model::{
                File, FileContainer,
                container::{ContainerConfig, NamingStrategy},
            },
            processor::{image::ImageProcessor, video::VideoProcessor},
            storage::{PersistentStore, TempStore},
        },
        snowflake_service::SnowflakeGenerator,
    },
    state::AppState,
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

pub struct ProcessResponse {
    pub processed_file: File,
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
        if container.is_empty() {
            return Err(MediaServiceError::NoFilesInContainer);
        }

        let uploader = container
            .uploader_id()
            .ok_or(MediaServiceError::UploaderIdNotSet)?;
        let config = container.config();
        let target_path = container.target_path()?.clone();

        if let Some(config) = &config {
            if config.is_file_id_contained() {
                container.file_id_contained()?;
            }
        }

        let files = container.take_files();

        // loop through each file in the container and process them

        let mut tasks = Vec::with_capacity(files.len());

        for file in files {
            let config = config.clone();
            let ts = self.temporary_store.clone();

            tasks.push(self.process_file(file, config, ts).await?);
        }

        let mut new_files = Vec::new();
        let mut ppc = Vec::new();

        for task in tasks {
            let res = task.await??;

            let file = res.processed_file;
            let post_container = res.post_container;

            new_files.push(file);

            if let Some(post_container) = post_container {
                ppc.push(post_container);
            }
        }

        container.replace_files(new_files);
        let mut tx = state.db_pool.begin().await?;

        // loop save file to database
        let container_dir = container.relative_path().to_string();
        for file in container.files_mut() {
            if file.is_deleted() && file.key_override().is_none() {
                continue;
            }
            // replace file path with contaer target path
            let file_path = if let Some(key_override) = file.key_override() {
                let path = key_override;
                // replace container dir from path with target path
                path.replace(&container_dir, &target_path)
            } else {
                let path = format!(
                    "{}/{}",
                    file.file_directory(),
                    file.file_name_with_extension()
                );
                // replace container dir from path with target path
                path.replace(&container_dir, &target_path)
            };

            let media_data_row = MediaDataRow {
                id: file.id().ok_or(MediaServiceError::ProcessingFailed)?,
                uploader_id: uploader,
                name: file.file_name_with_extension().to_string(),
                status: MediaStatus::Pending,
                path: file_path,
                thumbhash: file.placeholder().map(|s| s.to_string()),
                // next migration
                // original_name: file.original_name().map(|s| s.to_string()),
                ..Default::default()
            };

            // check if file already exists in database, if it does, update it, else insert it.
            if let Some(existing_media_id) =
                media::create::media_data_check_existing(&mut tx, &media_data_row).await?
            {
                file.set_id(existing_media_id);
            } else {
                media::create::media_data(&mut tx, &media_data_row).await?;

                let media_metadata_row = MediaMetadataRow {
                    media_id: file.id().ok_or(MediaServiceError::ProcessingFailed)?,
                    file_size: file.size() as i64,
                    mime_type: file.content_type().to_string(),
                    width: file.width().map(|w| w as i32),
                    height: file.height().map(|h| h as i32),
                    duration: file.duration(),
                };
                media::create::media_metadata(&mut tx, &media_metadata_row).await?;
            }
        }

        tx.commit().await?;

        container
            .finalize(self.persistent_store.clone(), self.temporary_store.clone())
            .await?;

        let mut tx = state.db_pool.begin().await?;
        for mut post_container in ppc {
            post_container.set_target_path(target_path.clone());

            // set media as processing for the post processing container
            for file in post_container.files_mut() {
                media::update::media_status(&mut tx, &file.id().unwrap(), &MediaStatus::Processing)
                    .await?;
            }

            let ts = self.temporary_store.clone();
            let ps = self.persistent_store.clone();
            let db = state.db_pool.clone();
            tokio::spawn(async move {
                let _ = MediaService::post_process_containment(db, post_container, ts, ps).await;
            });
        }

        // set the remaining files in the container as ready
        for file in container.files_mut() {
            media::update::media_status(&mut tx, &file.id().unwrap(), &MediaStatus::Ready).await?;
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
        for file in container.files_mut() {
            let result: Result<(), MediaServiceError> = async {
                match file.kind() {
                    MediaKind::Video => {
                        let processing_options = config
                            .clone()
                            .unwrap_or(ContainerConfig::new())
                            .get_processing_options()
                            .cloned()
                            .unwrap_or_default();
                        let pvpo = processing_options.get_post_video_processors().cloned();

                        let video_processor = VideoProcessor::new(false);

                        if let Some(video_processes) = pvpo {
                            tracing::info!(
                                "Starting post processing for video file: {} with processes: {:?}",
                                file.file_full_path().display(),
                                video_processes
                            );
                            for process in video_processes {
                                video_processor.run_post(&file, process).await?;
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
                media::update::media_status(&mut tx, &file.id().unwrap(), &MediaStatus::Failed)
                    .await?;
                container.abort(temp_store.clone()).await?;
                tx.commit().await?;
                return Err(e);
            }

            media::update::media_status(&mut tx, &file.id().unwrap(), &MediaStatus::Completed)
                .await?;
            tx.commit().await?;
        }

        container
            .finalize(persistent.clone(), temp_store.clone())
            .await?;
        container.abort(temp_store.clone()).await?;

        Ok(())
    }

    async fn process_file(
        &self,
        mut file: File,
        config: Option<ContainerConfig>,
        temp_store: Arc<dyn TempStore>,
    ) -> Result<JoinHandle<Result<ProcessResponse, MediaServiceError>>, MediaServiceError> {
        if !file.id().is_some() {
            let file_id = self.snowflake.generate_id()?;
            file.set_id(file_id);
        }

        let config = config.unwrap_or(ContainerConfig::new());
        // extract processor options from config
        let processing_options = config.get_processing_options().cloned().unwrap_or_default();

        // multi-thread
        let result: JoinHandle<Result<ProcessResponse, MediaServiceError>> =
            tokio::spawn(async move {
                match file.kind() {
                    MediaKind::Image => {
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
                        }

                        // generate thumbhash if requested
                        if config.is_generate_thumbhash() {
                            let thumbhash =
                                generate_thumbhash_from_file(file.file_full_path());
                            file.set_placeholder(thumbhash);
                        }

                        if let Some((w, h)) = inspector::probe_image(&file.file_full_path())? {
                            // Handle image dimensions if needed
                            file.set_width(w as u32);
                            file.set_height(h as u32);
                        }
                    }
                    MediaKind::Video => {
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

                        let is_file_id_contained = config.is_file_id_contained();
                        if fflages.video_thumbnail
                            && !config
                                .get_naming_strategy()
                                .eq(&NamingStrategy::OriginalName)
                        {
                            // check if video have to be in its own directory as

                            tracing::debug!(
                                "Video file ID contained: {}, Post video processors: {:?}",
                                is_file_id_contained,
                                pvpo
                            );
                            let put_image_directory = if is_file_id_contained || !pvpo.is_some() {
                                file.file_full_directory().to_path_buf()
                            } else {
                                file.file_full_directory()
                                    .join(format!("{}/", file.id().unwrap_or(-1)))
                            };

                            tracing::debug!(
                                "Putting video thumbnail in directory: {:?}",
                                put_image_directory
                            );

                            let file_name = format!("t_{}.png", file.id().unwrap_or(-1));

                            let thumbnail_path = put_image_directory.join(&file_name);

                            // write bytes to a file
                            if let Some(thumbnail_byte) = thumbnail_byte.clone() {
                                std::fs::create_dir_all(&put_image_directory)?;
                                std::fs::write(&thumbnail_path, &thumbnail_byte)?;
                            }

                            file.set_has_thumbnail(true);
                            // no tracking required
                        }

                        if config.is_generate_thumbhash() {
                            if let Some(thumbnail_byte) = thumbnail_byte {
                                let image = VipsImage::new_from_buffer(&thumbnail_byte, "")?;
                                let thumbhash = generate_thumbhash_from_image(image);
                                file.set_placeholder(thumbhash);
                            }
                        }

                        let (width, height, duration) =
                            inspector::probe_video(&file.file_full_path()).await?;
                        file.set_width(width.unwrap_or(0) as u32);
                        file.set_height(height.unwrap_or(0) as u32);
                        file.set_duration(duration.unwrap_or(0.0) as f32);

                        // early exit if no processing is required
                        // original should not be processed, only post processing is required.
                        if (vpo.is_none() && pvpo.is_none()) || fflages.video_post_orig {
                            let res = ProcessResponse {
                                processed_file: file,
                                post_container: None,
                            };
                            return Ok(res);
                        }

                        // default name to file id, and extension to detected extension
                        file.rename(&file.id().unwrap().to_string())?;
                        file.set_current_extension(file.detected_extension().to_string())?;

                        // * video processing doesnt support yet, only post processing that got supported.

                        if let Some(_pvpo) = pvpo {
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

                            tracing::debug!(
                                "Post processing container created: {:#?}",
                                post_container
                            );

                            // ! hard coded here specifically for this project
                            // set file path override to directory
                            let key_override = if is_file_id_contained {
                                file.file_directory().to_string()
                            } else {
                                format!("{}/{}/", file.file_directory(), file.id().unwrap_or(-1))
                            };
                            file.set_key_override(Some(key_override));

                            let res = ProcessResponse {
                                processed_file: file,
                                post_container: Some(post_container),
                            };
                            return Ok(res);
                        }
                    }
                    _ => {
                        let res = ProcessResponse {
                            processed_file: file,
                            post_container: None,
                        };
                        return Ok(res);
                    }
                }
                let res = ProcessResponse {
                    processed_file: file,
                    post_container: None,
                };
                Ok(res)
            });
        Ok(result)
    }
}
