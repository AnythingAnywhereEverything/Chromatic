use crate::application::{repository::RepositoryResult, service::media::model::container::RecentMediaType};
use sqlx::Transaction;

pub async fn conflict_recent_media(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    uploader_id: &i64,
    file_name: &str,
    media_type: RecentMediaType,
) -> RepositoryResult<Option<i64>> {
    let recent_table = match media_type {
        RecentMediaType::Avatar => "user_recent_avatar",
        RecentMediaType::Banner => "user_recent_banner",
    };

    let media_column = match media_type {
        RecentMediaType::Avatar => "avatar_id",
        RecentMediaType::Banner => "banner_id",
    };

    let query = format!(
        r#"
            SELECT media.id
            FROM media
            JOIN media_objects mo ON mo.media_id = media.id
            AND mo.name = $2
            AND mo.deleted_at IS NULL
            AND media.deleted_at IS NULL
            JOIN {recent_table} recent ON recent.{media_column} = media.id
            WHERE recent.user_id = $1
        "#
    );

    let exists: Option<i64> = sqlx::query_scalar(&query)
        .bind(uploader_id)
        .bind(file_name)
        .fetch_optional(tx.as_mut())
        .await?;

    Ok(exists)
}

// ! DEPRECATED: This function checks for media existence using the deprecated media_data table.
pub async fn media_exists(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: &i64,
) -> RepositoryResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM media_data
            WHERE id = $1
        )
        "#,
    )
    .bind(media_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(exists)
}
