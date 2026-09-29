use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::service::errors::AdminServiceError,
};

impl From<AdminServiceError> for APIError {
    fn from(error: AdminServiceError) -> Self {
        let (status, entry) = match error {
            AdminServiceError::UserNotFound => (
                StatusCode::NOT_FOUND,
                APIErrorEntry::new("User not found.")
                    .code(APIErrorCode::UserNotFound)
                    .kind(APIErrorKind::ResourceNotFound),
            ),

            AdminServiceError::SelfSuspensionForbidden => (
                StatusCode::FORBIDDEN,
                APIErrorEntry::new("Administrators cannot suspend themselves.")
                    .code(APIErrorCode::AdminSelfSuspensionForbidden)
                    .kind(APIErrorKind::AuthenticationError),
            ),

            AdminServiceError::SelfDemotionForbidden => (
                StatusCode::FORBIDDEN,
                APIErrorEntry::new("Administrators cannot revoke their own superuser role.")
                    .code(APIErrorCode::AdminSelfDemotionForbidden)
                    .kind(APIErrorKind::AuthenticationError),
            ),

            AdminServiceError::LastSuperuserProtected => (
                StatusCode::CONFLICT,
                APIErrorEntry::new("Cannot demote the last remaining superuser.")
                    .code(APIErrorCode::AdminLastSuperuserProtected)
                    .kind(APIErrorKind::ValidationError),
            ),

            AdminServiceError::IdGenerationFailed => {
                let entry = APIErrorEntry::new("Could not generate an audit id.")
                    .code(APIErrorCode::SnowflakeError)
                    .kind(APIErrorKind::SnowflakeError)
                    .trace_id();
                tracing::error!(
                    trace_id = entry.trace_id.as_deref().unwrap_or(""),
                    "admin audit id generation failed"
                );
                (StatusCode::INTERNAL_SERVER_ERROR, entry)
            }

            AdminServiceError::Database => {
                let entry = APIErrorEntry::new("A database error occurred.")
                    .code(APIErrorCode::DatabaseError)
                    .kind(APIErrorKind::DatabaseError)
                    .trace_id();
                tracing::error!(
                    trace_id = entry.trace_id.as_deref().unwrap_or(""),
                    "admin operation failed at the database layer"
                );
                (StatusCode::INTERNAL_SERVER_ERROR, entry)
            }
        };
        Self::from((status, entry))
    }
}
