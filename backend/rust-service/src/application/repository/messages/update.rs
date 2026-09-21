use crate::application::repository::messages::row::MessageRow;  
pub async fn update_message(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    message_id: i64,
    content: String,
    user_id: i64,
) -> Result<MessageRow, sqlx::Error> {
    sqlx::query_as::<_, MessageRow>(
        r#"
        UPDATE messages 
        SET content = $1 
        WHERE id = $2
        AND user_id = $3
        RETURNING *
        "#)
        .bind(content)
        .bind(message_id)
        .bind(user_id)
        .fetch_one(tx.as_mut())
        .await
}
