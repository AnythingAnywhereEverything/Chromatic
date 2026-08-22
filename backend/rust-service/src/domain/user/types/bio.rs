use crate::domain::user::errors::BioError;

static MAX_BIO_LENGTH: usize = 320;

#[derive(Debug, Clone)]
pub struct Bio(String);

impl Bio {
    /// Creates a new `Bio` after validating the input string.
    /// Validation rules:
    /// - Must not exceed `MAX_BIO_LENGTH` characters.
    /// Returns a `BioError` if validation fails.
    /// Example usage:
    /// ```
    /// # use chromatic::domain::user::types::Bio;
    /// let bio = Bio::new("This is my bio.").unwrap();
    /// assert_eq!(bio.as_str(), "This is my bio.");
    /// ```
    ///
    /// Example of exceeding maximum length:
    /// ```
    /// # use chromatic::domain::user::types::Bio;
    /// # use chromatic::domain::user::errors::BioError;
    /// let long_bio = "a".repeat(321);
    /// let bio = Bio::new(&long_bio);
    /// assert!(bio.is_err());
    /// assert_eq!(bio.err().unwrap(), BioError::TooLong(MAX_BIO_LENGTH));
    /// ```
    pub fn new(input: &str) -> Result<Self, BioError> {
        let trimmed = input.trim();

        if trimmed.len() > MAX_BIO_LENGTH {
            return Err(BioError::TooLong(MAX_BIO_LENGTH));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns a string slice containing the bio.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned string value of the bio.
    pub fn into_inner(self) -> String {
        self.0
    }
}
