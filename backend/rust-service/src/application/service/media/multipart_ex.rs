// src/application/service/media/multipart_ex.rs

use crate::application::{service::{
    errors::MediaServiceError, media::{
        storage::MediaStorage, types::{file::MultipartFile, media_options::{
            MultipartExtractorOptions, MultipartFieldCardinality, MultipartFieldKind, MultipartSchema
        }}
    },
}};
use axum::extract::{Multipart, multipart::Field};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;
use std::{sync::Arc, time::Instant};

pub struct MultipartExtractor {
    storage: Arc<dyn MediaStorage>,
}

impl MultipartExtractor {
    pub fn new(storage: Arc<dyn MediaStorage>) -> Self {
        Self { storage }
    }

    async fn cleanup_uploads(&self, uploads: &[MultipartFile]) {
        for upload in uploads {
            self.storage.delete_temp(&upload.get_relative_path()).await;

            tracing::trace!(
                path = %&upload.get_relative_path(),
                size = upload.get_size(),
                "Deleted temporary upload"
            );
        }
    }

    async fn save_temp_stream(
        &self,
        field: &mut Field<'_>,
        options: &MultipartExtractorOptions,
    ) -> Result<MultipartFile, MediaServiceError> {
        // * Generate a unique temporary file path
        let name = Uuid::new_v4().to_string();
        let relative_path = format!("upload/{}.part", name);
        let full_path = self.storage.temp_full_path(&relative_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = fs::File::create(&full_path).await?;
        let mut written = 0usize;
        let mut mime = String::new();
        let mut extension = String::new();
        let mut category = super::types::media_options::MediaCategory::Unknown;

        let write_result = async {
            // * throughput monitoring variables
            let start_time = Instant::now();
            let min_bytes_per_second = 512;

            // * cancel/disconnect is detected here when the multipart stream read fails
            while let Some(chunk) =
                tokio::time::timeout(std::time::Duration::from_secs(10), field.chunk())
                    .await
                    .map_err(|_| MediaServiceError::Timeout)?
                    .map_err(|e| MediaServiceError::MultipartError(e))?
            {
                written += chunk.len();

                if let Some(options) = Some(options) {
                    if let Some(max_file_size) = options.max_file_size {
                        if written > max_file_size {
                            return Err(MediaServiceError::FileTooLarge);
                        }
                    }

                    if (!mime.is_empty() || !extension.is_empty()) && chunk.len() >= 8192 {
                        // pass or err
                        super::utils::validate_media_type(&mime, &options.validation)?;

                        // compare against allowed types if provided
                        if let Some(filters) = &options.filter {
                            let types = super::utils::get_media_types_from_mime(&mime);
                            if types.is_empty() {
                                return Err(MediaServiceError::InvalidMediaType);
                            }

                            // filter the size of specific media types if provided
                            for media in types {
                                if let Some(filter) = filters.iter().find(|f| {
                                    // check if the media type is affected by the filter
                                    f.affected_types.as_ref().map_or(false, |t| t.contains(&media))
                                }) {
                                    if let Some(max_size) = filter.max_file_size {
                                        if written > max_size {
                                            return Err(MediaServiceError::FileTooLarge);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // if file size too small
                if mime.is_empty() || extension.is_empty() {
                    let (detected_mime, detected_extension) =
                        super::utils::get_mime_and_extension(&chunk)?;

                    mime = detected_mime;
                    extension = detected_extension;

                    category = super::utils::categorize(&mime);
                }

                let elapsed = start_time.elapsed().as_secs();
                if elapsed > 5 {
                    // Give the connection a 5-second grace period to spin up
                    let throughput = written / (elapsed as usize);
                    if throughput < min_bytes_per_second {
                        tracing::warn!("Dropped connection due to low throughput rate.");
                        return Err(MediaServiceError::TransmissionTooSlow);
                    }
                }

                file.write_all(&chunk).await?;
            }

            file.flush().await?;
            Ok::<(), MediaServiceError>(())
        }
        .await;


        match write_result {
            Ok(()) => Ok({
                let mut data = MultipartFile::new(
                    self.storage.clone(),
                    full_path,
                    relative_path,
                    name.clone(),
                    "part".to_string(),
                    category,
                    written,
                    mime,
                );

                data.rename_extension(&extension)?;

                tracing::trace!(
                    "Saved temporary upload: {:#?}",
                    data
                );

                data
            }),
            Err(err) => {
                // * partial temp file is removed on failure/cancel
                let _ = fs::remove_file(&full_path).await;
                Err(err)
            }
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
        uploaded_files: &mut Vec<MultipartFile>,
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
                    if let Some(max_files) = options.max_files {
                        if file_count >= max_files {
                            return Err(MediaServiceError::TooManyFiles(max_files));
                        }
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

                    let upload = self.save_temp_stream(
                            &mut field,
                            options,
                        )
                        .await?;

                    // * Track every successful upload for rollback.
                    uploaded_files.push(upload.clone());

                    let upload_value = serde_json::to_value(upload).map_err(|error| {
                        tracing::error!("Failed to serialize MultipartFile: {}", error);

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