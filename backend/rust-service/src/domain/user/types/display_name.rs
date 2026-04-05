use crate::domain::user::errors::DisplayNameError;

static MAX_DISPLAY_NAME_LENGTH: usize = 32;

#[derive(Debug, Clone)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(input: &str) -> Result<Self, DisplayNameError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(DisplayNameError::Empty);
        }

        if trimmed.chars().all(|c| c.is_whitespace()) || trimmed.is_empty() {
            return Err(DisplayNameError::Blank);
        }

        if trimmed.len() > MAX_DISPLAY_NAME_LENGTH {
            return Err(DisplayNameError::TooLong);
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