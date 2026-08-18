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
    pub avatar: Option<String>, // hash name
    pub avatar_thumbhash: Option<String>, // thumbhash
    pub banner: Option<String>, // hash name
    pub banner_thumbhash: Option<String>, // thumbhash
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
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
    pub resolved_at: chrono::DateTime<chrono::Utc>
}