use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use axum::extract::multipart::Field;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::time::Instant;
use uuid::Uuid;

use crate::application::service::errors::MediaServiceError;
use crate::application::service::media::storage::StorageResponse;
use crate::application::service::media::types::media_options::{
    FileSizeGate, TempUpload, ValidationOptions,
};
use crate::application::service::media::utils::{
    get_media_types_from_mime, get_mime_and_extension_validation_options, validate_media_type,
};

use super::MediaStorage;

pub struct LocalStorage {
    pub root: String,
    pub temp_root: String,
}

impl LocalStorage {
    pub fn new(root: String, temp_root: String) -> Self {
        Self { root, temp_root }
    }

    fn build_full_path(&self, path: &str) -> PathBuf {
        let mut full = PathBuf::from(&self.root);
        full.push(path);
        full
    }

    fn build_temp_full_path(&self, path: &str) -> PathBuf {
        let mut full = PathBuf::from(&self.temp_root);
        full.push(path);
        full
    }

    fn build_temp_relative_path(&self, prefix: &str) -> String {
        format!("{}/{}", prefix, Uuid::new_v4())
    }
}

#[async_trait]
impl MediaStorage for LocalStorage {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError> {
        let full = self.build_full_path(path);

        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(full, data).await?;
        Ok(())
    }

    async fn save_temp(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError> {
        let full = self.build_temp_full_path(path);

        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(full, data).await?;
        Ok(())
    }

    async fn delete(&self, path: &str) {
        let full = self.build_full_path(path);
        let _ = fs::remove_file(full).await;
    }

    async fn read(
        &self,
        path: &str,
        _mime_type: &str,
    ) -> Result<StorageResponse, MediaServiceError> {
        let full = self.build_full_path(path);
        let data = fs::read(full).await?;
        Ok(StorageResponse::Bytes(data))
    }

    async fn exists(&self, path: &str) -> Result<bool, MediaServiceError> {
        let full = self.build_full_path(path);
        Ok(full.exists())
    }

    // Fix: Return nothing when there is nothing to move
    async fn move_file(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError> {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).await?;
        }

        match fs::rename(from, to).await {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    // This function allows moving all files from one directory to another, creating the destination directory if it doesn't exist. It only moves files and ignores subdirectories.
    // warning: this method is slower than moving a whole directory, but it safer
    // due to it will just insert the file to the target destination and not replace the directory, which is safer for production use cases
    async fn move_all_to_directory(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError> {
        if !to.exists() {
            fs::create_dir_all(to).await?;
        }

        let mut entries = fs::read_dir(from).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file_type = entry.file_type().await?;
            if file_type.is_file() {
                let file_name = entry.file_name();
                let from_path = entry.path();
                let to_path = to.join(file_name);
                fs::rename(from_path, to_path).await?;
            }
        }

        // finally remove the source directory if it's empty
        if fs::read_dir(from).await?.next_entry().await?.is_none() {
            fs::remove_dir(from).await?;
        }

        Ok(())
    }

    // create directory of the exact path if it doesn't exist, otherwise do nothing
    async fn prepare_directory(&self, path: &Path) -> Result<(), MediaServiceError> {
        if !path.exists() {
            fs::create_dir_all(path).await?;
        }
        Ok(())
    }

    async fn copy_file(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError> {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::copy(from, to).await?;
        Ok(())
    }

    async fn save_temp_stream(
        &self,
        field: &mut Field<'_>,
        max_size: usize,
        validation: Option<&ValidationOptions>,
        filter_gate: Option<&Vec<FileSizeGate>>,
    ) -> Result<TempUpload, MediaServiceError> {
        // * Generate a unique temporary file path
        let relative_path = format!("upload/{}.part", Uuid::new_v4());
        let full_path = self.build_temp_full_path(&relative_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = fs::File::create(&full_path).await?;
        let mut written = 0usize;
        let mut validated = false;

        let mut mime = String::new();
        let mut extension = String::new();

        let write_result = async {
            // * throughput monitoring variables
            let start_time = Instant::now();
            let min_bytes_per_second = 512;

            // * cancel/disconnect is detected here when the multipart stream read fails
            while let Some(chunk) =
                tokio::time::timeout(std::time::Duration::from_secs(10), field.chunk())
                    .await
                    .map_err(|_| MediaServiceError::Timeout)?
                    .map_err(|e| MediaServiceError::MultipartError(e))?
            {
                written += chunk.len();

                if !validated && written > 8192 {
                    // Validate after 8KB of data has been written
                    if let Some(validation) = validation {
                        let (detected_mime, detected_extension) =
                            get_mime_and_extension_validation_options(
                                &chunk[..8192], // Use the first 8KB of the file for MIME type detection
                                validation,
                            )?;
                        mime = detected_mime;
                        extension = detected_extension;

                        validate_media_type(&mime, &Some(validation.clone()))?;
                    }

                    tracing::trace!("Detected MIME type: {}, extension: {}", mime, extension);
                    validated = true;
                };

                if validated {
                    if let Some(filter_gate) = filter_gate {
                        let mimes = get_media_types_from_mime(&mime);
                        for gate in filter_gate {
                            if mimes.contains(&gate.media_type) && written > gate.max_size {
                                return Err(MediaServiceError::FileTooLarge);
                            }
                        }
                    }
                }

                if written > max_size {
                    return Err(MediaServiceError::FileTooLarge);
                }

                let elapsed = start_time.elapsed().as_secs();
                if elapsed > 5 {
                    // Give the connection a 5-second grace period to spin up
                    let throughput = written / (elapsed as usize);
                    if throughput < min_bytes_per_second {
                        tracing::warn!("Dropped connection due to low throughput rate.");
                        return Err(MediaServiceError::TransmissionTooSlow);
                    }
                }

                file.write_all(&chunk).await?;
            }

            file.flush().await?;
            Ok::<(), MediaServiceError>(())
        }
        .await;

        match write_result {
            Ok(()) => Ok(TempUpload {
                path: relative_path,
                size: written,
                mime: mime,
                extension: extension,
            }),
            Err(err) => {
                // * partial temp file is removed on failure/cancel
                let _ = fs::remove_file(&full_path).await;
                Err(err)
            }
        }
    }

    async fn read_temp(&self, path: &str) -> Result<Vec<u8>, MediaServiceError> {
        let full = self.build_temp_full_path(path);
        let data = fs::read(full).await?;
        Ok(data)
    }

    async fn delete_temp(&self, path: &str) {
        let full = self.build_temp_full_path(path);
        let _ = fs::remove_file(full).await;
    }

    fn full_path(&self, path: &str) -> Result<PathBuf, MediaServiceError> {
        Ok(self.build_full_path(path))
    }

    fn temp_full_path(&self, path: &str) -> Result<PathBuf, MediaServiceError> {
        Ok(self.build_temp_full_path(path))
    }

    fn new_temp_relative_path(&self, prefix: &str) -> Result<String, MediaServiceError> {
        Ok(self.build_temp_relative_path(prefix))
    }
}
