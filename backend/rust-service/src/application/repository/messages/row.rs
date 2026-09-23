use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct MessageRow {
    pub id: String,
    pub user_id: String,
    pub target_id: String,
    pub content: String,
    pub has_attachment: bool,
    pub has_reactions: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[skip_serializing_none]
#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct MessageBaseRow{
    pub id: String,
    pub user_id: String,
    pub target_id: String,
    pub content: String,
    pub has_attachment: bool,
    pub has_reactions: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)] 
pub struct MessageReaction {
    pub message_id: String,
    pub user_id: String,
    pub reaction: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)] 
pub struct UserFollowRow{
    pub followed_id: i64,
}

#[derive(serde::Deserialize, serde::Serialize, sqlx::Type, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    User,
    Group,
}

impl ToString for TargetType {
    fn to_string(&self) -> String {
        match self {
            TargetType::User => "user".to_string(),
            TargetType::Group => "group".to_string(),
        }
    }
}