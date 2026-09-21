use sqlx::Transaction;

use crate::application::repository::{messages::row::MessageRow, post::row::HasAttachmentRow};
pub async fn send_message(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sender_id: i64,
    target_id: i64,
    target_type: &str,
    content: String,
    has_attachment: bool,
) -> Result<MessageRow, sqlx::Error> {
    sqlx::query_as::<_, MessageRow>(
        r#"INSERT INTO messages (
        sender_id, 
        target_id, 
        target_type, 
        content, 
        has_attachment
        )
        VALUES ($1, $2, $3, $4, $5) 
        RETURNING *"#,
    )
    .bind(sender_id)
    .bind(target_id)
    .bind(target_type)
    .bind(content)
    .bind(has_attachment)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn add_has_attachment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    target_id: i64,
    media_id: i64,
    target_type: String,
) -> Result<Vec<HasAttachmentRow>, sqlx::Error> {
    sqlx::query_as::<_, HasAttachmentRow>(
        r#"
            INSERT INTO media_attachments ( 
                target_id,
                media_id,
                target_type
            )
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(target_id)
    .bind(media_id)
    .bind(target_type)
    .fetch_all(tx.as_mut())
    .await
}
