use async_trait::async_trait;
use tokio::fs::File;
use std::path::{Path, PathBuf};

use super::{MediaStorage, local::LocalStorage};
use crate::application::service::{
    errors::MediaServiceError,
};

pub struct NginxStorage {
    // file uploads, temp streaming, and deletes remain identical!
    local: LocalStorage,
    internal_redirect_prefix: String,
}

impl NginxStorage {
    pub fn new(root: String, temp_root: String, internal_redirect_prefix: String) -> Self {
        Self {
            local: LocalStorage::new(root, temp_root),
            internal_redirect_prefix, // e.g., "/internal_local_cdn/"
        }
    }

    /// Generates the secret X-Accel-Redirect header value for Nginx
    pub fn get_nginx_redirect_header(&self, path: &str) -> String {
        format!("{}{}", self.internal_redirect_prefix, path)
    }
}

#[async_trait]
impl MediaStorage for NginxStorage {
    fn temp_root(&self) -> &str {
        self.local.temp_root()
    }

    async fn upload(&self, from: &str, to: &str) -> Result<(), MediaServiceError> {
        self.local.upload(from, to).await
    }

    // Reuse Local Logic completely for local writing and house-keeping
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError> {
        self.local.save(path, data).await
    }

    async fn delete(&self, path: &str) {
        self.local.delete(path).await;
    }

    async fn exists(&self, path: &str) -> Result<bool, MediaServiceError> {
        self.local.exists(path).await
    }

    async fn move_file(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError> {
        self.local.move_file(from, to).await
    }

    async fn copy_file(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError> {
        self.local.copy_file(from, to).await
    }

    async fn prepare_directory(&self, path: &Path) -> Result<(), MediaServiceError> {
        self.local.prepare_directory(path).await
    }

    async fn delete_temp(&self, path: &str) {
        self.local.delete_temp(path).await;
    }

    fn full_path(&self, path: &str) -> Result<PathBuf, MediaServiceError> {
        self.local.full_path(path)
    }

    async fn read(
        &self,
        path: &str,
    ) -> Result<File, MediaServiceError> {
        self.local.read(path).await
    }
}
