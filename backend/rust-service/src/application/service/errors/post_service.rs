use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

#[derive(Debug, Error)]
pub enum PostServiceError {
    #[error("Database error")]
    Database,
    
    #[error("ID generation failed.")]
    IdGenerationFailed,

    #[error("Failed to create post")]
    CreatePostFailed,

    #[error("Text content is too large")]
    PostTextContentTooLarge,

    #[error("Can not upload empty file")]
    NoEmptyFile,

    #[error("Comment not found or unauthorized")]
    CommentNotFoundOrUnauthorized,

    #[error("Post not found")]
    InvalidPost,
    
    #[error("Failed to update post")]
    UpdatePostFailed,

    #[error("Failed to delete post")]
    DeletePostFailed,

    #[error("Too many rows on get one")]
    UnexpectedMultipleRows,
}

impl From<sqlx::Error> for PostServiceError {
    fn from(_: sqlx::Error) -> Self {
        PostServiceError::Database
    }
}

impl From<SnowflakeServiceError> for PostServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        PostServiceError::IdGenerationFailed
    }
}