use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

#[derive(Debug, Error)]
pub enum ReportServiceError {
    #[error("You cannot report yourself")]
    CannotReportYourself,

    #[error("Unknown report target type")]
    InvalidTargetType,

    #[error("Unknown report type")]
    InvalidType,

    #[error("Report description is too long")]
    InvalidDescription,


    #[error("You have already reported this")]
    AlreadyOpen,

    #[error("Reported content not found")]
    TargetNotFound,

    #[error("ID generation failed")]
    IdGenerationFailed,

    #[error("Database error")]
    Database,
}

impl From<sqlx::Error> for ReportServiceError {
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}

impl From<SnowflakeServiceError> for ReportServiceError {
    fn from(_: SnowflakeServiceError) -> Self {
        Self::IdGenerationFailed
    }
}