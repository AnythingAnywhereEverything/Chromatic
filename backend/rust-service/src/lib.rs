#![allow(clippy::all)]
#![warn(clippy::nursery)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::cognitive_complexity)]

pub mod api;
pub mod application;
pub mod constant;
pub mod domain;
pub mod infrastructure;
pub use multipart_derive::Multipart;