use std::sync::{Arc, Mutex, OnceLock};

use crate::{
    arguments::{get_argument, Argument},
    backend::config::{Config, LogLevel},
    Error,
};

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

// A bunch of default values
pub static DEFAULT_REMOTE_URL: &str =
    "https://api.github.com/repos/Raspirus/yara-rules/releases/latest";
pub static DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Debug;
pub static DEFAULT_LANGUAGE: &str = "en_US";
pub static CONFIG_VERSION: usize = 0;
pub static CONFIG_FILE_NAME: &str = "raspirus.cfg";

/// Default web request timeout
pub static TIMEOUT: u64 = 240;

// Values changeable from cli arguments, env vars or config
static MIN_MATCHES: OnceLock<usize> = OnceLock::new();
static MAX_MATCHES: OnceLock<usize> = OnceLock::new();
static MAX_THREADS: OnceLock<usize> = OnceLock::new();
static LOGLEVEL: OnceLock<LogLevel> = OnceLock::new();
static REMOTE_URL: OnceLock<String> = OnceLock::new();

/// Fetches minmatches from CLI > Config
pub fn get_min_matches() -> usize {
    *MIN_MATCHES.get_or_init(|| match get_argument(&Argument::MinMatches(None)) {
        Some(Argument::MinMatches(Some(min_matches))) => min_matches,
        Some(_) | None => get_ro_config().unwrap_or_default().min_matches,
    })
}

/// Fetches maxmatches from CLI > Config
pub fn get_max_matches() -> usize {
    *MAX_MATCHES.get_or_init(|| match get_argument(&Argument::MaxMatches(None)) {
        Some(Argument::MaxMatches(Some(max_matches))) => max_matches,
        Some(_) | None => get_ro_config().unwrap_or_default().max_matches,
    })
}

/// Fetches maxthreads from CLI > Config
pub fn get_max_threads() -> usize {
    *MAX_THREADS.get_or_init(|| match get_argument(&Argument::Threads(None)) {
        Some(Argument::Threads(Some(max_threads))) => max_threads,
        Some(_) | None => get_ro_config().unwrap_or_default().max_threads,
    })
}

/// Fetch loglevel either from cli arg or config
pub fn get_loglevel() -> LogLevel {
    LOGLEVEL
        .get_or_init(|| {
            // fetch cli argument first, otherwise config
            if get_argument(&Argument::Debug).is_some() {
                LogLevel::Debug
            } else if get_argument(&Argument::Quiet).is_some() {
                LogLevel::Off
            } else {
                get_ro_config().unwrap_or_default().logging
            }
        })
        .clone()
}

/// Fetch remote url for udpates from CLI > Config
pub fn get_remote_url() -> String {
    REMOTE_URL
        .get_or_init(|| match get_argument(&Argument::Remote(None)) {
            Some(Argument::Remote(Some(remote_url))) => remote_url,
            Some(_) | None => get_ro_config().unwrap_or_default().remote_url,
        })
        .to_string()
}
