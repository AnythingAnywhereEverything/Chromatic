use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

pub async fn soft_delete_guild(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    guild_id: i64,
    owner_id: i64,
) -> RepositoryResult<u64> {
    let result = sqlx::query(
        r#"
        UPDATE guilds
        SET deleted_at = now()
        WHERE id = $1 AND owner_id = $2 AND deleted_at IS NULL
        "#,
    )
    .bind(guild_id)
    .bind(owner_id)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected())
}