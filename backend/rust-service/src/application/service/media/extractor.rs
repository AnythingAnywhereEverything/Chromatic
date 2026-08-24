use crate::application::service::{errors::media_service::ExtractionError, media::{
    inspector::{FileType, MediaKind, get_file_type, inspect_bytes},
    storage::TempStore,
}};
use axum::extract::multipart::{Field, Multipart};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub trait MultipartSchema {
    fn multipart_fields() -> &'static [MultipartField];

    fn multipart_field(name: &str) -> Option<&'static MultipartField> {
        Self::multipart_fields()
            .iter()
            .find(|field| field.name == name)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MultipartFieldKind {
    Text,
    File,
}

#[derive(Debug, Clone, Copy)]
pub enum MultipartFieldCardinality {
    Single,
    Optional,
    Many,
    OptionalMany,
}

#[derive(Debug, Clone, Copy)]
pub struct MultipartField {
    pub name: &'static str,
    pub kind: MultipartFieldKind,
    pub cardinality: MultipartFieldCardinality,
}

pub struct MultipartExtractor {
    pub working_storage: Arc<dyn TempStore>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationType {
    Whitelisted,
    Blacklisted,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileOptions {
    pub file_type: FileType,
    pub file_size: Option<u64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValidationOptions {
    pub validation_type: ValidationType,
    pub value: Vec<FileOptions>,
}

impl ValidationOptions {
    pub fn new(validation_type: ValidationType) -> Self {
        Self {
            validation_type,
            value: Vec::new(),
        }
    }

    pub fn new_whitelist() -> Self {
        Self {
            validation_type: ValidationType::Whitelisted,
            value: Vec::new(),
        }
    }

    pub fn new_blacklist() -> Self {
        Self {
            validation_type: ValidationType::Blacklisted,
            value: Vec::new(),
        }
    }

    pub fn add_type(mut self, file_type: FileType) -> Self {
        self.value.push(FileOptions {
            file_type,
            file_size: None,
        });
        self
    }

    pub fn add_type_with_size(mut self, file_type: FileType, file_size: u64) -> Self {
        self.value.push(FileOptions {
            file_type,
            file_size: Some(file_size),
        });
        self
    }
}

#[derive(Debug, Clone)]
pub struct ContainerFieldOptions {
    pub field_name: String,
    pub max_size: Option<u64>,
    pub max_files: Option<usize>,
    pub validation: Option<ValidationOptions>,
}

impl ContainerFieldOptions {
    pub fn new(field_name: String) -> Self {
        Self {
            field_name,
            max_size: None,
            max_files: None,
            validation: None,
        }
    }

    pub fn set_max_size(mut self, max_size: u64) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn set_max_files(mut self, max_files: usize) -> Self {
        self.max_files = Some(max_files);
        self
    }

    pub fn set_validation(mut self, validation: ValidationOptions) -> Self {
        self.validation = Some(validation);
        self
    }
}

/// Global variables
/// regardless of field
#[derive(Debug, Clone)]
pub struct ExtractorFileOptions {
    pub max_size: Option<u64>,
    pub max_files: Option<usize>,
    pub validation: Option<ValidationOptions>,
    // Per Container options
    // ! Will not override global options
    pub field_options: Option<Vec<ContainerFieldOptions>>,
}

impl Default for ExtractorFileOptions {
    fn default() -> Self {
        Self {
            max_size: None,
            max_files: None,
            validation: None,
            field_options: None,
        }
    }
}

async fn validate_file_type(
    chunk: &[u8],
    validation: &ValidationOptions,
) -> Result<(String, String, Vec<FileType>, MediaKind), ExtractionError> {
    let inspected = inspect_bytes(chunk).await?;
    let detected_extension = inspected.extension.clone();
    let detected_mime = inspected.mime.clone();
    let kind = inspected.kind.clone();
    let file_types = get_file_type(&detected_mime);

    let is_valid = match validation.validation_type {
        ValidationType::Whitelisted => file_types
            .iter()
            .any(|ft| validation.value.iter().any(|opt| opt.file_type == *ft)),
        ValidationType::Blacklisted => !file_types
            .iter()
            .any(|ft| validation.value.iter().any(|opt| opt.file_type == *ft)),
    };

    if !is_valid {
        tracing::error!(
            "Dropped connection due to invalid file type: {:?}",
            inspected.mime,
        );
        return Err(ExtractionError::InvalidFileType(inspected.mime));
    }

    Ok((detected_extension, detected_mime, file_types, kind))
}

impl MultipartExtractor {
    pub fn new(working_storage: Arc<dyn TempStore>) -> Self {
        Self { working_storage }
    }

    async fn save_stream(
        &self,
        container: &mut super::model::FileContainer,
        field: &mut Field<'_>,
        max_size: Option<u64>,
        validation: Option<ValidationOptions>,
    ) -> Result<(), ExtractionError> {
        tracing::info!(
            "Saving file stream for field: {}",
            field.name().unwrap_or("unknown")
        );
        let file = container.create_file()?;

        let file_path = file.file_full_path();

        tracing::info!("Saving file stream to temporary path: {:?}", file_path);

        let mut temp_file = tokio::fs::File::create(&file_path).await?;

        let mut written_bytes: usize = 0;
        // * Monitor Variables
        let start_time = std::time::Instant::now();
        let min_bytes_per_second = 512;

        let mut is_validated = false;
        let mut detected_extension = String::new();
        let mut detected_mime = String::new();

        let mut file_type = Vec::new();
        let mut kind = MediaKind::Generic;

        while let Some(chunk) =
            tokio::time::timeout(std::time::Duration::from_secs(10), field.chunk())
                .await
                .map_err(|_| ExtractionError::Timeout)?
                .map_err(|e| ExtractionError::MultipartError(e))?
        {
            written_bytes += chunk.len();
            if let Some(max_size) = max_size {
                if written_bytes as u64 > max_size {
                    tracing::warn!(
                        "Dropped connection due to exceeding max file size: {} bytes",
                        max_size
                    );
                    return Err(ExtractionError::TransmissionTooLarge);
                }
            }

            // check size based on validation options if provided
            if !file_type.is_empty() && validation.is_some() {
                let validation = validation.as_ref().unwrap();
                for ft in &file_type {
                    if let Some(file_opt) = validation.value.iter().find(|opt| opt.file_type == *ft)
                    {
                        if let Some(file_size) = file_opt.file_size {
                            if written_bytes as u64 > file_size {
                                tracing::warn!(
                                    "Dropped connection due to exceeding max file size for type {:?}: {} bytes",
                                    ft,
                                    file_size
                                );
                                return Err(ExtractionError::TransmissionTooLarge);
                            }
                        }
                    }
                }
            }

            // Validate the first 8 KB of the file to determine its type and validate against the provided options
            if !is_validated && chunk.len() > 8192 {
                // Inspect the chunk to determine the file type
                let (de, dm, ft, kd) = if let Some(validation) = &validation {
                    validate_file_type(&chunk[..8192], validation).await?
                } else {
                    let inspected = inspect_bytes(&chunk[..8192]).await?;
                    let ft = get_file_type(&inspected.mime);
                    (inspected.extension, inspected.mime, ft, inspected.kind)
                };
                detected_extension = de;
                detected_mime = dm;
                file_type = ft;
                kind = kd;

                is_validated = true;
            }

            let elapsed = start_time.elapsed().as_secs();
            if elapsed > 5 {
                // Give the connection a 5-second grace period to spin up
                let throughput = written_bytes / (elapsed as usize);

                if throughput < min_bytes_per_second {
                    tracing::warn!("Dropped connection due to low throughput rate.");
                    return Err(ExtractionError::TransmissionTooSlow);
                }
            }

            temp_file.write_all(&chunk).await?;
        }

        temp_file.flush().await?;

        // If the file was not validated during the streaming process, perform a final validation after the stream is complete
        // in case the file is smaller than 8 KB and was not validated during the streaming process
        if !is_validated {
            let mut temp_file = tokio::fs::File::open(&file_path).await?;
            let mut buffer = vec![0; 8192];
            let bytes_read = temp_file.read(&mut buffer).await?;

            let (de, dm, ft, kd) = if let Some(validation) = &validation {
                validate_file_type(&buffer[..bytes_read], validation).await?
            } else {
                let inspected = inspect_bytes(&buffer[..bytes_read]).await?;
                let ft = get_file_type(&inspected.mime);
                (inspected.extension, inspected.mime, ft, inspected.kind)
            };
            detected_extension = de;
            detected_mime = dm;
            file_type = ft;
            kind = kd;

            if let Some(validation) = validation.as_ref() {
                for ft in &file_type {
                    if let Some(file_opt) = validation.value.iter().find(|opt| opt.file_type == *ft)
                    {
                        if let Some(file_size) = file_opt.file_size {
                            if written_bytes as u64 > file_size {
                                tracing::warn!(
                                    "Dropped connection due to exceeding max file size for type {:?}: {} bytes",
                                    ft,
                                    file_size
                                );
                                return Err(ExtractionError::TransmissionTooLarge);
                            }
                        }
                    }
                }
            }
        }

        file.set_size(written_bytes as u64)
            .set_original_content_type(field.content_type().map(|ct| ct.to_string()))
            .set_original_name(field.file_name().map(|name| name.to_string()))
            .set_detected_extension(detected_extension)
            .set_content_type(detected_mime)
            .set_kind(kind);

        temp_file.flush().await?;
        Ok(())
    }

    pub async fn extract<T>(
        &self,
        mut multipart: Multipart,
        opts: Option<ExtractorFileOptions>,
    ) -> Result<T, ExtractionError>
    where
        T: DeserializeOwned + MultipartSchema,
    {
        // check if options are valid, if not warn
        if let Some(global) = &opts {
            // Check for conflicts between global and validation options

            if global.max_size.is_some() && global.validation.is_some() {
                for validation in &global.validation.as_ref().unwrap().value {
                    if let Some(file_size) = validation.file_size {
                        if file_size > global.max_size.unwrap() {
                            tracing::warn!(
                                "Validation file size ({}) is greater than global max_size ({}). Force using Global.",
                                file_size,
                                global.max_size.unwrap()
                            );
                        }
                    }
                }
            }

            if let Some(field_opts) = &global.field_options {
                for field_opt in field_opts {
                    if (global.max_files.is_some() && field_opt.max_files.is_some())
                        && (global.max_files.unwrap() < field_opt.max_files.unwrap())
                    {
                        tracing::warn!(
                            "Field option max_files ({}) is greater than global max_files ({}). Force using Global.",
                            field_opt.max_files.unwrap(),
                            global.max_files.unwrap()
                        );
                    }

                    if (global.max_size.is_some() && field_opt.max_size.is_some())
                        && (global.max_size.unwrap() < field_opt.max_size.unwrap())
                    {
                        tracing::warn!(
                            "Field option max_size ({}) is greater than global max_size ({}). Force using Global.",
                            field_opt.max_size.unwrap(),
                            global.max_size.unwrap()
                        );
                    }

                    // Validation options are not checked for conflicts, as they are more complex and may require more context to validate properly.
                    if global.validation.is_some() && field_opt.validation.is_some() {
                        // Check if the validation types are different, if so, log an error and return an error
                        if global.validation.as_ref().unwrap().validation_type
                            != field_opt.validation.as_ref().unwrap().validation_type
                        {
                            tracing::error!(
                                "Field option validation type ({:?}) is different from global validation type ({:?}).",
                                field_opt.validation.as_ref().unwrap().validation_type,
                                global.validation.as_ref().unwrap().validation_type
                            );
                            Err(ExtractionError::Misconfigured)?;
                        }

                        // Check if the validation values within field option are within the global validation values, if not, log an error and return an error
                        let global_validation_values = &global.validation.as_ref().unwrap().value;
                        let field_validation_values = &field_opt.validation.as_ref().unwrap().value;
                        for field_validation_value in field_validation_values {
                            // compare types
                            if !global_validation_values
                                .iter()
                                .any(|global_validation_value| {
                                    global_validation_value.file_type
                                        == field_validation_value.file_type
                                })
                            {
                                tracing::error!(
                                    "Field option validation value ({:?}) is not within global validation values ({:?}).",
                                    field_validation_value.file_type,
                                    global_validation_values
                                        .iter()
                                        .map(|v| v.file_type.clone())
                                        .collect::<Vec<_>>()
                                );
                                Err(ExtractionError::Misconfigured)?;
                            }
                        }
                    }
                }
            }
        }

        self.extract_file_container(&mut multipart, opts).await
    }

    async fn extract_file_container<T>(
        &self,
        multipart: &mut Multipart,
        opts: Option<ExtractorFileOptions>,
    ) -> Result<T, ExtractionError>
    where
        T: DeserializeOwned + MultipartSchema,
    {
        tracing::info!(
            "Starting multipart extraction for type: {}",
            std::any::type_name::<T>()
        );
        tracing::info!("Using options: {:#?}", opts);

        let mut values = serde_json::Map::new();

        let mut global_file_count = 0usize;

        let mut container_map = std::collections::HashMap::new();

        let result = async {
            while let Some(mut field) = multipart.next_field().await? {
                let field_name = field
                    .name()
                    .ok_or(ExtractionError::UnableToReadFieldName)?
                    .to_owned();
                let field_schema = T::multipart_field(&field_name).ok_or_else(|| {
                    ExtractionError::UnknownMultipartField(field_name.to_string())
                })?;

                match field_schema.kind {
                    MultipartFieldKind::Text => {
                        let value = field.text().await.map_err(|e| {
                            ExtractionError::FailedToReadMultipartField(
                                field_name.to_string(),
                                e.to_string(),
                            )
                        })?;

                        let value = parse_multipart_value(&value);

                        match field_schema.cardinality {
                            MultipartFieldCardinality::Many
                            | MultipartFieldCardinality::OptionalMany => {
                                let entry_name = field_name.to_string();

                                let entry = values
                                    .entry(entry_name.clone())
                                    .or_insert_with(|| serde_json::Value::Array(Vec::new()));

                                let array = entry.as_array_mut().ok_or_else(|| {
                                    ExtractionError::FailedToReadMultipartField(
                                        entry_name.clone(),
                                        "Invalid multipart field".to_string(),
                                    )
                                })?;

                                // * If the multipart value itself is an array, flatten it.
                                // * This allows Vec<i64> from either repeated fields or `[1, 2, 3]`.
                                match value {
                                    serde_json::Value::Array(items) => {
                                        array.extend(items);
                                    }
                                    value => {
                                        array.push(value);
                                    }
                                }
                            }

                            MultipartFieldCardinality::Single
                            | MultipartFieldCardinality::Optional => {
                                if values.contains_key(&field_name) {
                                    return Err(ExtractionError::DuplicateMultipartField(
                                        field_name.to_string(),
                                    ));
                                }

                                values.insert(field_name.to_string(), value);
                            }
                        }
                    }
                    MultipartFieldKind::File => {
                        let field_name = field_name.to_string();
                        global_file_count += 1;
                        if let Some(opts) = &opts {
                            if let Some(max_files) = opts.max_files {
                                if global_file_count > max_files {
                                    return Err(ExtractionError::TransmissionTooManyFiles);
                                }
                            }
                        }

                        tracing::info!("Processing file field: {}", field_name);
                        if !container_map.contains_key(&field_name) {
                            tracing::info!("Creating new file container for field: {}", field_name);
                            let container = super::model::FileContainer::new(
                                "upload".to_string(),
                                &self.working_storage,
                            )
                            .await
                            .map_err(|e| {
                                ExtractionError::FailedToReadMultipartField(
                                    field_name.clone(),
                                    e.to_string(),
                                )
                            })?;

                            container_map.insert(field_name.clone(), container);
                        }

                        tracing::info!("Saving file stream for field: {}", field_name);

                        let container = container_map
                            .get_mut(&field_name)
                            .expect("container was just inserted");

                        // check if there is a field specific option for max files first

                        let field_opt = opts.as_ref().and_then(|opts| {
                            opts.field_options.as_ref().and_then(|field_opts| {
                                field_opts.iter().find(|opt| opt.field_name == field_name)
                            })
                        });

                        // due to global was previously checked, we can safely check for field specific max files without checking global max files
                        if let Some(field_opt) = field_opt {
                            if let Some(max_files) = field_opt.max_files {
                                if container.length() >= max_files {
                                    return Err(ExtractionError::TransmissionTooManyFiles);
                                }
                            }
                        }

                        let field_max_size = field_opt.and_then(|opt| opt.max_size);

                        let max_size = {
                            let global_max_size = opts.as_ref().and_then(|opts| opts.max_size);
                            if global_max_size.is_some() && field_max_size.is_some() {
                                if global_max_size.unwrap() < field_max_size.unwrap() {
                                    global_max_size
                                } else {
                                    field_max_size
                                }
                            } else {
                                field_max_size.or(global_max_size)
                            }
                        };

                        let validation = {
                            // create new validation as both merge global and field validation options
                            let global_validation =
                                opts.as_ref().and_then(|opts| opts.validation.clone());
                            let field_validation = field_opt.and_then(|opt| opt.validation.clone());

                            // map based on field validation, filter out the types that are not in the global validation if both exist
                            // additionally, compare the file size in the field validation with the global validation,
                            // if the field validation file size is greater than the global validation file size, use the global validation file size instead
                            // if the global validation file size is None, use the field validation file size
                            match (global_validation, field_validation) {
                                (Some(global), Some(field)) => {
                                    let filtered_field_value: Vec<FileOptions> = field
                                        .value
                                        .into_iter()
                                        .filter(|field_opt| {
                                            global.value.iter().any(|global_opt| {
                                                global_opt.file_type == field_opt.file_type
                                            })
                                        })
                                        .map(|field_opt| {
                                            tracing::info!(
                                                "Merging validation options for file type: {:?}",
                                                field_opt.file_type
                                            );
                                            let global_opt = global
                                                .value
                                                .iter()
                                                .find(|global_opt| {
                                                    global_opt.file_type == field_opt.file_type
                                                })
                                                .unwrap();

                                            tracing::info!(
                                                "Global file size: {:?}, Field file size: {:?}",
                                                global_opt.file_size,
                                                field_opt.file_size
                                            );
                                            let file_size = match (
                                                global_opt.file_size,
                                                field_opt.file_size,
                                                max_size,
                                            ) {
                                                // If all three are Some, take the minimum of the three
                                                (
                                                    Some(global_size),
                                                    Some(field_size),
                                                    Some(max_size),
                                                ) => {
                                                    Some(global_size.min(field_size).min(max_size))
                                                }
                                                // If two are Some, take the minimum of the two
                                                (Some(global_size), Some(field_size), None) => {
                                                    Some(global_size.min(field_size))
                                                }
                                                (Some(global_size), None, Some(max_size)) => {
                                                    Some(global_size.min(max_size))
                                                }
                                                (None, Some(field_size), Some(max_size)) => {
                                                    Some(field_size.min(max_size))
                                                }
                                                // If one is Some, take that one
                                                (Some(global_size), None, None) => {
                                                    Some(global_size)
                                                }
                                                (None, Some(field_size), None) => Some(field_size),
                                                (None, None, Some(max_size)) => Some(max_size),
                                                // If all three are None, return None
                                                (None, None, None) => None,
                                            };
                                            tracing::info!(
                                                "Using file size: {:?} for file type: {:?}",
                                                file_size,
                                                field_opt.file_type
                                            );
                                            FileOptions {
                                                file_type: field_opt.file_type,
                                                file_size,
                                            }
                                        })
                                        .collect();
                                    Some(ValidationOptions {
                                        validation_type: global.validation_type,
                                        value: filtered_field_value,
                                    })
                                }
                                (Some(global), None) => Some(global),
                                (None, Some(field)) => Some(field),
                                (None, None) => None,
                            }
                        };

                        tracing::info!(
                            "Using max_size: {:?} and validation: {:?} for field: {}",
                            max_size,
                            validation,
                            field_name
                        );

                        self.save_stream(container, &mut field, max_size, validation)
                            .await?;

                        tracing::info!("Finished saving file stream for field: {}", field_name);
                    }
                }
            }

            // finally, deserialize the container map into the target struct
            // Example
            // username: String
            // profile_picture: FileContainer

            for (field_name, container) in &container_map {
                let value = serde_json::to_value(container)
                    .map_err(|e| ExtractionError::DeserializationFailed(e.to_string()))?;

                values.insert(field_name.clone(), value);
            }

            serde_json::from_value(serde_json::Value::Object(values))
                .map_err(|e| ExtractionError::DeserializationFailed(e.to_string()))
        }
        .await;

        if result.is_err() {
            for (field_name, container) in container_map.iter_mut() {
                tracing::info!("Aborting file container for field: {}", field_name);
                if let Err(e) = container.abort(self.working_storage.clone()).await {
                    tracing::error!("Failed to abort file container '{}': {}", field_name, e);
                }
            }
        }

        result
    }
}

fn parse_multipart_value(value: &str) -> serde_json::Value {
    // * JSON arrays/objects/primitives are preserved as their actual JSON types.
    // * Non-JSON input remains a String.
    serde_json::from_str(value.trim())
        .unwrap_or_else(|_| serde_json::Value::String(value.to_owned()))
}

// * Holy file extractor, I know.
