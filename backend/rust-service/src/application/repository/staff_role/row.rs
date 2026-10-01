use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Filters for `list_roles`.
#[derive(Debug, Clone, Default)]
pub struct ListStaffRolesOpts {
    /// Substring match against `name`.
    pub q: Option<String>,
    /// Keyset: rows strictly older in ordering by (position, id)?
    /// The list is ordered by position ASC, id ASC for stability.
    pub before: Option<i32>,
    pub before_id: Option<i64>,
    pub limit: i64,
}

/// A staff role row as stored in `staff_roles`.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StaffRoleRow {
    pub id: String,
    #[serde(skip)]
    pub id_raw: i64,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub permission_bitmask: Vec<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Detail of a staff role including member count.
///
/// `#[sqlx(flatten)]` rather than a hand-built tuple: a tuple of
/// `(StaffRoleRow, i64)` needs both elements to implement `Decode`, which a
/// composite row does not. Flattening keeps it a single `query_as` against one
/// `FromRow`, which is the point of reading both in one round trip.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StaffRoleDetail {
    #[sqlx(flatten)]
    pub role: StaffRoleRow,
    pub member_count: i64,
}

/// A member of a staff role (join of `user_staff_roles` + `users`).
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StaffRoleMemberRow {
    pub user_id: String,
    #[serde(skip)]
    pub user_id_raw: i64,
    /// `NOT NULL` on `users`, so this is not optional even though the join in
    /// `list_members` is a LEFT JOIN. A missing user row yields `NULL` here and
    /// the decode fails, which is correct: `user_staff_roles.user_id` cascades
    /// on hard delete, so a dangling reference means the database is corrupt.
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub assigned_by: String,
}

/// One page of staff roles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffRolePage {
    pub rows: Vec<StaffRoleRow>,
    pub has_more: bool,
}

/// One page of role members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffRoleMembersPage {
    pub rows: Vec<StaffRoleMemberRow>,
    pub has_more: bool,
}