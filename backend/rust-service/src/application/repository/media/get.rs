use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, media::row::MediaFullDataRow};
use sqlx::types::Json;

pub async fn media_full_data(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: &i64,
) -> RepositoryResult<MediaFullDataRow> {
    tracing::debug!("Fetching media full data for media_id: {}", media_id);

    let row = sqlx::query_scalar::<_, Json<MediaFullDataRow>>(
        r#"
        SELECT jsonb_build_object(
            'id', id::text,
            'uploader_id', uploader_id::text,
            'original_name', original_name,
            'original_content_type', original_content_type,
            'file_type', file_type,
            'lock_hash', lock_hash,
            'lock_expiration', lock_expiration,
            'processing_state', processing_state,
            'post_processing_state', post_processing_state,
            'flags', flags::text,
            'media_objects', media_objects,
            'media_object_metadata', media_object_metadata,
            'media_hls', media_hls,
            'media_hls_playlists', media_hls_playlists,
            'created_at', created_at,
            'updated_at', updated_at,
            'deleted_at', deleted_at
        )
        FROM get_media_by_id_without_playlists($1);
        "#,
    )
    .bind(media_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row.0)
}