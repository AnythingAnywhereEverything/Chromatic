use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, auth::row::LoginUserRow};

pub async fn create_credential(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    password_hash: &str,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO user_credentials (user_id, password_hash, password_updated_at)
        VALUES ($1, $2, NOW())
        "#,
    )
    .bind(user_id)
    .bind(password_hash)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}


pub async fn find_login_user_by_email(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    email: &str,
) -> Result<LoginUserRow, sqlx::Error> {
    sqlx::query_as::<_, LoginUserRow>(
        r#"
        SELECT
            u.id AS user_id,
            u.email,
            u.username,
            u.is_active,
            u.email_verified_at,
            c.password_hash
        FROM users u
        LEFT JOIN user_credentials c ON c.user_id = u.id
        WHERE u.email = $1 AND u.deleted_at IS NULL
        LIMIT 1
        "# 
        // + User active check done within service layer for a redirect to reactive
    )
    .bind(email)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn find_login_user_by_username_or_email(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    identifier: &str,
) -> Result<LoginUserRow, sqlx::Error> {
    sqlx::query_as::<_, LoginUserRow>(
        r#"
        SELECT
            u.id,
            u.email,
            u.username,
            u.is_active,
            u.email_verified_at,
            c.password_hash
        FROM users u
        LEFT JOIN user_credentials c ON c.user_id = u.id
        WHERE (u.email = $1 OR u.username = $1) AND u.deleted_at IS NULL
        LIMIT 1
        "# 
        // + User active check done within service layer for a redirect to reactive
    )
    .bind(identifier)
    .fetch_one(tx.as_mut())
    .await
}