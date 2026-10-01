use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::service::errors::StaffRoleServiceError,
};

impl From<StaffRoleServiceError> for APIError {
    fn from(error: StaffRoleServiceError) -> Self {
        let (status, entry) = match error {
            StaffRoleServiceError::RoleNotFound => (
                StatusCode::NOT_FOUND,
                APIErrorEntry::new("Staff role not found.")
                    .code(APIErrorCode::StaffRoleNotFound)
                    .kind(APIErrorKind::ResourceNotFound),
            ),

            StaffRoleServiceError::UserNotFound => (
                StatusCode::NOT_FOUND,
                APIErrorEntry::new("User not found.")
                    .code(APIErrorCode::UserNotFound)
                    .kind(APIErrorKind::ResourceNotFound),
            ),

            StaffRoleServiceError::NameTaken => (
                StatusCode::CONFLICT,
                APIErrorEntry::new("A staff role with that name already exists.")
                    .code(APIErrorCode::StaffRoleNameTaken)
                    .kind(APIErrorKind::ValidationError),
            ),

            StaffRoleServiceError::InvalidName => (
                StatusCode::UNPROCESSABLE_ENTITY,
                APIErrorEntry::new("Staff role name must be between 1 and 64 characters.")
                    .code(APIErrorCode::StaffRoleInvalidName)
                    .kind(APIErrorKind::ValidationError),
            ),

            StaffRoleServiceError::IdGenerationFailed => {
                let entry = APIErrorEntry::new("Could not generate an id.")
                    .code(APIErrorCode::SnowflakeError)
                    .kind(APIErrorKind::SnowflakeError)
                    .trace_id();
                tracing::error!(
                    trace_id = entry.trace_id.as_deref().unwrap_or(""),
                    "staff role id generation failed"
                );
                (StatusCode::INTERNAL_SERVER_ERROR, entry)
            }

            StaffRoleServiceError::Database => {
                let entry = APIErrorEntry::new("A database error occurred.")
                    .code(APIErrorCode::DatabaseError)
                    .kind(APIErrorKind::DatabaseError)
                    .trace_id();
                tracing::error!(
                    trace_id = entry.trace_id.as_deref().unwrap_or(""),
                    "staff role operation failed at the database layer"
                );
                (StatusCode::INTERNAL_SERVER_ERROR, entry)
            }
        };
        Self::from((status, entry))
    }
}