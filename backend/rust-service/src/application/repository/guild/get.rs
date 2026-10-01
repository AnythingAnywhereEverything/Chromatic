use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    guild::row::{GuildPage, GuildRow},
};

pub async fn by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    guild_id: i64,
    requester_id: Option<i64>,
) -> RepositoryResult<Option<GuildRow>> {
    let row = sqlx::query_as::<_, GuildRow>(
        r#"
        SELECT
            id::TEXT AS id,
            id AS id_raw,
            owner_id::TEXT AS owner_id,
            ($2::bigint IS NOT NULL AND guilds.owner_id = $2) AS is_owner,
            name,
            description,
            total_members,
            total_channels,
            created_at,
            updated_at,
            deleted_at
        FROM guilds
        WHERE id = $1
        "#
    )
    .bind(guild_id)
    .bind(requester_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn for_join(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    before: Option<chrono::DateTime<chrono::Utc>>,
    before_id: Option<i64>,
    limit: i64,
    requester_id: Option<i64>,
) -> RepositoryResult<GuildPage> {
    let limit = limit.clamp(1, 200);

    let rows = sqlx::query_as::<_, GuildRow>(
        r#"
        SELECT
            g.id::TEXT AS id,
            g.id AS id_raw,
            g.owner_id::TEXT AS owner_id,
            ($4::bigint IS NOT NULL AND g.owner_id = $4) AS is_owner,
            g.name,
            g.description,
            g.total_members,
            g.total_channels,
            g.created_at,
            g.updated_at,
            g.deleted_at
        FROM guilds g
        WHERE g.deleted_at IS NULL
        AND ($1::timestamptz IS NULL
            OR (g.created_at, g.id) < ($1::timestamptz, $2::bigint))
        ORDER BY g.created_at DESC, g.id DESC
        LIMIT $3
        "#
    )
    .bind(before)
    .bind(before_id)
    .bind(limit + 1)
    .bind(requester_id)
    .fetch_all(tx.as_mut())
    .await?;

    let has_more = rows.len() as i64 > limit;
    let mut rows = rows;
    rows.truncate(limit as usize);

    Ok(GuildPage { rows, has_more })
}