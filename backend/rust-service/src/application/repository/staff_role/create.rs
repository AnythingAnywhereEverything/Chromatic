use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    staff_role::row::StaffRoleRow,
};

/// Insert a new staff role and return it.
///
/// `id` is passed in because `staff_roles.id` is `BIGINT NOT NULL PRIMARY KEY`
/// with no `DEFAULT` and no sequence — same situation as `audit_logs`. The
/// service generates it from the snowflake generator, which keeps the repository
/// free of id generation like every other repository in this codebase.
///
/// A duplicate `name` returns `Ok(None)`. `name` is `UNIQUE`, and surfacing a
/// raw `sqlx::Error::Database` for a condition the caller can act on would force
/// the service to inspect sqlx's error internals. `Ok(None)` is the cheaper
/// contract: the caller already has to treat "no row came back" as the 404 path
/// in `find_by_id`, so it gets one more meaning for free.
pub async fn create_role(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: i64,
    name: &str,
    description: Option<&str>,
    position: i32,
    permission_bitmask: &[i64],
) -> RepositoryResult<Option<StaffRoleRow>> {
    let result = sqlx::query_as::<_, StaffRoleRow>(
        r#"
        INSERT INTO staff_roles (id, name, description, position, permission_bitmask)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (name) DO NOTHING
        RETURNING
            id::TEXT AS id,
            id AS id_raw,
            name,
            description,
            position,
            permission_bitmask,
            created_at,
            updated_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(position)
    .bind(permission_bitmask)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(result)
}