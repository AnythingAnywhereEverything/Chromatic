#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: i64,
    pub email: String,
    pub username: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserProfileFullRow {
    pub id: i64,
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_media_id: Option<i64>,
    pub banner_media_id: Option<i64>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}