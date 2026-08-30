use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, media::row::{MediaStatus, PostProcessingState, ProcessingState}};

// pub enum MediaStatus {
//     // Prioritized Over All Other Statuses
//     Locked,
//     // Prioritized Over Pending
//     Processing,
//     // Prioritized Over Failed
//     Pending,
//     // Prioritized Over Pending but not over Processing
//     Ready,
//     // Prioritized Over All Other Statuses
//     Failed,
// }

pub async fn post_processing_state(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: &i64,
    state: &PostProcessingState,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE media
        SET post_processing_state = $1
        WHERE id = $2
        "#,
    )
    .bind(state)
    .bind(media_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn processing_state(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: &i64,
    state: &ProcessingState,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE media
        SET processing_state = $1
        WHERE id = $2
        "#,
    )
    .bind(state)
    .bind(media_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn media_status(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: &i64,
    status: &MediaStatus,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE media_data
        SET status = $1
        WHERE id = $2
          AND CASE status
              WHEN 'pending'::media_status THEN
                  $1::media_status IN (
                      'processing'::media_status,
                      'ready'::media_status
                  )

              WHEN 'processing'::media_status THEN
                  $1::media_status IN (
                      'ready'::media_status,
                      'completed'::media_status,
                      'failed'::media_status,
                      'locked'::media_status
                  )

              WHEN 'ready'::media_status THEN
                  $1::media_status IN (
                      'completed'::media_status,
                      'failed'::media_status,
                      'locked'::media_status
                  )

              WHEN 'completed'::media_status THEN
                  $1::media_status = 'completed'::media_status

              WHEN 'failed'::media_status THEN
                  $1::media_status = 'failed'::media_status

              WHEN 'locked'::media_status THEN
                  $1::media_status = 'locked'::media_status
              END
        "#,
    )
    .bind(status)
    .bind(media_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}
