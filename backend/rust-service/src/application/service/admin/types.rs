use serde::Serialize;

use crate::application::repository::admin::row::{AdminAuditRow, AdminUserRow};

/// The admin detail view: one user plus their moderation history.
///
/// Composed in the service because it is not a single query's output — it is
/// `find_by_id` and `audit::list_for_target` read in one transaction, so the two
/// halves cannot disagree about what exists. The API layer serializes this
/// directly; no separate DTO, because `AdminUserRow` and `AdminAuditRow` are
#[derive(Debug, Clone, Serialize)]
pub struct AdminUserDetail {
    pub user: AdminUserRow,
    pub audit: Vec<AdminAuditRow>,
}
