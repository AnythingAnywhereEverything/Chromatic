use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

pub async fn insert(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: i64,
    reporter_id: i64,
    target_id: i64,
    target_type: &str,
    report_type: &str,
    report_data: serde_json::Value,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO reports (
            id,
            reporter_id,
            reported_target_id,
            reported_target_type,
            report_type,
            report_data,
            status
        )
        VALUES ($1, $2, $3, $4, $5, $6, 'pending')
        "#,
    )
    .bind(id)
    .bind(reporter_id)
    .bind(target_id)
    .bind(target_type)
    .bind(report_type)
    .bind(report_data)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}