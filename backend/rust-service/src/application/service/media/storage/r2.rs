use std::path::{Path, PathBuf};

use async_trait::async_trait;
use axum::extract::multipart::Field;

use crate::application::service::errors::MediaServiceError;
use crate::application::service::media::storage::StorageResponse;
use crate::application::service::media::types::TempUpload;

use super::MediaStorage;

pub struct R2Storage;

impl R2Storage {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MediaStorage for R2Storage {
    async fn save(&self, _path: &str, _data: &[u8]) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn save_temp(&self, _path: &str, _data: &[u8]) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn delete(&self, _path: &str) {
        todo!()
    }

    async fn read(
        &self,
        _path: &str,
        _mime_type: &str,
    ) -> Result<StorageResponse, MediaServiceError> {
        todo!()
    }

    async fn exists(&self, _path: &str) -> Result<bool, MediaServiceError> {
        todo!()
    }

    async fn move_file(&self, _from: &Path, _to: &Path) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn move_all_to_directory(&self, _from: &Path, _to: &Path) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn copy_file(&self, _from: &Path, _to: &Path) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn prepare_directory(&self, _path: &Path) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn save_temp_stream(
        &self,
        _field: &mut Field<'_>,
        _max_size: usize,
    ) -> Result<TempUpload, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }

    async fn read_temp(&self, _path: &str) -> Result<Vec<u8>, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }

    async fn delete_temp(&self, _path: &str) {}

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
