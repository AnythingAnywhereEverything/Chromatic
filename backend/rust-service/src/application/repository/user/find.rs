use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    user::row::{UserProfileFullRow, UserProfileMinimalRow, UserRow},
};

pub async fn profile_full_by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<UserProfileFullRow> {
    let row = sqlx::query_as::<_, UserProfileFullRow>(
        r#"
        SELECT 
            u.id,
            u.email,
            u.username,
            up.followers_count,
            up.following_count,
            up.display_name,
            up.bio,
            uf1.is_following,
            uf2.is_follower,
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash,
            m2.name AS banner,
            m2.thumbhash AS banner_thumbhash,
            u.created_at
        FROM users u
        LEFT JOIN user_profiles up
            ON u.id = up.user_id
        LEFT JOIN media_data m1
            ON up.avatar_media_id = m1.id
        LEFT JOIN media_data m2
            ON up.banner_media_id = m2.id
        LEFT JOIN (
            SELECT follower_id, user_id, true AS is_following
            FROM user_follow
            WHERE status = 'accepted'
        ) uf1
            ON u.id = uf1.user_id
            AND uf1.follower_id = $2
        LEFT JOIN (
            SELECT user_id, follower_id, true AS is_follower
            FROM user_follow
            WHERE status = 'accepted'
        ) uf2
            ON u.id = uf2.follower_id
            AND uf2.user_id = $2
        WHERE u.id = $1;
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn profile_full_by_username(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    username: &str,
    fetch_user_id: Option<i64>,
) -> RepositoryResult<UserProfileFullRow> {
    let row = sqlx::query_as::<_, UserProfileFullRow>(
        r#"
        SELECT
            u.id,
            u.email,
            u.username,
            up.followers_count,
            up.following_count,
            up.display_name,
            up.bio,
            uf1.is_following,
            uf2.is_follower,
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash,
            m2.name AS banner,
            m2.thumbhash AS banner_thumbhash,
            u.created_at
        FROM users u
        LEFT JOIN user_profiles up
            ON u.id = up.user_id
        LEFT JOIN media_data m1
            ON up.avatar_media_id = m1.id
        LEFT JOIN media_data m2
            ON up.banner_media_id = m2.id
        LEFT JOIN (
            SELECT follower_id, user_id, true AS is_following
            FROM user_follow
            WHERE status = 'accepted'
        ) uf1
            ON u.id = uf1.user_id
            AND uf1.follower_id = $2
        LEFT JOIN (
            SELECT user_id, follower_id, true AS is_follower
            FROM user_follow
            WHERE status = 'accepted'
        ) uf2
            ON u.id = uf2.follower_id
            AND uf2.user_id = $2
        WHERE u.username = $1;
        "#,
    )
    .bind(username)
    .bind(fetch_user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn profile_with_minimal_by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<UserProfileMinimalRow> {
    let row = sqlx::query_as::<_, UserProfileMinimalRow>(
        r#"
        SELECT 
            u.id, 
            u.email, 
            u.username, 
            up.display_name, 
            up.bio, 
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash,
            m2.name AS banner,
            m2.thumbhash AS banner_thumbhash,
            u.created_at
        FROM users u
        LEFT JOIN user_profiles up ON u.id = up.user_id
        LEFT JOIN media_data m1 ON up.avatar_media_id = m1.id
        LEFT JOIN media_data m2 ON up.banner_media_id = m2.id
        WHERE u.id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username FROM users WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn by_email(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    email: &str,
) -> RepositoryResult<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username FROM users WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn by_username(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    username: &str,
) -> RepositoryResult<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username FROM users WHERE username = $1
        "#,
    )
    .bind(username)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

/// ! lower performance than by_email or by_username, use it only when you want to support login with both email and username
pub async fn by_username_or_email(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    identifier: &str,
) -> RepositoryResult<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username FROM users WHERE username = $1 OR email = $1
        "#,
    )
    .bind(identifier)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}
