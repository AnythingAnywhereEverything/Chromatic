use hyper::StatusCode;

use crate::{api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind}, application::service::errors::PostServiceError};


impl From <PostServiceError> for APIError{
    fn from(error: PostServiceError) -> Self {
        let (status, entry) = match error {
            PostServiceError::PostTextContentTooLarge => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Text content too large.")
                .code(APIErrorCode::TextTooLarge)
                .kind(APIErrorKind::PostError)
            ),
            PostServiceError::NoEmptyFile => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Can not updaload empty file")
                .code(APIErrorCode::EmptyFile)
                .kind(APIErrorKind::PostError)
            ),
            PostServiceError::CreatePostFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new("Failed to create post")
                .kind(APIErrorKind::PostError)
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new("An unexpected error occurred")
                .kind(APIErrorKind::PostError)
            )
        };
        APIError::from((status, entry))
    }
}