mod error;
mod routes;
mod version;
mod extractors;

mod errors; // * Group error mappings for domain services, e.g. SessionServiceError -> APIError

// pub mod middleware; -- will be added when we have some middlewares, e.g. auth middleware
pub mod dtos;
pub mod handlers;
pub mod server;
pub use error::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind};
pub use extractors::{AdminUser, AuthUser, RequestAuth};
pub use version::APIVersion;
