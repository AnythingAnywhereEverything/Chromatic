//! Permission bitmask constants and helpers for staff roles.
//!
//! The schema stores a `BIGINT[]` `permission_bitmask` on `staff_roles`. Each
//! element is a codepoint for a permission. The codepoints are 1-based so that a
//! zero value can never be mistaken for "has a permission", and so that an
//! empty array means "no permissions at all".
//!
//! The admin panel only gates with `AdminUser` (superuser) for now. This module
//! exists so that (a) the data model matches the plan, (b) the service can
//! validate and store masks in a single place, and (c) a future permission-aware
//! gate can be introduced without a schema change.

/// One staff permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaffPermission {
    /// May access the admin panel.
    AccessAdminPanel = 1,
    /// May manage users (suspend/activate, role changes).
    ManageUsers = 2,
    /// May manage posts (edit/delete).
    ManagePosts = 3,
    /// May manage staff roles (create/update/delete/assign).
    ManageStaffRoles = 4,
    /// May view audit logs.
    ViewAudit = 5,
    /// May view platform statistics.
    ViewStats = 6,
}

impl StaffPermission {
    /// All defined permissions, in a stable order for UI rendering.
    pub fn all() -> Vec<Self> {
        vec![
            Self::AccessAdminPanel,
            Self::ManageUsers,
            Self::ManagePosts,
            Self::ManageStaffRoles,
            Self::ViewAudit,
            Self::ViewStats,
        ]
    }

    /// Codepoint as `i64` (matches `BIGINT`).
    pub fn as_i64(self) -> i64 {
        self as i64
    }
}

/// Return true if `mask` contains any of `flags`.
///
/// An empty `flags` returns `false` (the caller asked "has any?" of nothing).
/// An empty `mask` returns `false`.
pub fn contains_any(mask: &[i64], flags: &[StaffPermission]) -> bool {
    if flags.is_empty() || mask.is_empty() {
        return false;
    }
    // Both are small (six flags max). Linear scan is cheap and clear.
    for &f in flags {
        let c = f as i64;
        if mask.iter().any(|m| *m == c) {
            return true;
        }
    }
    false
}

/// Convert a list of permissions to the stored bitmask vector.
#[allow(dead_code)]
pub fn to_mask(perms: &[StaffPermission]) -> Vec<i64> {
    let mut v: Vec<i64> = perms.iter().map(|p| *p as i64).collect();
    v.sort_unstable_by(|a, b| a.cmp(b));
    v.dedup();
    v
}

/// Convert a stored mask back to a list of permissions (ignores unknown codes).
#[allow(dead_code)]
pub fn from_mask(mask: &[i64]) -> Vec<StaffPermission> {
    let mut out = Vec::new();
    for &m in mask {
        match m {
            1 => out.push(StaffPermission::AccessAdminPanel),
            2 => out.push(StaffPermission::ManageUsers),
            3 => out.push(StaffPermission::ManagePosts),
            4 => out.push(StaffPermission::ManageStaffRoles),
            5 => out.push(StaffPermission::ViewAudit),
            6 => out.push(StaffPermission::ViewStats),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_any_basic() {
        let mask = vec![1i64, 2, 3, 4, 5, 6];
        assert!(contains_any(
            &mask,
            &[StaffPermission::AccessAdminPanel]
        ));
        assert!(contains_any(&mask, &[StaffPermission::ManageStaffRoles]));
        assert!(contains_any(
            &mask,
            &[
                StaffPermission::ManagePosts,
                StaffPermission::ViewStats
            ]
        ));
    }

    #[test]
    fn contains_any_empty() {
        let mask = vec![1i64];
        assert!(!contains_any(&mask, &[]));
        assert!(!contains_any(&[], &[StaffPermission::AccessAdminPanel]));
        assert!(!contains_any(&[], &[]));
    }

    #[test]
    fn contains_any_miss() {
        let mask = vec![2i64, 3];
        assert!(!contains_any(&mask, &[StaffPermission::AccessAdminPanel]));
        assert!(!contains_any(&mask, &[StaffPermission::ViewAudit]));
    }

    #[test]
    fn to_from_mask_roundtrip() {
        let all = StaffPermission::all();
        let mask = to_mask(&all);
        assert_eq!(mask, vec![1, 2, 3, 4, 5, 6]);
        let back = from_mask(&mask);
        assert_eq!(back, all);
    }

    #[test]
    fn from_mask_ignores_unknown() {
        let mask = vec![1i64, 99, 2];
        let back = from_mask(&mask);
        assert_eq!(
            back,
            vec![
                StaffPermission::AccessAdminPanel,
                StaffPermission::ManageUsers
            ]
        );
    }
}
