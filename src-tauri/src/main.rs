// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config_file::Config;
use backend::utils::generic::{clear_cache, get_config};
use frontend::main::init_tauri;
use log::{error, info, LevelFilter};
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::sync::Arc;

mod backend;
mod frontend;
mod tests;
mod benches;

// config
static CONFIG_FILENAME: &str = "Raspirus.json";

// database settings
static DB_NAME: &str = "signatures.db";
static DB_TABLE: &str = "signatures";

// download settings
static MAX_RETRY: usize = 5;
static PARALLEL_DOWNLOADS: usize = 3;
static MAX_TIMEOUT: u64 = 120;

// global config instance
thread_local!(static CONFIG: RefCell<Arc<Config>> = 
    RefCell::new(Arc::new(Config::new().expect("Failed to get paths"))));

// NOTE: All functions with #[tauri::command] can and will be called from the GUI
// Their name should not be changed and any new functions should return JSON data
// using serde parsing

fn main() -> Result<(), String> {
    // We try to load the config, to make sure the rest of the programm will always have valid data to work with
    let config = get_config();

    // We check if we should log the application messages to a file or not, default is yes. Defined in the Config
    if config.logging_is_active {
        // Get logdir with Main Subdir
        let log_dir = config
            .paths
            .ok_or("No paths set. Is config initialized?".to_owned())?
            .logs
            .join("main");

        let log_config = ConfigBuilder::new()
            .add_filter_ignore("reqwest".to_string())
            .build();
        // Terminal logger is always used if logging so we add it right away
        let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(
            LevelFilter::Trace,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )];

        // If we are able to create both the file and directory path, we can start the FileLogger
        match fs::create_dir_all(&log_dir) {
            // Create a new file with the given name. Will overwrite the old/existisng file
            Ok(_) => match File::create(log_dir.join("app.log")) {
                Ok(log_file) => {
                    info!("Created logfile at {}", log_dir.display());
                    // file logger is only used if log path is defined
                    loggers.push(WriteLogger::new(LevelFilter::Debug, log_config, log_file));
                }
                Err(err) => error!("Failed creating logfile: {err}"),
            },
            Err(err) => error!("Failed creating logs folder: {err}"),
        }

        // Start loggers
        CombinedLogger::init(loggers).expect("Failed to initialize CombinedLogger");
    }

    // clear caches before starting ui
    clear_cache().map_err(|err| format!("Failed to clear caches: {err}"))?;

    // initializes the frontend
    init_tauri();

    Ok(())
}
