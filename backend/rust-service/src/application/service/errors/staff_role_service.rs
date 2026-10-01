use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

#[derive(Debug, Error)]
pub enum StaffRoleServiceError {
    #[error("Staff role not found")]
    RoleNotFound,

    #[error("User not found")]
    UserNotFound,

    #[error("A staff role with that name already exists")]
    NameTaken,

    #[error("Staff role name must not be blank")]
    InvalidName,

    #[error("ID generation failed.")]
    IdGenerationFailed,

    #[error("Database error.")]
    Database,
}

impl From<sqlx::Error> for StaffRoleServiceError {
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}

impl From<SnowflakeServiceError> for StaffRoleServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        Self::IdGenerationFailed
    }
}