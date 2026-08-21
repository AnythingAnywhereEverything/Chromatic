use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
};

pub async fn user_display_name(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    new_display_name: &str,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE user_profiles
        SET display_name = $1,
            updated_at = now()
        WHERE user_id = $2
        "#,
    )
    .bind(new_display_name)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn user_bio(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    new_bio: &str,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE user_profiles
        SET bio = $1,
            updated_at = now()
        WHERE user_id = $2
        "#,
    )
    .bind(new_bio)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn banner_media_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    banner_media_id: Option<i64>,
) -> RepositoryResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, Option<i64>>(
        r#"
        UPDATE user_profiles
        SET banner_media_id = $1
        WHERE user_id = $2
        RETURNING banner_media_id
        "#,
    )
    .bind(banner_media_id)
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn avatar_media_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    avatar_media_id: Option<i64>,
) -> RepositoryResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, Option<i64>>(
        r#"
        UPDATE user_profiles
        SET avatar_media_id = $1
        WHERE user_id = $2
        RETURNING avatar_media_id
        "#,
    )
    .bind(avatar_media_id)
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}