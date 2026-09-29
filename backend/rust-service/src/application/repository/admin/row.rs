use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Filters for `list_users` and the two search halves it composes.
///
/// Lives with the row types because both `get::list_users` and the `find::`
/// search functions take it, so it has to be visible to whichever module needs
/// it rather than hiding inside one of them.
///
/// The two booleans mirror the `users` columns of the same name. They are
/// deliberately `Option<bool>` rather than a status enum: the schema has two
/// independent boolean columns and nothing else, so a string status filter
/// would be a lossy re-encoding of what the caller can already ask for
/// directly. `None` means "do not filter on this column".
///
/// The note on the shared filter block in `list_users` and `find_exact` applies
/// here too: the SQL is duplicated on purpose, never assembled with `format!`.
#[derive(Debug, Clone, Default)]
pub struct ListUsersOpts {
    /// Free text match against username, email and display name.
    ///
    /// Substring, not exact, so the query goes through a sequential scan. See
    /// `list_users` for how the exact-match fast path is kept index-backed.
    pub query: Option<String>,

    /// The same `query`, pre-parsed as a user id by the service layer when it is
    /// all digits.
    ///
    /// Kept separate rather than casting inside SQL: `u.id = $n` with a real
    /// `i64` bind is index-eligible, whereas `u.id::TEXT = $n` or a regex-guarded
    /// cast is not. It also means a non-numeric query never reaches a cast and
    /// cannot error. A username is allowed to be all digits, so this is *not*
    /// an alternative to `query`, it is an addition to it.
    pub query_id: Option<i64>,

    /// `None` = any, `Some(true)` = not suspended, `Some(false)` = suspended.
    pub is_active: Option<bool>,

    /// `None` = any, `Some(true)` = superuser, `Some(false)` = regular user.
    pub is_superuser: Option<bool>,

    /// Keyset cursor: return rows strictly older than this `(created_at, id)`.
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    pub before_id: Option<i64>,

    pub limit: i64,
}

/// A user as seen by the admin panel.
///
/// Deliberately narrower than `UserProfileRow`: no follower/following *state*,
/// no pinned posts, no quote. Those are all requester-relative and meaningless
/// to a superuser looking at a moderation table.
///
/// Carries no credential, oauth or session fields, so this row is safe to
/// serialize toward a superuser without a second filtering pass.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AdminUserRow {

    pub id: String,

    /// The same id as a real `i64`, for ordering and for passing back into
    /// repository calls. Never serialized: `id` is the one that goes to the
    /// client, and emitting both would be redundant at best.
    ///
    /// This exists because the string form cannot be sorted correctly. As text,
    /// `"9" > "100" > "10"`, so any ordering or dedup on `id` has to use this
    /// instead.
    #[serde(skip)]
    pub id_raw: i64,

    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    /// The avatar file name (`media_objects.name`), for building
    /// `avatars/{id}/{name}`. `None` when the user has no avatar.
    pub avatar: Option<String>,
    pub avatar_thumbhash: Option<String>,
    pub is_active: bool,
    pub is_superuser: bool,
    pub email_verified_at: Option<chrono::DateTime<chrono::Utc>>,

    pub followers_count: Option<i32>,
    pub following_count: Option<i32>,
    pub posts_count: Option<i32>,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// One page of admin users.
///
/// Keyset pagination does not need a total count, so this carries `has_more`
/// rather than a row count. Fetching one row beyond the limit is how the
/// repository knows whether a next page exists, which saves a `COUNT(*)`
/// over the same filter on every page request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserPage {
    pub rows: Vec<AdminUserRow>,
    pub has_more: bool,
}

/// One `audit_logs` row.
///
/// `action_data` is the JSONB blob the caller wrote at mutation time, so its
/// shape is defined by the service layer rather than by a type here.
///
/// The usernames are LEFT JOINed from `users` so an entry survives even if the
/// actor is gone (`performed_by` is `ON DELETE CASCADE`, so a hard delete
/// would otherwise take the entry with it). The ids alone cannot identify a
/// user in the UI, hence the joined handles.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AdminAuditRow {
    /// See `AdminUserRow::id` for why these are strings.
    pub id: String,
    pub target_id: String,
    pub target_type: String,
    pub action: String,
    pub performed_by: String,
    /// Handle of the user the action was aimed at.
    pub target_username: Option<String>,
    /// Handle of the superuser who performed the action.
    pub performed_by_username: Option<String>,
    pub action_data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Aggregate counts for the admin dashboard.
///
/// One round trip rather than four `COUNT(*)` calls, and every figure is
/// `COUNT(*)` so it comes back as `i64`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AdminStatsRow {
    pub total_users: i64,
    pub active_users: i64,
    pub suspended_users: i64,
    pub superusers: i64,
    pub deleted_users: i64,
}
