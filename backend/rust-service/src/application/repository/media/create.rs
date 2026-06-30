use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, media::row::{MediaDataRow, MediaMetadataRow}};

pub async fn media_data(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_data: &MediaDataRow,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO media_data (id, user_id, media_url, media_preview_url, media_category, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(media_data.id)
    .bind(&media_data.user_id)
    .bind(&media_data.media_url)
    .bind(&media_data.media_preview_url)
    .bind(&media_data.media_category)
    .bind(media_data.created_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn media_metadata(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_metadata: &MediaMetadataRow,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO media_metadata (media_id, file_size, mime_type, width, height, duration)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(media_metadata.media_id)
    .bind(media_metadata.file_size)
    .bind(&media_metadata.mime_type)
    .bind(media_metadata.width)
    .bind(media_metadata.height)
    .bind(media_metadata.duration)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}
