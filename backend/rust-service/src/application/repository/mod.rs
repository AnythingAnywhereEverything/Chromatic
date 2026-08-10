pub mod user;
pub mod auth;
pub mod media;
pub mod post;
pub mod guild;

pub type RepositoryResult<T> = Result<T, sqlx::Error>;
