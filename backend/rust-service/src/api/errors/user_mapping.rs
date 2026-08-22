use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::service::errors::ProfileServiceError,
};

impl From<ProfileServiceError> for APIError {
    fn from(error: ProfileServiceError) -> Self {
        let (status, entry) = match error {
            ProfileServiceError::InvalidAvatarUpdate => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Invalid avatar update.")
                    .code(APIErrorCode::InvalidAvatarUpdate)
                    .kind(APIErrorKind::ValidationError),
            ),
            ProfileServiceError::InvalidBannerUpdate => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Invalid banner update.")
                    .code(APIErrorCode::InvalidBannerUpdate)
                    .kind(APIErrorKind::ValidationError),
            ),
            ProfileServiceError::NoUpdateFields => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("No update fields provided.")
                    .code(APIErrorCode::NoUpdateFields)
                    .kind(APIErrorKind::ValidationError),
            ),
            ProfileServiceError::DisplayNameError(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::DisplayNameError)
                    .kind(APIErrorKind::ValidationError),
            ),
            ProfileServiceError::BioError(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::BioError)
                    .kind(APIErrorKind::ValidationError),
            ),
            ProfileServiceError::QuotesError(e) => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new(&e.to_string())
                    .code(APIErrorCode::QuotesError)
                    .kind(APIErrorKind::ValidationError),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&error.to_string())
                    .code(APIErrorCode::ProfileServiceError)
                    .kind(APIErrorKind::SystemError),
            ),
        };
        APIError::from((status, entry))
    }
}
