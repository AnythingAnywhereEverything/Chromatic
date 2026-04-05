use std::sync::Arc;

use axum::extract::Multipart;
use infer::{self};
use libvips::{ops, VipsImage};
use serde::de::DeserializeOwned;
use tokio::{process::Command};
use std::fmt::Debug;

use crate::application::{
    config::Config,
    service::{
        errors::MediaServiceError,
        media::{
            processor::video::process_video_hls,
            types::{
                AllowedMediaType, ExtractedPayload, ImageTransform, MediaCategory, MediaMeta, MediaOptions, MediaProcessingMode, SavedMedia, TempUpload
            },
        },
        snowflake_service::SnowflakeGenerator,
    },
};

use super::storage::{local::LocalStorage, r2::R2Storage, MediaStorage};

pub struct MediaService {
    storage: Arc<dyn MediaStorage>,
    snowflake: SnowflakeGenerator,
}

fn categorize(mime: &str) -> MediaCategory {
    match mime {
        m if m.starts_with("image/") => MediaCategory::Image,
        m if m.starts_with("video/") => MediaCategory::Video,
        m if m.starts_with("audio/") => MediaCategory::Audio,

        "application/pdf" => MediaCategory::Document,

        "application/zip"
        | "application/x-tar"
        | "application/x-rar-compressed" => MediaCategory::Archive,

        "text/plain"
        | "application/json"
        | "application/javascript"
        | "text/x-rust"
        | "text/x-python" => MediaCategory::Code,

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

impl MediaService {
    pub fn new(snowflake: SnowflakeGenerator, config: Config) -> Self {
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

        Self { storage, snowflake }
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

                    payload = Some(
                        serde_json::from_str(&raw).map_err(|e| {
                            tracing::error!("Payload deserialize error: {}", e);
                            MediaServiceError::UnableToExtract
                        })?,
                    );
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

            let uploaded = self
                .storage
                .save_temp_stream(&mut field, max_size)
                .await?;

            files.push(uploaded);
        }

        Ok(files)
    }


    pub async fn save_media(
        &self,
        temp_uploaded: TempUpload,
        options: MediaOptions,
        old_relative_path: Option<String>,
    ) -> Result<SavedMedia, MediaServiceError> {

        // * read uploaded temporary file
        let raw = self.storage.read_temp(&temp_uploaded.path).await?;

        if raw.len() > options.max_size {
            return Err(MediaServiceError::SizeTooLarge);
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
                let (processed_bytes, final_ext) = match options.mode {
                    MediaProcessingMode::Sanitize => {
                        let mut image = VipsImage::new_from_buffer(&raw, "")?;

                        let mut width = image.get_width();
                        let mut height = image.get_height();

                        let pixels = width as i64 * height as i64;
                        if pixels > 25_000_000 {
                            return Err(MediaServiceError::InvalidMediaType);
                        }

                        if let Some(transform) = options.image_transform {
                            match transform {
                                ImageTransform::Resize { max_width, max_height } => {
                                    if width > max_width || height > max_height {
                                        let scale = (max_width as f64 / width as f64)
                                            .min(max_height as f64 / height as f64);

                                        image = ops::resize(&image, scale)?;
                                    }
                                }

                                ImageTransform::Crop { max_width, max_height, ratio } => {
                                    if let Some((rw, rh)) = ratio {
                                        let target_ratio = rw as f64 / rh as f64;
                                        let current_ratio = width as f64 / height as f64;

                                        let (crop_width, crop_height) =
                                            if current_ratio > target_ratio {
                                                ((height as f64 * target_ratio) as i32, height)
                                            } else {
                                                (width, (width as f64 / target_ratio) as i32)
                                            };

                                        let left = (width - crop_width) / 2;
                                        let top = (height - crop_height) / 2;

                                        image = ops::extract_area(
                                            &image,
                                            left,
                                            top,
                                            crop_width,
                                            crop_height,
                                        )?;

                                        width = image.get_width();
                                        height = image.get_height();
                                    }

                                    if width > max_width || height > max_height {
                                        let scale = (max_width as f64 / width as f64)
                                            .min(max_height as f64 / height as f64);

                                        image = ops::resize(&image, scale)?;
                                    }
                                }

                                ImageTransform::None => {}
                            }
                        }

                        let webp = image.image_write_to_buffer(".webp[strip]")?;
                        (webp, "webp")
                    }

                    _ => {
                        // * fallback to raw automatically
                        (raw.to_vec(), extension)
                    }
                };

                let id = self.snowflake.generate_id()?;
                let filename = format!("{}.{}", id, final_ext);
                let relative_path = format!("{}/{}", options.folder, filename);

                self.storage.save(&relative_path, &processed_bytes).await?;

                if let Some(old) = old_relative_path {
                    self.storage.delete(&old).await;
                }

                let (width, height) =
                    extract_image_meta(&processed_bytes).unwrap_or((0, 0));

                let meta = MediaMeta {
                    size: processed_bytes.len(),
                    mime: mime.to_string(),
                    width: Some(width),
                    height: Some(height),
                    duration: None,
                };

                return Ok(SavedMedia {
                    path: relative_path,
                    category,
                    meta,
                    video_manifest: None,
                })
            }

            MediaCategory::Video => {
                let video_id = self.snowflake.generate_id()?;
                let output_folder = format!("{}/{}", options.folder, video_id);

                let (output_path, video_manifest) = match options.mode {
                    MediaProcessingMode::Hls => {
                        let storage = self.storage.clone();
                        let input_path = temp_uploaded.path.clone();
                        let hls_output_path = output_folder.clone();

                        if options.hls_fallback {
                            let raw_filename = format!("raw.{}", extension);
                            let raw_output_path = format!("{}/{}", output_folder, raw_filename);

                            self.storage.save(&raw_output_path, &raw).await?;
                        }

                        tokio::spawn(async move {
                            // ! fire and forget, manifest is not available immediately
                            let _ = process_video_hls(input_path, hls_output_path, storage).await;

                            // TODO: Add DB update here
                        });

                        (
                            format!("{}/master.m3u8", output_folder),
                            None,
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

                let meta = MediaMeta {
                    size: raw.len(),
                    mime: mime.to_string(),
                    width: None,
                    height: None,
                    duration,
                };

                return Ok(SavedMedia {
                    path: output_path,
                    category,
                    meta,
                    video_manifest,
                });
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

                let meta = MediaMeta {
                    size: raw.len(),
                    mime: mime.to_string(),
                    width: None,
                    height: None,
                    duration: None,
                };

                return Ok(SavedMedia {
                    path: relative_path,
                    category,
                    meta,
                    video_manifest: None,
                })
            }
        }
    }
}