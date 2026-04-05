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
    pub avatar_url: Option<String>,
}