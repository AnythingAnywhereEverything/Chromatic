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
    /// Creates a new `Password` after validating the input string.
    /// Validation rules:
    /// - Must be between `MIN_PASSWORD_LENGTH` and `MAX_PASSWORD_LENGTH` characters.
    /// - Must contain at least one lowercase letter, one uppercase letter, one digit, and one special character from the set `@$!%*?&`.
    /// Returns a `PasswordError` if validation fails.
    /// 
    /// Example usage:
    /// ```
    /// # use chromatic::domain::user::types::Password;
    /// let password = Password::new("P@ssw0rd").unwrap();
    /// assert_eq!(password.as_str(), "P@ssw0rd");
    /// ```
    /// Example of invalid password:
    /// ```
    /// # use chromatic::domain::user::types::Password;
    /// # use chromatic::domain::user::errors::PasswordError;
    /// let password = Password::new("password");
    /// assert!(password.is_err());
    /// assert_eq!(password.err().unwrap(), PasswordError::InvalidFormat);
    /// ```
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