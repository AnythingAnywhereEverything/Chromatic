use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

/// Hard-delete a staff role. Returns rows affected so the caller can tell a real
/// delete from a miss.
///
/// This is a hard delete, and that is forced by the schema rather than chosen:
/// `staff_roles` has no `deleted_at`, so there is nowhere to record a soft
/// deletion. The audit row written by the service is the only remaining record
/// that the role ever existed.
///
/// `user_staff_roles.role_id` is `ON DELETE CASCADE`, so every assignment to
/// this role disappears in the same statement. That is why the service writes
/// one audit row per affected member: the cascade makes those assignments
/// unobservable afterwards, so the audit has to capture them while it still can.
pub async fn delete_role(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    role_id: i64,
) -> RepositoryResult<u64> {
    let result = sqlx::query(
        r#"
        DELETE FROM staff_roles WHERE id = $1
        "#,
    )
    .bind(role_id)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected())
}