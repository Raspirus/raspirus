use std::{path::PathBuf, str::FromStr, sync::Arc};

use log::{debug, error};

use crate::{
    backend::{
        config_file::ConfigFrontend,
        downloader,
        utils::{
            self,
            generic::{get_config, update_config},
        },
        yara_scanner::YaraScanner,
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
                Ok(matches) => {
                    debug!("CLI matches state: {matches:?}");
                    matches
                }
                Err(err) => {
                    error!("{}", err);
                    app.handle().exit(1);
                    return Ok(());
                }
            };

            // Iterate over each key and execute functions based on them
            matches.args.iter().for_each(|(key, data)| {
                println!("{}, {:?}", key, data);
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Starts the scanner over the GUI
#[tauri::command]
pub async fn start_scanner(window: tauri::Window, path: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut scanner = YaraScanner::new(Some(Arc::new(window)))?;
        let result = scanner.start(PathBuf::from_str(&path).map_err(|err| err.to_string())?);
        serde_json::to_string_pretty(&result).map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| err.to_string())?
}

// Updates the database over the GUi
#[tauri::command]
pub async fn update() -> Result<(), String> {
    tokio::task::spawn_blocking(|| downloader::update())
        .await
        .map_err(|err| err.to_string())?
        .await
}

// Returns a vector of all attached removable storage drives (USB) -> Unnecessary for the CLI
#[tauri::command]
pub async fn list_usb_drives() -> Result<String, String> {
    utils::usb_utils::list_usb_drives().await
}

// Creates the config from the GUI
#[tauri::command]
pub async fn save_config_fe(contents: Option<String>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let config_received =
            serde_json::from_str::<ConfigFrontend>(&contents.ok_or("Json was none".to_owned())?)
                .map_err(|err| err.to_string())?;
        let mut config = get_config();
        // update received fields
        config_received
            .logging_is_active
            .inspect(|val| config.logging_is_active = *val);
        config_received
            .scan_dir
            .inspect(|val| config.scan_dir = *val);
        config_received
            .min_matches
            .inspect(|val| config.min_matches = *val);
        config_received
            .max_matches
            .inspect(|val| config.max_matches = *val);
        // save updated config
        update_config(config)
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
pub async fn load_config_fe() -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        serde_json::to_string(&get_config())
            .map_err(|err| format!("Failed to convert config to json: {err}"))
    })
    .await
    .map_err(|err| err.to_string())?
}

/// verifies if there are any yara rules present
#[tauri::command]
pub async fn check_update() -> Result<bool, String> {
    tokio::task::spawn_blocking(|| downloader::check_update())
        .await
        .map_err(|err| err.to_string())?
        .await
}

#[tauri::command]
pub async fn download_logs() -> Result<String, String> {
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
    Ok(log_path.to_str().unwrap().to_string())
}

#[tauri::command]
pub async fn rules_version() -> Result<String, String> {
    tokio::task::spawn_blocking(|| get_config().rules_version)
        .await
        .map_err(|err| err.to_string())
}
