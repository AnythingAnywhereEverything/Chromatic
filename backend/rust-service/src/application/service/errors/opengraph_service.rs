use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenGraphServiceError {
    #[error("Invalid URL.")]
    InvalidUrl,

    #[error("Unsupported URL scheme.")]
    UnsupportedScheme,

    #[error("Failed to fetch URL.")]
    FetchFailed,

    #[error("Failed to read response.")]
    ResponseReadFailed,

    #[error("Failed to parse OpenGraph data.")]
    ParseFailed,

    #[error("Invalid response content type.")]
    InvalidContentType,

    #[error("Internal server error.")]
    InternalServer,
}

impl From<reqwest::Error> for OpenGraphServiceError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_builder() {
            Self::InvalidUrl
        } else if error.is_decode() {
            Self::ResponseReadFailed
        } else {
            Self::FetchFailed
        }
    }
}