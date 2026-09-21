use crate::{api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind}, application::service::errors::MessageServiceError};
use hyper::StatusCode;

impl From<MessageServiceError> for APIError {
    fn from(error: MessageServiceError) -> Self {
        let (status , entry) = match error {
            MessageServiceError::ContentTooLong => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Text content too large.")
                .code(APIErrorCode::TextTooLarge)
                .kind(APIErrorKind::MessageError)
            ),
            MessageServiceError::UserNotFound => (
                StatusCode::NOT_FOUND,
                APIErrorEntry::new("User not found.")
                .code(APIErrorCode::UserNotFound)
                .kind(APIErrorKind::MessageError)
            ),
            MessageServiceError::InvalidPayload => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Invalid payload.")
                .code(APIErrorCode::InvalidPayload)
                .kind(APIErrorKind::MessageError)   
            ),
            MessageServiceError::EmptyContent => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Content cannot be empty.")
                .code(APIErrorCode::EmptyContent)
                .kind(APIErrorKind::MessageError)
            ),
            MessageServiceError::MessageNotFound => (
                StatusCode::NOT_FOUND,
                APIErrorEntry::new("Message not found.")
                .code(APIErrorCode::MessageNotFound)
                .kind(APIErrorKind::MessageError)
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new("An unexpected error occurred.")
                .code(APIErrorCode::SystemError)
                .kind(APIErrorKind::SystemError)
            )
        };
        APIError::from((status, entry))
    }
}