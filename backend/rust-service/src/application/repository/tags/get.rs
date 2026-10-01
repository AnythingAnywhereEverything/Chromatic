use sqlx::Transaction;
use sqlx::Postgres;
use crate::application::repository::RepositoryResult;
use crate::application::repository::post::row::TagAttachmentFull;
use crate::application::repository::post::row::TagRow;

pub async fn get_all_tag_attachments_repo(
    tx: &mut Transaction<'_, sqlx::Postgres>,
) -> Result<Vec<TagRow>, sqlx::Error> {
    sqlx::query_as::<_, TagRow>(
        r#"
            SELECT
                id::text as id,
                tag_name
            FROM interest_tags
            ORDER BY id ASC
        "#,
    )
    .fetch_all(tx.as_mut())
    .await
}   

pub async fn tag_attachments(
    tx: &mut Transaction<'_, Postgres>,
    target_id: i64,
) -> Result<Vec<TagAttachmentFull>, sqlx::Error> {
    sqlx::query_as::<_, TagAttachmentFull>(
        r#"
            SELECT
                ta.target_id::text as target_id,
                ta.target_type,
                ta.tag_id::text as tag_id,
                it.tag_name
            FROM tag_attachments ta
            JOIN interest_tags it
                ON it.id = ta.tag_id
            WHERE ta.target_id = $1
        "#,
    )
    .bind(target_id)
    .fetch_all(tx.as_mut())
    .await
}

pub async fn tag(
    tx: &mut Transaction<'_, Postgres>,
    tag_name: String,
) -> RepositoryResult<Option<TagRow>> {
    let tag = sqlx::query_as::<_, TagRow>(
        r#"
            SELECT
                id::text as id,
                tag_name,
                popularity
            FROM interest_tags
            WHERE tag_name = $1
        "#,
    )
    .bind(tag_name)
    .fetch_optional(tx.as_mut())
    .await?;
    Ok(tag)
}
