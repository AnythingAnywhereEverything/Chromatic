pub mod user;
pub mod auth;
pub mod media;

pub type RepositoryResult<T> = Result<T, sqlx::Error>;
