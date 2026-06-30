use axum::extract::multipart::MultipartError;
use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

#[derive(Debug, Error)]
pub enum MediaServiceError {
    #[error("Invalid media type.")]
    InvalidMediaType,

    #[error("File missing.")]
    MediaMissing,

    #[error("File too large.")]
    FileTooLarge,

    #[error("Unable to extract payload.")]
    UnableToExtract,

    #[error("Internal system error.")]
    InternalServer,

    #[error("Storage unavailable")]
    StorageUnavailable,

    #[error("Failed to process the video")]
    ProcessingFailed,

    #[error("Operation timed out.")]
    Timeout,

    #[error("Invalid scale value.")]
    InvalidScale,

    #[error("Transmission too slow.")]
    TransmissionTooSlow,

    #[error(transparent)]
    MultipartError(#[from] MultipartError),

    #[error(transparent)]
    SnowflakeError(#[from] SnowflakeServiceError),

    #[error(transparent)]
    LibvipsError(#[from] libvips::error::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}