use axum::extract::multipart::MultipartError;
use thiserror::Error;

use crate::{
    application::service::errors::{MediaServiceError, SnowflakeServiceError},
    domain::user::errors::{BioError, DisplayNameError, QuotesError},
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
    TokioJoinError(#[from] tokio::task::JoinError),
}
