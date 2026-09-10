

use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, media::row::MediaObjectsRow};

/// returning path
pub async fn hard_delete_media_data(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: i64,
) -> RepositoryResult<MediaObjectsRow> {
    let row = sqlx::query_as::<_, MediaObjectsRow>(
        r#"
        DELETE FROM media
        USING media_objects
        WHERE media.id = media_objects.media_id
        AND media.id = $1
        RETURNING media_objects.*
        "#,
    )
    .bind(media_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn soft_delete_media_data(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: i64,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE media
        SET deleted_at = now()
        WHERE id = $1
        "#,
    )
    .bind(media_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}