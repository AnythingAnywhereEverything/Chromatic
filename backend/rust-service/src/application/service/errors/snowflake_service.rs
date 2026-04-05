use hyper::StatusCode;
use thiserror::Error;

use crate::api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind};

#[derive(Debug, Error)]
pub enum SnowflakeServiceError {
    #[error("Worker ID ({0}) exceeds maximum allowed value ({1}).")]
    InvalidWorkerId(u64, u64),

    #[error("Clock moved backward.")]
    ClockMoveBackward,
}

impl From<SnowflakeServiceError> for APIError {
    fn from(snowflake_error: SnowflakeServiceError) -> Self {
        let (status_code, code) = match snowflake_error {
            SnowflakeServiceError::InvalidWorkerId(_, _) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorCode::SnowflakeError,
            ),
            SnowflakeServiceError::ClockMoveBackward => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorCode::SnowflakeError,
            )
        };

        let error = APIErrorEntry::new(&snowflake_error.to_string())
            .code(code)
            .kind(APIErrorKind::SnowflakeError);

        Self {
            status: status_code.as_u16(),
            errors: vec![error],
        }
    }
}