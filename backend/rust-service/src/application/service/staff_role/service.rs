use serde_json::json;

use crate::application::{
    repository::{
        admin::create,
        staff_role::{
            assign, check,
            row::{
                ListStaffRolesOpts, StaffRoleDetail, StaffRoleMembersPage, StaffRolePage,
                StaffRoleRow,
            },
            update, {create as role_create, delete as role_delete, get},
        },
    },
    service::{
        errors::StaffRoleServiceError, staff_role::types::UserStaffRoles,
    },
    state::AppState,
};

const MAX_NAME_CHARS: usize = 64;

/// `audit_logs.target_type` for every staff-role action.
///
/// One target type rather than one per operation, because the ids live in
/// different tables: a role id and a user id are both `BIGINT` and would collide
/// in `target_id` if the audit feed were filtered by id alone. `admin::get`
/// already documents that `target_type` is the disambiguator.
const TARGET_TYPE: &str = "staff_role";

pub struct StaffRoleService;

impl StaffRoleService {
    /// One page of roles, ordered by position then id.
    ///
    /// Read-only, so no commit: the transaction is rolled back when the scope
    /// drops, exactly as `AdminService::list_users` does.
    pub async fn list_roles(
        state: &AppState,
        q: Option<&str>,
        before: Option<i32>,
        before_id: Option<i64>,
        limit: i64,
    ) -> Result<StaffRolePage, StaffRoleServiceError> {
        let opts = ListStaffRolesOpts {
            q: q.map(str::to_owned),
            before,
            before_id,
            limit,
        };

        let mut tx = state.db_pool.begin().await?;

        let page = get::list_roles(&mut tx, &opts).await?;

        Ok(page)
    }

    /// One role plus its member count.
    ///
    /// The member *list* is deliberately not here. A role can hold an unbounded
    /// number of users, so the detail view paginates them through `list_members`
    /// rather than having one role's size dictate the response size of every
    /// detail fetch.
    pub async fn get_role(
        state: &AppState,
        role_id: i64,
    ) -> Result<StaffRoleDetail, StaffRoleServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let detail = get::detail(&mut tx, role_id)
            .await?
            .ok_or(StaffRoleServiceError::RoleNotFound)?;

        Ok(detail)
    }

    /// One page of a role's members.
    ///
    /// Refuses an unknown role with a 404 rather than an empty page, so a stale
    /// role link is distinguishable from a role that legitimately has no members
    /// yet. Those are the same shape of answer, so the check is one `EXISTS`.
    pub async fn list_role_members(
        state: &AppState,
        role_id: i64,
        before_user_id: Option<i64>,
        limit: i64,
    ) -> Result<StaffRoleMembersPage, StaffRoleServiceError> {
        let mut tx = state.db_pool.begin().await?;

        if !check::exists(&mut tx, role_id).await? {
            return Err(StaffRoleServiceError::RoleNotFound);
        }

        let page = get::list_members(&mut tx, role_id, before_user_id, limit).await?;

        Ok(page)
    }

    /// Create a role.
    ///
    /// The name is trimmed and length-checked before the transaction opens,
    /// because those are properties of the request rather than of database state.
    ///
    /// `position` is not defaulted here — the column is `INT NOT NULL` with no
    /// default, so a caller that omits it would get a NOT NULL violation and a
    /// 500. The handler applies the default instead, where "the caller did not
    /// say" is still a visible input decision.
    pub async fn create_role(
        state: &AppState,
        performed_by: i64,
        name: &str,
        description: Option<&str>,
        position: i32,
        permission_bitmask: &[i64],
    ) -> Result<StaffRoleRow, StaffRoleServiceError> {
        let name = Self::normalize_name(name)?;

        // A blank description is stored as NULL rather than as an empty string,
        // matching the guild name validator: `""` and "no description" are not
        // the same thing to a client reading the row back.
        let description = description
            .map(str::trim)
            .filter(|d| !d.is_empty());

        let mut tx = state.db_pool.begin().await?;

        let role_id = state.snowflake_generator.generate_id()?;

        let Some(role) = role_create::create_role(
            &mut tx,
            role_id,
            &name,
            description,
            position,
            permission_bitmask,
        )
        .await?
        else {
            // `ON CONFLICT (name) DO NOTHING` returned no row, so the name is
            // taken. Distinguished here rather than surfaced as a raw database
            // error because it is a 409 the client can act on.
            return Err(StaffRoleServiceError::NameTaken);
        };

        let audit_id = state.snowflake_generator.generate_id()?;
        create::write(
            &mut tx,
            audit_id,
            role_id,
            TARGET_TYPE,
            "create_role",
            performed_by,
            json!({
                "name": role.name,
                "position": role.position,
                "permission_bitmask": role.permission_bitmask,
            }),
        )
        .await?;

        tx.commit().await?;

        Ok(role)
    }

    /// Partially update a role.
    ///
    /// The name collision check runs before the update, using `exclude_id` so
    /// re-saving a role under its own name is not a conflict. It is a `SELECT`
    /// without a lock, so two concurrent renames onto the same name can both pass
    /// it — but the `UPDATE` itself cannot, because `name` is `UNIQUE`, and one
    /// of them gets the database's unique violation instead. That is a 500 rather
    /// than the 409 this path would have returned, which is the honest outcome
    /// for a lost race and is the same trade `AdminService` makes elsewhere.
    pub async fn update_role(
        state: &AppState,
        performed_by: i64,
        role_id: i64,
        name: Option<&str>,
        description: Option<&str>,
        position: Option<i32>,
        permission_bitmask: Option<&[i64]>,
    ) -> Result<StaffRoleRow, StaffRoleServiceError> {
        // Validate before opening the transaction: an invalid name is a property
        // of the request, so there is nothing to read.
        let name = match name {
            Some(raw) => Some(Self::normalize_name(raw)?),
            None => None,
        };

        let description = description.map(str::trim).filter(|d| !d.is_empty());

        let mut tx = state.db_pool.begin().await?;

        let Some(previous) = get::find_by_id(&mut tx, role_id).await? else {
            return Err(StaffRoleServiceError::RoleNotFound);
        };

        if let Some(name) = name.as_deref() {
            if name != previous.name && check::name_taken(&mut tx, name, Some(role_id)).await? {
                return Err(StaffRoleServiceError::NameTaken);
            }
        }

        let Some(role) = update::update_role(
            &mut tx,
            role_id,
            name.as_deref(),
            description,
            position,
            permission_bitmask,
        )
        .await?
        else {
            return Err(StaffRoleServiceError::RoleNotFound);
        };

        let audit_id = state.snowflake_generator.generate_id()?;
        create::write(
            &mut tx,
            audit_id,
            role_id,
            TARGET_TYPE,
            "update_role",
            performed_by,
            json!({
                "previous": {
                    "name": previous.name,
                    "description": previous.description,
                    "position": previous.position,
                    "permission_bitmask": previous.permission_bitmask,
                },
                "new": {
                    "name": role.name,
                    "description": role.description,
                    "position": role.position,
                    "permission_bitmask": role.permission_bitmask,
                },
            }),
        )
        .await?;

        tx.commit().await?;

        Ok(role)
    }

    /// Hard-delete a role.
    ///
    /// The member ids are read *before* the delete, because the delete cascades
    /// them away: once it commits, "who was in this role" is no longer
    /// answerable from anywhere. The audit row records that set, so the trail
    /// survives the loss.
    ///
    /// A bodyless `POST` would be the alternative to `DELETE`, and the
    /// moderation routes in `admin_routes` use `POST` for suspend/activate. The
    /// difference is that those are state flips on a row that must survive,
    /// while this removes a row entirely — which is why it is `DELETE`, and why
    /// the frontend confirmation dialog is mandatory rather than advisory.
    pub async fn delete_role(
        state: &AppState,
        performed_by: i64,
        role_id: i64,
    ) -> Result<(), StaffRoleServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let Some(previous) = get::find_by_id(&mut tx, role_id).await? else {
            return Err(StaffRoleServiceError::RoleNotFound);
        };

        let member_ids = assign::role_member_ids(&mut tx, role_id).await?;

        let deleted = role_delete::delete_role(&mut tx, role_id).await?;
        if deleted == 0 {
            return Err(StaffRoleServiceError::RoleNotFound);
        }

        let audit_id = state.snowflake_generator.generate_id()?;
        create::write(
            &mut tx,
            audit_id,
            role_id,
            TARGET_TYPE,
            "delete_role",
            performed_by,
            json!({
                "name": previous.name,
                "position": previous.position,
                // Recorded so the cascade's effect is still reviewable after the
                // assignments themselves are gone.
                "revoked_from_user_ids": member_ids,
            }),
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }

    /// One user's assigned roles.
    pub async fn get_user_staff_roles(
        state: &AppState,
        user_id: i64,
    ) -> Result<UserStaffRoles, StaffRoleServiceError> {
        let mut tx = state.db_pool.begin().await?;

        // Confirms the user exists rather than reporting an empty set for an id
        // that was never a user. Uses the admin row because the same moderator
        // who is assigning roles has to be able to inspect a soft-deleted one.
        let exists: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (SELECT 1 FROM users WHERE id = $1)
            "#,
        )
        .bind(user_id)
        .fetch_one(tx.as_mut())
        .await?;

        if !exists {
            return Err(StaffRoleServiceError::UserNotFound);
        }

        let roles = get::list_user_roles(&mut tx, user_id).await?;

        let role_ids = roles.iter().map(|r| r.id.clone()).collect();

        Ok(UserStaffRoles {
            user_id: user_id.to_string(),
            roles,
            role_ids,
        })
    }

    /// Replace one user's role set.
    ///
    /// Every role id is verified before anything is written, so a request naming
    /// one bad id changes *nothing* rather than applying the valid ones and
    /// failing on the rest. That matters more than it looks: this is a `PUT`, and
    /// a partially-applied replace is exactly the state the caller cannot
    /// represent.
    ///
    /// Writes one audit row per added role and one per removed role, rather than
    /// one row for the whole request. Each individual grant is the thing an admin
    /// is accountable for; a single "roles changed" row would hide which one.
    ///
    /// `assigned_by` is the admin from the extractor, never a request field. The
    /// column has no foreign key precisely so the trail survives that admin's
    /// deletion, which means it must not be caller-controlled either.
    pub async fn set_user_staff_roles(
        state: &AppState,
        performed_by: i64,
        user_id: i64,
        role_ids: &[i64],
    ) -> Result<UserStaffRoles, StaffRoleServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let user_exists: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (SELECT 1 FROM users WHERE id = $1)
            "#,
        )
        .bind(user_id)
        .fetch_one(tx.as_mut())
        .await?;

        if !user_exists {
            return Err(StaffRoleServiceError::UserNotFound);
        }

        // Validate the whole set first, then write. Order matters: this has to
        // come before `diff`, so a bad id in the request produces a 404 rather
        // than a diff that silently omits it.
        for &role_id in role_ids {
            if !check::exists(&mut tx, role_id).await? {
                return Err(StaffRoleServiceError::RoleNotFound);
            }
        }

        let diff = assign::diff(&mut tx, user_id, role_ids).await?;

        // An empty diff still short-circuits: a no-op replace should not churn
        // `assigned_at`, and re-writing identical rows would make the audit
        // trail lie about when the assignment actually changed.
        if diff.added.is_empty() && diff.removed.is_empty() {
            let roles = get::list_user_roles(&mut tx, user_id).await?;
            let ids = roles.iter().map(|r| r.id.clone()).collect();
            return Ok(UserStaffRoles {
                user_id: user_id.to_string(),
                roles,
                role_ids: ids,
            });
        }

        assign::set_user_roles(&mut tx, user_id, role_ids, performed_by).await?;

        for &role_id in &diff.added {
            let audit_id = state.snowflake_generator.generate_id()?;
            create::write(
                &mut tx,
                audit_id,
                user_id,
                "user",
                "assign_staff_role",
                performed_by,
                json!({ "role_id": role_id.to_string() }),
            )
            .await?;
        }

        for &role_id in &diff.removed {
            let audit_id = state.snowflake_generator.generate_id()?;
            create::write(
                &mut tx,
                audit_id,
                user_id,
                "user",
                "revoke_staff_role",
                performed_by,
                json!({ "role_id": role_id.to_string() }),
            )
            .await?;
        }

        // Re-read rather than trusting the request, so the response is the set the
        // database actually holds.
        let roles = get::list_user_roles(&mut tx, user_id).await?;

        tx.commit().await?;

        let ids = roles.iter().map(|r| r.id.clone()).collect();

        Ok(UserStaffRoles {
            user_id: user_id.to_string(),
            roles,
            role_ids: ids,
        })
    }

    /// Trim a role name and reject one that is blank or too long.
    ///
    /// Trimmed before the length check, so a name of 70 spaces is rejected for
    /// being blank rather than for being long. `chars().count()` rather than
    /// `len()` because the cap is on characters: `VARCHAR(255)` counts
    /// characters in postgres too, so a byte length would reject valid input
    /// sooner than the column would.
    fn normalize_name(name: &str) -> Result<String, StaffRoleServiceError> {
        let name = name.trim();

        if name.is_empty() || name.chars().count() > MAX_NAME_CHARS {
            return Err(StaffRoleServiceError::InvalidName);
        }

        Ok(name.to_owned())
    }
}