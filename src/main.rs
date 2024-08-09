// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config_file::Config;
use frontend::iced::LocationSelection;
use iced::{Application, Settings};
use lazy_static::lazy_static;
use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use std::sync::Mutex;

use chrono::Local;

mod backend;
mod frontend;
mod tests;

/// config
static CONFIG_FILENAME: &str = "Raspirus.json";
static CONFIG_VERSION: &str = "3";

/// remote params
static DEFAULT_MIRROR: &str = "https://api.github.com/repos/Raspirus/yara-rules/releases/latest";
static DEFAULT_FILE: &str = "rulepirus.yarac";

/// default scan params
static DEFAULT_MIN_MATCHES: usize = 0;
static DEFAULT_MAX_MATCHES: usize = 20;

/// download settings
static MAX_TIMEOUT: u64 = 120;

static LOGGING_FILTER: LevelFilter = LevelFilter::Debug;

lazy_static! {
    /// Create string with current time
    static ref APPLICATION_LOG: String = format!("{}.log", Local::now().format("%Y_%m_%d_%H_%M_%S"));
    /// Global config instance
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new().expect("Failed to load config"));
    /// Supported languages
    static ref SUPPORTED_LANGUAGES: Vec<String> = vec!["en-US".to_owned(), "de-DE".to_owned(), "fr-FR".to_owned()];
    /// Symbols for selection
    static ref SELECTION_ICONS: Vec<LocationSelection> = vec![LocationSelection::USB { usb: None }, LocationSelection::Folder { path: None }, LocationSelection::File { path: None }];
}

fn main() -> Result<(), String> {
    // We check if we should log the application messages to a file or not, default is yes. Defined in the Config
    if CONFIG
        .lock()
        .expect("Failed to lock config")
        .logging_is_active
    {
        // Logdir for application
        let log_application = CONFIG
            .lock()
            .expect("Failed to lock config")
            .paths
            .clone()
            .ok_or("No paths set. Is config initialized?".to_owned())?
            .logs_app;

        let log_config = ConfigBuilder::new()
            .add_filter_ignore_str("reqwest")
            .add_filter_ignore_str("wgpu_core")
            .add_filter_ignore_str("iced_wgpu")
            .add_filter_ignore_str("cosmic_text")
            .add_filter_ignore_str("naga")
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
        let log_file = File::create(&log_application)
            .map_err(|err| format!("Failed to create application logfile: {err}"))?;
        loggers.push(WriteLogger::new(LOGGING_FILTER, log_config, log_file));

        // Start loggers
        CombinedLogger::init(loggers).expect("Failed to initialize CombinedLogger");
    }

    let mut settings = Settings::default();
    settings.window.exit_on_close_request = false;
    settings.id = Some("raspirus.app".to_owned());
    settings.fonts = vec![iced_aw::BOOTSTRAP_FONT_BYTES.into()];
    frontend::iced::Raspirus::run(settings).expect("Failed to start frontend");

    Ok(())
}
