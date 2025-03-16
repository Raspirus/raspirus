use lazy_static::lazy_static;

use crate::backend::log::LogLevel;

// More intricate globals
lazy_static! {
    /// String with log init time
    pub static ref APPLICATION_LOG: String = chrono::Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
}

// A bunch of default values
pub static DEFAULT_REMOTE_URL: &str = "https://api.github.com/repos/Raspirus/yara-rules/releases/latest";
pub static DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Debug;
pub static DEFAULT_LANGUAGE: &str = "en_US";
pub static CONFIG_VERSION: usize = 0;
pub static CONFIG_FILE_NAME: &str = "raspirus.cfg";
