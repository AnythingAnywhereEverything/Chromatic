
//TODO : andd email verification here.

use chrono::Utc;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::domain::{session::token::Token, user::errors::EmailError};


static VALID_EMAIL_FORMAT_RE: Lazy<Regex> =
    Lazy::new(|| 
        Regex::new(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        ).unwrap()
    );

#[derive(Debug, Clone)]
pub struct Email(String);

pub struct EmailToken {
    pub user_id: i64,
    pub timestamp: i64,
    pub random_string: String,
    pub full_token: String,
}

impl Email {
    pub fn new(input: &str) -> Result<Self, EmailError> {
        let email = input.to_lowercase();

        if !VALID_EMAIL_FORMAT_RE.is_match(&email) {
            return Err(EmailError::InvalidEmailFormat);
        }

        Ok(Self(email))
    }

    pub fn perse(input: &str) -> Result<EmailToken, EmailError> {
        let token: Vec<&str> = input.split("-").collect();

        if token.len() != 3 {
            return Err(EmailError::InvalidToken);
        }

        let decoded_user_id = Token::decode_to_string(token[0])?;
        let decoded_timestamp = Token::decode_to_string(token[1])?;

        // * random part stays base64 string (no UTF-8 decode needed)
        let random_string = token[2].to_string();

        Ok(EmailToken {
            user_id: decoded_user_id
                .parse::<i64>()
                .map_err(|_| EmailError::InvalidToken)?,
            timestamp: decoded_timestamp
                .parse::<i64>()
                .map_err(|_| EmailError::InvalidToken)?,
            random_string,
            full_token: input.to_string(),
        })
    }

    pub fn generate_validate_token(user_id: i64) -> EmailToken {

        let now = Utc::now();
        let timestamp_millis = now.timestamp_millis();
    
        let timestamp_part = timestamp_millis.to_string();

        let encoded_user_id = Token::new(user_id.to_string());
        let encoded_timestamp_part = Token::new(&timestamp_part);
        let encoded_random_string = Token::rand(120);
        
        let full_token = format!(
            "{}-{}-{}",
            encoded_user_id.into_inner(),
            encoded_timestamp_part.into_inner(),
            encoded_random_string.as_str(),
        );
        
        EmailToken {
            user_id,
            timestamp: timestamp_millis,
            random_string: encoded_random_string.into_inner(),
            full_token: full_token,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}