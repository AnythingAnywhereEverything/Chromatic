pub mod local;
pub mod nginx;
pub mod r2; // nginx storage

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use hyper::HeaderMap;
use tokio::fs::{self, File};

use async_trait::async_trait;
use uuid::Uuid;

use crate::application::service::media::storage::local::LocalStorage;
use crate::application::service::{
    errors::MediaServiceError,
};

pub enum StorageResponse {
    File(File),

    /// Used by Nginx (X-Accel-Redirect) or Cloud CNDs (302 Redirect URLs).
    Headers(HeaderMap),
}

/// Check if the given path is safe and does not contain any directory traversal sequences.
/// Though it could contain travel inner sequesnces, it should not be able to escape the root directory of the storage.
/// # Errors
/// Returns `MediaServiceError::InvalidFilePath` if the path is unsafe.
/// # Examples
/// ```
/// # use chromatic::application::service::errors::MediaServiceError;
/// # use chromatic::application::service::media::storage::safe_pathing;
/// let result = safe_pathing("../etc/passwd");
/// assert!(matches!(result, Err(MediaServiceError::InvalidFilePath)));
/// 
/// let result = safe_pathing("valid/path/to/file.txt");
/// assert!(result.is_ok());
/// 
/// let result = safe_pathing("invalid\\path\\to\\file.txt");
/// assert!(matches!(result, Err(MediaServiceError::InvalidFilePath)));
/// ```
pub fn safe_pathing(path: &str) -> Result<(), MediaServiceError> {
    if path.contains("..") || path.contains('\\') || path.starts_with('/') {
        return Err(MediaServiceError::InvalidFilePath);
    }
    Ok(())
}

#[async_trait]
pub trait MediaStorage: Send + Sync {
    fn temp_root(&self) -> &str;

    async fn save(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError>;
    async fn delete(&self, path: &str);
    async fn exists(&self, path: &str) -> Result<bool, MediaServiceError>;
    /// Moves a directory to another directory
    /// This allows upload container controls on each upload group.
    /// ## Parameters
    /// - `from`: The source path of the directory.
    /// - `to`: The destination path where the directory should be moved.
    /// ## Returns
    /// - `Result<(), MediaServiceError>`
    async fn upload(&self, from: &str, to: &str) -> Result<(), MediaServiceError>;

    async fn read(&self, path: &str) -> Result<File, MediaServiceError>;

    async fn prepare_directory(&self, path: &Path) -> Result<(), MediaServiceError>;

    
    // * local-processing helpers, unsupported on non-local storage for now
    fn full_path(&self, _path: &str) -> Result<PathBuf, MediaServiceError> {
        Err(MediaServiceError::ProcessingFailed)
    }
    
    /// Deletes a file from a temporary location.
    async fn delete_temp(&self, path: &str);
    
    // * --------------------------------
    // * local only helpers
    // * --------------------------------
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
    async fn copy_file(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError> {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::copy(from, to).await?;
        Ok(())
    }

    // * --------------------------------
    // * Build Paths
    // * --------------------------------

    fn temp_full_path(&self, path: &str) -> PathBuf {
        let mut full = PathBuf::from(&self.temp_root());
        full.push(path);
        full
    }

    fn new_temp_relative_path(&self, prefix: &str) -> String {
        format!("{}/{}", prefix, Uuid::new_v4())
    }
}

pub struct MediaStorageContainer{

    pub storage: Arc<dyn MediaStorage>,
}

impl Default for MediaStorageContainer {
    fn default() -> Self {
        let config = crate::application::config::load();
        let local_storage = LocalStorage::new(config.media_root, config.media_temp_root);
        MediaStorageContainer {
            storage: Arc::new(local_storage),
        }
    }
}