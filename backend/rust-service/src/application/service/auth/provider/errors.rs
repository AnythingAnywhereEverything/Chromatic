use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Failed to fetch user info from provider.")]
    FetchUserInfoFailed,

    #[error("Invalid access token.")]
    InvalidAccessToken,

    #[error("Provider service is unavailable.")]
    ServiceUnavailable,
}