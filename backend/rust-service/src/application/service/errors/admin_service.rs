use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

#[derive(Debug, Error)]
pub enum AdminServiceError {
    #[error("User not found")]
    UserNotFound,

    #[error("Administrators cannot suspend themselves")]
    SelfSuspensionForbidden,

    #[error("Administrators cannot revoke their own superuser role")]
    SelfDemotionForbidden,

    #[error("Cannot demote the last remaining superuser")]
    LastSuperuserProtected,

    #[error("ID generation failed.")]
    IdGenerationFailed,

    #[error("Database error.")]
    Database,
}

impl From<sqlx::Error> for AdminServiceError {
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}

impl From<SnowflakeServiceError> for AdminServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        Self::IdGenerationFailed
    }
}
