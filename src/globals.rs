use std::sync::{Arc, Mutex, OnceLock};

use crate::{
    backend::{config::Config, log::LogLevel},
    Error,
};

// global mutable values

/// Application startup time used for logging. Can be fetched via get_application_log
static APPLICATION_LOG: OnceLock<String> = OnceLock::new();
pub fn get_application_log() -> String {
    APPLICATION_LOG
        .get_or_init(|| chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string())
        .to_string()
}

/// Shared config file. Can be fetched via get_config
static CONFIG: OnceLock<Arc<Mutex<Config>>> = OnceLock::new();
/// Fetch config for writing
pub fn get_mut_config() -> Arc<Mutex<Config>> {
    CONFIG
        .get_or_init(|| Arc::new(Mutex::new(Config::default())))
        .clone()
}

/// Fetch config only for reading
pub fn get_ro_config() -> Result<Config, Error> {
    Ok(get_mut_config().lock()?.clone())
}

static LOGLEVEL: OnceLock<LogLevel> = OnceLock::new();
/// Fetch loglevel either from cli arg or config
pub fn get_loglevel() -> LogLevel {
    LOGLEVEL
        .get_or_init(|| {
            // fetch cli argument first, otherwise config
            if crate::arguments::get_argument(&crate::arguments::Argument::Debug).is_some() {
                LogLevel::Debug
            } else if crate::arguments::get_argument(&crate::arguments::Argument::Quiet).is_some() {
                LogLevel::Off
            } else {
                get_ro_config().unwrap_or_default().logging
            }
        })
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
