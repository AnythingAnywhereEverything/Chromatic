pub async fn create_notification(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: i64,
    target_id: i64,
    notification_type: &str,
    notification_data: serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO user_notifications (
            id,
            user_id,
            notification_type,
            notification_data,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(target_id)
    .bind(notification_type)
    .bind(notification_data)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn mark_notification_read(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: i64,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE user_notifications
        SET
            is_read = TRUE,
            updated_at = NOW()
        WHERE id = $1
          AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}
