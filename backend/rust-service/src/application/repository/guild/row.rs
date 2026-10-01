use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A `guilds` row as exposed by the repository.
///
/// The id is a string on the wire and a real `i64` for logic, exactly like
/// `AdminUserRow`: snowflake ids exceed JS safe-integers, so the serialized
/// form is always text, while ordering and lookups use the raw column.
///
/// `description` is `Option` because the schema column is nullable. The two
/// count fields are denormalized counters maintained by the repository
/// (`join::insert` bumps `total_members`, `create::channel` bumps
/// `total_channels`), so a freshly-created guild reads back with the accurate
/// totals only after the service has run the whole create transaction and
/// re-read the row (`get::by_id`).
///
/// `is_owner` is the one field that is not a column: it is derived from the
/// `owner_id` above and the id of whoever asked, so it is the only field that
/// makes the row mean different things to two different readers.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildRow {
    pub id: String,

    /// The same id as a real `i64`, for ordering and lookups. Never
    /// serialized; see `AdminUserRow::id_raw`.
    #[serde(skip)]
    pub id_raw: i64,

    pub owner_id: String,

    /// Whether the requester that produced this row owns the guild.
    ///
    /// Requester-relative, so it is computed by the query that produced the row
    /// rather than read from a column — the same treatment `PostRow::is_liked`
    /// gets in `post/get::base_comment`. That also means every query returning
    /// a `GuildRow` has to put it in the select list or the decode fails, which
    /// is a runtime error rather than a compile error.
    ///
    /// A plain `bool` rather than `Option<bool>`, unlike
    /// `UserProfileRow::is_follower`: `false` is the right answer for an
    /// anonymous requester, and dropping the key would force every consumer to
    /// tell "nobody asked" apart from "asked, and it is not yours" for no gain.
    /// `get::for_join` therefore guards the comparison with `IS NOT NULL` — a
    /// bare `owner_id = NULL` is SQL `NULL`, which does not decode to `false`.
    pub is_owner: bool,

    pub name: String,
    pub description: Option<String>,
    pub total_members: i32,
    pub total_channels: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// One page of joinable guilds.
///
/// Keyset pagination, so it carries `has_more` rather than a total count,
/// mirroring `AdminUserPage`. The repository fetches one row beyond the limit
/// and tells the caller whether a next page exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildPage {
    pub rows: Vec<GuildRow>,
    pub has_more: bool,
}

/// One `guild_members` row.
///
/// The membership's identity is the composite `(guild_id, user_id)` primary
/// key, so unlike single-id rows there is no one `id_raw`; both halves get a
/// string twin for the wire and a raw twin for the logic that needs them.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildMemberRow {
    pub guild_id: String,
    #[serde(skip)]
    pub guild_id_raw: i64,
    pub user_id: String,
    #[serde(skip)]
    pub user_id_raw: i64,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// One `guild_channels` row.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildChannelRow {
    pub id: String,
    #[serde(skip)]
    pub id_raw: i64,
    pub guild_id: String,
    pub name: String,
    pub channel_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A `guild_roles` row.
///
/// Only referenced by `role.rs` today, which is out of the create/join scope;
/// kept so that module keeps compiling.
#[derive(sqlx::FromRow, Debug)]
pub struct GuildRolesRow {
    pub id: i64,
    pub guild_id: i64,
    pub name: String,
    pub color: Option<String>,
    pub position: i32,
    pub permission_bitmask: Vec<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}