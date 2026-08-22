

use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

/// returning path
pub async fn hard_delete_media_data(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: i64,
) -> RepositoryResult<String> {
    let row= sqlx::query_scalar(
        r#"
        DELETE FROM media_data
        WHERE id = $1
        RETURNING path
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
        UPDATE media_data
        SET deleted_at = now()
        WHERE id = $1
        "#,
    )
    .bind(media_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}