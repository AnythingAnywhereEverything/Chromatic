use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::service::errors::MediaServiceError,
};

impl From<MediaServiceError> for APIError {
    fn from(error: MediaServiceError) -> Self {
        let (status, entry) = match error {
            MediaServiceError::MediaMissing => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("File is missing.")
                    .code(APIErrorCode::MediaMissingFile)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::UnableToExtract => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Unable to extract payload.")
                    .code(APIErrorCode::MediaUnableToExtract)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::MultipartError(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::MediaMultipartError)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::InvalidScale => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Invalid scale value.")
                    .code(APIErrorCode::MediaInvalidScale)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::InvalidMultipartField(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(format!("Invalid multipart field value: {}", e).as_str())
                    .code(APIErrorCode::MediaInvalidMultipartField)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::DuplicateMultipartField(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(format!("Duplicate multipart field: {}", e).as_str())
                    .code(APIErrorCode::MediaDuplicateMultipartField)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::TooManyFiles(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&format!(
                    "Too many files uploaded. Maximum allowed is {}.",
                    e
                ))
                .code(APIErrorCode::MediaTooManyFiles)
                .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::FileTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                APIErrorEntry::new("File is too large.")
                    .code(APIErrorCode::MediaFileTooLarge)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::InvalidMediaType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                APIErrorEntry::new("Invalid media type.")
                    .code(APIErrorCode::MediaInvalidFileType)
                    .kind(APIErrorKind::MediaError),
            ),
            MediaServiceError::UnknownMultipartField(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&format!("Unknown multipart field: {}", e))
                    .code(APIErrorCode::MediaUnknownMultipartField)
                    .kind(APIErrorKind::MediaError),
            ),
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
