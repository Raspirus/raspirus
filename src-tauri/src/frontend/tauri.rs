use log::error;

use crate::backend::{
    config_file::ConfigFrontend,
    utils::{
        self,
        generic::{get_config, update_config},
    },
};
use tauri_plugin_cli::CliExt;

use super::functions::{cli_dbupdate, cli_gui, not_implemented};

pub fn init_tauri() {
    // Builds the Tauri connection
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            // Default to GUI if the app was opened with no CLI args.
            if std::env::args_os().count() <= 1 {
                cli_gui(app.handle().clone())?;
            }
            // Else, we start in CLI mode and parse the given parameters
            let matches = match app.cli().matches() {
                Ok(matches) => {
                    println!("{matches:?}");
                    matches
                }
                Err(err) => {
                    error!("{}", err);
                    app.handle().exit(1);
                    return Ok(());
                }
            };
            /*
            // Iterate over each key and execute functions based on them
            matches.args.iter().for_each(|(key, data)| {
                if data.occurrences > 0 || key.as_str() == "help" || key.as_str() == "version" {
                    // Define all CLI commands/arguments here and in the tauri.conf.json file
                    // WARNING: If the commmand is not defined in the tauri.conf.json file, it can't be used here
                    match key.as_str() {
                        "gui" => {
                            if let Err(err) = cli_gui(app.handle().clone()) {
                                error!("GUI Error: {}", err);
                            }
                        }
                        "scan" => cli_scanner(app.handle().clone(), data),
                        "update-db" => cli_dbupdate(app.handle().clone()),
                        "help" => print_data(app.handle().clone(), data),
                        "version" => print_data(app.handle().clone(), data),
                        _ => not_implemented(app.handle().clone()),
                    }
                }
            });
            */
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
            .obfuscated_is_active
            .inspect(|val| config.obfuscated_is_active = *val);
        config_received
            .db_location
            .inspect(|val| config.db_location = val.clone());
        config_received
            .scan_dir
            .inspect(|val| config.scan_dir = *val);
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

    /*
    let downloads_dir = Manager::path(Path::new("."))
        .download_dir()
        .expect("Failed to get download directory");
        */
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
pub async fn get_hash_count_fe() -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        // read hash count from config to avoid long waits on count
        let config = get_config();
        Ok(config.hash_count.to_string())
    })
    .await
    .map_err(|err| err.to_string())?
}
