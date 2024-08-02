// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config_file::Config;
use frontend::tauri::init_tauri;
use lazy_static::lazy_static;
use log::{info, LevelFilter};
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
use std::fs;
use std::fs::File;
use std::sync::Mutex;

use chrono::{DateTime, Local};

mod backend;
mod frontend;
mod tests;

// config
static CONFIG_FILENAME: &str = "Raspirus.json";
static CONFIG_VERSION: &str = "2";

// remote params
static DEFAULT_MIRROR: &str = "https://api.github.com/repos/Raspirus/yara-rules/releases/latest";
static DEFAULT_FILE: &str = "rulepirus.yarac";

// default scan params
static DEFAULT_MIN_MATCHES: usize = 0;
static DEFAULT_MAX_MATCHES: usize = 20;

// download settings
static MAX_TIMEOUT: u64 = 120;

static LOGGING_FILTER: LevelFilter = LevelFilter::Debug;

lazy_static! {
    // Create string with current time
    static ref APPLICATION_LOG: String = format!("{}.log", Local::now().format("%Y_%m_%d_%H_%M_%S"));
    // Global config instance
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new().expect("Failed to load config"));
}

// NOTE: All functions with #[tauri::command] can and will be called from the GUI
// Their name should not be changed and any new functions should return JSON data
// using serde parsing

fn main() -> Result<(), String> {
    // We check if we should log the application messages to a file or not, default is yes. Defined in the Config
    if CONFIG
        .lock()
        .expect("Failed to get config")
        .logging_is_active
    {
        // Create string with current time
        let now: DateTime<Local> = Local::now();
        let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();

        // Logdir for application
        let log_dir = CONFIG
            .lock()
            .expect("Failed to get config")
            .paths
            .clone()
            .ok_or("No paths set. Is config initialized?".to_owned())?
            .logs_app;

        fs::create_dir_all(&log_dir)
            .map_err(|err| format!("Failed to create application logdir: {err}"))?;
        let log_file_path = log_dir.join(format!("{now_str}.log"));

        let log_config = ConfigBuilder::new()
            .add_filter_ignore_str("reqwest")
            .add_filter_ignore_str("hyper_util")
            .add_filter_ignore_str("cranelift_codegen")
            .add_filter_ignore_str("wasmtime")
            .add_filter_ignore_str("aho_corasick")
            .build();
        // Terminal logger is always used if logging so we add it right away
        let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(
            LOGGING_FILTER,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )];

        // If we are able to create both the file and directory path, we can start the FileLogger
        let log_file = File::create(&log_file_path)
            .map_err(|err| format!("Failed to create application logfile: {err}"))?;
        info!(
            "Created application log file at {}",
            log_file_path.to_string_lossy()
        );
        loggers.push(WriteLogger::new(LOGGING_FILTER, log_config, log_file));

        // Start loggers
        CombinedLogger::init(loggers).expect("Failed to initialize CombinedLogger");
    }

    // initializes the frontend
    init_tauri();

    Ok(())
}
