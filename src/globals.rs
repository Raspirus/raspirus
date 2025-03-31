use std::sync::{Arc, Mutex, OnceLock};

use crate::backend::{config::Config, log::LogLevel};

// global mutable values

/// Application startup time used for logging. Can be fetched via get_application_log
pub static APPLICATION_LOG: OnceLock<String> = OnceLock::new();
pub fn get_application_log() -> String {
    APPLICATION_LOG
        .get_or_init(|| chrono::Utc::now().format("%Y-%m-%dT:%H:%M:%SZ").to_string())
        .to_string()
}

/// Shared config file. Can be fetched via get_config
pub static CONFIG: OnceLock<Arc<Mutex<Config>>> = OnceLock::new();
pub fn get_config() -> Arc<Mutex<Config>> {
    CONFIG
        .get_or_init(|| Arc::new(Mutex::new(Config::default())))
        .clone()
}

// A bunch of default values
pub static DEFAULT_REMOTE_URL: &str =
    "https://api.github.com/repos/Raspirus/yara-rules/releases/latest";
pub static DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Debug;
pub static DEFAULT_LANGUAGE: &str = "en_US";
pub static CONFIG_VERSION: usize = 0;
pub static CONFIG_FILE_NAME: &str = "raspirus.cfg";

/// Default web request timeout
pub static TIMEOUT: u64 = 240;

pub const APP_ID: &str = "io.github.raspirus.raspirus";