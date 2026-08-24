use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind}, application::service::errors::{MediaServiceError, media_service::{ContainerError, ExtractionError}},
};

impl From<ContainerError> for APIError {
    fn from(error: ContainerError) -> Self {
        let (status, entry) = match error {
            ContainerError::InitializationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new("Invalid container.")
                    .code(APIErrorCode::MediaInitializationFailed)
                    .kind(APIErrorKind::MediaError),
            ),
            ContainerError::MissingTargetPath => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new("Missing target path.")
                    .code(APIErrorCode::MediaMisconfigured)
                    .kind(APIErrorKind::MediaError),
            ),
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::SystemError)
                    .kind(APIErrorKind::MediaError),
            ),
        };
        APIError::from((status, entry))
    }
}

impl From<ExtractionError> for APIError {
    fn from(error: ExtractionError) -> Self {
        let (status, entry) = match error {
            ExtractionError::MultipartError(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::MediaMultipartError)
                    .kind(APIErrorKind::MediaError),
            ),
            ExtractionError::ContainerError(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::MediaContainerError)
                    .kind(APIErrorKind::MediaError),
            ),
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::SystemError)
                    .kind(APIErrorKind::MediaError),
            ),
        };
        APIError::from((status, entry))
    }
}

impl From<MediaServiceError> for APIError {
    fn from(error: MediaServiceError) -> Self {
        let (status, entry) = match error {
            MediaServiceError::TransmissionTooSlow => (
                StatusCode::REQUEST_TIMEOUT,
                APIErrorEntry::new("Transmission too slow.")
                    .code(APIErrorCode::MediaTransmissionTooSlow)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::Timeout => (
                StatusCode::REQUEST_TIMEOUT,
                APIErrorEntry::new("Operation timed out.")
                    .code(APIErrorCode::MediaTimeout)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::SnowflakeError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::MediaSnowflakeError)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::LibvipsError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::MediaLibvipsError)
                    .kind(APIErrorKind::MediaError),
            ),
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::SystemError)
                    .kind(APIErrorKind::MediaError),
            ),
        };
        APIError::from((status, entry))
    }
}
