use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

/// Errors raised by the guild service.
///
/// Unit variants with fixed messages, so removing the carried sqlx/snowflake
/// error keeps this type serializable-stable and lets the handler layer own
/// what the user actually sees. `InvalidName` is the one validation failure a
/// create request can produce (the schema has no other constraints beyond the
/// foreign keys and the `id` primary key).
#[derive(Debug, Error)]
pub enum GuildServiceError {
    #[error("Guild name must be 1-128 characters")]
    InvalidName,

    #[error("Guild not found")]
    GuildNotFound,

    #[error("User is already a member of this guild")]
    AlreadyMember,

    #[error("Only the guild owner can do that")]
    NotOwner,

    #[error("ID generation failed.")]
    IdGenerationFailed,

    #[error("Database error.")]
    Database,
}

impl From<sqlx::Error> for GuildServiceError {
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}

impl From<SnowflakeServiceError> for GuildServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        Self::IdGenerationFailed
    }
}