use base64::{Engine, engine::general_purpose};
use rand::RngCore;

use crate::domain::session::errors::TokenError;

#[derive(Debug)]
pub struct Token(String);

impl Token {
    pub fn new<T: AsRef<[u8]>>(input: T) -> Self {
        Self(general_purpose::STANDARD_NO_PAD.encode(input))
    }

    /// Generates a random token of the specified byte length, encoded in base64 without padding.
    /// Use for generating secure tokens for sessions, email validation, etc.
    /// 
    /// Validation of the generated token can be done using `decode_to_string` or `decode_raw` methods.
    /// 
    /// Example usage:
    /// ```
    /// # use chromatic::domain::session::token::Token;
    /// let token = Token::rand(16); // Generates a random 16-byte token
    /// assert!(!token.as_str().is_empty());
    /// ```
    pub fn rand(byte_len: usize) -> Self {
        let mut bytes = vec![0u8; byte_len];
        rand::rng().fill_bytes(&mut bytes);

        Self(general_purpose::STANDARD_NO_PAD.encode(bytes))
    }

    /// Decodes a base64-encoded token string into a UTF-8 string.
    /// Returns a `TokenError::InvalidToken` if decoding fails or if the decoded bytes are not valid UTF-8.
    /// 
    /// Example usage:
    /// ```
    /// # use chromatic::domain::session::token::Token;
    /// let original = "Hello, World!";
    /// let token = Token::new(original);
    /// let decoded = Token::decode_to_string(token.as_str()).unwrap();
    /// assert_eq!(original, decoded);
    /// ```
    pub fn decode_to_string<T: AsRef<[u8]>>(input: T) -> Result<String, TokenError> {
        let decoded = general_purpose::STANDARD_NO_PAD
            .decode(input)
            .map_err(|_| TokenError::InvalidToken)?;

        String::from_utf8(decoded).map_err(|_| TokenError::InvalidToken)
    }

    /// Decodes a base64-encoded token string into raw bytes.
    /// Returns a `TokenError::InvalidToken` if decoding fails.
    /// 
    /// Example usage:
    /// ```
    /// # use chromatic::domain::session::token::Token;
    /// let original = b"Hello, World!";
    /// let token = Token::new(original);
    /// let decoded = Token::decode_raw(token.as_str()).unwrap();
    /// assert_eq!(original, decoded.as_slice());
    /// ```
    pub fn decode_raw<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, TokenError> {
        general_purpose::STANDARD_NO_PAD
            .decode(input)
            .map_err(|_| TokenError::InvalidToken)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}