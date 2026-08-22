use crate::domain::user::errors::QuotesError;

static MAX_QUOTES_LENGTH: usize = 256;

#[derive(Debug, Clone)]
pub struct Quotes(String);

impl Quotes {
    /// Creates a new `Quotes` after validating the input string.
    /// Validation rules:
    /// - Must not exceed `MAX_QUOTES_LENGTH` characters.
    /// Returns a `QuotesError` if validation fails.
    /// Example usage:
    /// ```
    /// # use chromatic::domain::user::types::Quotes;
    /// # use chromatic::domain::user::errors::QuotesError;
    /// let quotes = Quotes::new("This is my quote.").unwrap();
    /// assert_eq!(quotes.as_str(), "This is my quote.");
    /// ```
    ///
    /// Example of exceeding maximum length:
    /// ```
    /// # use chromatic::domain::user::types::Quotes;
    /// # use chromatic::domain::user::errors::QuotesError;
    /// let long_quotes = "a".repeat(257);
    /// let quotes = Quotes::new(&long_quotes);
    /// assert!(quotes.is_err());
    /// assert_eq!(quotes.err().unwrap(), QuotesError::TooLong(MAX_QUOTES_LENGTH));
    /// ```
    pub fn new(input: &str) -> Result<Self, QuotesError> {
        let trimmed = input.trim();

        if trimmed.len() > MAX_QUOTES_LENGTH {
            return Err(QuotesError::TooLong(MAX_QUOTES_LENGTH));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns a string slice containing the quotes.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned string value of the quotes.
    pub fn into_inner(self) -> String {
        self.0
    }
}
