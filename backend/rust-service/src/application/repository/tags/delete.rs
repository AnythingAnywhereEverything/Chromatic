use sqlx::{Postgres, Transaction};

pub async fn delete_tag_attachment(
    tx: &mut Transaction<'_, Postgres>,
    target_id: i64,
    tag_id: i64,
) -> Result<u64, sqlx::Error> {
    let row = sqlx::query(
        r#"
            DELETE FROM tag_attachments
            WHERE target_id = $1
              AND tag_id = $2
        "#,
    )
    .bind(target_id)
    .bind(tag_id)
    .execute(tx.as_mut())
    .await?;

    Ok(row.rows_affected())
}

pub async fn tag(tx: &mut Transaction<'_, Postgres>, tag_id: i64) -> Result<u64, sqlx::Error> {
    sqlx::query(
        r#"
            UPDATE interest_tags
            SET popularity = popularity - 1
            WHERE id = $1
        "#,
    )
    .bind(tag_id)
    .execute(tx.as_mut())
    .await?;

    let row = sqlx::query(
        r#"
            DELETE FROM interest_tags
            WHERE id = $1
              AND popularity < 1
        "#,
    )
    .bind(tag_id)
    .execute(tx.as_mut())
    .await?;

    Ok(row.rows_affected())
}
