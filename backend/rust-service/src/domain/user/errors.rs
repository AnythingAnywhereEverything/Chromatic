use thiserror::Error;

use crate::domain::session::errors::TokenError;

#[derive(Debug, Error)]
pub enum UsernameError {
    #[error("Username contains invalid characters.")]
    InvalidCharacters,
    #[error("Username cannot contain consecutive periods.")]
    ConsecutivePeriods,
    #[error("Username must be between 3 and 32 characters.")]
    InvalidLength,
    #[error("Username derived from email has invalid format.")]
    InvalidFormat,
}

#[derive(Debug, Error)]
pub enum DisplayNameError {
    #[error("Display name cannot be empty.")]
    Empty,
    #[error("Display name cannot exceed 32 characters.")]
    TooLong,
    #[error("Display name cannot be blank.")]
    Blank
}

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Invalid email format.")]
    InvalidEmailFormat,
    #[error("Invalid token.")]
    InvalidToken
}

#[derive(Debug, Error)]
pub enum PhoneNumberError {
    #[error("Invalid phone structure")]
    InvalidStructure,

    #[error("Invalid phone number format")]
    InvalidFormat
}

impl From<TokenError> for EmailError {
    fn from(_: TokenError) -> Self {
        Self::InvalidToken
    }
}