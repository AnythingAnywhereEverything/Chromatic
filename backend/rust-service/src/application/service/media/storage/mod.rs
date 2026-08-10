pub mod local;
pub mod r2;
pub mod nginx; // nginx storage

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use axum::{http::HeaderMap, extract::multipart::Field};

use crate::application::service::errors::MediaServiceError;
use crate::application::service::media::types::TempUpload;

pub enum StorageResponse {
    /// Used by LocalStorage when Rust must read and stream the file bytes directly.
    Bytes(Vec<u8>),
    /// Used by Nginx (X-Accel-Redirect) or Cloud CNDs (302 Redirect URLs).
    Headers(HeaderMap),
}


#[async_trait]
pub trait MediaStorage: Send + Sync {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError>;
    async fn delete(&self, path: &str);
    async fn exists(&self, path: &str) -> Result<bool, MediaServiceError>;

    async fn read(&self, path: &str, mime_type: &str) -> Result<StorageResponse, MediaServiceError>;
    
    async fn move_file(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError>;
    async fn copy_file(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError>;
    async fn prepare_directory(&self, path: &Path) -> Result<(), MediaServiceError>;
    async fn move_all_to_directory(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError>;
    
    // * local-processing helpers, unsupported on non-local storage for now
    fn full_path(&self, _path: &str) -> Result<PathBuf, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }
    
    /// Saves a file stream to a temporary location, returning the relative path and size of the saved file.
    async fn save_temp_stream(
        &self,
        field: &mut Field<'_>,
        max_size: usize,
    ) -> Result<TempUpload, MediaServiceError>;

    async fn save_temp(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError>;

    /// Reads a file from a temporary location, returning the file bytes.
    async fn read_temp(&self, path: &str) -> Result<Vec<u8>, MediaServiceError>;

    /// Deletes a file from a temporary location.
    async fn delete_temp(&self, path: &str);

    /// Returns the full path of a file in a temporary location.
    fn temp_full_path(&self, _path: &str) -> Result<PathBuf, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }

    /// Generates a new relative path for a temporary file, using the given prefix.
    fn new_temp_relative_path(&self, _prefix: &str) -> Result<String, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }
}