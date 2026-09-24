use std::fmt::{Display, Formatter, Result};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
    
#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    pub status: u16,
    pub errors: Vec<APIErrorEntry>,
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let api_error = serde_json::to_string_pretty(&self).unwrap_or_default();
        write!(f, "{}", api_error)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorCode {
    // Authentication
    AuthenticationWrongCredentials,
    AuthenticationMissingCredentials,
    AuthenticationTokenCreationError,
    AuthenticationInvalidToken,
    AuthenticationForbidden,
    
    AuthenticationEmailAlreadyRegistered,
    AuthenticationUsernameAlreadyRegistered,
    
    AuthenticationInvalidDisplayName,
    AuthenticationInvalidUsername,
    AuthenticationInvalidEmail,
    AuthenticationInvalidPassword,

    // Media
    MediaFileTooLarge,
    MediaInvalidFileType,
    MediaMissingFile,
    MediaUnableToExtract,
    MediaMultipartError,
    MediaSnowflakeError,
    MediaLibvipsError,
    MediaInvalidScale,
    MediaInitializationFailed,
    MediaUnsupportedContainer,
    MediaInvalidMultipartField,
    MediaDuplicateMultipartField,
    MediaTransmissionTooSlow,
    MediaTimeout,
    MediaTooManyFiles,
    MediaUnknownMultipartField,
    MediaInvalidCropScale,
    MediaFileIsEmpty,
    MediaContainerError,

    MediaMisconfigured,

    // Message
    MessageNotFound,
    InvalidPayload,
    EmptyContent,

    // Post
    TextTooLarge,
    EmptyFile,
    TagIdError,

    // Comment
    CommentNotFoundOrUnauthorized,
    // Session
    SessionError,

    // User
    UserNotFound,
    CannotFollowYourself,
    InvalidFollowOperation,
    FollowRequestNotFound,
    InvalidSettingUpdate,

    // Profile
    InvalidAvatarUpdate,
    InvalidBannerUpdate,
    ProfileServiceError,
    NoUpdateFields,
    DisplayNameError,
    BioError,
    QuotesError,

    // Transaction
    TransactionNotFound,
    TransferInsufficientFunds,
    TransferSourceAccountNotFound,
    TransferDestinationAccountNotFound,
    TransferAccountsAreSame,
    ResourceNotFound,

    ApiVersionError,

    DatabaseError,

    RedisError,

    SnowflakeError,

    Argon2Error,

    SystemError, // Fallback for unexpected errors that do not fit into other categories. Should be used sparingly.
}

impl Display for APIErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::json!(self).as_str().unwrap_or_default()
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorKind {
    AuthenticationError,

    ResourceNotFound,

    ValidationError,

    DatabaseError,

    RedisError,
    
    PostError,

    MessageError,

    CommentError,

    SnowflakeError,
    
    SessionError,

    MediaError,

    SystemError, // Fallback for uncategorized errors
}

impl Display for APIErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::json!(self).as_str().unwrap_or_default()
        )
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct APIErrorEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
}

impl APIErrorEntry {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            timestamp: Utc::now(),
            ..Default::default()
        }
    }

    pub fn code<S: ToString>(mut self, code: S) -> Self {
        self.code = Some(code.to_string());
        self
    }

    pub fn kind<S: ToString>(mut self, kind: S) -> Self {
        self.kind = Some(kind.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn detail(mut self, detail: serde_json::Value) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_owned());
        self
    }

    pub fn instance(mut self, instance: &str) -> Self {
        self.instance = Some(instance.to_owned());
        self
    }

    pub fn trace_id(mut self) -> Self {
        // Generate a new trace id.
        let mut trace_id = uuid::Uuid::new_v4().to_string();
        trace_id.retain(|c| c != '-');
        self.trace_id = Some(trace_id);
        self
    }

    pub fn help(mut self, help: &str) -> Self {
        self.help = Some(help.to_owned());
        self
    }
}

impl From<StatusCode> for APIErrorEntry {
    fn from(status_code: StatusCode) -> Self {
        let error_message = status_code.to_string();
        let error_code = error_message.replace(' ', "_").to_lowercase();
        Self::new(&error_message).code(error_code)
    }
}

impl From<sqlx::Error> for APIErrorEntry {
    fn from(e: sqlx::Error) -> Self {
        // Do not disclose database-related internal specifics, except for debug builds.
        if cfg!(debug_assertions) {
            let (code, kind) = match e {
                sqlx::Error::RowNotFound => (
                    APIErrorCode::ResourceNotFound,
                    APIErrorKind::ResourceNotFound,
                ),
                _ => (APIErrorCode::DatabaseError, APIErrorKind::DatabaseError),
            };
            Self::new(&e.to_string()).code(code).kind(kind).trace_id()
        } else {
            // Build the entry with a trace id to find the exact error in the log when needed.
            let error_entry = Self::from(StatusCode::INTERNAL_SERVER_ERROR).trace_id();
            let trace_id = error_entry.trace_id.as_deref().unwrap_or("");
            // The error must be logged here. Otherwise, we would lose it.
            tracing::error!("SQLx error: {}, trace id: {}", e.to_string(), trace_id);
            error_entry
        }
    }
}

impl From<redis::RedisError> for APIErrorEntry {
    fn from(e: redis::RedisError) -> Self {
        // Do not disclose Redis-related internal specifics, except for debug builds.
        if cfg!(debug_assertions) {
            Self::new(&e.to_string())
                .code(APIErrorCode::RedisError)
                .kind(APIErrorKind::RedisError)
                .description(&format!("Redis error: {}", e))
                .trace_id()
        } else {
            // Build the entry with a trace id to find the exact error in the log when needed.
            let error_entry = Self::from(StatusCode::INTERNAL_SERVER_ERROR).trace_id();
            let trace_id = error_entry.trace_id.as_deref().unwrap_or("");
            // The error must be logged here. Otherwise, we would lose it.
            tracing::error!("Redis error: {}, trace id: {}", e.to_string(), trace_id);
            error_entry
        }
    }
}

impl From<(StatusCode, Vec<APIErrorEntry>)> for APIError {
    fn from(error_from: (StatusCode, Vec<APIErrorEntry>)) -> Self {
        let (status_code, errors) = error_from;
        Self {
            status: status_code.as_u16(),
            errors,
        }
    }
}

impl From<(StatusCode, APIErrorEntry)> for APIError {
    fn from(error_from: (StatusCode, APIErrorEntry)) -> Self {
        let (status_code, error_entry) = error_from;
        Self {
            status: status_code.as_u16(),
            errors: vec![error_entry],
        }
    }
}

impl From<StatusCode> for APIError {
    fn from(status_code: StatusCode) -> Self {
        Self {
            status: status_code.as_u16(),
            errors: vec![status_code.into()],
        }
    }
}

impl From<sqlx::Error> for APIError {
    fn from(error: sqlx::Error) -> Self {
        let status_code = match error {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status: status_code.as_u16(),
            errors: vec![APIErrorEntry::from(error)],
        }
    }
}

impl From<redis::RedisError> for APIError {
    fn from(error: redis::RedisError) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            errors: vec![APIErrorEntry::from(error)],
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        tracing::error!("Error response: {:?}", self);
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}
