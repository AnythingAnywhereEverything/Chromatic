use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: i64,
    pub email: String,
    pub username: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserProfileMinimalRow {
    pub id: i64,
    pub email: Option<String>,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>, // hash name
    pub avatar_thumbhash: Option<String>, // thumbhash
    pub banner: Option<String>, // hash name
    pub banner_thumbhash: Option<String>, // thumbhash
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

// strip sensitive data out for profile
// sensitive data should be in credential (e.g., password, email)
#[skip_serializing_none]
#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct UserProfileRow {
    pub id: String,
    // skip optional fields for minimal profile
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub quote: Option<String>,
    pub is_follower: Option<bool>,
    pub is_following: Option<bool>,
    pub followers_count: Option<i32>,
    pub following_count: Option<i32>,
    pub posts_count: Option<i32>,
    pub pinned_posts: Option<Vec<i64>>, // array of post ids
    pub avatar: Option<String>, // hash name
    pub avatar_thumbhash: Option<String>, // thumbhash
    pub banner: Option<String>, // hash name
    pub banner_thumbhash: Option<String>, // thumbhash
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// * target_type is enum for User / Guild

#[derive(Debug, sqlx::FromRow)]
pub struct ReportUserAndGuildRow {
    pub id: i64,
    pub reporter_id: i64,
    pub target_id: i64,
    pub target_type: String,
    pub report_category: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct FollowUserRow {
    pub user_id: String,
    pub follower_id: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct UserSettingRow {
    pub setting_key: String,
    pub setting_value: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct PendingFollowRow {
    pub follower_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,       // hash name
    pub avatar_thumbhash: Option<String>, // thumbhash
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::Type, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SettingsType {
    Account,
    Security,
    Privacy,
    Notification,
    Display,
    Message,
}

impl ToString for SettingsType {
    fn to_string(&self) -> String {
        match self {
            SettingsType::Account => "account".to_string(),
            SettingsType::Security => "security".to_string(),
            SettingsType::Privacy => "privacy".to_string(),
            SettingsType::Notification => "notification".to_string(),
            SettingsType::Display => "display".to_string(),
            SettingsType::Message => "message".to_string(),
        }
    }
}