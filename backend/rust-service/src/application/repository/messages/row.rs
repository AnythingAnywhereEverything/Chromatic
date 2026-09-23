use serde::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::application::repository::{media::row::Attachment, user::row::UserProfileRow};


#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct MessageRow {
    pub id: String,
    pub profile: Json<UserProfileRow>,
    pub target_id: String,
    pub content: String,
    pub has_attachment: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct MessageBaseRow{
    pub id: String,
    pub user_id: i64,
    pub target_id: String,
    pub target_type: String,
    pub content: String,
    pub has_attachment: bool,
    pub has_reactions: bool,
    pub attachment: Json<Vec<Attachment>>,
    pub reactions: Json<Vec<MessageReaction>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)] 
pub struct MessageReaction {
    pub message_id: String,
    pub user_id: String,
    pub reaction: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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