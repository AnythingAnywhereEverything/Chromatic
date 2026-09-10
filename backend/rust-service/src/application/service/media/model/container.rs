use uuid::Uuid;

use crate::application::repository::media::row::MediaType;
use crate::application::service::errors::media_service::ContainerError;
use crate::application::service::media::processor::types::MediaProcessorOptions;
use crate::application::service::media::storage::{PersistentStore, TempStore};
use crate::application::service::snowflake_service::SnowflakeGenerator;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum NamingStrategy {
    FinalHash,
    OriginalName,
    FileId,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileContainer {
    uploader_id: Option<i64>,
    container_type: String,
    container_id: String,
    relative_path: String,
    full_path: PathBuf,
    files: Vec<super::File>,
    target_path: Option<String>,
    config: Option<ContainerConfig>,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RecentMediaType {
    Avatar,
    Banner,
}

/// Use on service layer to configure the container's behavior and storage options.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContainerConfig {
    check_conflict: Option<RecentMediaType>,
    naming_strategy: NamingStrategy,
    processing_options: Option<MediaProcessorOptions>,
    generate_thumbhash: bool,
    animated_image_indicator: bool,
    is_file_id_contained: bool,
}

/// Resolved duplicate file ID
/// use full for when there are more than 1 media kinds
/// e.g. video that has thumbnail image associated with it
#[derive(Debug, Clone)]
pub struct ResolvedFileContainer {
    pub id: i64,
    pub check_conflict_type: Option<RecentMediaType>,
    pub original_name: String,
    pub original_content_type: String,
    pub file_type: MediaType,
    pub has_post_processing: bool,
    pub flags: i64,
    pub meta: Option<ResolvedMeta>,
    pub files: Vec<super::File>,
}

#[derive(Debug, Clone)]
pub struct ResolvedMeta {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,
}

impl FileContainer {
    pub fn resolve_files(&self) -> Vec<ResolvedFileContainer> {
        let mut resolved_files_map: std::collections::HashMap<i64, ResolvedFileContainer> = std::collections::HashMap::new();

        for file in &self.files {
            if let Some(file_id) = file.id() {
                let entry = resolved_files_map.entry(file_id).or_insert_with(|| ResolvedFileContainer {
                    id: file_id,
                    check_conflict_type: self.config.as_ref().and_then(|c| c.get_check_conflict()).cloned(),
                    original_name: file.original_name().unwrap_or_default().to_string(),
                    original_content_type: file.original_content_type().unwrap_or_default().to_string(),
                    file_type: file.category().clone(),
                    has_post_processing: false,
                    flags: file.flags(),
                    meta: Some(ResolvedMeta {
                        width: file.width(),
                        height: file.height(),
                        duration: file.duration(),
                    }),
                    files: Vec::new(),
                });
                entry.files.push(file.clone());
            }
        }

        resolved_files_map.into_values().collect()
    }
}

impl ContainerConfig {
    pub fn new() -> Self {
        Self {
            check_conflict: None,
            naming_strategy: NamingStrategy::FileId,
            generate_thumbhash: false,
            animated_image_indicator: false,
            processing_options: None,
            is_file_id_contained: false,
        }
    }

    pub fn is_file_id_contained(&self) -> bool {
        self.is_file_id_contained
    }

    pub fn set_file_id_contained(mut self, contained: bool) -> Self {
        self.is_file_id_contained = contained;
        self
    }

    pub fn get_naming_strategy(&self) -> &NamingStrategy {
        &self.naming_strategy
    }

    pub fn set_check_conflict(mut self, check_conflict: RecentMediaType) -> Self {
        self.check_conflict = Some(check_conflict);
        self
    }

    pub fn get_check_conflict(&self) -> Option<&RecentMediaType> {
        self.check_conflict.as_ref()
    }

    pub fn get_processing_options(&self) -> Option<&MediaProcessorOptions> {
        self.processing_options.as_ref()
    }

    pub fn is_generate_thumbhash(&self) -> bool {
        self.generate_thumbhash
    }

    pub fn is_animated_image_indicator(&self) -> bool {
        self.animated_image_indicator
    }

    pub fn set_generate_thumbhash(mut self, enabled: bool) -> Self {
        self.generate_thumbhash = enabled;
        self
    }

    pub fn set_animated_image_indicator(mut self, enabled: bool) -> Self {
        self.animated_image_indicator = enabled;
        self
    }

    pub fn set_processing_options(mut self, options: MediaProcessorOptions) -> Self {
        self.processing_options = Some(options);
        self
    }

    pub fn set_naming_strategy(mut self, strategy: NamingStrategy) -> Self {
        self.naming_strategy = strategy;
        self
    }
}

/// Specialized methods file container operations.
impl FileContainer {
    /// Use this for thread-safe id generation for files within the container.
    pub fn prepare_ids(
        &mut self,
        snowflake_generator: &SnowflakeGenerator,
    ) -> Result<&mut Self, ContainerError> {
        for file in &mut self.files {
            let file_id = snowflake_generator.generate_id()?;
            file.set_id(file_id);
        }
        Ok(self)
    }

    pub fn file_id_contained(&mut self) -> Result<&mut Self, ContainerError> {
        for file in &self.files {
            if file.id().is_none() {
                tracing::warn!(
                    "File with name '{}' does not have an ID set. Operation skipped",
                    file.file_name()
                );
                return Err(ContainerError::FileWithoutId);
            }
        }
        // create new directory for the container based on each file's id
        for file in &mut self.files {
            let file_id = file.id().ok_or(ContainerError::FileWithoutId)?;
            let new_directory = format!("{}/{}", self.full_path.to_string_lossy(), file_id);
            let directory_key = format!("{}/{}", self.relative_path, file_id);
            file.move_to_directory(PathBuf::from(new_directory), &directory_key)?;
        }
        Ok(self)
    }
}

/// Basic Structure
impl FileContainer {
    pub async fn new(
        container_type: String,
        storage: &Arc<dyn TempStore>,
    ) -> Result<Self, ContainerError> {
        let container_id = Uuid::new_v4().to_string();
        let container_path = format!("{}/{}", container_type, container_id);
        storage.create_dir(&container_path).await?;

        let full_path = storage.path(&container_path);

        Ok(Self {
            uploader_id: None,
            container_type,
            container_id,
            relative_path: container_path,
            full_path: full_path,
            files: Vec::new(),
            target_path: None,
            config: None,
        })
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    pub fn set_uploader_id(&mut self, uploader_id: i64) -> &mut Self {
        self.uploader_id = Some(uploader_id);
        self
    }

    pub fn uploader_id(&self) -> Option<i64> {
        self.uploader_id
    }

    pub fn target_path(&self) -> Result<String, ContainerError> {
        self.target_path
            .clone()
            .ok_or(ContainerError::MissingTargetPath)
    }

    pub fn path(&self) -> &str {
        &self.relative_path
    }

    pub fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    pub fn set_config(&mut self, config: ContainerConfig) {
        self.config = Some(config);
    }

    pub fn config(&self) -> Option<ContainerConfig> {
        self.config.clone()
    }

    pub fn set_target_path(&mut self, target_path: String) -> &mut Self {
        self.target_path = Some(target_path);
        self
    }

    pub fn add_file(&mut self, file: super::File) -> Result<(), ContainerError> {
        // check if the file is within the container's relative path
        if !file.file_directory().starts_with(&self.relative_path) {
            return Err(ContainerError::FileOutsideContainer);
        }

        self.files.push(file);
        Ok(())
    }

    /// Creates a new file in the container and returns a handle to it.
    pub fn create_file(&mut self) -> Result<&mut super::File, ContainerError> {
        let file_id = Uuid::new_v4().to_string();
        let mut file = super::File::new();

        let directory_full_path = self.full_path.clone();

        file.set_file_name(file_id)
            .set_current_extension(".part".to_string())?
            .set_file_directory(self.relative_path.clone())
            .set_full_file_directory(directory_full_path);

        self.files.push(file);

        self.files.last_mut().ok_or(ContainerError::FileJustPushed)
    }

    pub fn get_file_mut(&mut self, index: usize) -> Option<&mut super::File> {
        self.files.get_mut(index)
    }

    pub fn files(&self, index: usize) -> Option<&super::File> {
        self.files.get(index)
    }

    pub async fn abort(&mut self, storage: Arc<dyn TempStore>) -> Result<(), ContainerError> {
        self.files.clear();
        storage.delete(&self.relative_path).await?;
        Ok(())
    }

    /// Special case if need retain datas.
    pub async fn abort_retain(&mut self, storage: Arc<dyn TempStore>) -> Result<(), ContainerError> {
        storage.delete(&self.relative_path).await?;
        Ok(())
    }

    pub fn length(&self) -> usize {
        self.files.len()
    }

    pub fn files_mut(&mut self) -> &mut Vec<super::File> {
        &mut self.files
    }

    pub fn take_files(&mut self) -> Vec<super::File> {
        std::mem::take(&mut self.files)
    }

    pub fn replace_files(&mut self, files: Vec<super::File>) {
        self.files = files;
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub async fn finalize(
        &mut self,
        persistent: Arc<dyn PersistentStore>,
        temp_store: Arc<dyn TempStore>,
    ) -> Result<(), ContainerError> {
        let result = async {
            if self.files.is_empty() {
                return Err(ContainerError::EmptyContainer);
            }

            let target_path = match &self.target_path {
                Some(path) => path,
                None => return Err(ContainerError::MissingTargetPath),
            };

            persistent.put_dir(&self.full_path, &target_path).await?;

            Ok(())
        }
        .await;

        // about if error
        if result.is_err() {
            tracing::error!(
                "Error finalizing container: {:?}. Attempting to clean up temp storage.",
                result.as_ref().err()
            );
        }

        // attempt clean up directory in temp storage regardless of finalization result
        if let Err(cleanup_err) = temp_store.delete(&self.relative_path).await {
            tracing::error!(
                "Failed to clean up temp storage for container {}: {:?}",
                self.relative_path,
                cleanup_err
            );
        }

        result
    }
}
