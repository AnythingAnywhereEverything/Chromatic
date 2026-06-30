use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
};

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