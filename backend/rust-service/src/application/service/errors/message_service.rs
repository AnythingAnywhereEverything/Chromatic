use crate::application::service::errors::media_service::{ExtractionError, MediaServiceError};
use crate::application::service::errors::profile_service::ProfileServiceError;
use crate::application::service::errors::snowflake_service::SnowflakeServiceError;
use axum::extract::multipart::MultipartError;
use deadpool_redis::PoolError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageServiceError {
    #[error("User not found")]
    UserNotFound,

    #[error("Message not found")]
    MessageNotFound,

    #[error("Empty content")]
    EmptyContent,

    #[error("Invalid payload")]
    InvalidPayload,

    #[error("Content too long")]
    ContentTooLong,

    #[error(transparent)]
    MediaServiceError(#[from] MediaServiceError),

    #[error(transparent)]
    MultipartError(#[from] MultipartError),

    #[error(transparent)]
    MediaExtractorError(#[from] ExtractionError),

    #[error(transparent)]
    SnowflakeError(#[from] SnowflakeServiceError),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    RedisPoolError(#[from] PoolError),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),

    #[error(transparent)]
    ProfileServiceError(#[from] ProfileServiceError),
}
