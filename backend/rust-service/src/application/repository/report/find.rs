use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, report::row::ReportTarget};

pub const CONTENT_EXCERPT_CHARS: usize = 280;
pub async fn exists_open_locked(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    reporter_id: i64,
    target_type: &str,
    target_id: i64,
) -> RepositoryResult<bool> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1))")
        .bind(format!(
            "report:{reporter_id}:{target_type}:{target_id}"
        ))
        .execute(tx.as_mut())
        .await?;

    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM reports
            WHERE reporter_id = $1
              AND reported_target_id = $2
              AND reported_target_type = $3
              AND status = 'pending'
        )
        "#,
    )
    .bind(reporter_id)
    .bind(target_id)
    .bind(target_type)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(exists)
}

pub async fn load_target(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    target_type: &str,
    target_id: i64,
) -> RepositoryResult<Option<ReportTarget>> {
    match target_type {
        "user" => {
            let row = sqlx::query_as::<_, (String, i64)>(
                r#"
                SELECT u.username, u.id
                FROM users u
                WHERE u.id = $1
                  AND u.deleted_at IS NULL
                "#,
            )
            .bind(target_id)
            .fetch_optional(tx.as_mut())
            .await?;

            Ok(row.map(|(username, id)| ReportTarget {
                author_id: Some(id),
                snapshot_username: Some(username),
                snapshot_content: None,
            }))
        }

        "post" | "comment" => {
            let table = if target_type == "post" {
                "media_posts"
            } else {
                "media_comments"
            };

            let row = sqlx::query_as::<_, (String, i64)>(&format!(
                r#"
                SELECT content, user_id
                FROM {table}
                WHERE id = $1
                  AND deleted_at IS NULL
                "#
            ))
            .bind(target_id)
            .fetch_optional(tx.as_mut())
            .await?;

            Ok(row.map(|(content, author_id)| ReportTarget {
                author_id: Some(author_id),
                snapshot_username: None,
                snapshot_content: Some(truncate_excerpt(&content)),
            }))
        }


        _ => Err(sqlx::Error::Protocol(
            "unknown report target type".to_string(),
        )),
    }
}

fn truncate_excerpt(content: &str) -> String {
    if content.chars().count() <= CONTENT_EXCERPT_CHARS {
        return content.to_string();
    }

    let mut excerpt: String = content.chars().take(CONTENT_EXCERPT_CHARS).collect();
    excerpt.push('…');
    excerpt
}