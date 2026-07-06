use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, media::row::{MediaDataWithMetadataRow}};

pub async fn media_full_data(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: &i64,
) -> RepositoryResult<MediaDataWithMetadataRow> {

    tracing::debug!("Fetching media full data for media_id: {}", media_id);

    let row = sqlx::query_as::<_, MediaDataWithMetadataRow>(
        r#"
        SELECT
            md.id,
            md.media_url,
            md.media_preview_url,
            md.media_category,
            md.media_status,
            md.created_at,
            mm.file_size,
            mm.mime_type,
            mm.width,
            mm.height,
            mm.duration
        FROM media_data md
        LEFT JOIN media_metadata mm ON mm.media_id = md.id
        WHERE md.id = $1
        LIMIT 1
        "#,
    )
    .bind(media_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}