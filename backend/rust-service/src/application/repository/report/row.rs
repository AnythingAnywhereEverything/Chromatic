use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// What `load_target` found for the reported thing, reduced to the handful of
/// fields worth freezing into `report_data`.
///
/// Not a `FromRow` type: it is assembled in Rust from whichever table the
/// target type names, so there is no single query for it to come from.
///
/// `snapshot_username` and `snapshot_content` are alternatives, not a pair —
/// a user target has no content to copy and a post target has no username of
/// its own. Which one is populated is decided by `reported_target_type`, which
/// the caller already has.
#[derive(Debug, Clone, Default)]
pub struct ReportTarget {
    /// Who owns the target: the reported user, or the author of the reported
    /// post or comment. This is what makes self-reporting a post detectable at
    /// all, since `reported_target_id` never refers to the reporter.
    pub author_id: Option<i64>,
    /// For a `user` target. Frozen so the report still reads sensibly if that
    /// account is later hard-deleted, which `reports` has no FK to prevent.
    pub snapshot_username: Option<String>,
    /// For a `post` or `comment` target. Truncated by the caller, never the
    /// whole column: `media_posts.content` is `VARCHAR(2500)` and copying it
    /// whole would bloat every report row for a preview that nobody reads in
    /// full.
    pub snapshot_content: Option<String>,
}

/// Filters for `list_reports`.
///
/// Every field is optional, and an absent filter is not the same as a falsy
/// one — `?status=pending` is a real request for the pending queue, so the
/// repository must not treat "no status" as "status = nothing".
#[derive(Debug, Clone, Default)]
pub struct ListReportsOpts {
    /// `None` = any status. `Some("pending")` = the open queue.
    pub status: Option<String>,
    /// `None` = any target type.
    pub target_type: Option<String>,
    /// `None` = any target. `Some(id)` = every report against one thing.
    pub target_id: Option<i64>,
    /// Keyset cursor: return rows strictly older than this `(created_at, id)`.
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    pub before_id: Option<i64>,
    pub limit: i64,
}

/// One report, as the moderation queue sees it.
///
/// Ids are strings because snowflake ids are 17 digits and
/// `Number.MAX_SAFE_INTEGER` is 16, so a client that parses one corrupts it.
/// Each is therefore paired with an `*_raw` `i64` that is never serialized: the
/// string form cannot be sorted correctly, since as text `"9" > "100" > "10"`,
/// so ordering and dedup have to use the raw value. This is the same
/// arrangement `AdminUserRow` uses.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReportRow {
    pub id: String,
    #[serde(skip)]
    pub id_raw: i64,

    /// `"user"`, `"post"`, or `"comment"`.
    pub reported_target_type: String,
    pub reported_target_id: String,
    #[serde(skip)]
    pub reported_target_id_raw: i64,
    pub report_type: String,
    pub description: Option<String>,
    pub report_data: serde_json::Value,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub reporter_id: String,
    pub reporter_username: Option<String>,
    pub reporter_display_name: Option<String>,

    pub target_username: Option<String>,
    pub target_display_name: Option<String>,
    pub target_deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    pub target_content: Option<String>,
    pub target_author_id: Option<String>,
}

/// One page of reports.
///
/// Keyset pagination, so there is no total count. `has_more` comes from
/// fetching one row past the limit, which saves a `COUNT(*)` over the same
/// filter on every page request. Same shape as `AdminUserPage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPage {
    pub rows: Vec<ReportRow>,
    pub has_more: bool,
}