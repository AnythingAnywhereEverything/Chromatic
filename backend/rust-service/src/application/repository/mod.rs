pub mod user;
pub mod report;
pub mod admin;
pub mod auth;
pub mod media;
pub mod post;
pub mod guild;
pub mod messages;
pub mod tags;
pub mod staff_role;

pub type RepositoryResult<T> = Result<T, sqlx::Error>;
