use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::service::errors::SessionServiceError,
};

impl From<SessionServiceError> for APIError {
    fn from(error: SessionServiceError) -> Self {
        let (status, entry) = match error {
            SessionServiceError::MissingCredential => (
                StatusCode::UNAUTHORIZED,
                APIErrorEntry::new("Missing credential.")
                    .code(APIErrorCode::AuthenticationMissingCredentials)
                    .kind(APIErrorKind::AuthenticationError),
            ),
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::SessionError)
                    .kind(APIErrorKind::SessionError),
            ),
        };
        APIError::from((status, entry))
    }
}
