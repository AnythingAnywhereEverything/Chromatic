use base64::{Engine, engine::general_purpose};
use rand::RngCore;

use crate::domain::session::errors::TokenError;

#[derive(Debug)]
pub struct Token(String);

impl Token {
    pub fn new<T: AsRef<[u8]>>(input: T) -> Self {
        Self(general_purpose::STANDARD_NO_PAD.encode(input))
    }

    pub fn rand(byte_len: usize) -> Self {
        let mut bytes = vec![0u8; byte_len];
        rand::rng().fill_bytes(&mut bytes);

        Self(general_purpose::STANDARD_NO_PAD.encode(bytes))
    }

    pub fn decode_to_string<T: AsRef<[u8]>>(input: T) -> Result<String, TokenError> {
        let decoded = general_purpose::STANDARD_NO_PAD
            .decode(input)
            .map_err(|_| TokenError::InvalidToken)?;

        String::from_utf8(decoded).map_err(|_| TokenError::InvalidToken)
    }

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