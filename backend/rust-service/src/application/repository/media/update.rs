use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, media::row::MediaStatus};

pub async fn media_status(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: &i64,
    status: &MediaStatus,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE media_data
        SET media_status = $1
        WHERE id = $2
        "#,
    )
    .bind(status)
    .bind(media_id)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}