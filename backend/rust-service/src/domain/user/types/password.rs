use crate::domain::user::errors::PasswordError;

static MAX_PASSWORD_LENGTH: usize = 128;
static MIN_PASSWORD_LENGTH: usize = 8;

pub fn is_valid_password(password: &str) -> bool {
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "@$!%*?&".contains(c));
    let is_long_enough = password.len() >= 8;

    has_lowercase && has_uppercase && has_digit && has_special && is_long_enough
}


#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(input: &str) -> Result<Self, PasswordError> {
        let trimmed = input.trim();

        if trimmed.len() < MIN_PASSWORD_LENGTH {
            return Err(PasswordError::TooShort);
        }

        if trimmed.len() > MAX_PASSWORD_LENGTH {
            return Err(PasswordError::TooLong);
        }

        if !is_valid_password(trimmed) {
            return Err(PasswordError::InvalidFormat);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}