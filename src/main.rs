// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config_file::Config;
use frontend::iced::{LocationSelection, Raspirus};
use iced::Settings;
use lazy_static::lazy_static;
use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use std::sync::Mutex;

use chrono::Local;
use iced::advanced::graphics::image::image_rs::ImageFormat;
use iced::window::icon;

mod backend;
mod frontend;
mod tests;

/// Macro for locale, allows to use t!() for string translations
#[macro_use]
extern crate rust_i18n;

/// config
static CONFIG_FILENAME: &str = "Raspirus.json";
static CONFIG_VERSION: &str = "4";

/// remote params
static DEFAULT_MIRROR: &str = "https://api.github.com/repos/Raspirus/yara-rules/releases/latest";
static DEFAULT_FILE: &str = "rulepirus.yarac";

/// default scan params
static DEFAULT_MIN_MATCHES: usize = 0;
static DEFAULT_MAX_MATCHES: usize = 20;

static MAX_ZIP_FILE_SIZE: u64 = 1073741824;

/// download settings
static MAX_TIMEOUT: u64 = 120;

#[cfg(debug_assertions)]
static LOGGING_FILTER: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
static LOGGING_FILTER: LevelFilter = LevelFilter::Info;

lazy_static! {
    /// Create string with current time
    static ref APPLICATION_LOG: String = format!("{}.log", Local::now().format("%Y_%m_%d_%H_%M_%S"));
    /// Global config instance
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new().expect("Failed to load config"));
    /// Supported languages
    static ref SUPPORTED_LANGUAGES: Vec<String> = vec!["en".to_owned(), "de".to_owned(), "it".to_owned()];
    /// Symbols for selection
    static ref SELECTION_ICONS: Vec<LocationSelection> = vec![LocationSelection::Usb { usb: None }, LocationSelection::Folder { path: None }, LocationSelection::File { path: None }];
}

fn main() -> Result<(), String> {
    // Set locale
    // TODO: https://github.com/longbridgeapp/rust-i18n?tab=readme-ov-file#usage
    i18n!("src/assets/locales", fallback = "en");

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

        let mut log_config = ConfigBuilder::new();

        #[cfg(debug_assertions)]
        let log_config = log_config
            .add_filter_ignore_str("wgpu_core")
            .add_filter_ignore_str("iced_wgpu")
            .add_filter_ignore_str("iced_winit")
            .add_filter_ignore_str("cosmic_text")
            .add_filter_ignore_str("naga")
            .add_filter_ignore_str("cranelift_codegen")
            .add_filter_ignore_str("wasmtime")
            .add_filter_ignore_str("aho_corasick")
            .add_filter_ignore_str("walrus")
            .add_filter_ignore_str("wgpu_hal")
            .add_filter_ignore_str("Naga")
            .add_filter_ignore_str("sctk");

        let log_config = log_config.add_filter_ignore_str("reqwest");

        let log_config = log_config.build();

        // Terminal logger is always used if logging so we add it right away
        let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(
            LOGGING_FILTER,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )];

        // If we are able to create both the file and directory path, we can start the FileLogger
        let log_file = File::create(log_application)
            .map_err(|err| format!("Failed to create application logfile: {err}"))?;
        loggers.push(WriteLogger::new(LOGGING_FILTER, log_config, log_file));

        // Start loggers
        CombinedLogger::init(loggers).expect("Failed to initialize CombinedLogger");
    }

    const ICON_BYTES: &[u8] = include_bytes!("assets/logo.ico");
    let mut settings = Settings::default();
    let mut window_settings = iced::window::Settings::default();
    settings.id = Some("raspirus.app".to_owned());
    settings.fonts = vec![iced_fonts::BOOTSTRAP_FONT_BYTES.into()];
    window_settings.icon = icon::from_file_data(ICON_BYTES, Option::from(ImageFormat::Ico)).ok();
    iced::application("Raspirus", Raspirus::update, Raspirus::view)
        .settings(settings)
        .exit_on_close_request(true)
        .window(window_settings)
        .subscription(Raspirus::subscription)
        .run()
        .unwrap();

    Ok(())
}
