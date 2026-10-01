use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

/// Append one `reports` row.
///
/// `id` is passed in because `reports.id` is `BIGINT NOT NULL PRIMARY KEY` with
/// no `DEFAULT` and no sequence, exactly like `audit_logs.id`. The caller
/// generates it from the snowflake generator.
///
/// `status` is written explicitly rather than left to the column default. The
/// default is `'pending'` and will stay that way, but a report's whole purpose
/// is to sit in a moderation queue, and having the state visible in the `INSERT`
/// beats having to go read the schema to learn what a fresh row means.
///
/// `created_at` and `updated_at` *are* left to their defaults, deliberately.
/// `updated_at` is the only timestamp this table has for "when was this report
/// last acted on" — nothing else in the schema records it — so it has to be
/// bumped by the later resolution work and never by the insert. Writing `NOW()`
/// here would be a no-op today, and a trap for anyone who later assumes an
/// insert moved it.
///
/// `report_data` is `JSONB NOT NULL`, so the caller must always pass a JSON
/// value. There is no "no details given" representation at the column level; an
/// absent optional field is `null` *inside* the object.
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