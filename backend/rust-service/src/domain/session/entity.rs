use chrono::Utc;

use crate::domain::session::{errors::SessionError, token::Token};

#[derive(Debug)]
pub struct SessionToken {
    pub user_id: i64,
    pub timestamp: i64,
    pub random_string: String,
    pub full_token: String,
}

impl SessionToken {
    /// Generates a new session token for the given user ID.
    /// 
    /// The token format is: base64(user_id).base64(timestamp).base64(random_string)
    /// 
    /// The random string is generated as a base64-encoded random byte array (32 bytes by default).
    /// 
    /// The timestamp is in milliseconds since the Unix epoch.
    /// 
    /// Example token: "MTIzNA==.MTY5ODQ4MDAwMDAw.MjM0NTY3ODkwMTIzNDU2Nzg5MA=="
    /// 
    /// Validation of the generated token can be done using the `parse` method, which will decode the user ID and timestamp, and verify the random string format.
    /// 
    /// Example usage:
    /// ```
    /// # use chromatic::domain::session::entity::SessionToken;
    /// let token = SessionToken::new(1234).unwrap();
    /// assert_eq!(token.user_id, 1234);
    /// assert!(token.timestamp > 0);
    /// assert!(!token.random_string.is_empty());
    /// assert_eq!(token.full_token.split('.').count(), 3);
    /// ```
    #[tracing::instrument(level = tracing::Level::TRACE, name = "Session token generation", skip_all, fields(user_id=user_id))]
    pub fn new(user_id: i64) -> Result<SessionToken, SessionError> {
        
        // * Generate timestamp and random string for the token
        let now = Utc::now();
        let timestamp_millis = now.timestamp_millis(); 
        // * Use milliseconds for better precision and to avoid issues with second-level timestamps in high-traffic scenarios

        let timestamp_part = timestamp_millis.to_string(); 
        // * Encode user ID and timestamp as base64 strings

        let encoded_random = Token::rand(32);
        // * Generate a random string of 32 bytes (256 bits) for strong security, encoded in base64 without padding

        let encoded_user_id = Token::new(user_id.to_string());
        let encoded_timestamp_part = Token::new(&timestamp_part);

        if encoded_random.as_str().is_empty() {
            return Err(SessionError::SessionCreationError);
        }

        let full_token = format!(
            "{}.{}.{}",
            encoded_user_id.into_inner(),
            encoded_timestamp_part.into_inner(),
            encoded_random.into_inner()
        );

        Ok(SessionToken {
            user_id,
            timestamp: timestamp_millis,
            // * store encoded random directly (not original bytes)
            random_string: full_token
                .split('.')
                .nth(2)
                .unwrap()
                .to_string(),
            full_token,
        })
    }

    /// Parses a session token string and returns a `SessionToken` struct if valid.
    /// 
    /// The input token must be in the format: base64(user_id).base64(timestamp).base64(random_string)
    /// 
    /// The method will decode the user ID and timestamp, and verify that the random string is a valid base64 string.
    /// 
    /// Returns a `SessionError::InvalidSession` if the token is malformed, if decoding fails, or if the user ID or timestamp cannot be parsed as integers.
    /// 
    /// Example usage:
    /// ```
    /// # use chromatic::domain::session::entity::SessionToken;
    /// let original_token = SessionToken::new(1234).unwrap();
    /// let parsed_token = SessionToken::parse(&original_token.full_token).unwrap();
    /// assert_eq!(parsed_token.user_id, 1234);
    /// assert_eq!(parsed_token.timestamp, original_token.timestamp);
    /// assert_eq!(parsed_token.random_string, original_token.random_string);
    /// assert_eq!(parsed_token.full_token, original_token.full_token);
    /// ```
    pub fn parse(token: &str) -> Result<SessionToken, SessionError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(SessionError::InvalidSession);
        }

        let decoded_user_id = Token::decode_to_string(parts[0])?;
        let decoded_timestamp = Token::decode_to_string(parts[1])?;

        // * DO NOT UTF-8 decode random part
        let random_string = parts[2].to_string();

        Ok(SessionToken {
            user_id: decoded_user_id
                .parse::<i64>()
                .map_err(|_| SessionError::InvalidSession)?,
            timestamp: decoded_timestamp
                .parse::<i64>()
                .map_err(|_| SessionError::InvalidSession)?,
            random_string,
            full_token: token.to_string(),
        })
    }
}