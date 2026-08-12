use std::path::{Path, PathBuf};

use async_trait::async_trait;
use tokio::fs;

use crate::application::service::errors::MediaServiceError;
use crate::application::service::media::storage::StorageResponse;

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
}

#[async_trait]
impl MediaStorage for LocalStorage {
    fn temp_root(&self) -> &str {
        &self.temp_root
    }

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
            // If it's a directory, we move entire directory to the target destination, which is faster than moving files one by one
            else if file_type.is_dir() {
                let dir_name = entry.file_name();
                let from_path = entry.path();
                let to_path = to.join(dir_name);
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
}
