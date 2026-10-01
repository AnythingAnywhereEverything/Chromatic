use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, staff_role::row::StaffRoleRow};

pub async fn update_role(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    role_id: i64,
    name: Option<&str>,
    description: Option<&str>,
    position: Option<i32>,
    permission_bitmask: Option<&[i64]>,
) -> RepositoryResult<Option<StaffRoleRow>> {
    let row = sqlx::query_as::<_, StaffRoleRow>(
        r#"
        UPDATE staff_roles SET
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            position = COALESCE($4, position),
            permission_bitmask = COALESCE($5, permission_bitmask),
            updated_at = now()
        WHERE id = $1
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
    .bind(role_id)
    .bind(name)
    .bind(description)
    .bind(position)
    .bind(permission_bitmask)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row)
}