pub mod user;
pub mod auth;

pub type RepositoryResult<T> = Result<T, sqlx::Error>;
