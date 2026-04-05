#[derive(Debug, sqlx::FromRow)]
pub struct UserOAuthRow {
    pub id: i64,
    pub user_id: i64,
    pub provider: String,
    pub provider_user_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct LoginUserRow {
    pub id: i64,
    pub email: String,
    pub username: Option<String>,
    pub email_verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
    pub password_hash: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SessionRow {
    pub id: i64,
    pub user_id: i64,
    pub session_token: String,
    pub user_agent: String,
    pub ip_address: String,
    pub created_at: chrono::NaiveDateTime,
}