pub mod local;
pub mod r2;

use std::path::PathBuf;

use async_trait::async_trait;
use axum::extract::multipart::Field;

use crate::application::service::errors::MediaServiceError;
use crate::application::service::media::types::TempUpload;

#[async_trait]
pub trait MediaStorage: Send + Sync {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError>;
    async fn delete(&self, path: &str);
    async fn exists(&self, path: &str) -> Result<bool, MediaServiceError>;

    async fn read(&self, path: &str) -> Result<Vec<u8>, MediaServiceError>;

    async fn save_temp_stream(
        &self,
        field: &mut Field<'_>,
        max_size: usize,
    ) -> Result<TempUpload, MediaServiceError>;

    async fn read_temp(&self, path: &str) -> Result<Vec<u8>, MediaServiceError>;
    async fn delete_temp(&self, path: &str);

    // * local-processing helpers, unsupported on non-local storage for now
    fn full_path(&self, _path: &str) -> Result<PathBuf, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }

    fn temp_full_path(&self, _path: &str) -> Result<PathBuf, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }

    fn new_temp_relative_path(&self, _prefix: &str) -> Result<String, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }
}