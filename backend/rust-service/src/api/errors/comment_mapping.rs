use hyper::StatusCode;

use crate::{api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind}, application::service::errors::CommentServiceError};

impl From <CommentServiceError> for APIError{
    fn from(error: CommentServiceError) -> Self {
        let (status, entry) = match error {
            CommentServiceError::CommentNotFoundOrUnauthorized => (
                StatusCode::NOT_FOUND,
                APIErrorEntry::new("Comment not found or unauthorized")
                .code(APIErrorCode::CommentNotFoundOrUnauthorized)
                .kind(APIErrorKind::CommentError)
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new("An unexpected error occurred")
                .kind(APIErrorKind::CommentError)
            )
        };
        APIError::from((status, entry))
    }
}