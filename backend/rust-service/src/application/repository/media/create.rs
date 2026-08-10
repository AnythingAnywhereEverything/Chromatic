use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    media::row::{MediaDataRow, MediaMetadataRow},
};

pub async fn media_data(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_data: &MediaDataRow,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO media_data (
            id, 
            uploader_id, 
            name, 
            path, 
            status, 
            thumbhash, 
            lock_hash, 
            created_at, 
            updated_at, 
            lock_expiration, 
            deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(media_data.id)
    .bind(media_data.uploader_id)
    .bind(&media_data.name)
    .bind(&media_data.path)
    .bind(&media_data.status)
    .bind(&media_data.thumbhash)
    .bind(&media_data.lock_hash)
    .bind(media_data.created_at)
    .bind(media_data.updated_at)
    .bind(media_data.lock_expiration)
    .bind(media_data.deleted_at)
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
