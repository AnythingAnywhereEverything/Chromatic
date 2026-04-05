use deadpool_redis::PoolError;
use thiserror::Error;

use crate::{
    application::{service::errors::SnowflakeServiceError}, domain::session::errors::SessionError,
};

#[derive(Debug, Error)]
pub enum SessionServiceError {
    #[error("Missing credential.")]
    MissingCredential,

    #[error("Invalid Session ID.")]
    InvalidSessionID,

    #[error("Unable to hash the session.")]
    UnableToHashSession,

    #[error("ID generation failed.")]
    IdGenerationFailed,

    #[error("Database error.")]
    Database,

    #[error("Redis error.")]
    Redis,

    #[error(transparent)]
    Validation(#[from] SessionError),
}
impl From<SnowflakeServiceError> for SessionServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        SessionServiceError::IdGenerationFailed
    }
}

impl From<sqlx::Error> for SessionServiceError {
    fn from(_: sqlx::Error) -> Self {
        SessionServiceError::Database
    }
}

impl From<PoolError>
    for SessionServiceError
{
    fn from(_: PoolError) -> Self {
        SessionServiceError::Redis
    }
}

impl From<redis::RedisError> for SessionServiceError {
    fn from(_: redis::RedisError) -> Self {
        SessionServiceError::Redis
    }
}
