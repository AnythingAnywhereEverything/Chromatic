use async_trait::async_trait;
use axum::{extract::multipart::Field, http::HeaderMap};
use std::path::{Path, PathBuf};

use super::{MediaStorage, local::LocalStorage};
use crate::application::service::media::multipart_ex::FileSizeGate;
use crate::application::service::media::types::{TempUpload, ValidationOptions};
use crate::application::service::{errors::MediaServiceError, media::storage::StorageResponse};

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
    // Reuse Local Logic completely for local writing and house-keeping
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError> {
        self.local.save(path, data).await
    }

    async fn save_temp(&self, path: &str, data: &[u8]) -> Result<(), MediaServiceError> {
        self.local.save_temp(path, data).await
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

    async fn move_all_to_directory(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError> {
        self.local.move_all_to_directory(from, to).await
    }

    async fn copy_file(&self, from: &Path, to: &Path) -> Result<(), MediaServiceError> {
        self.local.copy_file(from, to).await
    }

    async fn prepare_directory(&self, path: &Path) -> Result<(), MediaServiceError> {
        self.local.prepare_directory(path).await
    }

    async fn save_temp_stream(
        &self,
        field: &mut Field<'_>,
        max_size: usize,
        validation: Option<&ValidationOptions>,
        filter_gate: Option<&Vec<FileSizeGate>>,
    ) -> Result<TempUpload, MediaServiceError> {
        self.local.save_temp_stream(field, max_size, validation, filter_gate).await
    }

    async fn read_temp(&self, path: &str) -> Result<Vec<u8>, MediaServiceError> {
        self.local.read_temp(path).await
    }

    async fn delete_temp(&self, path: &str) {
        self.local.delete_temp(path).await;
    }

    fn full_path(&self, path: &str) -> Result<PathBuf, MediaServiceError> {
        self.local.full_path(path)
    }

    fn temp_full_path(&self, path: &str) -> Result<PathBuf, MediaServiceError> {
        self.local.temp_full_path(path)
    }

    fn new_temp_relative_path(&self, prefix: &str) -> Result<String, MediaServiceError> {
        self.local.new_temp_relative_path(prefix)
    }

    async fn read(
        &self,
        path: &str,
        mime_type: &str,
    ) -> Result<StorageResponse, MediaServiceError> {
        if !self.local.exists(path).await.unwrap_or(false) {
            return Err(MediaServiceError::MediaMissing);
        }

        let mut headers = HeaderMap::new();

        let redirect_path = format!("{}{}", self.internal_redirect_prefix, path);
        headers.insert(
            "X-Accel-Redirect",
            axum::http::HeaderValue::from_str(&redirect_path).unwrap(),
        );

        headers.insert(
            "Content-Type",
            axum::http::HeaderValue::from_str(mime_type).unwrap(),
        );

        // No bytes are returned, as the actual file is served by Nginx via the X-Accel-Redirect header.
        // Accel-Redirect is a mechanism in Nginx that allows internal redirection to a different location, often used for serving files securely.
        Ok(StorageResponse::Headers(headers))
    }
}
