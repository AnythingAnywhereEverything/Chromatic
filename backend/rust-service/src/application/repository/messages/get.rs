use crate::application::repository::{
    RepositoryResult,
    messages::row::{MessageBaseRow, MessageRow},
};

pub async fn is_message_exist(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sender_id: i64,
    message_id: i64,
) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_as::<_, (bool,)>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM messages m
            WHERE m.id = $1
            AND m.user_id = $2
            AND m.deleted_at IS NULL
        )
        "#,
    )
    .bind(message_id)
    .bind(sender_id)
    .fetch_one(tx.as_mut())
    .await?;
    Ok(exists.0)
}

pub async fn get_messages(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    target_id: i64,
    target_type: &str,
    limit: i64,
) -> Result<Vec<MessageRow>, sqlx::Error> {
    sqlx::query_as::<_, MessageRow>(
        r#"
        SELECT * FROM messages m
        WHERE 
        ((m.user_id = $1 AND m.target_id = $2)
        OR (m.user_id = $2 AND m.target_id = $1)) 
        AND m.target_type = $3 LIMIT $4
        "#,
    )
    .bind(user_id)
    .bind(target_id)
    .bind(target_type)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await
}

pub async fn get_messages_rows(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    target_id: i64,
    before: chrono::DateTime<chrono::Utc>,
    before_id: Option<i64>,
    limit: i32,
) -> Result<Vec<MessageRow>, sqlx::Error> {
    sqlx::query_as::<_, MessageRow>(
        r#"
        SELECT
            m.id::text AS id,
            m.user_id::text AS user_id,
            m.target_id::text AS target_id,
            m.content,
            m.has_attachment,
            m.has_reactions,
            m.created_at,
            m.updated_at
        FROM messages m
        WHERE ((m.user_id = $1 AND m.target_id = $2)
        OR (m.user_id = $2 AND m.target_id = $1)) 
        AND m.deleted_at IS NULL
        AND (
            m.created_at < $3
            OR (m.created_at = $3 AND ($4::bigint IS NULL OR m.id < $4))
        )
        ORDER BY m.created_at DESC, m.id DESC
        LIMIT $5
        "#,
    )
    .bind(user_id)
    .bind(target_id)
    .bind(before)
    .bind(before_id)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await
}

pub async fn base_message(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    message_id: i64,
    user_id: i64,
) -> Result<Option<MessageBaseRow>, sqlx::Error> {
    sqlx::query_as::<_, MessageBaseRow>(
        r#"
        SELECT
            m.id::text AS id,
            m.user_id::text AS user_id,
            m.target_id::text AS target_id,
            m.content,
            m.has_attachment,
            m.has_reactions,
            m.created_at,
            m.updated_at
        FROM messages m
        WHERE m.id = $1
          AND (
              (m.user_id = $2 OR target_id = $2)
          )
          AND m.deleted_at IS NULL
        "#,
    )
    .bind(message_id)
    .bind(user_id)
    .fetch_optional(tx.as_mut())
    .await
}

pub async fn get_id_for_new_messages(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<Vec<i64>> {
    let mutual_users = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT uf.user_id
        FROM user_follow uf
        WHERE uf.follower_id = $1
          AND EXISTS (
              SELECT 1
              FROM user_follow mutual
              WHERE mutual.user_id = $1
                AND mutual.follower_id = uf.user_id
          )
        "#,
    )
    .bind(user_id)
    .fetch_all(tx.as_mut())
    .await?;


    Ok(mutual_users.into_iter().map(|(id,)| id).collect())
}
