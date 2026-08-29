use sqlx::{Transaction};
use crate::application::repository::post::row::{CommentRow, CreateCommentResult};

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
        select 
        	mc.id ,
        	mc.post_id ,
        	mc.user_id ,
        	mc."content" ,
        	mc.total_likes ,
        	mc.has_attachment ,
        	mc.created_at ,
        	mc.updated_at,
        	u.username ,
        	up.display_name,
        	avatar_md.path AS avatar_path,
            avatar_mdt.mime_type AS avatar_mime,
            avatar_md.thumbhash AS avatar_thumbhash,
            up.followers_count,
            up.following_count,
            EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.target_id = mc.id
                    AND ml.target_type = 'comment'
                    AND ml.user_id = $2
                    AND ml.is_like = TRUE
                ) AS is_liked,
        	COALESCE(att.attachments, '[]'::json) AS media_attachment
        from media_comments mc 
        LEFT JOIN LATERAL (
            SELECT json_agg(
                json_build_object(
                    'id', md.id::text,
                    'user_id', md.uploader_id,
                    'path', md.path,
                    'created_at', md.created_at,
                    'thumbhash', md.thumbhash,
                    'name', md.name,
                    'flags', md.flags,
                    'updated_at', md.updated_at,
                    'status', md.status,
                    'file_size', mdt.file_size,
                    'mime_type', mdt.mime_type,
                    'width', mdt.width,
                    'height', mdt.height,
                    'duration', mdt.duration
                )
                ORDER BY md.id
            ) AS attachments
            FROM media_attachments a
            JOIN media_data md ON md.id = a.media_id
            JOIN media_metadata mdt ON mdt.media_id = md.id
            WHERE a.target_id = mc.id
              AND md.status = 'completed'
        ) att ON TRUE
        left join users u 
        	on mc.user_id = u.id
        LEFT JOIN user_profiles up
             ON mc.user_id = up.user_id
        LEFT JOIN media_data avatar_md
        	ON avatar_md.id = up.avatar_media_id
        LEFT JOIN media_metadata avatar_mdt
        	ON avatar_mdt.media_id = avatar_md.id
        where mc.post_id = $1
        AND mc.status != 'inactive'
        ORDER BY mc.total_likes DESC
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_all(tx.as_mut())
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
                mc.id,
                mc.post_id,
                mc.user_id,
                u.username,
                up.display_name,
                avatar_md.path AS avatar_path,
                avatar_mdt.mime_type AS avatar_mime,
                avatar_md.thumbhash AS avatar_thumbhash,
                up.followers_count,
                up.following_count,
                mc.content,
                mc.total_likes,
                mc.has_attachment,
                mc.created_at,
                mc.updated_at,
                EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.target_id = mc.id
                    AND ml.target_type = 'comment'
                    AND ml.user_id = $2
                    AND ml.is_like = TRUE
                ) AS is_liked,
                COALESCE(att.attachments, '[]'::json) AS media_attachment
                FROM media_comments mc
                 LEFT JOIN LATERAL (
                    SELECT json_agg(
                        json_build_object(
                            'id', md.id::text,
                            'user_id', md.uploader_id,
                            'path', md.path,
                            'created_at', md.created_at,
                            'thumbhash', md.thumbhash,
                            'flags', md.flags,
                            'name', md.name,
                            'updated_at', md.updated_at,
                            'status', md.status,
                            'file_size', mdt.file_size,
                            'mime_type', mdt.mime_type,
                            'width', mdt.width,
                            'height', mdt.height,
                            'duration', mdt.duration
                        )
                        ORDER BY md.id
                    ) AS attachments
                    FROM media_attachments a
                    JOIN media_data md ON md.id = a.media_id
                    JOIN media_metadata mdt ON mdt.media_id = md.id
                    WHERE a.target_id = mc.id
                      AND md.status = 'completed'
                ) att ON TRUE
                LEFT JOIN users u
                    ON mc.user_id = u.id
                LEFT JOIN user_profiles up
                    ON mc.user_id = up.user_id
                LEFT JOIN media_data avatar_md
                    ON avatar_md.id = up.avatar_media_id
                LEFT JOIN media_metadata avatar_mdt
                    ON avatar_mdt.media_id = avatar_md.id
                WHERE
                    mc.id = $1
                AND mc.status != 'inactive'
                ORDER BY mc.id DESC
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
                status = "inactive"
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