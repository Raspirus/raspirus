// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config_file::Config;
use backend::utils;
use directories_next::ProjectDirs;
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};
use tauri::api::cli::ArgData;
use std::fs::File;
use std::{env, fs};

mod backend;
mod tests;

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
            eprintln!("{}", err);
            app.handle().exit(1);
            return Ok(());
        }
    };

    for (key, data) in matches.args {
        if data.occurrences > 0 || key.as_str() == "help" || key.as_str() == "version" {
            // Define all CLI commands/arguments here and in the tauri.conf.json file
            match key.as_str() {
                "gui" => {
                    if let Err(err) = cli_gui(app.handle()) {
                        eprintln!("GUI Error: {}", err);
                        // Handle the error as needed
                    }
                }
                "example" => cli_example(app.handle()),
                "scanfile" => cli_scanner(app.handle(), data),
                "help" => print_data(app.handle(), data),
                "version" => print_data(app.handle(), data),
                _ => print_data(app.handle(), data),
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

fn print_data(app: tauri::AppHandle, data: ArgData) {
    if let Some(json_str) = data.value.as_str() {
        let unescaped_str = json_str.replace("\\n", "\n").replace("\\t", "\t");
        println!("{}", unescaped_str);
        app.exit(0);
    } else {
        // Handle the case where data.value is not a string
        eprintln!("data.value is not a string");
        app.exit(1);
    }
}

fn cli_gui(app: tauri::AppHandle) -> Result<(), tauri::Error> {
  println!("showing gui");
  #[cfg(all(not(debug_assertions), windows))]
  remove_windows_console();
  tauri::WindowBuilder::new(&app, "raspirus", tauri::WindowUrl::App("index.html".into()))
    .title("Raspirus")
    .inner_size(800., 480.)
    .resizable(true)
    .build()?;
  println!("this won't show on Windows release builds");
  Ok(())
}


fn cli_scanner(app: tauri::AppHandle, data: ArgData) {
    if let Some(json_str) = data.value.as_str() {
        let unescaped_str = json_str.replace("\\n", "\n").replace("\\t", "\t");
        println!("{}", unescaped_str);
        match utils::scanner_utils::sync_start_scanner(None, unescaped_str) {
            Ok(res) => {
                println!("Result: {res}");
                app.exit(0);
            },
            Err(err) => {
                eprintln!("Error: {err}");
                app.exit(1);
            },
        }
    } else {
        // Handle the case where data.value is not a string
        eprintln!("data.value is not a string");
        app.exit(1);
    }
}

fn cli_example(app: tauri::AppHandle) {
  println!("sleeping for example");
  std::thread::sleep(std::time::Duration::from_secs(5));
  app.exit(0);
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
    utils::scanner_utils::start_scanner(Some(window), path).await
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
