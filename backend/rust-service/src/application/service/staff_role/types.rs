use serde::Serialize;

use crate::application::repository::staff_role::row::StaffRoleRow;

/// One user's staff roles.
///
/// The read half of the assignment endpoints. `role_ids` is the same list the
/// `PUT` body carries, so the frontend can seed its form from this and submit it
/// back unchanged — no client-side diffing, and the set the server holds is the
/// only source of truth about what is assigned.
#[derive(Debug, Clone, Serialize)]
pub struct UserStaffRoles {
    pub user_id: String,
    pub roles: Vec<StaffRoleRow>,
    pub role_ids: Vec<String>,
}