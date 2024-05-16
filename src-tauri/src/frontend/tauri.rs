use std::path::Path;

use log::{error, info};

use crate::backend::{
    config_file::Config,
    db_ops::DBOps,
    utils::{
        self,
        generic::{get_config, update_config},
    },
};

use super::functions::{cli_dbupdate, cli_gui, cli_scanner, not_implemented, print_data};

pub fn init_tauri() {
    // Builds the Tauri connection
    tauri::Builder::default()
        .setup(|app| {
            // Default to GUI if the app was opened with no CLI args.
            if std::env::args_os().count() <= 1 {
                cli_gui(app.handle())?;
            }
            // Else, we start in CLI mode and parse the given parameters
            let matches = match app.get_cli_matches() {
                Ok(matches) => matches,
                Err(err) => {
                    error!("{}", err);
                    app.handle().exit(1);
                    return Ok(());
                }
            };
            // Iterate over each key and execute functions based on them
            matches.args.iter().for_each(|(key, data)| {
                if data.occurrences > 0 || key.as_str() == "help" || key.as_str() == "version" {
                    // Define all CLI commands/arguments here and in the tauri.conf.json file
                    // WARNING: If the commmand is not defined in the tauri.conf.json file, it can't be used here
                    match key.as_str() {
                        "gui" => {
                            if let Err(err) = cli_gui(app.handle()) {
                                error!("GUI Error: {}", err);
                            }
                        }
                        "scan" => cli_scanner(app.handle(), data),
                        "update-db" => cli_dbupdate(app.handle()),
                        "help" => print_data(app.handle(), data),
                        "version" => print_data(app.handle(), data),
                        _ => not_implemented(app.handle()),
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_scanner,
            list_usb_drives,
            update_database,
            patch,
            check_raspberry,
            load_config_fe,
            save_config_fe,
            download_logs,
            check_update,
            get_hash_count_fe
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Starts the scanner over the GUI
#[tauri::command]
pub async fn start_scanner(window: tauri::Window, path: String) -> Result<String, String> {
    tokio::task::spawn_blocking(|| utils::scanner_utils::start_scanner(Some(window), path))
        .await
        .map_err(|err| err.to_string())?
}

// Checks if we are currently on a Raspberry Pi,
// because a couple options are not supported on that device and will be disabled on the GUI
#[tauri::command]
pub async fn check_raspberry() -> Result<bool, String> {
    Ok(std::env::consts::ARCH == "arm")
}

// Updates the database over the GUi
#[tauri::command]
pub async fn update_database(window: tauri::Window) -> Result<String, String> {
    tokio::task::spawn_blocking(|| utils::update_utils::update(Some(window)))
        .await
        .map_err(|err| err.to_string())?
}

#[tauri::command]
pub async fn patch(patchfile: String) -> Result<(usize, usize, usize), String> {
    tokio::task::spawn_blocking(move || utils::update_utils::patch(&patchfile))
        .await
        .map_err(|err| err.to_string())?
        .map_err(|err| err.to_string())
}

// Returns a vector of all attached removable storage drives (USB) -> Unnecessary for the CLI
#[tauri::command]
pub async fn list_usb_drives() -> Result<String, String> {
    utils::usb_utils::list_usb_drives().await
}

// Creates the config from the GUI
#[tauri::command]
pub fn save_config_fe(contents: Option<String>) -> Result<(), String> {
    let mut config = serde_json::from_str::<Config>(&contents.ok_or("Json was none".to_owned())?)
        .map_err(|err| err.to_string())?;
    config.paths = get_config().paths;
    update_config(config)
}

#[tauri::command]
pub fn load_config_fe() -> Result<String, String> {
    serde_json::to_string(&get_config())
        .map_err(|err| format!("Failed to convert config to json: {err}"))
}

#[tauri::command]
pub async fn check_update() -> Result<bool, String> {
    tokio::task::spawn_blocking(utils::update_utils::check_update_necessary)
        .await
        .map_err(|err| err.to_string())?
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn download_logs() -> Result<String, String> {
    let log_dir = get_config()
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .logs
        .join("main");
    let app_log_path = log_dir.join("app.log");

    let downloads_dir =
        tauri::api::path::download_dir().ok_or("Failed to get download directory".to_string())?;

    let destination_path = downloads_dir.join("log.txt");

    // If there's an error during copying, return an error message
    std::fs::copy(app_log_path, &destination_path)
        .map_err(|err| format!("Error copying log file: {err}"))?;
    // If the copy operation is successful, return Ok indicating success
    Ok(destination_path.to_str().unwrap().to_string())
}

#[tauri::command]
pub fn get_hash_count_fe() -> Result<String, String> {
    // try to get a usable database path
    let config = get_config();
    let data_dir = config
        .clone()
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .data;
    let db_path = Path::new(&config.db_location);
    let db_file_str = if !config.db_location.is_empty() && db_path.exists() && db_path.is_file() {
        info!("Using specific DB path {}", config.db_location);
        config.db_location.clone()
    } else {
        // if not we use the default path
        data_dir.join(crate::DB_NAME).display().to_string()
    };

    // connect to database
    let db_connection = DBOps::new(db_file_str.as_str()).map_err(|err| {
        error!("{err}");
        err.to_string()
    })?;
    Ok(db_connection.count_hashes().map_err(|err| err.to_string())?.to_string())
}
