use crate::application::service::{
    errors::MediaServiceError,
    media::{
        storage::MediaStorage,
        types::media_options::{
            MultipartExtractorOptions, MultipartFieldCardinality, MultipartFieldKind,
            MultipartSchema, TempUpload,
        },
    },
};
use axum::extract::Multipart;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::sync::Arc;

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

            tracing::trace!("Name check: {}", name);

            let schema = T::multipart_field(&name)
                .ok_or_else(|| MediaServiceError::UnknownMultipartField(name.clone()))?;

            match schema.kind {
                MultipartFieldKind::File => {
                    if file_count >= options.limits.max_files {
                        return Err(MediaServiceError::TooManyFiles(
                            options.limits.max_files,
                        ));
                    }

                    match schema.cardinality {
                        MultipartFieldCardinality::Single
                        | MultipartFieldCardinality::Optional => {
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
                        MultipartFieldCardinality::Single
                        | MultipartFieldCardinality::Optional => {
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
                        tracing::error!(
                            "Failed to read multipart field '{}': {}",
                            name,
                            error
                        );
                        MediaServiceError::UnableToExtract
                    })?;

                    tracing::trace!("Raw multipart value '{}': {}", name, value);

                    let value = parse_multipart_value(&value);

                    tracing::trace!("Parsed multipart value '{}': {}", name, value);

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

                            // * If the multipart value itself is an array, flatten it.
                            // * This allows Vec<i64> from either repeated fields or `[1, 2, 3]`.
                            match value {
                                Value::Array(items) => {
                                    array.extend(items);
                                }
                                value => {
                                    array.push(value);
                                }
                            }
                        }

                        MultipartFieldCardinality::Single
                        | MultipartFieldCardinality::Optional => {
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
    // * JSON arrays/objects/primitives are preserved as their actual JSON types.
    // * Non-JSON input remains a String.
    serde_json::from_str(value.trim())
        .unwrap_or_else(|_| Value::String(value.to_owned()))
}