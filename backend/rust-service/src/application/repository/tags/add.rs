use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    post::row::{TagAttachmentRow, TagRow, TagTarget},
};

pub async fn tag_target(
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

pub async fn tag(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    tag_id: i64,
    tag_name: String,
) -> RepositoryResult<TagRow> {
    let tag = sqlx::query_as::<_, TagRow>(
        r#"
            INSERT INTO interest_tags (id, tag_name, popularity)
            VALUES ($1, $2, $3)
            ON CONFLICT (tag_name) DO UPDATE SET popularity = interest_tags.popularity + 1
            RETURNING id::text as id, tag_name, popularity
        "#,
    )
    .bind(tag_id)
    .bind(tag_name)
    .bind(1)
    .fetch_one(tx.as_mut())
    .await?;
    Ok(tag)
}
