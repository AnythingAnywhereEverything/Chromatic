use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    admin::row::{AdminUserRow, ListUsersOpts},
};

/// Look up one user.
///
/// Unlike the list queries this does *not* exclude soft-deleted rows, because
/// a moderator needs to be able to inspect a deleted account. The `deleted_at`
/// field on the row says which it was.
pub async fn find_by_id(tx: &mut Transaction<'_, sqlx::Postgres>, user_id: i64) -> RepositoryResult<Option<AdminUserRow>> {
    let row = sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT
            u.id::TEXT AS id,
            u.id AS id_raw,
            u.username,
            u.email,
            u.is_active,
            u.is_superuser,
            u.email_verified_at,
            u.created_at,
            u.updated_at,
            u.deleted_at,
            up.display_name,
            up.followers_count,
            up.following_count,
            up.posts_count,
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash
        FROM users u
        LEFT JOIN user_profiles up ON up.user_id = u.id
        LEFT JOIN media_objects m1
            ON up.avatar_media_id = m1.media_id AND m1.kind = 'original'
        WHERE u.id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row)
}

/// The index-backed half of the search: equality on the three unique columns.
///
/// Capped at 3 because that is the most it can return — one row per column.
/// Anything beyond that is not an exact match.
///
/// The gate is written so the whole block collapses to a no-op when no search
/// was requested: with both parameters null the first disjunct is true.
/// `u.id = COALESCE($2, -1)` is the idiom for "match the id only if one was
/// given", because `u.id = NULL` is never true and would need a second guard.
pub async fn find_exact(tx: &mut Transaction<'_, sqlx::Postgres>, opts: &ListUsersOpts) -> RepositoryResult<Vec<AdminUserRow>> {
    let rows = sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT
            u.id::TEXT AS id,
            u.id AS id_raw,
            u.username,
            u.email,
            u.is_active,
            u.is_superuser,
            u.email_verified_at,
            u.created_at,
            u.updated_at,
            u.deleted_at,
            up.display_name,
            up.followers_count,
            up.following_count,
            up.posts_count,
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash
        FROM users u
        LEFT JOIN user_profiles up ON up.user_id = u.id
        LEFT JOIN media_objects m1
            ON up.avatar_media_id = m1.media_id AND m1.kind = 'original'
        WHERE u.deleted_at IS NULL
        AND (($1::text IS NULL AND $2::bigint IS NULL)
            OR u.id = COALESCE($2::bigint, -1)
            OR u.email = $1::text
            OR u.username = $1::text)
        AND ($3::boolean IS NULL OR u.is_active = $3::boolean)
        AND ($4::boolean IS NULL OR u.is_superuser = $4::boolean)
        AND ($5::timestamptz IS NULL
            OR (u.created_at, u.id) < ($5::timestamptz, $6::bigint))
        ORDER BY u.created_at DESC, u.id DESC
        LIMIT 3
        "#
    )
    .bind(opts.query.as_deref())
    .bind(opts.query_id)
    .bind(opts.is_active)
    .bind(opts.is_superuser)
    .bind(opts.before)
    .bind(opts.before_id)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(rows)
}

/// The substring half of the search. Sequential scan by nature; see
/// `list_users` for why it cannot be merged into the same predicate as
/// `find_exact`.
///
/// `exclude` drops ids already returned by the exact step so the page does not
/// show the same user twice. An empty list matches nothing, and
/// `NOT (false)` is true, so no special case is needed for the common
/// "no exact match" path.
pub async fn list_substring(tx: &mut Transaction<'_, sqlx::Postgres>, opts: &ListUsersOpts, exclude: &[i64], limit: i64) -> RepositoryResult<Vec<AdminUserRow>> {
    let rows = sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT
            u.id::TEXT AS id,
            u.id AS id_raw,
            u.username,
            u.email,
            u.is_active,
            u.is_superuser,
            u.email_verified_at,
            u.created_at,
            u.updated_at,
            u.deleted_at,
            up.display_name,
            up.followers_count,
            up.following_count,
            up.posts_count,
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash
        FROM users u
        LEFT JOIN user_profiles up ON up.user_id = u.id
        LEFT JOIN media_objects m1
            ON up.avatar_media_id = m1.media_id AND m1.kind = 'original'
        WHERE u.deleted_at IS NULL
        AND ($1::text IS NULL
            OR u.username ILIKE '%' || $1::text || '%'
            OR u.email ILIKE '%' || $1::text || '%'
            OR up.display_name ILIKE '%' || $1::text || '%')
        AND ($2::boolean IS NULL OR u.is_active = $2::boolean)
        AND ($3::boolean IS NULL OR u.is_superuser = $3::boolean)
        AND ($4::timestamptz IS NULL
            OR (u.created_at, u.id) < ($4::timestamptz, $5::bigint))
        AND NOT (u.id = ANY($6::bigint[]))
        ORDER BY u.created_at DESC, u.id DESC
        LIMIT $7
        "#
    )
    .bind(opts.query.as_deref())
    .bind(opts.is_active)
    .bind(opts.is_superuser)
    .bind(opts.before)
    .bind(opts.before_id)
    .bind(exclude)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(rows)
}