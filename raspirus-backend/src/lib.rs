/// All config related functionality
pub use crate::config::config::{Config, LogLevel};
/// The crate unified error
pub use crate::error::Error;

/// Arguments
pub mod arguments;
mod config;
mod error;
pub mod globals;
pub mod scanner;
