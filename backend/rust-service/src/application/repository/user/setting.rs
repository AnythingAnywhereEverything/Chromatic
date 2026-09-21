
use serde_json::json;

use crate::application::repository::user::row::{SettingsType, UserSettingRow};
// * I'll start on seperate function

pub async fn init_setting_notification(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
        let setting_value = json!({
        "post": {
            "following_person": 1,
            "likes": "friend",
            "tags": "everyone",
            "comments": "friend",
            "comment_likes": 1
        },
        "follow": {
            "follow_request": 1,
        },
        "message":{
            "new_message": 1,
            "message_requests": 1
        }
    });
    
    sqlx::query(
        r#"
        INSERT INTO user_settings (user_id, setting_key, setting_value,created_at, updated_at)
        VALUES ($1, 'notification', $2, NOW(), NOW())
        "#
    )
    .bind(user_id)
    .bind(setting_value)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

// ? i'm not sure if there anymore setting I missed
pub async fn init_setting_privacy(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let setting_value = json!({
        "profile_visibility": "everyone",
        "who_can_follow_me": "everyone",
    });

    sqlx::query(
        r#"
        INSERT INTO user_settings (user_id, setting_key, setting_value,created_at, updated_at)
        VALUES ($1, 'privacy', $2, NOW(), NOW())
        "#
    )
    .bind(user_id)
    .bind(setting_value)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn init_setting_message(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let setting_value = json!({
        "message_requests": "everyone",
        "add_to_message_group": "everyone"
    });

    sqlx::query(
        r#"
        INSERT INTO user_settings (user_id, setting_key, setting_value,created_at, updated_at)
        VALUES ($1, 'message', $2, NOW(), NOW())
        "#
    )
    .bind(user_id)
    .bind(setting_value)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn get_setting_type(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    setting_key: SettingsType,
) -> Result<UserSettingRow, sqlx::Error> {
    let row: UserSettingRow = sqlx::query_as(
        r#"
        SELECT setting_key, setting_value, created_at, updated_at
        FROM user_settings
        WHERE user_id = $1 AND setting_key = $2
        "#
    )
    .bind(user_id)
    .bind(setting_key.to_string())
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}