use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::application::service::errors::media_service::StorageError;

#[async_trait]
pub trait TempStore: Send + Sync {
    fn root(&self) -> &Path;
    fn path(&self, key: &str) -> PathBuf;
    async fn copy_to_temp(&self, source: &Path, key: &str) -> Result<(), StorageError>;
    async fn create_dir(&self, key: &str) -> Result<(), StorageError>;
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;
}

#[async_trait]
pub trait PersistentStore: Send + Sync {
    async fn put_file(&self, source: &Path, key: &str) -> Result<(), StorageError>;
    async fn put_dir(&self, source: &Path, key: &str) -> Result<(), StorageError>;
    async fn put_bytes(&self, bytes: &[u8], key: &str) -> Result<(), StorageError>;
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;
    async fn read_file(&self, key: &str) -> Result<(tokio::fs::File, PathBuf), StorageError>;
}

pub struct LocalStorage {
    temp_root: PathBuf,
    persistent_root: PathBuf,
}

impl LocalStorage {
    pub fn new(temp_root: &str, persistent_root: &str) -> Self {
        Self {
            temp_root: PathBuf::from(temp_root),
            persistent_root: PathBuf::from(persistent_root),
        }
    }
}

#[async_trait]
impl TempStore for LocalStorage {
    fn root(&self) -> &Path {
        &self.temp_root
    }

    fn path(&self, key: &str) -> PathBuf {
        self.temp_root.join(key)
    }

    async fn copy_to_temp(&self, source: &Path, key: &str) -> Result<(), StorageError> {
        let dest_path = self.path(key);
        if let Some(parent) = dest_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::copy(source, &dest_path).await?;
        Ok(())
    }

    async fn create_dir(&self, key: &str) -> Result<(), StorageError> {
        tracing::info!("Creating directory in temp storage: {}", key);
        let dir_path = self.path(key);
        tracing::info!("Full path for new directory: {:?}", dir_path);
        tokio::fs::create_dir_all(&dir_path).await?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let dir_path = self.path(key);
        tokio::fs::remove_dir_all(&dir_path).await?;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        let dir_path = self.path(key);
        Ok(tokio::fs::metadata(&dir_path).await?.is_dir())
    }
}

#[async_trait]
impl PersistentStore for LocalStorage {
    /// Put a file from a temporary location to a persistent location.
    /// * For local storage, it is better to move the file rather than copy it.
    async fn put_file(&self, source: &Path, key: &str) -> Result<(), StorageError> {
        let dest_path = self.persistent_root.join(key);
        if let Some(parent) = dest_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::rename(source, &dest_path).await?;
        Ok(())
    }

    async fn read_file(&self, key: &str) -> Result<(tokio::fs::File, PathBuf), StorageError> {
        let file_path = self.persistent_root.join(key);
        let file = tokio::fs::File::open(&file_path).await?;
        Ok((file, file_path))
    }

    async fn put_dir(&self, source: &Path, key: &str) -> Result<(), StorageError> {
        let dest_path = self.persistent_root.join(key);
        
        // Ensure the destination directory exists
        if let Some(parent) = dest_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // try moving directory directly, if it fails, move each entries one by one
        if let Err(_) = tokio::fs::rename(source, &dest_path).await {
            // tries to move each entries from source to destination directory
            let mut entries = tokio::fs::read_dir(source).await?;
            while let Some(entry) = entries.next_entry().await? {
                let entry_type = entry.file_type().await?;
                let file_name = entry.file_name();
                let from_path = entry.path();
                let to_path = dest_path.join(file_name.clone());

                if entry_type.is_dir() {
                    tokio::fs::create_dir_all(&to_path).await?;
                    let key = format!("{}/{}", key, file_name.into_string().unwrap_or_default());
                    Self::put_dir(&self, &from_path, &key).await?;
                } else {
                    tokio::fs::rename(&from_path, &to_path).await?;
                }
            }
        }

        Ok(())
    }

    async fn put_bytes(&self, bytes: &[u8], key: &str) -> Result<(), StorageError> {
        let dest_path = self.persistent_root.join(key);
        if let Some(parent) = dest_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&dest_path, bytes).await?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let path: PathBuf = self.persistent_root.join(key);

        // check if key is a file or directory
        let metadata = tokio::fs::metadata(&path).await?;
        if metadata.is_file() {
            tokio::fs::remove_file(&path).await?;
        } else if metadata.is_dir() {
            tokio::fs::remove_dir_all(&path).await?;
        }
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        let dir_path = self.persistent_root.join(key);
        Ok(tokio::fs::metadata(&dir_path).await?.is_dir())
    }
}
