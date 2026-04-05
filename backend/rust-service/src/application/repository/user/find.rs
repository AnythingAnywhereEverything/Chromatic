use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    user::row::{UserProfileFullRow, UserRow},
};

pub async fn profile_full_by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<UserProfileFullRow> {
    let row = sqlx::query_as::<_, UserProfileFullRow>(
        r#"
        SELECT 
            u.id, u.email, u.username, up.display_name, up.bio, up.avatar_url
        FROM users u
        LEFT JOIN user_profiles up ON u.id = up.user_id
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
