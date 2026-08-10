use sqlx::Transaction;
use crate::application::repository::post::row::CommentRow;

// todo : impl the media attachment
// todo : update comment
// * The like comment function using the like_post in post

pub async fn create_comment(
    // tx : &mut Transaction<'_,sqlx::Postgres>,
    // id: i64,
    // post_id: i64,
    // user_id: i64,
    // content: &str
) -> Result<(), sqlx::Error> {
    // sqlx::query_as::<_, CommentRow>(
    //     r#"
    //     "#
    // )
    // ;

    Ok(())
}
pub async fn get_comment(
    tx : &mut Transaction<'_,sqlx::Postgres>,
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
                cm.updated_at

            COALESCE(att.attachments, '[]'::json) AS attachments,

            FROM media_comments

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
                    ORDER by md.id
                ) AS attachments
                 FROM media_attachments a
                 JOIN media_data md
                    ON md.id = a.target_id
                WHERE 
                    a.media_id = m.Id
                    AND md.media_status != 'pending'
            ) att ON TRUE

            WHERE
                status != 'inactive'

            ORDER BY cm.total_likes DESC
        "#
    )
    .fetch_all(tx.as_mut())
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
