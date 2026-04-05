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