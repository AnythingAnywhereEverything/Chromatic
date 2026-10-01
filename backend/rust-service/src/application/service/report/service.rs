use serde_json::json;

use crate::application::{
    repository::report::{self, find},
    service::errors::ReportServiceError,
    state::AppState,
};

/// What a `reports` row can point at.
///
/// One `reported_target_type` per report rather than three columns or three
/// tables, because `reported_target_id` is a bare `BIGINT` with no foreign key —
/// the schema stores a polymorphic reference and this list is the vocabulary for
/// the discriminator. A post id and a user id are both `BIGINT` and would be
/// indistinguishable if the type were not carried alongside.
///
/// `user` is the only one reachable from the UI so far. `post` and `comment` are
/// accepted and validated because the schema and this service are polymorphic,
/// but no menu item files them yet.
const TARGET_TYPES: [&str; 3] = ["user", "post", "comment"];

/// Why someone is reporting something.
///
/// The `VARCHAR(64)` column is sized for these and nothing else: the values are
/// matched exactly against this list, with no normalization, so a client and the
/// server cannot end up disagreeing about whether `"Hate Speech"` and
/// `"hate_speech"` are the same thing. Adding a member here is the only way a
/// new category exists.
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

/// Longest description accepted, in characters.
///
/// `media_stories.description` uses `VARCHAR(1000)` for free text a user typed,
/// so this matches the longest free-text column already in the schema. The
/// reason it is capped at all is that `report_data` is copied into every row of
/// the moderation queue's read path.
const MAX_DESCRIPTION_CHARS: usize = 1000;

pub struct ReportService;

impl ReportService {
    /// File one report against `target_id`.
    ///
    /// Stateless, so an associated function — matching `AdminService` and
    /// `StaffRoleService`. The authorization gate is deliberately not here:
    /// `RequestAuth`/`AdminUser` own that, so no handler can forget it.
    ///
    /// The eight steps run in a deliberate order, and the order is the design:
    ///
    /// 1–3 are properties of the *request*, so they are checked before the
    /// transaction opens. There is nothing to read, and a malformed request
    /// should not take a database connection. Same reasoning as the
    /// self-suspension check in `AdminService::set_user_active`.
    ///
    /// 4–6 depend on database state, so they run inside the transaction. The
    /// duplicate check is the reason this needs a transaction at all, and it
    /// takes an advisory lock of its own — see `report::exists_open_locked`.
    ///
    /// 7 is `author_id` rather than `reported_target_id`, which is what catches
    /// reporting your own post. Step 2's equality check cannot see it: for a
    /// `post` target the id names the post, never the reporter.
    pub async fn submit(
        state: &AppState,
        reporter_id: i64,
        target_id: i64,
        target_type: &str,
        report_type: &str,
        description: Option<&str>,
    ) -> Result<(), ReportServiceError> {
        // 1. Unknown target type. Checked first because it decides how steps 2
        // and 7 are evaluated at all; validating it last would mean running the
        // other rules against a target type that does not exist.
        if !TARGET_TYPES.contains(&target_type) {
            return Err(ReportServiceError::InvalidTargetType);
        }

        // 2. Self-report. Only meaningful for a `user` target, where
        // `reported_target_id` is the account being accused. For a post or a
        // comment the id names the content, so this comparison would never
        // match and the real self-report case is step 7 instead.
        if target_type == "user" && reporter_id == target_id {
            return Err(ReportServiceError::CannotReportYourself);
        }

        // 3. Vocabulary and length. Trim before measuring so a description of
        //    nothing but whitespace fails as empty rather than passing a
        //    length check it only barely satisfies.
        if !REPORT_TYPES.contains(&report_type) {
            return Err(ReportServiceError::InvalidType);
        }

        let description = description.map(str::trim).filter(|d| !d.is_empty());

        if description.is_some_and(|d| d.chars().count() > MAX_DESCRIPTION_CHARS) {
            return Err(ReportServiceError::InvalidDescription);
        }

        let mut tx = state.db_pool.begin().await?;

        // 4. Duplicate. Advisory-locked inside `exists_open_locked`, so the
        //    check and the insert below cannot be interleaved by a second
        //    submit of the same pair.
        if find::exists_open_locked(&mut tx, reporter_id, target_type, target_id).await? {
            return Err(ReportServiceError::AlreadyOpen);
        }

        // 5. Target must be live. `load_target` also hands back the snapshot
        //    fields, so this is not a separate existence query.
        let Some(target) = find::load_target(&mut tx, target_type, target_id).await? else {
            return Err(ReportServiceError::TargetNotFound);
        };

        // 6. Do not let anyone report their own content. Reached only by a
        //    `post` or `comment` target; a `user` target already left at step 2.
        if target.author_id == Some(reporter_id) {
            return Err(ReportServiceError::CannotReportYourself);
        }

        // `report_data` is `JSONB NOT NULL`, so an absent description is `null`
        // *inside* the object rather than a null object. Which keys are present
        // depends on the target type: a user has a handle to freeze, a post or
        // comment has text, and neither has both.
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

        // 7. `reports.id` has no DEFAULT and no sequence, so the id is
        //    generated here exactly as `admin::create::write` does for
        //    `audit_logs.id`. `status` is left to the repository.
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