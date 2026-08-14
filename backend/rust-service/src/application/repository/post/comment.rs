use sqlx::Transaction;
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
) -> Result<Vec<CommentRow>, sqlx::Error> {
    sqlx::query_as::<_, CommentRow>(
        r#"
        SELECT
            cm.id,
            cm.post_id,
            cm.user_id,
            cm.content,
            cm.has_attachment,
            cm.total_likes,
            cm.created_at,
            cm.updated_at,
            COALESCE(att.attachments, '[]'::json) AS attachments
        FROM media_comments cm
        LEFT JOIN LATERAL (
            SELECT json_agg(
                json_build_object(
                    'id', md.id,
                    'user_id', md.user_id,
                    'media_url', md.media_url,
                    'media_preview_url', md.media_preview_url,
                    'media_category', md.media_category,
                    'media_status', md.media_status,
                    'created_at', md.created_at
                )
                ORDER BY md.id
            ) AS attachments
            FROM media_attachments a
            JOIN media_data md
                ON md.id = a.target_id
            WHERE
                a.media_id = cm.id
                AND md.media_status != 'pending'
        ) att ON TRUE
        WHERE
            cm.post_id = $1
            AND cm.status != 'inactive'
        ORDER BY cm.total_likes DESC
        "#,
    )
    .bind(post_id)
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
    comment_id: i64
) -> Result<CommentRow, sqlx::Error> {
    sqlx::query_as::<_,CommentRow>(
        r#"
            SELECT *
            FROM media_comments
            WHERE id = $1
        "#  
    )
    .bind(comment_id)
    .fetch_one(&mut **tx)
    .await
}

pub async fn delete_comment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    comment_id: i64,
    user_id: i64
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        r#"
        DELETE FROM media_comments
        WHERE id = $1 AND user_id = $2
        "#
    )
    .bind(comment_id)
    .bind(user_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
