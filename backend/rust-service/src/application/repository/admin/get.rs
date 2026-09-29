use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    admin::{
        find::{find_exact, list_substring},
        row::{AdminAuditRow, AdminStatsRow, AdminUserPage, ListUsersOpts},
    },
};

/// One page of users, exact matches first.
///
/// The search is two steps because it has to be: the substring branch
/// (`ILIKE '%q%'`) carries a leading wildcard, so no index can serve it, and a
/// single leading-wildcard branch in an `OR` forces the whole predicate into a
/// sequential scan. Exact match therefore has to be its own query (`find_exact`)
/// against the indexed unique columns, and its rows come out ahead.
///
/// Exact rows top up with substring results to fill the limit. There is no
/// hiding: an exact username or email match is also a substring match, so the
/// fast path only truly adds id matches. With no search text, `find_exact`
/// seeds the newest rows and the substring step fills the page — an unfiltered
/// list is the full list, not the exact step's three-row cap.
///
/// Soft-deleted users are excluded throughout: they are not part of the
/// moderation queue.
pub async fn list_users(tx: &mut Transaction<'_, sqlx::Postgres>, opts: &ListUsersOpts) -> RepositoryResult<AdminUserPage> {
    // Clamped because a negative `LIMIT` is an error in postgres, not a
    // no-op, and a zero limit would make `has_more` meaningless. The caller is
    // expected to clamp too; this is the backstop that keeps a bad query
    // parameter from becoming a 500.
    let limit = opts.limit.clamp(1, 200);

    let exact = find_exact(tx, opts).await?;

    if exact.len() as i64 >= limit {
        // Exact matches alone fill the page. One extra substring row tells us
        // whether anything follows, since the exact step is capped at 3.
        let exclude: Vec<i64> = exact.iter().map(|r| r.id_raw).collect();
        let has_more = exact.len() as i64 > limit
            || list_substring(tx, opts, &exclude, 1).await?.len() > 0;

        let mut rows = exact;
        rows.truncate(limit as usize);

        return Ok(AdminUserPage { rows, has_more });
    }

    // Top up with substring matches. Skipped only for a pure id lookup that
    // missed: there is no free text to match, so a scan would return everyone
    // instead of a meaningful empty page. `query_id` cannot be set without
    // `query` through the service (`AdminService::list_users` derives it from
    // the free text), so when the list is merely unfiltered — no `q` at all —
    // this always runs. Without it, `find_exact`'s `LIMIT 3` would cap an
    // unsearched page at three rows, hiding all but the newest accounts.
    let mut rows = exact;
    if opts.query.is_some() || opts.query_id.is_none() {
        let exclude: Vec<i64> = rows.iter().map(|r| r.id_raw).collect();
        let filler = list_substring(tx, opts, &exclude, limit + 1 - rows.len() as i64).await?;
        rows.extend(filler);
    }

    // Re-sort because the two steps are ordered independently. Must use
    // `id_raw`: the string `id` sorts lexically, so "9" would beat "100".
    rows.sort_by(|a, b| {
        b.created_at.cmp(&a.created_at).then_with(|| b.id_raw.cmp(&a.id_raw))
    });

    let has_more = rows.len() as i64 > limit;
    rows.truncate(limit as usize);

    Ok(AdminUserPage { rows, has_more })
}

/// Dashboard counts, in one round trip.
///
/// `COUNT(*) FILTER (WHERE ...)` is the same `i64` the tuple form in
/// `user/check.rs` returns.
///
/// Note that `deleted_users` is *not* a fourth state, it overlaps the others:
/// soft deletion sets `deleted_at` and leaves `is_active` alone, so a deleted
/// account is counted as active *and* as deleted. That makes
/// `active_users + suspended_users = total_users` an invariant, and
/// `total_users - deleted_users` the live-user count. Do not subtract
/// `deleted_users` out of `active_users`.
pub async fn stats(tx: &mut Transaction<'_, sqlx::Postgres>) -> RepositoryResult<AdminStatsRow> {
    let row = sqlx::query_as::<_, AdminStatsRow>(
        r#"
        SELECT
            COUNT(*) AS total_users,
            COUNT(*) FILTER (WHERE is_active) AS active_users,
            COUNT(*) FILTER (WHERE NOT is_active) AS suspended_users,
            COUNT(*) FILTER (WHERE is_superuser) AS superusers,
            COUNT(*) FILTER (WHERE deleted_at IS NOT NULL) AS deleted_users
        FROM users
        "#
    )
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

/// Most recent audit entries across every target, newest first.
///
/// Ties on `created_at` are broken by `id` so the page boundary is stable;
/// snowflake ids are monotonic, so this is a total order in practice.
///
/// The `ORDER BY` name is table-qualified, and that is load-bearing. Because
/// the select list aliases `id::TEXT AS id`, a bare `ORDER BY id` resolves to
/// the *output alias* and sorts lexically — verified against ids 9, 10 and 100,
/// where the bare form returns `9, 100, 10`. Only `audit_logs.id` refers to
/// the real `bigint` and orders correctly.
pub async fn list_recent(tx: &mut Transaction<'_, sqlx::Postgres>, limit: i64) -> RepositoryResult<Vec<AdminAuditRow>> {
    let rows = sqlx::query_as::<_, AdminAuditRow>(
        r#"
        SELECT
            audit_logs.id::TEXT AS id,
            audit_logs.target_id::TEXT AS target_id,
            audit_logs.target_type,
            audit_logs.action,
            audit_logs.performed_by::TEXT AS performed_by,
            audit_logs.action_data,
            audit_logs.created_at,
            target.username AS target_username,
            actor.username AS performed_by_username
        FROM audit_logs
        LEFT JOIN users target ON target.id = audit_logs.target_id
        LEFT JOIN users actor ON actor.id = audit_logs.performed_by
        ORDER BY audit_logs.created_at DESC, audit_logs.id DESC
        LIMIT $1
        "#
    )
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(rows)
}

/// Audit history for one target, newest first.
///
/// This is the per-user view. `target_type` is not filtered because ids are
/// only unique within a target type, so the caller must pass an id that is
/// unambiguous in context; filtering on `target_type` too would be more
/// defensive but the admin routes only ever ask about `user`.
pub async fn list_for_target(tx: &mut Transaction<'_, sqlx::Postgres>, target_id: i64, limit: i64) -> RepositoryResult<Vec<AdminAuditRow>> {
    let rows = sqlx::query_as::<_, AdminAuditRow>(
        r#"
        SELECT
            audit_logs.id::TEXT AS id,
            audit_logs.target_id::TEXT AS target_id,
            audit_logs.target_type,
            audit_logs.action,
            audit_logs.performed_by::TEXT AS performed_by,
            audit_logs.action_data,
            audit_logs.created_at,
            target.username AS target_username,
            actor.username AS performed_by_username
        FROM audit_logs
        LEFT JOIN users target ON target.id = audit_logs.target_id
        LEFT JOIN users actor ON actor.id = audit_logs.performed_by
        WHERE audit_logs.target_id = $1
        ORDER BY audit_logs.created_at DESC, audit_logs.id DESC
        LIMIT $2
        "#
    )
    .bind(target_id)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(rows)
}