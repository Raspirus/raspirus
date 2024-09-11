use std::{path::PathBuf, sync::Arc};

use log::{debug, error, info, trace, warn};

use crate::backend::utils::generic::FrontendLog;
use crate::{
    backend::{
        config_file::{Config, ConfigFrontend},
        downloader::{self, RemoteError},
        utils::{self, generic::generate_virustotal, usb_utils::UsbDevice},
        yara_scanner::{Skipped, TaggedFile, YaraScanner},
    },
    frontend::functions::cli_scanner,
};
use crate::CONFIG;

use super::functions::{cli_gui, cli_update, not_implemented};

// Starts the scanner over the GUI
#[tauri::command]
async fn start_scanner(
    window: tauri::Window,
    path: PathBuf,
) -> Result<(Vec<TaggedFile>, Vec<Skipped>), String> {
    tokio::task::spawn_blocking(move || {
        let mut scanner = YaraScanner::new(Some(Arc::new(window)))?;
        scanner.start(path)
    })
    .await
    .map_err(|err| err.to_string())?
}

// Updates the database over the GUi
#[tauri::command]
async fn update() -> Result<(), RemoteError> {
    downloader::update().await
}

// Returns a vector of all attached removable storage drives (USB) -> Unnecessary for the CLI
#[tauri::command]
async fn list_usb_drives() -> Result<Vec<UsbDevice>, String> {
    utils::usb_utils::list_usb_drives().await
}

// Creates the config from the GUI
#[tauri::command]
async fn save_config_fe(received: ConfigFrontend) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let mut config = CONFIG.lock().expect("Failed to lock config");
        // update received fields
        received
            .logging_is_active
            .inspect(|val| config.logging_is_active = *val);
        received.scan_dir.inspect(|val| config.scan_dir = *val);
        received
            .min_matches
            .inspect(|val| config.min_matches = *val);
        received
            .max_matches
            .inspect(|val| config.max_matches = *val);
        // save updated config
        config.save()
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
async fn load_config_fe() -> Result<Config, String> {
    tokio::task::spawn_blocking(|| CONFIG.lock().expect("Failed to lock config").clone())
        .await
        .map_err(|err| err.to_string())
}

/// verifies if there are any yara rules present
#[tauri::command]
async fn check_update() -> Result<bool, RemoteError> {
    downloader::check_update().await
}

#[tauri::command]
async fn download_logs() -> Result<PathBuf, String> {
    let config = CONFIG.lock().expect("Failed to lock config").clone();
    let app_log = config
        .paths
        .clone()
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .logs_app;

    let download_path = config
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .downloads
        .join("log.txt");

    // If there's an error during copying, return an error message
    std::fs::copy(&app_log, &download_path)
        .map_err(|err| format!("Error copying log file: {err}"))?;
    // If the copy operation is successful, return Ok indicating success
    Ok(download_path)
}

#[tauri::command]
async fn rules_version() -> Result<String, String> {
    tokio::task::spawn_blocking(|| {
        CONFIG
            .lock()
            .expect("Failed to lock config")
            .rules_version
            .clone()
    })
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
async fn lookup_file(file: String) -> Result<String, String> {
    let file = serde_json::from_str::<PathBuf>(&file).map_err(|err| err.to_string())?;
    tokio::task::spawn_blocking(|| generate_virustotal(file))
        .await
        .map_err(|err| err.to_string())?
}

#[tauri::command]
async fn log_frontend(msg: FrontendLog) {
    match msg {
        FrontendLog::Error(msg) => error!("Frontend: {msg}"),
        FrontendLog::Warn(msg) => warn!("Frontend: {msg}"),
        FrontendLog::Info(msg) => info!("Frontend: {msg}"),
        FrontendLog::Debug(msg) => debug!("Frontend: {msg}"),
        FrontendLog::Trace(msg) => trace!("Frontend: {msg}"),
    }
}
