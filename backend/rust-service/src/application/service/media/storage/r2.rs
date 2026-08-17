use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::application::service::errors::MediaServiceError;
use tokio::fs::File;

use super::MediaStorage;

pub struct R2Storage;

impl R2Storage {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MediaStorage for R2Storage {
    fn temp_root(&self) -> &str {
        "/tmp"
    }

    async fn upload(&self, _from: &str, _to: &str) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn save(&self, _path: &str, _data: &[u8]) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn delete(&self, _path: &str) {
        todo!()
    }

    async fn read(
        &self,
        _path: &str,
    ) -> Result<File, MediaServiceError> {
        todo!()
    }

    async fn exists(&self, _path: &str) -> Result<bool, MediaServiceError> {
        todo!()
    }

    async fn move_file(&self, _from: &Path, _to: &Path) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn copy_file(&self, _from: &Path, _to: &Path) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn prepare_directory(&self, _path: &Path) -> Result<(), MediaServiceError> {
        todo!()
    }

    async fn delete_temp(&self, _path: &str) {}

    fn full_path(&self, _path: &str) -> Result<PathBuf, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }
}
