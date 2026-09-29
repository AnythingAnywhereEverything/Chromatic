use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

/// Delete every session row for a user, inside the caller's transaction.
///
/// Called on suspension so the sessions are gone from postgres in the same
/// commit as the `is_active` flip. The redis side is cleared afterwards by the
/// service, which cannot be transactional.
pub async fn delete_sessions(tx: &mut Transaction<'_, sqlx::Postgres>, user_id: i64) -> RepositoryResult<u64> {
    let result = sqlx::query(
        r#"
        DELETE FROM sessions WHERE user_id = $1
        "#
    )
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected())
}