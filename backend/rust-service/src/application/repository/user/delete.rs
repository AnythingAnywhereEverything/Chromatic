// Delete user

use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

pub async fn delete_user(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<bool> {
    sqlx::query(
        r#"
        DELETE FROM users WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    // The deletion is cascade will handle the removal of related data, so we just return true to indicate success.

    Ok(true)
}

pub async fn soft_delete_user(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<bool> {
    sqlx::query(
        r#"
        UPDATE users SET deleted_at = NOW() WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(true)
}

pub async fn user_banner_media_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, Option<i64>>(
        r#"
        SELECT banner_media_id FROM user_profiles WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn user_avatar_media_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, Option<i64>>(
        r#"
        SELECT avatar_media_id FROM user_profiles WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}