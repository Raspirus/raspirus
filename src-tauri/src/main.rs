#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use backend::config_file::Config;
use backend::db_ops::DBOps;
use backend::file_scanner;
use directories_next::ProjectDirs;
use log::{error, info, warn, LevelFilter};
use serde::{Deserialize, Serialize};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::process::exit;
use std::{env, fs, path::Path, time};

mod backend;
mod tests;

fn main() {
    let mut config = Config::new();
    config.save().expect("Couldn't write config to system");
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
async fn start_scanner(
    window: tauri::Window,
    path: String
) -> Result<String, String> {
    let db_name = "signatures.db";
    let config = Config::new().load().expect("Couldn't load config");
    let mut db_file_str = config.db_location;

    if db_file_str.is_empty() {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(&program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
    } else {
        if Path::new(&db_file_str).to_owned().exists() && Path::new(&db_file_str).to_owned().is_file() {
            info!("Using specific DB path {}", db_file_str);
        } else {
            info!("Falling back to default DB file (signatures.db)");
            let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
                .expect("Failed to get project directories.");
            let program_dir = project_dirs.data_dir();
            fs::create_dir_all(&program_dir).expect("Failed to create program directory.");
            let db_file_path = program_dir.join(db_name);
            db_file_str = db_file_path.to_string_lossy().to_string();
        }
    }

    let mut fs = match file_scanner::FileScanner::new(&path, db_file_str.as_str(), Some(window)) {
        Ok(fs) => fs,
        Err(err) => {
            error!("{}", err);
            return Err(err.to_string());
        }
    };
    let mut config = Config::new();
    config = config.load().expect("Unable to load config");
    let obfuscated = config.obfuscated_is_active;
    warn!("Obfuscated mode is: {}", obfuscated);
    let dirty_files = match fs.search_files(obfuscated) {
        Ok(files) => files,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };
    println!("Dirty files received: {:?}", dirty_files);
    Ok(serde_json::to_string(&dirty_files).expect("Error when trying to parse vector to string"))
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
    let db_name = "signatures.db";
    let config = Config::new().load().expect("Couldn't load config");
    let mut db_file_str = config.db_location;

    if db_file_str.is_empty() {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(&program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
    } else {
        if Path::new(&db_file_str).to_owned().exists() && Path::new(&db_file_str).to_owned().is_file() {
            info!("Using specific DB path {}", db_file_str);
        } else {
            info!("Falling back to default DB file (signatures.db)");
            let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
                .expect("Failed to get project directories.");
            let program_dir = project_dirs.data_dir();
            fs::create_dir_all(&program_dir).expect("Failed to create program directory.");
            let db_file_path = program_dir.join(db_name);
            db_file_str = db_file_path.to_string_lossy().to_string();
        }
    }

    let mut db_connection = match DBOps::new(db_file_str.as_str(), Some(window)) {
        Ok(db_conn) => db_conn,
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    };

    let big_tic = time::Instant::now();
    match tokio::task::spawn_blocking(move || {
        let hash_count;
        match db_connection.update_db() {
            Ok(res) => {
                hash_count = res;
            }
            Err(err) => {
                error!("{err}");
                exit(-1);
            }
        }
        hash_count
    })
    .await
    {
        Ok(res) => {
            let big_toc = time::Instant::now();
            info!(
                "Updated DB in {} seconds",
                big_toc.duration_since(big_tic).as_secs_f64()
            );
            Ok(serde_json::to_string(&res).unwrap_or_default())
        }
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UsbDevice {
    name: String,
    path: String,
}

/// Lists the USB drives attached to the system.
///
/// # Returns
///
/// A `Result` object containing a vector of strings representing the paths to the USB drives, or an `Err` with an error message if an error occurred.
#[tauri::command]
async fn list_usb_drives() -> Result<String, String> {
    let mut usb_drives = Vec::new();

    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        info!("Trying to retrieve USB drives from Unix-like OS");
        let username = match env::var("USER") {
            Ok(val) => val,
            Err(_) => panic!("Could not get current username"),
        };

        let dir_path = format!("/media/{}", username);
        let entries = match fs::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        for entry in entries {
            let entry = entry.expect("I couldn't read something inside the directory");
            let path = entry.path();

            usb_drives.push(UsbDevice {
                name: entry
                    .file_name()
                    .into_string()
                    .expect("File name is strange"),
                path: path
                    .as_path()
                    .to_str()
                    .expect("Path is strange")
                    .to_string(),
            });
        }
    } else if cfg!(target_os = "windows") {
        #[cfg(windows)]
        let mut win_usb_drives = list_usb_windows();
        #[cfg(windows)]
        usb_drives.append(&mut win_usb_drives);
    } else {
        warn!("Not retrieving USBs -> Wrong OS");
    }
    Ok(serde_json::to_string(&usb_drives).unwrap())
}

#[cfg(windows)]
fn list_usb_windows() -> Vec<UsbDevice> {
    use std::ffi::{OsStr, OsString};
    use std::iter::once;
    use std::os::windows::prelude::OsStrExt;
    use winapi::um::fileapi::GetDriveTypeW;
    use winapi::um::winbase::DRIVE_REMOVABLE;

    info!("Trying to retrieve USB drives from Windows OS");
    let mut usb_drives = Vec::new();
    let drive_letters: Vec<OsString> = vec![
        OsString::from("A"),
        OsString::from("B"),
        OsString::from("C"),
        OsString::from("D"),
        OsString::from("E"),
        OsString::from("F"),
        OsString::from("G"),
        OsString::from("H"),
        OsString::from("I"),
        OsString::from("J"),
        OsString::from("K"),
        OsString::from("L"),
        OsString::from("M"),
        OsString::from("N"),
        OsString::from("O"),
        OsString::from("P"),
        OsString::from("Q"),
        OsString::from("R"),
        OsString::from("S"),
        OsString::from("T"),
        OsString::from("U"),
        OsString::from("V"),
        OsString::from("W"),
        OsString::from("X"),
        OsString::from("Y"),
        OsString::from("Z"),
    ];
    for letter in drive_letters {
        let drive_path = letter.clone().into_string().unwrap() + ":\\";
        let drive_path = Path::new(&drive_path);
        let drive_name = drive_path.file_name().unwrap_or_default();
        let drive_path = drive_path.to_str().unwrap();
        let wide_path = OsStr::new(&drive_path)
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<_>>();
        let drive_type = unsafe { GetDriveTypeW(wide_path.as_ptr()) };

        match fs::metadata(drive_path) {
            Ok(metadata) => {
                if metadata.is_dir() && drive_type == DRIVE_REMOVABLE {
                    info!("Found Drive: {}", drive_path);
                    usb_drives.push(UsbDevice {
                        name: drive_path.to_string() + " " + &drive_name.to_string_lossy(),
                        path: drive_path.to_string(),
                    });
                }
            }
            Err(_) => {}
        }
    }
    return usb_drives;
}

#[tauri::command]
async fn create_config(contents: Option<String>) -> Result<String, String> {
    let config = if let Some(contents) = contents {
        serde_json::from_str(&contents).map_err(|err| err.to_string())?
    } else {
        Config::new().load().map_err(|err| err.to_string())?
    };

    config.save().map_err(|err| err.to_string())?;
    let config_str = serde_json::to_string(&config).unwrap();

    Ok(config_str)
}
