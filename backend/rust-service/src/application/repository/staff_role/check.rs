use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

/// Does a role with this id exist?
///
/// Used to turn a write against a deleted role into a 404 instead of a silent
/// no-op. `set_user_roles` relies on it for every id in the requested set.
pub async fn exists(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    role_id: i64,
) -> RepositoryResult<bool> {
    let row: Option<(bool,)> = sqlx::query_as(
        r#"
        SELECT EXISTS (SELECT 1 FROM staff_roles WHERE id = $1)
        "#,
    )
    .bind(role_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row.map(|r| r.0).unwrap_or(false))
}

/// Is this name already taken by another role?
///
/// `exclude_id` is the role being updated, so re-saving a role under its own
/// name is not a collision. Without it an update would 409 on itself.
pub async fn name_taken(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    name: &str,
    exclude_id: Option<i64>,
) -> RepositoryResult<bool> {
    let row: Option<(bool,)> = sqlx::query_as(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM staff_roles
            WHERE name = $1
              AND ($2::BIGINT IS NULL OR id <> $2)
        )
        "#,
    )
    .bind(name)
    .bind(exclude_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row.map(|r| r.0).unwrap_or(false))
}

/// Does this user hold this role?
pub async fn user_has_role(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    role_id: i64,
) -> RepositoryResult<bool> {
    let row: Option<(bool,)> = sqlx::query_as(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM user_staff_roles
            WHERE user_id = $1 AND role_id = $2
        )
        "#,
    )
    .bind(user_id)
    .bind(role_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row.map(|r| r.0).unwrap_or(false))
}