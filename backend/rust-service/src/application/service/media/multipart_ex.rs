// src/application/service/media/multipart_ex.rs

use crate::application::service::{
    errors::MediaServiceError,
    media::{
        storage::MediaStorage,
        types::{MediaType, TempUpload, ValidationOptions},
    },
};
use axum::extract::Multipart;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultipartFieldKind {
    Text,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub trait MultipartSchema {
    fn multipart_fields() -> &'static [MultipartField];

    fn multipart_field(name: &str) -> Option<&'static MultipartField> {
        Self::multipart_fields()
            .iter()
            .find(|field| field.name == name)
    }
}

pub struct MultipartLimits {
    pub max_file_size: usize,
    pub max_files: usize,
}

pub struct MultipartExtractorOptions {
    // Limit for other types of files
    pub limits: MultipartLimits,

    // Optional validation options for uploaded files
    pub validation: Option<ValidationOptions>,

    // Optional size filter gates for uploaded files
    // This allows for different size limits based on the media type of the uploaded file.
    pub size_filter_gate: Option<Vec<FileSizeGate>>,
}

impl Default for MultipartExtractorOptions {
    fn default() -> Self {
        Self {
            limits: MultipartLimits {
                max_file_size: 10 * 1024 * 1024, // 10 MB
                max_files: 5,
            },
            validation: None,
            size_filter_gate: None,
        }
    }
}

pub struct FileSizeGate {
    pub media_type: MediaType,
    pub max_size: usize,
}

pub struct MultipartExtractor {
    storage: Arc<dyn MediaStorage>,
}

impl MultipartExtractor {
    pub fn new(storage: Arc<dyn MediaStorage>) -> Self {
        Self { storage }
    }

    async fn cleanup_uploads(&self, uploads: &[TempUpload]) {
        for upload in uploads {
            self.storage.delete_temp(&upload.path).await;

            tracing::trace!(
                path = %upload.path,
                size = upload.size,
                "Deleted temporary upload"
            );
        }
    }

    pub async fn extract<T>(
        &self,
        mut multipart: Multipart,
        options: MultipartExtractorOptions,
    ) -> Result<T, MediaServiceError>
    where
        T: DeserializeOwned + MultipartSchema,
    {
        let mut uploaded_files = Vec::new();

        let result = self
            .extract_inner::<T>(&mut multipart, &mut uploaded_files, &options)
            .await;

        match result {
            Ok(value) => Ok(value),

            Err(error) => {
                self.cleanup_uploads(&uploaded_files).await;
                Err(error)
            }
        }
    }

    async fn extract_inner<T>(
        &self,
        multipart: &mut Multipart,
        uploaded_files: &mut Vec<TempUpload>,
        options: &MultipartExtractorOptions,
    ) -> Result<T, MediaServiceError>
    where
        T: DeserializeOwned + MultipartSchema,
    {
        let mut values = Map::new();
        let mut file_count = 0usize;

        while let Some(mut field) = multipart.next_field().await.map_err(|error| {
            tracing::error!("Multipart field extraction failed: {}", error);

            MediaServiceError::MultipartError(error)
        })? {
            let name = field
                .name()
                .ok_or(MediaServiceError::UnableToExtract)?
                .to_owned();

            let schema = T::multipart_field(&name)
                .ok_or_else(|| MediaServiceError::UnknownMultipartField(name.clone()))?;

            match schema.kind {
                MultipartFieldKind::File => {
                    if file_count >= options.limits.max_files {
                        return Err(MediaServiceError::TooManyFiles(options.limits.max_files));
                    }

                    match schema.cardinality {
                        MultipartFieldCardinality::Single | MultipartFieldCardinality::Optional => {
                            if values.contains_key(&name) {
                                return Err(MediaServiceError::DuplicateMultipartField(name));
                            }
                        }

                        MultipartFieldCardinality::Many
                        | MultipartFieldCardinality::OptionalMany => {}
                    }

                    file_count += 1;

                    let upload = self
                        .storage
                        .save_temp_stream(
                            &mut field,
                            options.limits.max_file_size,
                            options.validation.as_ref(),
                            options.size_filter_gate.as_ref(),
                        )
                        .await?;

                    // * Track every successful upload for rollback.
                    uploaded_files.push(upload.clone());

                    let upload_value = serde_json::to_value(upload).map_err(|error| {
                        tracing::error!("Failed to serialize TempUpload: {}", error);

                        MediaServiceError::InvalidMultipartField(name.clone())
                    })?;

                    match schema.cardinality {
                        MultipartFieldCardinality::Single | MultipartFieldCardinality::Optional => {
                            values.insert(name, upload_value);
                        }

                        MultipartFieldCardinality::Many
                        | MultipartFieldCardinality::OptionalMany => {
                            let entry_name = name.clone();

                            let entry = values
                                .entry(name)
                                .or_insert_with(|| Value::Array(Vec::new()));

                            let array = entry.as_array_mut().ok_or_else(|| {
                                MediaServiceError::InvalidMultipartField(entry_name)
                            })?;

                            array.push(upload_value);
                        }
                    }
                }

                MultipartFieldKind::Text => {
                    let value = field.text().await.map_err(|error| {
                        tracing::error!("Failed to read multipart field '{}': {}", name, error);

                        MediaServiceError::UnableToExtract
                    })?;

                    let value = parse_multipart_value(&value);

                    match schema.cardinality {
                        MultipartFieldCardinality::Many
                        | MultipartFieldCardinality::OptionalMany => {
                            let entry_name = name.clone();

                            let entry = values
                                .entry(name)
                                .or_insert_with(|| Value::Array(Vec::new()));

                            let array = entry.as_array_mut().ok_or_else(|| {
                                MediaServiceError::InvalidMultipartField(entry_name)
                            })?;

                            array.push(value);
                        }

                        MultipartFieldCardinality::Single | MultipartFieldCardinality::Optional => {
                            if values.contains_key(&name) {
                                return Err(MediaServiceError::DuplicateMultipartField(name));
                            }

                            values.insert(name, value);
                        }
                    }
                }
            }
        }

        serde_json::from_value(Value::Object(values)).map_err(|error| {
            tracing::error!("Multipart payload deserialization failed: {}", error);

            MediaServiceError::UnableToExtract
        })
    }
}

fn parse_multipart_value(value: &str) -> Value {
    // * Parse JSON primitives first, then treat the value as String.
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_owned()))
}
