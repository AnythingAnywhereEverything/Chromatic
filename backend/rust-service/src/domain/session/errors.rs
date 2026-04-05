use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("session creation error")]
    SessionCreationError,
    #[error("Invalid session.")]
    InvalidSession,
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Token creation error.")]
    TokenCreationError,
    #[error("Invalid token.")]
    InvalidToken,
}

impl From<TokenError> for SessionError {
    fn from(_: TokenError) -> Self {
        Self::InvalidSession
    }
}