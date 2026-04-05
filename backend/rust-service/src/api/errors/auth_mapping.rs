use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::service::errors::AuthServiceError,
};

impl From<AuthServiceError> for APIError {
    fn from(error: AuthServiceError) -> Self {
        let (status, entry) = match error {
            AuthServiceError::EmailAlreadyRegistered => (
                StatusCode::CONFLICT,
                APIErrorEntry::new("Email is already registered.")
                    .code(APIErrorCode::AuthenticationEmailAlreadyRegistered)
                    .kind(APIErrorKind::AuthenticationError),
            ),
            AuthServiceError::UsernameAlreadyTaken => (
                StatusCode::CONFLICT,
                APIErrorEntry::new("Username is already taken.")
                    .code(APIErrorCode::AuthenticationUsernameAlreadyRegistered)
                    .kind(APIErrorKind::AuthenticationError),
            ),
            AuthServiceError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                APIErrorEntry::new("Invalid credentials.")
                    .code(APIErrorCode::AuthenticationWrongCredentials)
                    .kind(APIErrorKind::AuthenticationError),
            ),

            AuthServiceError::UserValidation(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::AuthenticationInvalidUsername)
                    .kind(APIErrorKind::AuthenticationError),
            ),
            AuthServiceError::EmailValidation(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::AuthenticationInvalidEmail)
                    .kind(APIErrorKind::AuthenticationError),
            ),

            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::SystemError)
                    .kind(APIErrorKind::SystemError),
            ),
        };
        APIError::from((status, entry))
    }
}
