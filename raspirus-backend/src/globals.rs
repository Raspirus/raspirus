use std::{
    borrow::Cow,
    sync::{Arc, Mutex, OnceLock},
};

use crate::{
    cli::{ArgumentValue, Parser},
    config::config::{Config, LogLevel},
};

type Error = crate::Error;

/// Application startup time used for logging. Can be fetched via get_application_log
static APPLICATION_LOG: OnceLock<String> = OnceLock::new();
pub fn get_application_log_filename() -> String {
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

/// Holds the CLI parser which also contains all values after .parse() gets called
static PARSER: OnceLock<Arc<Mutex<Parser>>> = OnceLock::new();
/// Creates the parser and parses the arguments
pub fn get_parser() -> Result<Arc<Mutex<Parser>>, Error> {
    if let Some(parser) = PARSER.get() {
        return Ok(parser.clone());
    }

    #[rustfmt::skip]
    let mut parser = Parser::default()
        .add_arg('h',        "help",        t!("ARGUMENTS.ARGUMENT.HELP"),        ArgumentValue::None)
        .add_arg('n', "nogui", t!("ARGUMENTS.ARGUMENT.NOGUI"), ArgumentValue::None)
        .add_arg('f', "fullscreen", t!("ARGUMENTS.ARGUMENT.FULLSCREEN"), ArgumentValue::None)
        .add_arg('u', "update", t!("ARGUMENTS.ARGUMENT.UPDATE"), ArgumentValue::None)
        .add_arg('d', "debug", t!("ARGUMENTS.ARGUMENT.DEBUG"), ArgumentValue::None)
        .add_arg('q', "quiet", t!("ARGUMENTS.ARGUMENT.QUIET"), ArgumentValue::None)
        .add_arg('s', "scan", t!("ARGUMENTS.ARGUMENT.SCAN"), ArgumentValue::String(None))
        .add_arg('j', "json", t!("ARGUMENTS.ARGUMENT.JSON"), ArgumentValue::None)
        .add_arg('t', "threads", t!("ARGUMENTS.ARGUMENT.THREADS"), ArgumentValue::Number(None))
        .add_arg('x', "max", t!("ARGUMENTS.ARGUMENT.MAX"), ArgumentValue::Number(None))
        .add_arg('i', "min", t!("ARGUMENTS.ARGUMENT.MIN"), ArgumentValue::Number(None))
        .add_arg('r', "remote", t!("ARGUMENTS.ARGUMENT.REMOTE"), ArgumentValue::String(None));
    parser.parse(std::env::args())?;
    let arc_mut_parser = Arc::new(Mutex::new(parser));
    let _ = PARSER.set(arc_mut_parser.clone());
    Ok(arc_mut_parser)
}

/// Queries the parser for a specific argument. Returns none if argument was not present, otherwise
/// some with the DEFAULT_LANGUAGE
pub fn get_argument<T>(index: (Option<char>, Option<T>)) -> Result<Option<ArgumentValue>, Error>
where
    T: Into<String> + Clone,
{
    Ok(get_parser()?.lock()?.get_argument(index))
}

// A bunch of default values
pub static DEFAULT_REMOTE_URL: &str =
    "https://api.github.com/repos/raspirus/yara-rules/releases/latest";
pub static DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Debug;
pub static DEFAULT_LANGUAGE: &str = "en_US";
pub static CONFIG_VERSION: usize = 7;
pub static CONFIG_FILE_NAME: &str = "raspirus.cfg";

/// Default web request timeout
pub static TIMEOUT: u64 = 240;

// Values changeable from cli arguments, env vars or config
static MIN_MATCHES: OnceLock<usize> = OnceLock::new();
static MAX_MATCHES: OnceLock<usize> = OnceLock::new();
static THREADS: OnceLock<usize> = OnceLock::new();
static LOGLEVEL: OnceLock<LogLevel> = OnceLock::new();
static REMOTE_URL: OnceLock<String> = OnceLock::new();

/// Fetches minmatches from CLI > Config
pub fn get_min_matches() -> Result<usize, Error> {
    if let Some(min_matches) = MIN_MATCHES.get() {
        return Ok(*min_matches);
    }

    let min_matches = match get_argument((Some('i'), Some("min")))? {
        Some(ArgumentValue::Number(Some(min_matches))) => min_matches,
        _ => get_ro_config().unwrap_or_default().scanner.min_matches,
    };
    let _ = MIN_MATCHES.set(min_matches);
    Ok(min_matches)
}

/// Fetches maxmatches from CLI > Config
pub fn get_max_matches() -> Result<usize, Error> {
    if let Some(max_matches) = MAX_MATCHES.get() {
        return Ok(*max_matches);
    }

    let max_matches = match get_argument((Some('x'), Some("max")))? {
        Some(ArgumentValue::Number(Some(max_matches))) => max_matches,
        _ => get_ro_config().unwrap_or_default().scanner.max_matches,
    };
    let _ = MAX_MATCHES.set(max_matches);
    Ok(max_matches)
}

/// Fetches maxthreads from CLI > Config
pub fn get_max_threads() -> Result<usize, Error> {
    if let Some(threads) = THREADS.get() {
        return Ok(*threads);
    }

    let threads = match get_argument((Some('t'), Some("threads")))? {
        Some(ArgumentValue::Number(Some(threads))) => threads,
        _ => get_ro_config().unwrap_or_default().scanner.max_threads,
    };
    let _ = THREADS.set(threads);
    Ok(threads)
}

/// Fetch loglevel either from cli arg or config
pub fn get_loglevel() -> Result<LogLevel, Error> {
    if let Some(loglevel) = LOGLEVEL.get() {
        return Ok(loglevel.clone());
    }

    let loglevel = if get_argument((Some('d'), Some("debug")))?.is_some() {
        LogLevel::Debug
    } else if get_argument((Some('q'), Some("quiet")))?.is_some() {
        LogLevel::Off
    } else {
        get_ro_config().unwrap_or_default().logging
    };
    let _ = LOGLEVEL.set(loglevel.clone());
    Ok(loglevel)
}

/// Fetch remote url for udpates from CLI > Config
pub fn get_remote_url() -> Result<String, Error> {
    if let Some(remote_url) = REMOTE_URL.get() {
        return Ok(remote_url.clone());
    }

    let remote_url = match get_argument((Some('r'), Some("remote")))? {
        Some(ArgumentValue::String(Some(remote_url))) => remote_url,
        _ => get_ro_config().unwrap_or_default().remote_url,
    };
    let _ = REMOTE_URL.set(remote_url.clone());
    Ok(remote_url)
}

/// Translates a key using the backend translation file
pub fn translate(key: &str) -> Cow<'_, str> {
    t!(key)
}
