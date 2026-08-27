use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult, media::row::{MediaDataRow, MediaMetadataRow},
};

/// Check if it have the same path and uploader_id and not deleted, if so, we will update the existing record instead of inserting a new one.
/// if so, we reuse the existing record and return the uploaded media id, otherwise we insert a new record and return the new media id.
pub async fn media_data_check_existing(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_data: &MediaDataRow,
) -> RepositoryResult<Option<i64>> {
    let existing_media_id: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT id FROM media_data
        WHERE path = $1 AND uploader_id = $2 AND deleted_at IS NULL
        "#,
    )
    .bind(&media_data.path)
    .bind(media_data.uploader_id)
    .fetch_optional(tx.as_mut())
    .await?;
    Ok(existing_media_id)
}


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
            flags,
            thumbhash, 
            lock_hash, 
            created_at, 
            updated_at, 
            lock_expiration,
            original_name,
            original_content_type,
            deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "#,
    )
    .bind(media_data.id)
    .bind(media_data.uploader_id)
    .bind(&media_data.name)
    .bind(&media_data.path)
    .bind(&media_data.status)
    .bind(&media_data.flags)
    .bind(&media_data.thumbhash)
    .bind(&media_data.lock_hash)
    .bind(media_data.created_at)
    .bind(media_data.updated_at)
    .bind(media_data.lock_expiration)
    .bind(&media_data.original_name)
    .bind(&media_data.original_content_type)
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
