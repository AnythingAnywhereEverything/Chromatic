use axum::http::StatusCode;

use crate::{
    api::{
        APIError,
        APIErrorCode,
        APIErrorEntry,
        APIErrorKind,
    },
    application::service::errors::OpenGraphServiceError,
};

impl From<OpenGraphServiceError> for APIError {
    fn from(error: OpenGraphServiceError) -> Self {
        let (status, entry) = match error {
            OpenGraphServiceError::InvalidUrl
            | OpenGraphServiceError::UnsupportedScheme => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Invalid URL.")
                    .code(APIErrorCode::OpenGraphError)
                    .kind(APIErrorKind::ValidationError),
            ),

            OpenGraphServiceError::FetchFailed => (
                StatusCode::BAD_GATEWAY,
                APIErrorEntry::new("Failed to fetch URL.")
                    .code(APIErrorCode::OpenGraphError)
                    .kind(APIErrorKind::OpenGraphError),
            ),

            OpenGraphServiceError::ResponseReadFailed
            | OpenGraphServiceError::ParseFailed
            | OpenGraphServiceError::InvalidContentType => (
                StatusCode::BAD_GATEWAY,
                APIErrorEntry::new("Failed to process OpenGraph data.")
                    .code(APIErrorCode::OpenGraphError)
                    .kind(APIErrorKind::OpenGraphError),
            ),

            OpenGraphServiceError::InternalServer => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new("Internal server error.")
                    .code(APIErrorCode::OpenGraphError)
                    .kind(APIErrorKind::OpenGraphError),
            ),
        };

        APIError::from((status, entry))
    }
}