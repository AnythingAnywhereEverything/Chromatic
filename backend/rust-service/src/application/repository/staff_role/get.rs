use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    staff_role::row::{
        ListStaffRolesOpts, StaffRoleDetail, StaffRoleMemberRow, StaffRoleMembersPage,
        StaffRolePage, StaffRoleRow,
    },
};


pub async fn list_roles(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    opts: &ListStaffRolesOpts,
) -> RepositoryResult<StaffRolePage> {
    let limit = opts.limit.clamp(1, 200);

    let mut rows: Vec<StaffRoleRow> = sqlx::query_as::<_, StaffRoleRow>(
        r#"
        SELECT
            staff_roles.id::TEXT AS id,
            staff_roles.id AS id_raw,
            staff_roles.name,
            staff_roles.description,
            staff_roles.position,
            staff_roles.permission_bitmask,
            staff_roles.created_at,
            staff_roles.updated_at
        FROM staff_roles
        WHERE
            ($1::TEXT IS NULL OR staff_roles.name ILIKE '%' || $1 || '%')
            AND (
                $2::INT IS NULL
                OR (staff_roles.position, staff_roles.id) > ($2, $3)
            )
        ORDER BY staff_roles.position ASC, staff_roles.id ASC
        LIMIT $4
        "#,
    )
    .bind(&opts.q)
    .bind(opts.before)
    .bind(opts.before_id)
    .bind(limit + 1)
    .fetch_all(tx.as_mut())
    .await?;

    let has_more = rows.len() as i64 > limit;
    rows.truncate(limit as usize);

    Ok(StaffRolePage { rows, has_more })
}

/// Fetch one role by id, or `None` when it does not exist.
pub async fn find_by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: i64,
) -> RepositoryResult<Option<StaffRoleRow>> {
    let row = sqlx::query_as::<_, StaffRoleRow>(
        r#"
        SELECT
            staff_roles.id::TEXT AS id,
            staff_roles.id AS id_raw,
            staff_roles.name,
            staff_roles.description,
            staff_roles.position,
            staff_roles.permission_bitmask,
            staff_roles.created_at,
            staff_roles.updated_at
        FROM staff_roles
        WHERE staff_roles.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn detail(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: i64,
) -> RepositoryResult<Option<StaffRoleDetail>> {
    let row = sqlx::query_as::<_, StaffRoleDetail>(
        r#"
        SELECT
            staff_roles.id::TEXT AS id,
            staff_roles.id AS id_raw,
            staff_roles.name,
            staff_roles.description,
            staff_roles.position,
            staff_roles.permission_bitmask,
            staff_roles.created_at,
            staff_roles.updated_at,
            (
                SELECT COUNT(*)
                FROM user_staff_roles
                WHERE user_staff_roles.role_id = staff_roles.id
            ) AS member_count
        FROM staff_roles
        WHERE staff_roles.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row)
}


pub async fn list_members(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    role_id: i64,
    before_user_id: Option<i64>,
    limit: i64,
) -> RepositoryResult<StaffRoleMembersPage> {
    let limit = limit.clamp(1, 200);

    let mut rows: Vec<StaffRoleMemberRow> = sqlx::query_as::<_, StaffRoleMemberRow>(
        r#"
        SELECT
            user_staff_roles.user_id::TEXT AS user_id,
            user_staff_roles.user_id AS user_id_raw,
            users.username,
            user_profiles.display_name,
            media_objects.name AS avatar,
            user_staff_roles.assigned_at,
            user_staff_roles.assigned_by::TEXT AS assigned_by
        FROM user_staff_roles
        LEFT JOIN users ON users.id = user_staff_roles.user_id
        LEFT JOIN user_profiles ON user_profiles.user_id = user_staff_roles.user_id
        LEFT JOIN media_objects
            ON user_profiles.avatar_media_id = media_objects.media_id
            AND media_objects.kind = 'original'
        WHERE
            user_staff_roles.role_id = $1
            AND ($2::BIGINT IS NULL OR user_staff_roles.user_id < $2)
        ORDER BY user_staff_roles.user_id ASC
        LIMIT $3
        "#,
    )
    .bind(role_id)
    .bind(before_user_id)
    .bind(limit + 1)
    .fetch_all(tx.as_mut())
    .await?;

    let has_more = rows.len() as i64 > limit;
    rows.truncate(limit as usize);

    Ok(StaffRoleMembersPage { rows, has_more })
}

pub async fn list_user_roles(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<Vec<StaffRoleRow>> {
    let rows = sqlx::query_as::<_, StaffRoleRow>(
        r#"
        SELECT
            staff_roles.id::TEXT AS id,
            staff_roles.id AS id_raw,
            staff_roles.name,
            staff_roles.description,
            staff_roles.position,
            staff_roles.permission_bitmask,
            staff_roles.created_at,
            staff_roles.updated_at
        FROM user_staff_roles
        JOIN staff_roles ON staff_roles.id = user_staff_roles.role_id
        WHERE user_staff_roles.user_id = $1
        ORDER BY staff_roles.position ASC, staff_roles.id ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(rows)
}