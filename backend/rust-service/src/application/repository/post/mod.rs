pub mod row;
pub mod post;
pub mod comment;
pub mod find;

pub type RepositoryResult<T> = Result<T, sqlx::Error>;
