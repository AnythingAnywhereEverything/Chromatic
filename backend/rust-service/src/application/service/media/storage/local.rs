use std::path::PathBuf;

use async_trait::async_trait;
use axum::extract::multipart::Field;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::time::Instant;
use uuid::Uuid;

use crate::application::service::errors::MediaServiceError;
use crate::application::service::media::types::TempUpload;

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

    async fn delete(&self, path: &str) {
        let full = self.build_full_path(path);
        let _ = fs::remove_file(full).await;
    }

    async fn read(&self, path: &str) -> Result<Vec<u8>, MediaServiceError> {
        let full = self.build_full_path(path);
        let data = fs::read(full).await?;
        Ok(data)
    }

    async fn exists(&self, path: &str) -> Result<bool, MediaServiceError> {
        let full = self.build_full_path(path);
        Ok(full.exists())
    }

    async fn save_temp_stream(
        &self,
        field: &mut Field<'_>,
        max_size: usize,
    ) -> Result<TempUpload, MediaServiceError> {
        let relative_path = format!("upload/{}.part", Uuid::new_v4());
        let full_path = self.build_temp_full_path(&relative_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = fs::File::create(&full_path).await?;
        let mut written = 0usize;

        let write_result = async {
            // * throughput monitoring variables
            let start_time = Instant::now();
            let min_bytes_per_second = 512;

            // * cancel/disconnect is detected here when the multipart stream read fails
            while let Some(chunk) = tokio::time::timeout(std::time::Duration::from_secs(10), field.chunk())
                .await
                .map_err(|_| MediaServiceError::Timeout)?
                .map_err(|e| MediaServiceError::MultipartError(e))?
            {
                written += chunk.len();

                if written > max_size {
                    return Err(MediaServiceError::FileTooLarge);
                }

                let elapsed = start_time.elapsed().as_secs();
                if elapsed > 5 { // Give the connection a 5-second grace period to spin up
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