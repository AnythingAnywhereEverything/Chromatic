use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    report::row::{ListReportsOpts, ReportPage, ReportRow},
};

/// One page of reports, newest first.
///
/// Keyset paginated on `(created_at, id)`, and `has_more` is derived by
/// fetching one row past the limit rather than counting — the same trade
/// `AdminUserPage` makes.
///
/// The `ORDER BY` name is table-qualified, and that is load-bearing. Because the
/// select list aliases `r.id::TEXT AS id`, a bare `ORDER BY id` binds to the
/// *output alias* and sorts lexically — verified against ids 9, 10 and 100,
/// where the bare form returns `9, 100, 10`. Only `r.id` is the real `bigint`.
///
/// Ties on `created_at` break on `id`, which is monotonic snowflake, so this is a
/// total order and the page boundary is stable.
///
/// The three target `LEFT JOIN`s are each guarded by `reported_target_type`
/// rather than switched on in Rust, because a user id and a post id are both
/// `BIGINT`: unguarded, every report would join against all three tables and
/// match whichever row happened to share its id. The guard is in the `ON`
/// clause rather than the `WHERE`, so a report whose target is gone still comes
/// back, with its target columns null.
pub async fn list_reports(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    opts: &ListReportsOpts,
) -> RepositoryResult<ReportPage> {
    // Clamped for the reason `admin::get::list_users` clamps: a negative
    // `LIMIT` is an error in postgres rather than a no-op, and a zero limit
    // would make `has_more` meaningless.
    let limit = opts.limit.clamp(1, 200);

    let mut rows = sqlx::query_as::<_, ReportRow>(
        r#"
        SELECT
            r.id::TEXT AS id,
            r.id AS id_raw,
            r.reported_target_type,
            r.reported_target_id::TEXT AS reported_target_id,
            r.reported_target_id AS reported_target_id_raw,
            r.report_type,
            r.report_data ->> 'description' AS description,
            r.report_data,
            r.status,
            r.created_at,
            r.updated_at,
            r.reporter_id::TEXT AS reporter_id,
            reporter.username AS reporter_username,
            reporter_profile.display_name AS reporter_display_name,
            target_user.username AS target_username,
            target_profile.display_name AS target_display_name,
            target_user.deleted_at AS target_deleted_at,
            COALESCE(target_post.content, target_comment.content) AS target_content,
            COALESCE(target_post.user_id, target_comment.user_id)::TEXT AS target_author_id
        FROM reports r
        LEFT JOIN users reporter
            ON reporter.id = r.reporter_id
        LEFT JOIN user_profiles reporter_profile
            ON reporter_profile.user_id = reporter.id
        LEFT JOIN users target_user
            ON r.reported_target_type = 'user'
            AND target_user.id = r.reported_target_id
        LEFT JOIN user_profiles target_profile
            ON target_profile.user_id = target_user.id
        LEFT JOIN media_posts target_post
            ON r.reported_target_type = 'post'
            AND target_post.id = r.reported_target_id
        LEFT JOIN media_comments target_comment
            ON r.reported_target_type = 'comment'
            AND target_comment.id = r.reported_target_id
        WHERE ($1::text IS NULL OR r.status = $1::text)
          AND ($2::text IS NULL OR r.reported_target_type = $2::text)
          AND ($3::bigint IS NULL OR r.reported_target_id = $3::bigint)
          AND ($4::timestamptz IS NULL
              OR (r.created_at, r.id) < ($4::timestamptz, $5::bigint))
        ORDER BY r.created_at DESC, r.id DESC
        LIMIT $6
        "#,
    )
    .bind(opts.status.as_deref())
    .bind(opts.target_type.as_deref())
    .bind(opts.target_id)
    .bind(opts.before)
    .bind(opts.before_id)
    // One row past the limit, which is what `has_more` is derived from.
    .bind(limit + 1)
    .fetch_all(tx.as_mut())
    .await?;

    let has_more = rows.len() as i64 > limit;
    rows.truncate(limit as usize);

    Ok(ReportPage { rows, has_more })
}