use thiserror::Error;

use crate::application::service::errors::SnowflakeServiceError;

/// Everything that can stop a report from being filed.
///
/// The split that matters: `CannotReportYourself` and `InvalidTargetType` are
/// properties of the request and are checked before the transaction opens,
/// while the rest depend on database state that can change between the check
/// and the insert.
#[derive(Debug, Error)]
pub enum ReportServiceError {
    #[error("You cannot report yourself")]
    CannotReportYourself,

    /// A client sent a `target_type` outside the supported set.
    #[error("Unknown report target type")]
    InvalidTargetType,

    #[error("Unknown report type")]
    InvalidType,

    #[error("Report description is too long")]
    InvalidDescription,

    /// The reporter already has an unreviewed report against this target.
    #[error("You have already reported this")]
    AlreadyOpen,

    /// The target does not exist, or has been soft-deleted. Indistinguishable
    /// on purpose — a reportable thing and a never-existed thing are the same
    /// answer to a client, and saying otherwise turns this endpoint into an
    /// oracle for which ids are real.
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