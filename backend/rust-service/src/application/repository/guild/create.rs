use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    guild::row::{GuildChannelRow, GuildRow},
};

pub async fn guild(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: i64,
    owner_id: i64,
    name: &str,
    description: Option<&str>,
) -> RepositoryResult<GuildRow> {
    let row = sqlx::query_as::<_, GuildRow>(
        r#"
        INSERT INTO guilds (id, owner_id, name, description)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id::TEXT AS id,
            id AS id_raw,
            owner_id::TEXT AS owner_id,
            TRUE AS is_owner,
            name,
            description,
            total_members,
            total_channels,
            created_at,
            updated_at,
            deleted_at
        "#
    )
    .bind(id)
    .bind(owner_id)
    .bind(name)
    .bind(description)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn channel(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: i64,
    guild_id: i64,
    name: &str,
    channel_type: &str,
) -> RepositoryResult<GuildChannelRow> {
    let row = sqlx::query_as::<_, GuildChannelRow>(
        r#"
        INSERT INTO guild_channels (id, guild_id, name, channel_type)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id::TEXT AS id,
            id AS id_raw,
            guild_id::TEXT AS guild_id,
            name,
            channel_type,
            created_at,
            updated_at,
            deleted_at
        "#
    )
    .bind(id)
    .bind(guild_id)
    .bind(name)
    .bind(channel_type)
    .fetch_one(tx.as_mut())
    .await?;

    sqlx::query(
        r#"
        UPDATE guilds
        SET total_channels = total_channels + 1, updated_at = now()
        WHERE id = $1
        "#
    )
    .bind(guild_id)
    .execute(tx.as_mut())
    .await?;

    Ok(row)
}