use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    guild::row::GuildMemberRow,
};

pub async fn insert(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    guild_id: i64,
    user_id: i64,
) -> RepositoryResult<Option<GuildMemberRow>> {
    let member = sqlx::query_as::<_, GuildMemberRow>(
        r#"
        INSERT INTO guild_members (guild_id, user_id)
        VALUES ($1, $2)
        ON CONFLICT (guild_id, user_id) DO NOTHING
        RETURNING
            guild_id::TEXT AS guild_id,
            guild_id AS guild_id_raw,
            user_id::TEXT AS user_id,
            user_id AS user_id_raw,
            joined_at,
            deleted_at
        "#
    )
    .bind(guild_id)
    .bind(user_id)
    .fetch_optional(tx.as_mut())
    .await?;

    if member.is_some() {
        sqlx::query(
            r#"
            UPDATE guilds
            SET total_members = total_members + 1, updated_at = now()
            WHERE id = $1
            "#
        )
        .bind(guild_id)
        .execute(tx.as_mut())
        .await?;
    }

    Ok(member)
}