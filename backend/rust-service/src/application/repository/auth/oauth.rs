use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, auth::row::UserOAuthRow};

pub async fn link_oauth_account(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    oauth_id: i64,
    user_id: i64,
    provider: &str,
    provider_user_id: &str,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO user_oauth (id, user_id, provider, provider_user_id)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(oauth_id)
    .bind(user_id)
    .bind(provider)
    .bind(provider_user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn find_by_provider_and_user_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    provider: &str,
    provider_user_id: &str,
) -> RepositoryResult<Option<UserOAuthRow>> {
    let row = sqlx::query_as::<_, UserOAuthRow>(
        r#"
        SELECT id, user_id, provider, provider_user_id
        FROM user_oauth
        WHERE provider = $1 AND provider_user_id = $2
        "#,
    )
    .bind(provider)
    .bind(provider_user_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row)
}
