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

/// Load the reported thing, or `None` if it is gone.
///
/// `target_type` picks the table, which is the whole reason this is a `match`
/// rather than a query: `reported_target_id` is a bare `BIGINT` with no foreign
/// key, so nothing in the schema can check that it names a row that exists. A
/// user id and a post id are both `BIGINT` and would be indistinguishable if the
/// type were not carried alongside.
///
/// Every branch excludes soft-deleted rows, so `None` means "not reportable":
/// either the id never existed or the thing has already been taken down. This
/// is deliberately stricter than `get::list_reports`, which *does* return
/// soft-deleted targets — a report filed before moderation must stay readable
/// afterwards, but filing a *new* one against a deleted post must not work.
///
/// The existing loaders are not reused here. `post::find::get_post_by_id`
/// calls a SQL function with `fetch_one`, so a miss surfaces as
/// `sqlx::Error::RowNotFound` rather than `None`, and the comment loaders
/// LEFT JOIN the author without rejecting a soft-deleted one. Both would need
/// unwrapping and re-checking, which is more work than the three queries below.
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
            // One query for both types: the two tables carry the same two
            // columns this needs, so a `match` on the table name would only
            // duplicate the truncate.
            let table = if target_type == "post" {
                "media_posts"
            } else {
                "media_comments"
            };

            // The table name is interpolated, not bound, because it cannot be a
            // parameter in SQL. This is not an injection point: both branches are
            // literals from the match above, never anything derived from the
            // request — `target_type` has already been matched to one of them.
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

        // An unknown type is the caller's bug, not user input to be tolerated.
        // The service validates the type before calling, so reaching this arm
        // means the two layers disagree; returning `None` would report the
        // target as missing, which is the wrong error.
        _ => Err(sqlx::Error::Protocol(
            "unknown report target type".to_string(),
        )),
    }
}

/// Trim a content snapshot to `CONTENT_EXCERPT_CHARS`.
///
/// Counts and slices on a char boundary rather than a byte one: `content` is
/// `VARCHAR`, so Postgres counts characters, and slicing bytes could split a
/// multi-byte code point and leave invalid UTF-8 in the JSONB.
fn truncate_excerpt(content: &str) -> String {
    if content.chars().count() <= CONTENT_EXCERPT_CHARS {
        return content.to_string();
    }

    let mut excerpt: String = content.chars().take(CONTENT_EXCERPT_CHARS).collect();
    excerpt.push('…');
    excerpt
}