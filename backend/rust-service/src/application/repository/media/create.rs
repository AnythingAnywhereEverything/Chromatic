use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult, media::row::{MediaHls, MediaHlsPlaylist, MediaObjectMetadataRow, MediaObjectsRow, MediaRow},
};


pub async fn media_create(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_row: &MediaRow,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO media (
            id,
            uploader_id,

            original_name,
            original_content_type,
            
            file_type,

            lock_hash,
            lock_expiration,

            processing_state,
            post_processing_state,

            flags,
            
            created_at,
            updated_at,
            deleted_at
        )
        VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10,
            $11, $12, $13
        )
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(media_row.id)
    .bind(media_row.uploader_id)
    .bind(&media_row.original_name)
    .bind(&media_row.original_content_type)
    .bind(&media_row.file_type)
    .bind(&media_row.lock_hash)
    .bind(media_row.lock_expiration)
    .bind(&media_row.processing_state)
    .bind(&media_row.post_processing_state)
    .bind(&media_row.flags)
    .bind(media_row.created_at)
    .bind(media_row.updated_at)
    .bind(media_row.deleted_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn media_object_create(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_object_row: &MediaObjectsRow,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO media_objects (
            media_id,
            kind,
            storage_key,
            content_type,

            size,
            name,
            thumbhash,

            created_at,
            updated_at,
            deleted_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(media_object_row.media_id)
    .bind(&media_object_row.kind)
    .bind(&media_object_row.storage_key)
    .bind(&media_object_row.content_type)
    .bind(media_object_row.size)
    .bind(&media_object_row.name)
    .bind(&media_object_row.thumbhash)
    .bind(media_object_row.created_at)
    .bind(media_object_row.updated_at)
    .bind(media_object_row.deleted_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn media_object_metadata_create(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_object_metadata_row: &MediaObjectMetadataRow,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO media_object_metadata (
            media_id,
            width,
            height,
            duration,

            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (media_id) DO NOTHING
        "#,
    )
    .bind(media_object_metadata_row.media_id)
    .bind(&media_object_metadata_row.width)
    .bind(&media_object_metadata_row.height)
    .bind(&media_object_metadata_row.duration)
    .bind(media_object_metadata_row.created_at)
    .bind(media_object_metadata_row.updated_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn media_hls_create(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_hls_row: &MediaHls,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO media_hls (
            media_id,
            master_playlist,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(media_hls_row.media_id)
    .bind(&media_hls_row.master_playlist)
    .bind(media_hls_row.created_at)
    .bind(media_hls_row.updated_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn media_hls_playlist_create(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_hls_playlist_row: &MediaHlsPlaylist,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO media_hls_playlists (
            media_id,
            resolution,
            playlist_storage_key,
            segment_count,
            segment_duration,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(media_hls_playlist_row.media_id)
    .bind(&media_hls_playlist_row.resolution)
    .bind(&media_hls_playlist_row.playlist_storage_key)
    .bind(media_hls_playlist_row.segment_count)
    .bind(media_hls_playlist_row.segment_duration)
    .bind(media_hls_playlist_row.created_at)
    .bind(media_hls_playlist_row.updated_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}
