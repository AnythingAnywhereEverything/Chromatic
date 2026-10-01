use serde_json::json;

use crate::application::{
    repository::report::{self, find},
    service::errors::ReportServiceError,
    state::AppState,
};

const TARGET_TYPES: [&str; 3] = ["user", "post", "comment"];

const REPORT_TYPES: [&str; 8] = [
    "spam",
    "harassment",
    "hate_speech",
    "nsfw",
    "violence",
    "impersonation",
    "misinformation",
    "other",
];

const MAX_DESCRIPTION_CHARS: usize = 1000;

pub struct ReportService;

impl ReportService {
    pub async fn submit(
        state: &AppState,
        reporter_id: i64,
        target_id: i64,
        target_type: &str,
        report_type: &str,
        description: Option<&str>,
    ) -> Result<(), ReportServiceError> {

        if !TARGET_TYPES.contains(&target_type) {
            return Err(ReportServiceError::InvalidTargetType);
        }

        if target_type == "user" && reporter_id == target_id {
            return Err(ReportServiceError::CannotReportYourself);
        }


        if !REPORT_TYPES.contains(&report_type) {
            return Err(ReportServiceError::InvalidType);
        }

        let description = description.map(str::trim).filter(|d| !d.is_empty());

        if description.is_some_and(|d| d.chars().count() > MAX_DESCRIPTION_CHARS) {
            return Err(ReportServiceError::InvalidDescription);
        }

        let mut tx = state.db_pool.begin().await?;


        if find::exists_open_locked(&mut tx, reporter_id, target_type, target_id).await? {
            return Err(ReportServiceError::AlreadyOpen);
        }

        let Some(target) = find::load_target(&mut tx, target_type, target_id).await? else {
            return Err(ReportServiceError::TargetNotFound);
        };


        if target.author_id == Some(reporter_id) {
            return Err(ReportServiceError::CannotReportYourself);
        }

        let report_data = match target_type {
            "user" => json!({
                "description": description,
                "reported_username": target.snapshot_username,
            }),
            _ => json!({
                "description": description,
                "content_excerpt": target.snapshot_content,
                "author_id": target.author_id,
            }),
        };
        let id = state.snowflake_generator.generate_id()?;

        report::create::insert(
            &mut tx,
            id,
            reporter_id,
            target_id,
            target_type,
            report_type,
            report_data,
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }
}