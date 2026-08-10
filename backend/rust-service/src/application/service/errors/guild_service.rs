use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

#[derive(Debug, Error)]
pub enum GuildServiceError {
    #[error("Database error")]
    Database,

    #[error("ID generation failed.")]
    IdGenerationFailed,

    // * Guild related errors
    #[error("Failed to create guild")]
    FailedToCreateGuild,

    #[error("Failed to update guild")]
    FailedToUpdateGuild,

    #[error("Failed to delete guild")]
    FailedToDeleteGuild,

    #[error("Failed to remove member from guild")]
    FailedToRemoveMember,

    #[error("Failed to get guild")]
    InvalidGuild,

    // * member related errors
    #[error("Failed to join guild")]
    FailedToJoinGuild,

    // AKA ban
    #[error("Failed to remove member from guild")]
    FailedToRemoveMemberFromGuild,

    // * channels related errors
    #[error("Failed to create channel")]
    FailedToCreateChannel,

    #[error("Failed to update channel")]
    FailedToUpdateChannel,

    #[error("Failed to delete channel")]
    FailedToDeleteChannel,

    #[error("Failed to get channel")]
    InvalidChannel,

    // * role related errors
    #[error("Failed to create role")]
    FailedToCreateRole,

    #[error("Failed to update role")]
    FailedToUpdateRole,

    #[error("Failed to delete role")]
    FailedToDeleteRole,

    // * media related errors
    #[error("Failed to create guild asset")]
    FailedToCreateGuildAsset,

    #[error("Failed to update guild asset")]
    FailedToUpdateGuildAsset,

    #[error("Failed to delete guild asset")]
    FailedToDeleteGuildAsset,

    #[error("Failed to get guild asset")]
    InvalidGuildAsset,

    // * asset related
    #[error("Failed to create media")]
    FailedToCreateMedia,

    #[error("Failed to update media")]
    FailedToUpdateMedia,

    #[error("Failed to delete media")]
    FailedToDeleteMedia,

    #[error("Failed to get media")]
    InvalidMedia,

}

impl From<sqlx::Error> for GuildServiceError {
    fn from(_: sqlx::Error) -> Self {
        GuildServiceError::Database
    }
}

impl From<SnowflakeServiceError> for GuildServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        GuildServiceError::IdGenerationFailed
    }
}