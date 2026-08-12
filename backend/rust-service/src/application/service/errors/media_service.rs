use axum::extract::multipart::MultipartError;
use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

#[derive(Debug, Error)]
pub enum MediaServiceError {
    #[error("File name count mismatch.")]
    FileNameCountMismatch,

    #[error("Invalid media type.")]
    InvalidMediaType,

    #[error("File missing.")]
    MediaMissing,

    #[error("File too large.")]
    FileTooLarge,

    #[error("Unable to extract payload.")]
    UnableToExtract,

    #[error("Unknown multipart field: {0}.")]
    UnknownMultipartField(String),

    #[error("Too many files uploaded. Maximum allowed is {0}.")]
    TooManyFiles(usize),

    #[error("Duplicate multipart field: {0}.")]
    DuplicateMultipartField(String),

    #[error("Invalid multipart field value: {0}.")]
    InvalidMultipartField(String),

    #[error("Video trimming failed.")]
    VideoTrimFailed,

    #[error("Internal system error.")]
    InternalServer,

    #[error("Storage unavailable")]
    StorageUnavailable,

    #[error("Failed to process the video")]
    ProcessingFailed,

    #[error("Operation timed out.")]
    Timeout,

    #[error("Invalid crop scale value: {0}.")]
    InvalidCropScale(f32),

    #[error("Invalid scale value.")]
    InvalidScale,

    #[error("Failed to strip metadata.")]
    MetadataStripFailed,

    #[error("Failed to generate thumbnail.")]
    ThumbnailGenerationFailed,

    #[error("Transmission too slow.")]
    TransmissionTooSlow,

    #[error(transparent)]
    MultipartError(#[from] MultipartError),

    #[error(transparent)]
    SnowflakeError(#[from] SnowflakeServiceError),

    #[error(transparent)]
    LibvipsError(#[from] rs_vips::error::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
}