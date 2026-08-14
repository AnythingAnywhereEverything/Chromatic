use thiserror::Error;

use crate::application::service::errors::{ SnowflakeServiceError};

#[derive(Debug, Error)]
pub enum CommentServiceError{
    #[error("Database error")]
    Database,

    #[error("ID generation failed.")]
    IdGenerationFailed,

    #[error("Failed to create comment")]
    CreateCommentFailed,

    #[error("Failed to update comment")]
    UpdateCommentFailed,

    #[error("Comment not found or unauthorized")]
    CommentNotFoundOrUnauthorized
}

impl From<sqlx::Error> for CommentServiceError {
    fn from(_: sqlx::Error) -> Self {
        CommentServiceError::Database
    }
}

impl From<SnowflakeServiceError> for CommentServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        CommentServiceError::IdGenerationFailed
    }
}