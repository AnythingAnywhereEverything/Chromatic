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