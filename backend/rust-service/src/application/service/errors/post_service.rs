use deadpool_redis::PoolError;
use thiserror::Error;

use crate::application::service::errors::{
    MediaServiceError, ProfileServiceError, SnowflakeServiceError, media_service::ExtractionError,
};

#[derive(Debug, Error)]
pub enum PostServiceError {
    #[error("Nothing to update")]
    NothingToUpdate,

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
    PostNotFound,

    #[error("Failed to update post")]
    UpdatePostFailed,

    #[error("Failed to delete post")]
    DeletePostFailed,

    #[error("Too many rows on get one")]
    UnexpectedMultipleRows,

    #[error("Can't not find tag id")]
    TagIdNotFound,

    #[error(transparent)]
    MediaServiceError(#[from] MediaServiceError),

    #[error(transparent)]
    MediaExtractorError(#[from] ExtractionError),

    #[error(transparent)]
    ProfileServiceError(#[from] ProfileServiceError),

    #[error(transparent)]
    RedisPoolError(#[from] PoolError),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl From<SnowflakeServiceError> for PostServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        PostServiceError::IdGenerationFailed
    }
}
