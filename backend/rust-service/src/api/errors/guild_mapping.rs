use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::service::errors::GuildServiceError,
};

impl From<GuildServiceError> for APIError {
    fn from(error: GuildServiceError) -> Self {
        let (status, entry) = match error {
            GuildServiceError::InvalidName => (
                StatusCode::UNPROCESSABLE_ENTITY,
                APIErrorEntry::new("Guild name must be 1-128 characters.")
                    .code(APIErrorCode::GuildInvalidName)
                    .kind(APIErrorKind::ValidationError),
            ),

            GuildServiceError::GuildNotFound => (
                StatusCode::NOT_FOUND,
                APIErrorEntry::new("Guild not found.")
                    .code(APIErrorCode::GuildNotFound)
                    .kind(APIErrorKind::ResourceNotFound),
            ),

            GuildServiceError::AlreadyMember => (
                StatusCode::CONFLICT,
                APIErrorEntry::new("You are already a member of this guild.")
                    .code(APIErrorCode::AlreadyMember)
                    .kind(APIErrorKind::ValidationError),
            ),

            // `AuthenticationError` rather than a bespoke kind: it is what the
            // superuser gate already uses for a 403 (`AdminUser::forbidden`),
            // and the requester here is authenticated too — they are simply not
            // the owner. The *code* is guild-specific so a client can tell this
            // apart from "superuser privileges are required".
            GuildServiceError::NotOwner => (
                StatusCode::FORBIDDEN,
                APIErrorEntry::new("Only the guild owner can do that.")
                    .code(APIErrorCode::NotGuildOwner)
                    .kind(APIErrorKind::AuthenticationError),
            ),

            GuildServiceError::IdGenerationFailed => {
                let entry = APIErrorEntry::new("Could not generate an id.")
                    .code(APIErrorCode::SnowflakeError)
                    .kind(APIErrorKind::SnowflakeError)
                    .trace_id();
                tracing::error!(
                    trace_id = entry.trace_id.as_deref().unwrap_or(""),
                    "guild id generation failed"
                );
                (StatusCode::INTERNAL_SERVER_ERROR, entry)
            }

            GuildServiceError::Database => {
                let entry = APIErrorEntry::new("A database error occurred.")
                    .code(APIErrorCode::DatabaseError)
                    .kind(APIErrorKind::DatabaseError)
                    .trace_id();
                tracing::error!(
                    trace_id = entry.trace_id.as_deref().unwrap_or(""),
                    "guild operation failed at the database layer"
                );
                (StatusCode::INTERNAL_SERVER_ERROR, entry)
            }
        };
        Self::from((status, entry))
    }
}