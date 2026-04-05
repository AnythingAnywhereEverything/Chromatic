use thiserror::Error;

use crate::{
    application::service::errors::{SessionServiceError, SnowflakeServiceError},
    domain::{session::errors::SessionError, user::{EmailError, errors::{DisplayNameError, UsernameError}}},
};

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Username is already taken.")]
    UsernameAlreadyTaken,

    #[error("Email is already registered.")]
    EmailAlreadyRegistered,

    #[error("Database error.")]
    Database,

    #[error("Invalid credentials.")]
    InvalidCredentials,

    #[error("OAuth account already linked.")]
    OAuthAlreadyLinked,

    #[error("ID generation failed.")]
    IdGenerationFailed,

    #[error("Failed to logout.")]
    LogoutFailed,

    #[error("Unable to hash the password.")]
    UnableToHashSession,

    #[error("Failed to generate session.")]
    GenerateSessionFailed,

    #[error("Failed to make session.")]
    MakeSessionFailed,

    #[error("User not found.")]
    UserNotFound,

    #[error(transparent)]
    UserValidation(#[from] UsernameError),

    #[error(transparent)]
    EmailValidation(#[from] EmailError),

    #[error(transparent)]
    DisplayNameValidation(#[from] DisplayNameError),
}

impl From<sqlx::Error> for AuthServiceError {
    fn from(_: sqlx::Error) -> Self {
        AuthServiceError::Database
    }
}

impl From<SessionError> for AuthServiceError {
    fn from(_: SessionError) -> Self {
        AuthServiceError::GenerateSessionFailed
    }
}

impl From<SessionServiceError> for AuthServiceError {
    fn from(_: SessionServiceError) -> Self {
        AuthServiceError::MakeSessionFailed
    }
}

impl From<SnowflakeServiceError> for AuthServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        AuthServiceError::IdGenerationFailed
    }
}
