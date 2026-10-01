use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

pub async fn guild_is_joinable(tx: &mut Transaction<'_, sqlx::Postgres>, guild_id: i64) -> RepositoryResult<bool> {
    let row: Option<(bool,)> = sqlx::query_as(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM guilds
            WHERE id = $1 AND deleted_at IS NULL
        )
        "#
    )
    .bind(guild_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row.map(|r| r.0).unwrap_or(false))
}