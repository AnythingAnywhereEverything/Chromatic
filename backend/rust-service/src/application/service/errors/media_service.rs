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
    SizeTooLarge,

    #[error("Unable to extract payload.")]
    UnableToExtract,

    #[error("Internal system error.")]
    InternalServer,

    #[error("Storage unavailable")]
    StorageUnavailable,

    #[error("Failed to process the video")]
    ProcessingFailed
}

impl From<MultipartError> for MediaServiceError {
    fn from(_: MultipartError) -> Self {
        MediaServiceError::InternalServer
    }
}

impl From<libvips::error::Error> for MediaServiceError {
    fn from(_: libvips::error::Error) -> Self {
        MediaServiceError::InternalServer
    }
}

impl From<std::io::Error> for MediaServiceError {
    fn from(_: std::io::Error) -> Self {
        MediaServiceError::InternalServer
    }
}

impl From<SnowflakeServiceError> for MediaServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        MediaServiceError::InternalServer
    }
}
