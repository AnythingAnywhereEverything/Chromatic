use sqlx::{Postgres, Transaction};

use crate::application::repository::RepositoryResult;

pub async fn post(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    user_id: i64,
) -> RepositoryResult<u64> {
    let delete = sqlx::query(
        r#"
            UPDATE media_posts
            SET status = 'inactive',
            deleted_at = now()
            WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(&mut **tx)
    .await?;

    Ok(delete.rows_affected())
}
