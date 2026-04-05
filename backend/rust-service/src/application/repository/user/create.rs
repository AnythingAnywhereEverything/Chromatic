use sqlx::Transaction;

use crate::{application::repository::RepositoryResult, domain::user::{User, types::DisplayName}};

pub async fn user(tx: &mut Transaction<'_, sqlx::Postgres>, user: &User) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO users (id, email, username)
        VALUES ($1, $2, $3)
        "#
    )
    .bind(user.id)
    .bind(user.email.as_str())
    .bind(user.username.as_ref().map(|u| u.as_str()))
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn user_profile(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    display_name: Option<DisplayName>,
    bio: Option<&str>,
    avatar_url: Option<&str>,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        INSERT INTO user_profiles (user_id, display_name, bio, avatar_url)
        VALUES ($1, $2, $3, $4)
        "#
    )
    .bind(user_id)
    .bind(display_name.as_ref().map(|d| d.as_str()))
    .bind(bio)
    .bind(avatar_url)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}