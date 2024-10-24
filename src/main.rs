// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config_file::Config;
use frontend::iced::{Language, LocationSelection, Raspirus};
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

// Sets the locale globally
rust_i18n::i18n!("src/assets/locales", fallback = "en");

/// config
static CONFIG_FILENAME: &str = "Raspirus.json";
static CONFIG_VERSION: &str = "7";

/// remote params
static DEFAULT_MIRROR: &str = "https://api.github.com/repos/Raspirus/yara-rules/releases/latest";
static DEFAULT_FILE: &str = "rulepirus.yarac";

/// default scan params
static DEFAULT_MIN_MATCHES: usize = 0;
static DEFAULT_MAX_MATCHES: usize = 20;

/// should be a gigabyte but who knows, maybe its a terabyte :3 youll know once your pc explodes
static MAX_ZIP_FILE_SIZE: u64 = 1073741824;

/// download settings
static MAX_TIMEOUT: u64 = 120;

#[cfg(debug_assertions)]
static LOGGING_FILTER: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
static LOGGING_FILTER: LevelFilter = LevelFilter::Info;

/// icons
static ONLINE: &[u8] = include_bytes!("assets/icons/online.svg").as_slice();
static CHEVRON_DOWN: &[u8] = include_bytes!("assets/icons/chevron-down.svg").as_slice();
static CHEVRON_LEFT: &[u8] = include_bytes!("assets/icons/chevron-left.svg").as_slice();
static ARROW_BADGE_DOWN: &[u8] = include_bytes!("assets/icons/arrow-badge-down.svg").as_slice();
static ARROW_BADGE_UP: &[u8] = include_bytes!("assets/icons/arrow-badge-up.svg").as_slice();
static CHECK: &[u8] = include_bytes!("assets/icons/check.svg").as_slice();
static CLIPBOARD_DATA: &[u8] = include_bytes!("assets/icons/clipboard-data.svg").as_slice();
static CPU: &[u8] = include_bytes!("assets/icons/cpu.svg").as_slice();
static DATABASE_IMPORT: &[u8] = include_bytes!("assets/icons/database-import.svg").as_slice();
static USB: &[u8] = include_bytes!("assets/icons/usb.svg").as_slice();
static HOURGLASS_HIGH: &[u8] = include_bytes!("assets/icons/hourglass-high.svg").as_slice();
static DOWNLOAD: &[u8] = include_bytes!("assets/icons/download.svg").as_slice();
static EXCLAMATION_CIRCLE: &[u8] = include_bytes!("assets/icons/exclamation-circle.svg").as_slice();
static FILE: &[u8] = include_bytes!("assets/icons/file.svg").as_slice();
static FILE_DESCRIPTION: &[u8] = include_bytes!("assets/icons/file-description.svg").as_slice();
static FILE_DOWNLOAD: &[u8] = include_bytes!("assets/icons/file-download.svg").as_slice();
static FOLDER: &[u8] = include_bytes!("assets/icons/folder.svg").as_slice();
static GIT_COMMIT: &[u8] = include_bytes!("assets/icons/git-commit.svg").as_slice();
static GLOBE: &[u8] = include_bytes!("assets/icons/globe.svg").as_slice();
static HEXAGON_LETTER: &[u8] = include_bytes!("assets/icons/hexagon-letter-r.svg").as_slice();
static HOME: &[u8] = include_bytes!("assets/icons/home.svg").as_slice();
static LICENSE: &[u8] = include_bytes!("assets/icons/license.svg").as_slice();
static REFRESH: &[u8] = include_bytes!("assets/icons/refresh.svg").as_slice();
static SETTINGS: &[u8] = include_bytes!("assets/icons/settings.svg").as_slice();
static USER_CODE: &[u8] = include_bytes!("assets/icons/user-code.svg").as_slice();
static THEME_TOGGLE: &[u8] = include_bytes!("assets/icons/theme-toggle.svg").as_slice();
static ZOOM: &[u8] = include_bytes!("assets/icons/zoom.svg").as_slice();

/// flags
static FLAG_DE: &[u8] = include_bytes!("assets/flags/de.svg").as_slice();
static FLAG_EN: &[u8] = include_bytes!("assets/flags/en.svg").as_slice();
static FLAG_IT: &[u8] = include_bytes!("assets/flags/it.svg").as_slice();
static FLAG_FR: &[u8] = include_bytes!("assets/flags/fr.svg").as_slice();

/// logo
static USB_VECTOR: &[u8] = include_bytes!("assets/usb-vector.svg").as_slice();
static LOGO_VECTOR: &[u8] = include_bytes!("assets/logo-vector.svg").as_slice();
static LOGO_VECTOR_STR: &str = include_str!("assets/logo-vector.svg");

lazy_static! {
    /// Create string with current time
    static ref APPLICATION_LOG: String = format!("{}.log", Local::now().format("%Y_%m_%d_%H_%M_%S"));
    /// Global config instance
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new().expect("Failed to load config"));
    /// Supported languages
    static ref SUPPORTED_LANGUAGES: Vec<Language> = vec![
        Language::new("de", "Deutsch", FLAG_DE),
        Language::new("en", "English", FLAG_EN),
        Language::new("it", "Italiano", FLAG_IT),
        Language::new("fr", "Français", FLAG_FR)
    ];
    //
    static ref TARGET_SELECTIONS: Vec<(LocationSelection, &'static [u8])> = vec![
        (LocationSelection::Usb { usb: None }, crate::USB),
        (LocationSelection::Folder { path: None }, crate::FOLDER),
        (LocationSelection::File { path: None }, crate::FILE)
    ];
    /// Supported archives
    static ref SUPPORTED_ARCHIVES: Vec<String> = vec!["zip".to_owned(), "xz".to_owned(), "zstd".to_owned(), "bzip2".to_owned(), "deflate64".to_owned()];
    /// Symbols for selection
    static ref SELECTION_ICONS: Vec<LocationSelection> = vec![LocationSelection::Usb { usb: None }, LocationSelection::Folder { path: None }, LocationSelection::File { path: None }];
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

        let mut log_config = ConfigBuilder::new();

        #[cfg(not(debug_assertions))]
        let log_config = log_config
            .add_filter_ignore_str("iced_winit")
            .add_filter_ignore_str("aho_corasick")
            .add_filter_ignore_str("walrus")
            .add_filter_ignore_str("wgpu_hal")
            .add_filter_ignore_str("Naga")
            .add_filter_ignore_str("sctk");

        let log_config = log_config
            .add_filter_ignore_str("naga")
            .add_filter_ignore_str("iced_wgpu")
            .add_filter_ignore_str("cosmic_text")
            .add_filter_ignore_str("cranelift_codegen")
            .add_filter_ignore_str("wasmtime")
            .add_filter_ignore_str("wgpu_core")
            .add_filter_ignore_str("reqwest");

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

    rust_i18n::set_locale(&CONFIG.lock().expect("Failed to lock config").language);

    const ICON_BYTES: &[u8] = include_bytes!("assets/logo.ico");
    let mut settings = Settings::default();
    let mut window_settings = iced::window::Settings::default();
    settings.id = Some("raspirus.app".to_owned());

    window_settings.icon = icon::from_file_data(ICON_BYTES, Option::from(ImageFormat::Ico)).ok();

    iced::application("Raspirus", Raspirus::update, Raspirus::view)
        .settings(settings)
        .exit_on_close_request(true)
        .window(window_settings)
        .subscription(Raspirus::subscription)
        .centered()
        .theme(|app| {
            if app.dark_mode {
                iced::Theme::Dark
            } else {
                iced::Theme::Light
            }
        })
        .scale_factor(|app| {
            app.scale as f64 / 100.0
            /*

            // TODO: We need a way to get the screen size consistently across devices to get proper scaling. for now the user has to set it manually
            // Get the display size (width, height)
            let (width, height) = (1920, 1080);
            let minimum_dimension = width.min(height) as f64;

            let scaling_factor = minimum_dimension / 1080.0;

            // Ensure the scaling factor is clamped within reasonable bounds (to avoid extremely large or small values)
            let clamped_scaling = scaling_factor.clamp(0.1, 1.0);
            debug!("Scaling factor: {}", clamped_scaling);

            clamped_scaling
            */
        })
        .run()
        .expect("Failed to run application");

    Ok(())
}
