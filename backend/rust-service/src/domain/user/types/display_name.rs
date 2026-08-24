use crate::domain::user::errors::DisplayNameError;

static MAX_DISPLAY_NAME_LENGTH: usize = 32;

#[derive(Debug, Clone)]
pub struct DisplayName(String);

impl DisplayName {
    /// Creates a new `DisplayName` after validating the input string.
    /// Validation rules:
    /// - Must not exceed `MAX_DISPLAY_NAME_LENGTH` characters.
    /// Returns a `DisplayNameError` if validation fails.
    /// Example usage:
    /// ```
    /// # use chromatic::domain::user::types::DisplayName;
    /// let name = DisplayName::new("Alice").unwrap();
    /// assert_eq!(name.as_str(), "Alice");
    /// ```
    /// 
    /// Example of exceeding maximum length:
    /// ```
    /// # use chromatic::domain::user::types::DisplayName;
    /// # use chromatic::domain::user::errors::DisplayNameError;
    /// let long_name = "a".repeat(33);
    /// let name = DisplayName::new(&long_name);
    /// assert!(name.is_err());
    /// assert_eq!(name.err().unwrap(), DisplayNameError::TooLong(32));
    /// ```
    pub fn new(input: &str) -> Result<Self, DisplayNameError> {
        let trimmed = input.trim();

        if trimmed.len() > MAX_DISPLAY_NAME_LENGTH {
            return Err(DisplayNameError::TooLong(MAX_DISPLAY_NAME_LENGTH));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns a string slice containing the display name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned string value of the display name.
    pub fn into_inner(self) -> String {
        self.0
    }
}