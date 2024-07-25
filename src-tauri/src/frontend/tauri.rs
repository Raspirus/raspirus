use std::{path::PathBuf, sync::Arc};

use log::error;

use crate::{
    backend::{
        config_file::{Config, ConfigFrontend},
        downloader::{self, RemoteError},
        utils::{
            self,
            generic::{generate_virustotal, get_config, update_config}, usb_utils::UsbDevice,
        },
        yara_scanner::{Skipped, TaggedFile, YaraScanner},
    },
    frontend::functions::cli_scanner,
};
use tauri_plugin_cli::CliExt;

use super::functions::{cli_gui, cli_update, not_implemented};

pub fn init_tauri() {
    // Builds the Tauri connection
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Default to GUI if the app was opened with no CLI args.
            if std::env::args_os().count() <= 1 {
                cli_gui(app.handle().clone())?;
            }
            // Else, we start in CLI mode and parse the given parameters
            let matches = match app.cli().matches() {
                Ok(matches) => matches,
                Err(err) => {
                    error!("{}", err);
                    app.handle().exit(1);
                    return Ok(());
                }
            };

            // Iterate over each key and execute functions based on them
            matches.args.iter().for_each(|(key, data)| {
                if data.occurrences > 0 && key.as_str() != "help" && key.as_str() != "version" {
                    // Define all CLI commands/arguments here and in the tauri.conf.json file
                    // WARNING: If the commmand is not defined in the tauri.conf.json file, it can't be used here
                    match key.as_str() {
                        "gui" => {
                            if let Err(err) = cli_gui(app.handle().clone()) {
                                error!("GUI Error: {}", err);
                            }
                        }
                        "scan" => cli_scanner(
                            app.handle().clone(),
                            match data.value.clone() {
                                serde_json::Value::String(data) => data,
                                _ => {
                                    error!("Received invalid data {data:?} for 'scan'");
                                    app.handle().exit(-1);
                                    return;
                                }
                            },
                        ),
                        "db-update" => cli_update(app.handle().clone()),
                        _ => not_implemented(app.handle().clone()),
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_scanner,
            list_usb_drives,
            update,
            load_config_fe,
            save_config_fe,
            download_logs,
            check_update,
            rules_version,
            lookup_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Starts the scanner over the GUI
#[tauri::command]
async fn start_scanner(window: tauri::Window, path: PathBuf) -> Result<(Vec<TaggedFile>, Vec<Skipped>), String> {
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
        let mut config = get_config();
        // update received fields
        received
            .logging_is_active
            .inspect(|val| config.logging_is_active = *val);
        received
            .scan_dir
            .inspect(|val| config.scan_dir = *val);
        received
            .min_matches
            .inspect(|val| config.min_matches = *val);
        received
            .max_matches
            .inspect(|val| config.max_matches = *val);
        // save updated config
        update_config(config)
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
async fn load_config_fe() -> Result<Config, String> {
    tokio::task::spawn_blocking(|| {
        get_config()
    })
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
    let log_dir = get_config()
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .logs
        .join("main");
    let app_log_path = log_dir.join("app.log");

    let log_path = get_config()
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .downloads
        .join("log.txt");

    // If there's an error during copying, return an error message
    std::fs::copy(app_log_path, &log_path)
        .map_err(|err| format!("Error copying log file: {err}"))?;
    // If the copy operation is successful, return Ok indicating success
    Ok(log_path)
}

#[tauri::command]
async fn rules_version() -> Result<String, String> {
    tokio::task::spawn_blocking(|| get_config().rules_version)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn lookup_file(file: String) -> Result<String, String> {
    let file = serde_json::from_str::<PathBuf>(&file)
        .map_err(|err| err.to_string())?;
    tokio::task::spawn_blocking(|| generate_virustotal(file))
        .await
        .map_err(|err| err.to_string())?
}
