pub mod errors;

pub mod types;
pub mod entity;
pub mod profile;

pub use entity::User;

// ! ERRORS
pub use errors::EmailError;
pub use errors::PhoneNumberError;
pub use errors::UsernameError;