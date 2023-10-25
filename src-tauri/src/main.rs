// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config_file::Config;
use backend::utils;
use directories_next::ProjectDirs;
use log::{error, info, LevelFilter, debug, warn};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};
use tauri::api::cli::ArgData;
use std::fs::File;
use std::fs;

mod backend;
mod tests;

// NOTE: All functions with #[tauri::command] can and will be called from the GUI
// Their name should not be changed and any new functions should return JSON data
// using serde parsing

fn main() {
    // We immediatley try to load the config at startup, or create a new one. The config defines the application states
    let mut config = Config::new();
    config = config.load().expect("Couldn't load config at startup");
    let write_to_file = config.logging_is_active;
    
    // We check if we should log the application messages to a file or not, default is yes. Defined in the Config
    if write_to_file {
        // We use ProjectDirs to find a suitable location for our logging file
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Logs")
            .expect("Failed to get project directories.");
        let log_dir = project_dirs.data_local_dir().join("main"); // Create a "main" subdirectory

        // If we are able to create both the file and directory path, we can start the FileLogger
        match fs::create_dir_all(&log_dir) {
            Ok(_) => {
                // Create a new file with the given name. Will overwrite the old/existisng file
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
                        
                        // Terminal logger is used if for development
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
    for (key, data) in matches.args {
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
                _ => not_done(app.handle()),
            }
        }
    }    
    Ok(())
    })
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


#[cfg(all(not(debug_assertions), windows))]
fn remove_windows_console() {
  unsafe {
    windows_sys::Win32::System::Console::FreeConsole();
  }
}
// Basically prints the given data with \n and \t correctly formatted
fn print_data(app: tauri::AppHandle, data: ArgData) {
    if let Some(json_str) = data.value.as_str() {
        let unescaped_str = json_str.replace("\\n", "\n").replace("\\t", "\t");
        debug!("{}", unescaped_str);
        app.exit(0);
    } else {
        // Handle the case where data.value is not a string
        error!("data.value is not a string");
        app.exit(1);
    }
}

// If a command is not yet implemented
fn not_done(app: tauri::AppHandle) {
    warn!("Function not implemented yet");
    app.exit(2);
}

// Starts the GUI without attaching a CLI
fn cli_gui(app: tauri::AppHandle) -> Result<(), tauri::Error> {
  debug!("showing gui");
  #[cfg(all(not(debug_assertions), windows))]
  remove_windows_console();
  tauri::WindowBuilder::new(&app, "raspirus", tauri::WindowUrl::App("index.html".into()))
    .title("Raspirus")
    .inner_size(800., 480.)
    .resizable(true)
    .build()?;
  debug!("this won't show on Windows release builds");
  Ok(())
}

// Starts the scanner on the CLI
fn cli_scanner(app: tauri::AppHandle, data: ArgData) {
    if let Some(json_str) = data.value.as_str() {
        let unescaped_str = json_str.replace("\\n", "\n").replace("\\t", "\t");
        debug!("Data provided: {}", unescaped_str);
        match utils::scanner_utils::sync_start_scanner(None, unescaped_str) {
            Ok(res) => {
                info!("Result: {res}");
                app.exit(0);
            },
            Err(err) => {
                error!("Error: {err}");
                app.exit(1);
            },
        }
    } else {
        // Handle the case where data.value is not a string
        error!("data.value is not a string");
        app.exit(1);
    }
}

// Updates the DB over the CLI
fn cli_dbupdate(app: tauri::AppHandle) {
    match utils::update_utils::sync_update_database(None) {
        Ok(res) => {
            info!("Result: {res}");
            app.exit(0);
        },
        Err(err) => {
            error!("Error: {err}");
            app.exit(1);
        },
    }
}

// Starts the scanner over the GUI
#[tauri::command]
async fn start_scanner(window: tauri::Window, path: String) -> Result<String, String> {
    utils::scanner_utils::start_scanner(Some(window), path).await
}

// Checks if we are currently on a Raspberry Pi, 
// because a couple options are not supported on that device and will be disabled on the GUI
#[tauri::command]
async fn check_raspberry() -> Result<bool, String> {
    let arch = std::env::consts::ARCH;

    if arch == "arm" {
        Ok(true)
    } else {
        Ok(false)
    }
}

// Updates the database over the GUi
#[tauri::command]
async fn update_database(window: tauri::Window) -> Result<String, String> {
    utils::update_utils::update_database(Some(window)).await
}

// Returns a vector of all attached removable storage drives (USB) -> Unnecessary for the CLI
#[tauri::command]
async fn list_usb_drives() -> Result<String, String> {
    utils::usb_utils::list_usb_drives().await
}


// Creates the config from the GUI
#[tauri::command]
async fn create_config(contents: Option<String>) -> Result<String, String> {
    let config = if let Some(contents) = contents {
        serde_json::from_str(&contents).map_err(|err| err.to_string())?
    } else {
        Config::new().load().expect("Error while loading config")
    };

    config.save().map_err(|err| err.to_string())?;
    let config_str = serde_json::to_string(&config).expect("Issue with transforming congig to Serde string");

    Ok(config_str)
}

// Not yet implemented
pub async fn auto_update_scheduler(tauri_win: tauri::Window, hour: i32, weekday: i32) {
    // ISSUE: Needs to restart app to apply new update schedule
    utils::update_utils::auto_update_scheduler(Some(tauri_win), hour, weekday).await
}
