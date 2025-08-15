/// Translations
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("locales");

/// All config related functionality
pub use crate::config::config::{Config, LogLevel};
/// The crate unified error
pub use crate::error::Error;

/// Everything cli argument related
pub mod cli;
mod config;
mod error;
/// Global values needed for both headless and GUI build
pub mod globals;
/// The Scanning module
pub mod scanner;
