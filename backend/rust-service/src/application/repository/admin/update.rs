use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

/// Suspend or reinstate a user. Returns the number of rows affected so the
/// service can tell a real update from a no-op on a missing row.
pub async fn set_is_active(tx: &mut Transaction<'_, sqlx::Postgres>, user_id: i64, is_active: bool) -> RepositoryResult<u64> {
    let result = sqlx::query(
        r#"
        UPDATE users SET is_active = $2, updated_at = now()
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(user_id)
    .bind(is_active)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected())
}

/// Grant or revoke superuser. Same rows-affected contract as `set_is_active`.
///
/// The "refuse to demote the last superuser" rule is *not* enforced here. This
/// function runs inside a transaction the service owns, and a `SELECT` guard
/// here would be a check that does not lock, so two concurrent demotions could
/// both observe a count of 2 and both write. The service checks under
/// `SELECT ... FOR UPDATE` instead.
pub async fn set_is_superuser(tx: &mut Transaction<'_, sqlx::Postgres>, user_id: i64, is_superuser: bool) -> RepositoryResult<u64> {
    let result = sqlx::query(
        r#"
        UPDATE users SET is_superuser = $2, updated_at = now()
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(user_id)
    .bind(is_superuser)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected())
}