use axum::extract::multipart::MultipartError;
use deadpool_redis::PoolError;
use thiserror::Error;

use crate::{
    application::service::errors::{MediaServiceError, SnowflakeServiceError, media_service::{ContainerError, StorageError}}, domain::user::errors::{BioError, DisplayNameError, QuotesError},
};
use rs_vips::error::Error as LibvipsError;

#[derive(Debug, Error)]
pub enum ProfileServiceError {
    #[error("Invalid avatar update.")]
    InvalidAvatarUpdate,

    #[error("No update fields provided.")]
    NoUpdateFields,

    #[error("Invalid banner update.")]
    InvalidBannerUpdate,

    #[error("Invalid follow operation.")]
    InvalidFollowOperation,

    #[error("Cannot follow yourself.")]
    CannotFollowYourself,
    
    #[error("Cannot follow user due to their privacy settings.")]
    CannotFollowUser,
    
    #[error(transparent)]
    DisplayNameError(#[from] DisplayNameError),
    #[error(transparent)]
    BioError(#[from] BioError),
    #[error(transparent)]
    QuotesError(#[from] QuotesError),

    #[error(transparent)]
    MediaServiceError(#[from] MediaServiceError),

    #[error(transparent)]
    MultipartError(#[from] MultipartError),

    #[error(transparent)]
    SnowflakeError(#[from] SnowflakeServiceError),

    #[error(transparent)]
    LibvipsError(#[from] LibvipsError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    ContainerError(#[from] ContainerError),

    #[error(transparent)]
    StorageError(#[from] StorageError),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    RedisPoolError(#[from] PoolError),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}
