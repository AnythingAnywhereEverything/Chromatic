
pub async fn delete_message(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    message_id: i64,
    user_id: i64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE messages 
        SET deleted_at = NOW() 
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(message_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;
    Ok(result.rows_affected())
}
