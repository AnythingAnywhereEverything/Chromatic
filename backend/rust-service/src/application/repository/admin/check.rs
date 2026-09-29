use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

/// The admin gate: is this user a superuser?
///
/// Re-read from postgres on every admin request and never cached. Per-user
/// state behind a shared redis key is the failure mode that lets a stale
/// cache outlive a revocation, and the session layer already has that bug
/// (see `todo-admin.md` §6).
///
/// A missing or soft-deleted user is not a superuser, so this never returns
/// `true` for a row that `find_by_id` would refuse to show.
pub async fn is_superuser(tx: &mut Transaction<'_, sqlx::Postgres>, user_id: i64) -> RepositoryResult<bool> {
    let row: Option<(bool,)> = sqlx::query_as(
        r#"
        SELECT is_superuser FROM users
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(user_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row.map(|r| r.0).unwrap_or(false))
}

/// Can this user still authenticate? Active and not soft-deleted.
///
/// Distinct from `is_superuser`: the admin gate only needs the role, this is
/// the "account is usable" check.
pub async fn is_usable(tx: &mut Transaction<'_, sqlx::Postgres>, user_id: i64) -> RepositoryResult<bool> {
    let row: Option<(bool,)> = sqlx::query_as(
        r#"
        SELECT is_active FROM users
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(user_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row.map(|r| r.0).unwrap_or(false))
}

/// How many superusers exist. Used to refuse demoting the last one.
///
/// Counts soft-deleted rows out, so a deleted superuser never blocks a
/// legitimate demotion.
pub async fn count_superusers(tx: &mut Transaction<'_, sqlx::Postgres>) -> RepositoryResult<i64> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM users
        WHERE is_superuser = true AND deleted_at IS NULL
        "#
    )
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row.0)
}

/// Serialize superuser count-and-write against other demotions, then count.
///
/// This is the race-free way to guard the last-superuser rule. The service
/// calls it inside the same transaction as `set_is_superuser`, so two
/// concurrent demotions queue here and the second one observes the first one's
/// committed write instead of both reading a stale count of 2.
///
/// Uses a transaction-scoped advisory lock rather than `LOCK TABLE`:
///
/// * `LOCK TABLE ... IN SHARE ROW EXCLUSIVE MODE` is a utility statement, and
///   Postgres refuses to `PREPARE` those, so `sqlx::query` would fail at
///   runtime. Verified: `PREPARE p AS LOCK TABLE ...` is a syntax error.
/// * A table-level lock would also block every unrelated write to `users`
///   (registration, profile edits) for the duration. An advisory lock on one
///   named key blocks only concurrent admin demotions.
///
/// The key is `hashtext` of a fixed string instead of a raw integer so the
/// intent is readable and the same name is used everywhere. It is scoped to
/// this database and released automatically on commit or rollback.
pub async fn count_superusers_locked(tx: &mut Transaction<'_, sqlx::Postgres>) -> RepositoryResult<i64> {
    sqlx::query(
        r#"
        SELECT pg_advisory_xact_lock(hashtext('admin:superuser_demotion'))
        "#
    )
    .execute(tx.as_mut())
    .await?;

    count_superusers(tx).await
}