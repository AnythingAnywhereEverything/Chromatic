use std::sync::Arc;

use axum::extract::Multipart;
use infer::{self};
use libvips::{VipsImage, ops};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tokio::process::Command;

use crate::application::{
    config::Config,
    repository::media::{
        self,
        row::{MediaDataRow, MediaStatus},
    },
    service::{
        errors::MediaServiceError,
        media::{
            processor::{image, video::process_video_hls},
            types::{
                AllowedMediaType, CropStyle, ExtractedPayload, ImageTransform, MediaCategory,
                MediaOptions, MediaProcessingMode, TempUpload,
            },
        },
        snowflake_service::SnowflakeGenerator,
    },
};

use super::storage::{MediaStorage, local::LocalStorage, r2::R2Storage};

pub struct MediaService {
    storage: Arc<dyn MediaStorage>,
    snowflake: SnowflakeGenerator,
    connection: sqlx::PgPool,
}

fn categorize(mime: &str) -> MediaCategory {
    match mime {
        // categorize based on mime type
        m if m.starts_with("image/") => MediaCategory::Image,
        m if m.starts_with("video/") => MediaCategory::Video,
        m if m.starts_with("audio/") => MediaCategory::Audio,

        "application/pdf" => MediaCategory::Document,

        "application/zip" | "application/x-tar" | "application/x-rar-compressed" => {
            MediaCategory::Archive
        }

        "text/plain"
        | "application/json"
        | "application/javascript"
        | "text/x-rust"
        | "text/x-python"
        | "text/x-java"
        | "text/x-c++"
        | "text/x-c" => MediaCategory::Code,

        _ => MediaCategory::Unknown,
    }
}

fn extract_image_meta(raw: &[u8]) -> Result<(i32, i32), MediaServiceError> {
    let image = VipsImage::new_from_buffer(raw, "")?;
    Ok((image.get_width(), image.get_height()))
}

async fn get_video_duration(path: &str) -> Result<f32, MediaServiceError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output()
        .await?;

    let duration = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<f32>()
        .map_err(|_| MediaServiceError::ProcessingFailed)?;

    Ok(duration)
}

/// Transforms the given image according to the specified `ImageTransform`.
/// # Arguments
/// * `image` - The input image to be transformed.
/// * `transform` - The transformation to be applied to the image.
/// # Returns
/// A `Result` containing the transformed image or a `MediaServiceError` if an error occurs during the transformation process.
fn transform_image(
    image: VipsImage,
    transform: ImageTransform,
) -> Result<VipsImage, MediaServiceError> {
    let width = image.get_width();
    let height = image.get_height();

    match transform {
        ImageTransform::Resize {
            rz_width,
            rz_height,
        } => Ok(image::resize_image(
            image,
            width as u32,
            height as u32,
            rz_width,
            rz_height,
        )?),
        ImageTransform::Crop { style, position } => match style {
            CropStyle::Absolute { width, height } => Ok(image::crop_image_absolute(
                image,
                width as u32,
                height as u32,
                width,
                height,
                position,
            )?),
            CropStyle::Normalized { width, height } => Ok(image::crop_image_normalized(
                image,
                width as u32,
                height as u32,
                width,
                height,
                position,
            )?),
            CropStyle::Ratio {
                ratio: (rw, rh),
                scale,
            } => Ok(image::crop_image_ratio(
                image,
                width as u32,
                height as u32,
                (rw, rh),
                scale,
                position,
            )?),
        },
        ImageTransform::None => Ok(image),
    }
}

impl MediaService {
    pub fn new(snowflake: SnowflakeGenerator, config: Config, connection: sqlx::PgPool) -> Self {
        let driver = config.media_driver;

        let storage: Arc<dyn MediaStorage> = match driver.as_str() {
            "r2" => {
                let r2 = R2Storage::new();
                Arc::new(r2)
            }
            _ => {
                let root = config.media_root;
                let temp_root = config.media_temp_root;
                Arc::new(LocalStorage::new(root, temp_root))
            }
        };

        Self {
            storage,
            snowflake,
            connection,
        }
    }

    pub async fn extract_payload_with_type<T: DeserializeOwned + Debug>(
        &self,
        mut multipart: Multipart,
        file_max_size: usize,
    ) -> Result<ExtractedPayload<T>, MediaServiceError> {
        let mut files = Vec::new();
        let mut payload: Option<T> = None;

        while let Some(mut field) = multipart.next_field().await? {
            let name = field.name().unwrap_or("").to_string();

            match name.as_str() {
                "payload" => {
                    let raw = field
                        .text()
                        .await
                        .map_err(|_| MediaServiceError::UnableToExtract)?;

                    payload = Some(serde_json::from_str(&raw).map_err(|e| {
                        tracing::error!("Payload deserialize error: {}", e);
                        MediaServiceError::UnableToExtract
                    })?);
                }

                _ => {
                    let uploaded = self
                        .storage
                        .save_temp_stream(&mut field, file_max_size)
                        .await?;

                    files.push(uploaded);
                }
            }
        }

        let payload = payload.ok_or(MediaServiceError::UnableToExtract)?;

        Ok(ExtractedPayload { payload, files })
    }

    pub async fn extract_multipart_bytes(
        &self,
        mut multipart: Multipart,
        field_filter: Option<&str>,
        max_size: usize,
    ) -> Result<Vec<TempUpload>, MediaServiceError> {
        let mut files = Vec::new();

        while let Some(mut field) = multipart.next_field().await? {
            let name = field.name().unwrap_or("").to_string();

            if let Some(filter) = field_filter {
                if name != filter {
                    continue;
                }
            }

            let uploaded = self.storage.save_temp_stream(&mut field, max_size).await?;

            files.push(uploaded);
        }

        Ok(files)
    }

    /// Saves the uploaded media file to the storage and returns the saved media information.
    ///
    /// # Arguments
    ///
    /// * `temp_uploaded` - The temporary uploaded media file.
    /// * `options` - The media options for processing and saving the media.
    /// * `old_relative_path` - An optional old relative path to delete after saving the new media.
    ///
    /// # Returns
    ///
    /// A `Result` containing the saved media information or a `MediaServiceError` if an error occurs during the process.
    pub async fn save_media(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        uploader_id: i64,
        temp_uploaded: TempUpload,
        options: MediaOptions,
        old_relative_path: Option<String>,
    ) -> Result<i64, MediaServiceError> {
        // * read uploaded temporary file
        let raw = self.storage.read_temp(&temp_uploaded.path).await?;

        if raw.len() > options.max_size {
            return Err(MediaServiceError::FileTooLarge);
        }

        let detected = infer::get(&raw).ok_or(MediaServiceError::InvalidMediaType)?;
        let mime = detected.mime_type();
        let extension = detected.extension();

        let category = categorize(mime);

        // * optional strict filtering
        if let Some(allowed) = &options.allowed_types {
            let matched = allowed.iter().any(|allowed| match allowed {
                AllowedMediaType::Jpeg => mime == "image/jpeg",
                AllowedMediaType::Png => mime == "image/png",
                AllowedMediaType::WebP => mime == "image/webp",
                AllowedMediaType::Mp4 => mime == "video/mp4",
            });

            if !matched {
                return Err(MediaServiceError::InvalidMediaType);
            }
        }

        match category {
            MediaCategory::Image => {
                let (processed_bytes, preview, final_ext) = match options.mode {
                    // Use only for image that was uploaded to public
                    MediaProcessingMode::Sanitize => {
                        let mut image = if mime == "image/gif" {
                            VipsImage::new_from_buffer(&raw, "[n=-1]")?
                        } else {
                            VipsImage::new_from_buffer(&raw, "")?
                        };

                        if image.get_n_pages() <= 1 {
                            let width = image.get_width();
                            let height = image.get_height();

                            let pixels = width as i64 * height as i64;
                            if pixels > 25_000_000 {
                                return Err(MediaServiceError::InvalidMediaType);
                            }

                            if let Some(transform) = options.image_transform {
                                image = transform_image(image, transform)?;
                            }

                            let webp = image.image_write_to_buffer(".webp[strip]")?;
                            (webp, None, "webp")
                        } else {
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

                                let width = frame.get_width();
                                let height = frame.get_height();
                                let pixels = width as i64 * height as i64;
                                if pixels > 25_000_000 {
                                    return Err(MediaServiceError::InvalidMediaType);
                                }

                                if let Some(transform) = options.image_transform {
                                    frame = transform_image(frame, transform)?;
                                }

                                frames.push(frame);
                            }

                            let first_frame = &frames[0].clone();
                            let frame_height = first_frame.get_height();
                            let frame_width = first_frame.get_width();

                            // * Join frames into a single image with the same height as the first frame

                            let options = ops::ArrayjoinOptions {
                                across: 1,
                                shim: 0,
                                background: vec![0.0],
                                halign: ops::Align::Low,
                                valign: ops::Align::Low,
                                hspacing: frame_width,
                                vspacing: frame_height,
                            };

                            let joined = ops::arrayjoin_with_opts(&mut frames, &options)?;

                            let webp = joined.image_write_to_buffer(&format!(
                                ".webp[page-height={}]",
                                frame_height
                            ))?;

                            let preview_image =
                                first_frame.image_write_to_buffer(".webp[strip]")?;

                            (webp, Some(preview_image), "webp")
                        }
                    }

                    _ => {
                        // * fallback to raw automatically
                        (raw.to_vec(), None, extension)
                    }
                };

                let id = self.snowflake.generate_id()?;
                let filename = format!("{}.{}", id, final_ext);
                let relative_path = format!("{}/{}", options.folder, filename);

                self.storage.save(&relative_path, &processed_bytes).await?;

                if let Some(old) = old_relative_path {
                    self.storage.delete(&old).await;
                }

                let preview_path = if let Some(preview_image) = preview {
                    let preview_filename = format!("{}_preview.{}", id, final_ext);
                    let preview_relative_path = format!("{}/{}", options.folder, preview_filename);

                    self.storage
                        .save(&preview_relative_path, &preview_image)
                        .await?;

                    Some(preview_relative_path)
                } else {
                    None
                };

                // * Saved all media to storage successfully
                // * saved data to db

                let (width, height) = extract_image_meta(&processed_bytes).unwrap_or((0, 0));

                let media_data = MediaDataRow {
                    id,
                    user_id: uploader_id,
                    media_url: relative_path.clone(),
                    media_preview_url: preview_path.clone(),
                    media_category: category,
                    media_status: MediaStatus::Processing,
                    created_at: chrono::Utc::now().naive_utc(),
                };

                let media_metadata = media::row::MediaMetadataRow {
                    media_id: id,
                    file_size: processed_bytes.len() as i64,
                    mime_type: mime.to_string(),
                    width: Some(width),
                    height: Some(height),
                    duration: None,
                };

                media::create::media_data(tx, &media_data).await?;

                media::create::media_metadata(tx, &media_metadata).await?;

                return Ok(id);
            }

            MediaCategory::Video => {
                let video_id = self.snowflake.generate_id()?;
                let output_folder = format!("{}/{}", options.folder, video_id);

                let (output_path, preview_path) = match options.mode {
                    MediaProcessingMode::Hls => {
                        let storage = self.storage.clone();
                        let input_path = temp_uploaded.path.clone();
                        let hls_output_path = output_folder.clone();

                        if options.hls_fallback {
                            let raw_filename = format!("raw.{}", extension);
                            let raw_output_path = format!("{}/{}", output_folder, raw_filename);

                            self.storage.save(&raw_output_path, &raw).await?;
                        }

                        // preview image for video is optional, if manual_preview is set, use that instead of generating one
                        let has_manual_preview = options.manual_preview.is_some();
                        if let Some(manual_preview) = options.manual_preview {
                            let preview_filename = format!("preview.{}", extension);
                            let preview_output_path =
                                format!("{}/{}", output_folder, preview_filename);

                            self.storage
                                .save(
                                    &preview_output_path,
                                    &self.storage.read_temp(&manual_preview.path).await?,
                                )
                                .await?;
                        } else {
                            // generate preview image for video
                            let preview_filename = format!("preview.jpg");
                            let preview_output_path =
                                format!("{}/{}", output_folder, preview_filename);

                            let mut cmd = Command::new("ffmpeg");
                            cmd.arg("-i")
                                .arg(&temp_uploaded.path)
                                .arg("-ss")
                                .arg("00:00:00.500")
                                .arg("-vframes")
                                .arg("1")
                                .arg(&preview_output_path)
                                .stdout(std::process::Stdio::null())
                                .stderr(std::process::Stdio::null());

                            let status = cmd.spawn()?.wait().await?;

                            if !status.success() {
                                let _ = tokio::fs::remove_dir_all(&temp_uploaded.path).await;
                                return Err(MediaServiceError::ProcessingFailed);
                            }
                        }

                        let output_folder_clone = output_folder.clone();
                        let storage_clone = storage.clone();
                        let video_id_clone = video_id.clone();
                        let connection_pool = self.connection.clone();
                        tokio::spawn(async move {
                            let _manifists =
                                process_video_hls(input_path, hls_output_path, storage).await?;

                            // check if the master.m3u8 exists, if not, delete the output folder and return error
                            let master_path = format!("{}/master.m3u8", output_folder_clone);
                            if !storage_clone.exists(&master_path).await? {
                                let _ = storage_clone.delete(&output_folder_clone).await;
                                return Err(MediaServiceError::ProcessingFailed);
                            }

                            // Update media status to Completed after video processing completes
                            let mut tx = connection_pool.begin().await?;
                            media::update::media_status(
                                &mut tx,
                                &video_id_clone,
                                &MediaStatus::Completed,
                            )
                            .await?;
                            tx.commit().await?;

                            Ok::<(), MediaServiceError>(())
                        });

                        (
                            format!("{}/master.m3u8", output_folder),
                            if has_manual_preview {
                                Some(format!("{}/preview.{}", output_folder, extension))
                            } else {
                                Some(format!("{}/preview.jpg", output_folder))
                            },
                        )
                    }

                    _ => {
                        // * fallback to raw file save
                        let filename = format!("{}.{}", video_id, extension);
                        let output_path = format!("{}/{}", options.folder, filename);

                        self.storage.save(&output_path, &raw).await?;

                        (output_path, None)
                    }
                };

                if let Some(old) = old_relative_path {
                    self.storage.delete(&old).await;
                }

                let duration = get_video_duration(&temp_uploaded.path).await.ok();

                let media_data = MediaDataRow {
                    id: video_id,
                    user_id: uploader_id,
                    media_url: output_path.clone(),
                    media_preview_url: preview_path.clone(),
                    media_category: category,
                    media_status: MediaStatus::Processing,
                    created_at: chrono::Utc::now().naive_utc(),
                };

                let media_metadata = media::row::MediaMetadataRow {
                    media_id: video_id,
                    file_size: raw.len() as i64,
                    mime_type: mime.to_string(),
                    width: None,
                    height: None,
                    duration,
                };

                media::create::media_data(tx, &media_data).await?;
                media::create::media_metadata(tx, &media_metadata).await?;

                return Ok(video_id);
            }

            _ => {
                // * fallback for audio, document, archive, etc.
                let id = self.snowflake.generate_id()?;
                let filename = format!("{}.{}", id, extension);
                let relative_path = format!("{}/{}", options.folder, filename);

                self.storage.save(&relative_path, &raw).await?;

                if let Some(old) = old_relative_path {
                    self.storage.delete(&old).await;
                }

                return Ok(id);
            }
        }
    }
}
