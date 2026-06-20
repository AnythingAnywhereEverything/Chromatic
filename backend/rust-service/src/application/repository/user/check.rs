use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

pub async fn username_taken(tx: &mut Transaction<'_, sqlx::Postgres>, username: &str) -> RepositoryResult<bool> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM users WHERE username = $1
        AND deleted_at IS NULL
        "#
    )
    .bind(username)
    .fetch_one(tx.as_mut())
    .await?;
    Ok(count.0 > 0)
}

pub async fn email_taken(tx: &mut Transaction<'_, sqlx::Postgres>, email: &str) -> RepositoryResult<bool> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM users WHERE email = $1
        "#
    )
    .bind(email)
    .fetch_one(tx.as_mut())
    .await?;
    Ok(count.0 > 0)
}