use axum::http::StatusCode;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::service::errors::ReportServiceError,
};

/// The three validation refusals are 400 rather than 422 because every one of
/// them is a field the client got wrong, and the fix is in the request the
/// client already holds: `CannotReportYourself` by dropping the target,
/// `InvalidTargetType`/`InvalidType` by choosing from the dropdown,
/// `InvalidDescription` by shortening it.
///
/// `AlreadyOpen` is 409 because it is not a malformed request. The request was
/// well-formed and would have been accepted at any other moment; something
/// about current state conflicts with it. Same reasoning as
/// `AdminLastSuperuserProtected`.
///
/// `TargetNotFound` is a 404 that deliberately does not distinguish "this id
/// never existed" from "this thing is soft-deleted". Both mean the same thing to
/// a client, and splitting them would turn this endpoint into an oracle for
/// which ids are real.
impl From<ReportServiceError> for APIError {
    fn from(error: ReportServiceError) -> Self {
        let (status, entry) = match error {
            ReportServiceError::CannotReportYourself => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("You cannot report yourself.")
                    .code(APIErrorCode::ReportCannotReportYourself)
                    .kind(APIErrorKind::ValidationError),
            ),
            ReportServiceError::InvalidTargetType => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Unknown report target type.")
                    .code(APIErrorCode::ReportInvalidTargetType)
                    .kind(APIErrorKind::ValidationError),
            ),
            ReportServiceError::InvalidType => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Unknown report type.")
                    .code(APIErrorCode::ReportInvalidType)
                    .kind(APIErrorKind::ValidationError),
            ),
            ReportServiceError::InvalidDescription => (
                StatusCode::BAD_REQUEST,
                APIErrorEntry::new("Report description is too long.")
                    .code(APIErrorCode::ReportInvalidDescription)
                    .kind(APIErrorKind::ValidationError),
            ),
            ReportServiceError::AlreadyOpen => (
                StatusCode::CONFLICT,
                APIErrorEntry::new("You have already reported this.")
                    .code(APIErrorCode::ReportAlreadyOpen)
                    .kind(APIErrorKind::ValidationError),
            ),
            ReportServiceError::TargetNotFound => (
                StatusCode::NOT_FOUND,
                APIErrorEntry::new("Reported content not found.")
                    .code(APIErrorCode::ReportTargetNotFound)
                    .kind(APIErrorKind::ResourceNotFound),
            ),
            ReportServiceError::IdGenerationFailed | ReportServiceError::Database => (
                StatusCode::INTERNAL_SERVER_ERROR,
                APIErrorEntry::new(&error.to_string())
                    .code(APIErrorCode::SystemError)
                    .kind(APIErrorKind::SystemError),
            ),
        };

        Self::from((status, entry))
    }
}