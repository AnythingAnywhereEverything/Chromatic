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
        WHERE ((m.user_id = $1 AND m.target_id = $2)
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

pub async fn get_messages_id(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    target_id: i64,
    before: chrono::DateTime<chrono::Utc>,
    limit: i32,
) -> Result<Vec<i64>, sqlx::Error> {
    // pagination based on the 'before' timestamp
    let messages = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT m.id AS message_id
        FROM messages m
        WHERE ((m.user_id = $1 AND m.target_id = $2)
        OR (m.user_id = $2 AND m.target_id = $1)) 
        AND m.deleted_at IS NULL
        AND m.created_at < $3
        ORDER BY m.created_at DESC
        LIMIT $4
        "#,
    )
    .bind(user_id)
    .bind(target_id)
    .bind(before)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await?;
    Ok(messages.into_iter().map(|(id,)| id).collect())
}

pub async fn base_message(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    message_id: i64,
    user_id: i64,
) -> Result<Option<MessageBaseRow>, sqlx::Error> {
    sqlx::query_as::<_, MessageBaseRow>(
        r#"
        SELECT 
            m.id,
            m.user_id,
            m.target_id,
            m.content,
            m.has_attachment,
            COALESCE(
                (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'id', gma.id::text,
                            'file_type', gma.file_type,
                            'processing_state', gma.processing_state,
                            'post_processing_state', gma.post_processing_state,
                            'flags', gma.flags::text,

                            'media_objects', gma.media_objects,
                            'media_object_metadata', gma.media_object_metadata,

                            'media_hls', gma.media_hls,
                            'media_hls_playlists', gma.media_hls_playlists,

                            'created_at', gma.created_at,
                            'updated_at', gma.updated_at
                        )
                        ORDER BY gma.id
                    )
                    FROM media_attachments ma
                    JOIN get_media_by_id_without_playlists(ma.media_id) gma
                        ON gma.id = ma.media_id
                    WHERE ma.target_id = m.id
                    AND gma.deleted_at IS NULL
                ),
                '[]'::jsonb
            ) AS attachments,
            m.has_reactions,
            COALESCE(
                (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'message_id', mr.message_id,
                            'user_id', mr.user_id,
                            'reaction', mr.reaction,
                            'reacted_at', mr.reacted_at
                        )
                        ORDER BY mr.reacted_at
                    )
                    FROM message_reactions mr
                    WHERE mr.message_id = m.id
                ),
                '[]'::jsonb
            ) AS reactions,
            m.created_at,
            m.updated_at
        FROM messages m
        WHERE m.id = $1
        AND m.user_id = $2
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
    let followed_users = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT 
            uf.follower_id
        FROM user_follow uf
        WHERE uf.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(followed_users.into_iter().map(|(id,)| id).collect())
}