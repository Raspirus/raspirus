#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use backend::config_file::Config;
use backend::utils;
use directories_next::ProjectDirs;
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::{env, fs};

mod backend;
mod tests;

fn main() {
    let mut config = Config::new();
    config = config.load().expect("Couldn't load config at startup");
    let write_to_file = config.logging_is_active;

    if write_to_file {
        // We use ProjectDirs to find a suitable location for our logging file
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Logs")
            .expect("Failed to get project directories.");
        let log_dir = project_dirs.data_local_dir().join("main"); // Create a "main" subdirectory

        // If we are able to create both the file and directory path, we can start the FileLogger
        match fs::create_dir_all(&log_dir) {
            Ok(_) => {
                match File::create(log_dir.join("app.log")) {
                    Ok(log_file) => {
                        info!(
                            "Created logfile at DIR: {} NAME: app.log",
                            log_dir.display()
                        );
                        // Define both File logger and Terminal logger
                        let file_logger = WriteLogger::new(
                            LevelFilter::Debug,
                            simplelog::Config::default(),
                            log_file,
                        );
                        let term_logger = TermLogger::new(
                            LevelFilter::Debug,
                            simplelog::Config::default(),
                            TerminalMode::Mixed,
                            ColorChoice::Auto,
                        );
                        // Start both loggers concurrently
                        CombinedLogger::init(vec![file_logger, term_logger])
                            .expect("Failed to initialize CombinedLogger");
                    }
                    Err(err) => {
                        error!("Failed creating logfile: {err}");
                    }
                };
            }
            Err(err) => error!("Failed creating logs folder: {err}"),
        }
    } else {
        // If logging is disabled, only the terminal logger will run
        TermLogger::init(
            LevelFilter::Debug,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
        .expect("Failed to init TermLogger");
    }

    // Builds the Tauri connections for each function listed here
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_scanner,
            list_usb_drives,
            update_database,
            check_raspberry,
            create_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Starts the scanner for the given path and updates the database if update is true.
///
/// # Arguments
///
/// * `path` - The path to scan.
/// * `update` - Whether to update the database before scanning.
/// * `dbfile` - An optional path to a specific database file.
///
/// # Returns
///
/// An empty `Result` object if the scanner was successfully started, or an `Err` with an error message if an error occurred.
#[tauri::command]
async fn start_scanner(window: tauri::Window, path: String) -> Result<String, String> {
    utils::scanner_utils::start_scanner(window, path).await
}

#[tauri::command]
async fn check_raspberry() -> Result<bool, String> {
    let arch = std::env::consts::ARCH;

    if arch == "arm" {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
async fn update_database(window: tauri::Window) -> Result<String, String> {
    utils::update_utils::update_database(window).await
}

/// Lists the USB drives attached to the system.
///
/// # Returns
///
/// A `Result` object containing a vector of strings representing the paths to the USB drives, or an `Err` with an error message if an error occurred.
#[tauri::command]
async fn list_usb_drives() -> Result<String, String> {
    utils::usb_utils::list_usb_drives().await
}

#[tauri::command]
async fn create_config(contents: Option<String>) -> Result<String, String> {
    let config = if let Some(contents) = contents {
        serde_json::from_str(&contents).map_err(|err| err.to_string())?
    } else {
        Config::new().load().expect("Error while loading config")
    };

    config.save().map_err(|err| err.to_string())?;
    let config_str = serde_json::to_string(&config).unwrap();

    Ok(config_str)
}

pub async fn auto_update_scheduler(tauri_win: tauri::Window, hour: i32, weekday: i32) {
    // ISSUE: Needs to restart app to apply new update schedule
    utils::update_utils::auto_update_scheduler(tauri_win, hour, weekday).await
}
