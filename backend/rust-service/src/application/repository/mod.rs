pub mod user;
pub mod auth;
pub mod media;
pub mod post;
pub mod guild;
pub mod messages;
pub mod tags;

pub type RepositoryResult<T> = Result<T, sqlx::Error>;
