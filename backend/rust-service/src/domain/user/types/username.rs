use once_cell::sync::Lazy;
use regex::Regex;

use crate::domain::user::UsernameError;

//* Statics or Constant */
static VALID_USERNAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z0-9._]+$").unwrap());
static MIN_LENGTH: usize = 3;
static MAX_LENGTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    /// Creates a new `Username` after validating the input string.
    /// Validation rules:
    /// - Must be between `MIN_LENGTH` and `MAX_LENGTH` characters.
    /// - Must only contain lowercase letters, digits, periods, or underscores.
    /// - Must not contain consecutive periods.
    /// Returns a `UsernameError` if validation fails.
    ///
    /// Example usage:
    /// ```
    /// # use chromatic::domain::user::types::Username;
    /// let username = Username::new("valid.username_123").unwrap();
    /// assert_eq!(username.as_str(), "valid.username_123");
    /// ```
    /// Example of invalid username:
    /// ```
    /// # use chromatic::domain::user::types::Username;
    /// # use chromatic::domain::user::errors::UsernameError;
    /// let username = Username::new("Invalid Username!");
    /// assert!(username.is_err());
    /// assert_eq!(username.err().unwrap(), UsernameError::InvalidCharacters);
    /// ```
    pub fn new(input: &str) -> Result<Self, UsernameError> {
        let username = input.to_lowercase();

        // Length rule
        if username.len() < MIN_LENGTH || username.len() > MAX_LENGTH {
            return Err(UsernameError::InvalidLength);
        }

        // Allowed characters
        if !VALID_USERNAME_RE.is_match(&username) {
            return Err(UsernameError::InvalidCharacters);
        }

        // No consecutive periods
        if username.contains("..") {
            return Err(UsernameError::ConsecutivePeriods);
        }

        Ok(Self(username))
    }

    /// ! Deprecated method to create a username from an email address.
    /// This method is not recommended for general use as it may produce non-unique or undesirable usernames.
    /// It is primarily intended for legacy support or specific use cases where a username must be derived from an email.
    /// Validation rules for the generated username:
    /// - Extracts the local part of the email (before the '@' symbol).
    /// - Converts to lowercase and removes invalid characters (only allows letters, digits, periods, and underscores).
    /// - Replaces multiple consecutive periods with a single period and trims leading/trailing periods.
    /// - Validates the resulting username against the same rules as `Username::new`.
    /// Returns a `UsernameError` if the generated username is invalid.
    pub fn from_email(email: &str) -> Result<Self, UsernameError> {
        let local = email.split('@').next().unwrap_or("user");

        let filtered: String = local
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '_')
            .collect();

        let mut cleaned = filtered.replace("..", ".");
        cleaned = cleaned.trim_matches('.').to_string();

        if cleaned.is_empty() {
            return Err(UsernameError::InvalidFormat);
        }

        Self::new(&cleaned)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}