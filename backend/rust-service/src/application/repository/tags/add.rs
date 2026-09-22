use sqlx::Transaction;

use crate::application::repository::post::row::{TagAttachmentRow, TagTarget};

pub async fn add_tags_target(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    target_id: i64,
    target_type: TagTarget,
    tag_id: i64,
) -> Result<TagAttachmentRow, sqlx::Error> {
    sqlx::query_as::<_, TagAttachmentRow>(
        r#"
            INSERT INTO tag_attachments (
                target_id,
                target_type,
                tag_id
            )
            VALUES($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(target_id)
    .bind(target_type)
    .bind(tag_id)
    .fetch_one(tx.as_mut())
    .await
}