use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    domain::user::errors::DisplayNameError,
};

impl From<DisplayNameError> for APIError {
    fn from(error: DisplayNameError) -> Self {
        let (status, entry) = match error {
            DisplayNameError::TooLong => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Displayname exceeds maximum length of 32 characters.")
                    .code(APIErrorCode::AuthenticationInvalidDisplayName)
                    .kind(APIErrorKind::AuthenticationError),
            ),
            _ => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Invalid display name.")
                    .code(APIErrorCode::AuthenticationInvalidDisplayName)
                    .kind(APIErrorKind::AuthenticationError),
            ),
        };
        APIError::from((status, entry))
    }
}
