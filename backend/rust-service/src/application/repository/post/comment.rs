use sqlx::{Transaction};
use crate::application::repository::post::row::{CommentRow, CreateCommentResult, TotalLikesRow};

// todo : impl the media attachment
// todo : update comment
// * The like comment function using the like_post in post


pub async fn create_comment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: &i64,
    post_id: i64,
    user_id: i64,
    content: &str,
    has_attachment: bool,
) -> Result<CreateCommentResult, sqlx::Error> {
    sqlx::query_as::<_, CreateCommentResult>(
        r#"
        WITH inserted AS (
            INSERT INTO media_comments (
                id, post_id, user_id, content, has_attachment, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING *
        ),
        updated AS (
            UPDATE media_posts
            SET total_comments = total_comments + 1
            WHERE id = $2
            RETURNING total_comments, total_likes
        )
        SELECT
            inserted.id,
            inserted.post_id,
            inserted.user_id,
            inserted.content,
            inserted.has_attachment,
            inserted.created_at,
            inserted.updated_at,
            updated.total_comments,
            updated.total_likes
        FROM inserted, updated
        "#,
    )
    .bind(id)
    .bind(post_id)
    .bind(user_id)
    .bind(content)
    .bind(has_attachment)
    .fetch_one(&mut **tx)
    .await
}


pub async fn get_comment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    post_id: i64,
    user_id: Option<i64>,
) -> Result<Vec<CommentRow>, sqlx::Error> {
    sqlx::query_as::<_, CommentRow>(
        r#"
        SELECT
            (
                SELECT json_build_object(
                    'id', u.id::TEXT,
                    'username', u.username,
                    'display_name', up.display_name,
                    'avatar', mo.name,
                    'avatar_thumbhash', mo.thumbhash,
                    'created_at', u.created_at
                )
                FROM "users" u
                JOIN user_profiles up
                    ON up.user_id = u.id
                LEFT JOIN media_objects mo
                    ON mo.media_id = up.avatar_media_id
                WHERE u.id = c.user_id
                  AND u.deleted_at IS NULL
                LIMIT 1
            ) AS author,

            c.id::TEXT AS id,
            c.post_id::TEXT AS post_id,
            c.content,

            EXISTS (
                SELECT 1
                FROM media_likes ml
                WHERE ml.target_id = c.id
                  AND ml.target_type = 'comment'
                  AND ml.user_id = $2
                  AND ml.is_like = TRUE
            ) AS is_liked,

            c.total_likes,
            c.has_attachment,

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
                    WHERE ma.target_id = c.id
                      AND gma.deleted_at IS NULL
                ),
                '[]'::jsonb
            ) AS attachments,

            c.created_at,
            c.updated_at

        FROM media_comments c
        WHERE c.post_id = $1
          AND c.deleted_at IS NULL
          AND c.status != 'inactive'
        ORDER BY c.total_likes DESC
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_all(tx.as_mut())
    .await
}

pub async fn get_comment_by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    comment_id: i64,
    user_id: Option<i64>,
) -> Result<CommentRow, sqlx::Error> {
    sqlx::query_as::<_, CommentRow>(
        r#"
        SELECT
            (
                SELECT json_build_object(
                    'id', u.id::TEXT,
                    'username', u.username,
                    'display_name', up.display_name,
                    'avatar', mo.name,
                    'avatar_thumbhash', mo.thumbhash,
                    'created_at', u.created_at
                )
                FROM "users" u
                JOIN user_profiles up
                    ON up.user_id = u.id
                LEFT JOIN media_objects mo
                    ON mo.media_id = up.avatar_media_id
                WHERE u.id = c.user_id
                  AND u.deleted_at IS NULL
                LIMIT 1
            ) AS author,
            c.id::TEXT as id,
            c.post_id::TEXT as post_id,
            c.content,
            (
                EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.target_id = c.id
                      AND ml.target_type = 'comment'
                      AND ml.user_id = $2
                      AND ml.is_like = TRUE
                )
            ) AS is_liked,
            c.total_likes,
            c.has_attachment,
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
                    WHERE ma.target_id = c.id
                      AND gma.deleted_at IS NULL
                ),
                '[]'::jsonb
            ) AS attachments,
            c.created_at,
            c.updated_at
        FROM media_comments c
        WHERE c.id = $1
          AND c.deleted_at IS NULL
          AND c.status != 'inactive'
        "#,
    )
    .bind(comment_id)
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn update_comment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    comment_id: i64,
    content: &str,
    has_attachment: bool,
) -> Result<CommentRow, sqlx::Error> {
    sqlx::query_as::<_, CommentRow>(
        r#"
            UPDATE media_comments
            SET
                content = $1,
                has_attachment = $2,
                updated_at = NOW()
            WHERE id = $3
            RETURNING *
        "#,
    )
    .bind(content)
    .bind(has_attachment)
    .bind(comment_id)
    .fetch_one(&mut **tx)
    .await
}

pub async fn get_specific_comment(
    tx : &mut Transaction<'_,sqlx::Postgres>,
    comment_id: i64,
    user_id: i64,
) -> Result<CommentRow, sqlx::Error> {
    sqlx::query_as::<_,CommentRow>(
        r#"
        SELECT
            (
                SELECT json_build_object(
                    'id', u.id::TEXT,
                    'username', u.username,
                    'display_name', up.display_name,
                    'avatar', mo.name,
                    'avatar_thumbhash', mo.thumbhash,
                    'created_at', u.created_at
                )
                FROM "users" u
                JOIN user_profiles up
                    ON up.user_id = u.id
                LEFT JOIN media_objects mo
                    ON mo.media_id = up.avatar_media_id
                WHERE u.id = c.user_id
                  AND u.deleted_at IS NULL
            ) AS author,
            c.id::TEXT as id,
            c.post_id::TEXT as post_id,
            c.content,
            (
                EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.target_id = c.id
                      AND ml.target_type = 'comment'
                      AND ml.user_id = $2
                      AND ml.is_like = TRUE
                )
            ) AS is_liked,
            c.total_likes,
            c.has_attachment,
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
                    WHERE ma.target_id = c.id
                      AND gma.deleted_at IS NULL
                ),
                '[]'::jsonb
            ) AS attachments,
            c.created_at,
            c.updated_at
        FROM media_comments c
        WHERE c.post_id = $1
          AND c.deleted_at IS NULL
          AND c.status != 'inactive'
        "#  
    )
    .bind(comment_id)
    .bind(user_id)
    .fetch_one(&mut **tx)
    .await
}

pub async fn delete_comment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    comment_id: i64,
    user_id: i64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        WITH deleted AS (
            UPDATE media_comments
            SET deleted_at = NOW(),
                updated_at = NOW(),
                status = 'inactive'
            WHERE id = $1
              AND user_id = $2
              AND deleted_at IS NULL
            RETURNING post_id
        )
        UPDATE media_posts
        SET total_comments = GREATEST(total_comments - 1, 0)
        WHERE id = (SELECT post_id FROM deleted)
        "#,
    )
    .bind(comment_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected())
}

pub async fn liked_comment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    comment_id: i64,
    user_id: i64,
    is_like: bool,
) -> Result<TotalLikesRow, sqlx::Error> {
    sqlx::query_as::<_, TotalLikesRow>(
        r#"
    WITH previous AS (
        SELECT is_like
        FROM media_likes
        WHERE user_id = $1
          AND target_id = $2
          AND target_type = $4
    ),
    upsert AS (
        INSERT INTO media_likes (
            user_id,
            target_id,
            target_type,
            created_at,
            updated_at,
            is_like
        )
        VALUES (
            $1,
            $2,
            'comment',
            NOW(),
            NOW(),
            $3
        )
        ON CONFLICT (target_id, user_id, target_type)
        DO UPDATE SET
            is_like = EXCLUDED.is_like,
            updated_at = NOW()
        RETURNING is_like
    )
    UPDATE media_comments
    SET total_likes = total_likes +
        CASE
            WHEN NOT EXISTS (SELECT 1 FROM previous)
                 AND $3 = TRUE
                THEN 1

            WHEN (SELECT is_like FROM previous) = FALSE
                 AND $3 = TRUE
                THEN 1

            WHEN (SELECT is_like FROM previous) = TRUE
                 AND $3 = FALSE
                THEN -1

            ELSE 0
        END
    WHERE id = $2
    RETURNING id, total_likes
        "#,
    )
    .bind(user_id)
    .bind(comment_id)
    .bind(is_like) // Assuming $3 is the is_like boolean value
    .bind("comment") // This binds to $4, the target_type
    .fetch_one(tx.as_mut())
    .await
}