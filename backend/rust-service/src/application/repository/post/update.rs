use sqlx::{Postgres, Transaction};

use crate::application::repository::{
    RepositoryResult,
};

pub async fn content(
    tx: &mut Transaction<'_, Postgres>,
    post_id: i64,
    content: String,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE posts
        SET content = $1
        WHERE id = $2
        "#,
    )
    .bind(content)
    .bind(post_id)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}
