#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::Path, time, fs, thread, env, path::PathBuf};
use backend::file_scanner;
use winapi::um::fileapi;
use log::{error, info, warn};

mod backend;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_scanner, list_usb_drives])
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
fn start_scanner(path: String, update: bool, dbfile: Option<String>) -> Result<(), String> {
    match pretty_env_logger::try_init() {
        Ok(()) => {
            info!("Logger initialized!");
        }
        Err(err) => {
            warn!("Failed initializing logger: {err}");
        }
    }

    let mut use_db = "signatures.db".to_owned();
    match dbfile {
        Some(fpath) => {
            if Path::new(&fpath).to_owned().exists() && Path::new(&fpath).to_owned().is_file() {
                info!("Using specific DB path {}", fpath);
                use_db = fpath.to_owned();
            } else {
                info!("Falling back to default DB file (signatures.db)");
            }
        }
        None => {
            info!("Path is None; Falling back to default DB file (signatures.db)");
        }
    };

    let mut fs = match file_scanner::FileScanner::new(&path, &use_db) {
        Ok(fs) => fs,
        Err(err) => {
            error!("{}", err);
            return Err(err.to_string());
        }
    };
    if update {
        let big_tic = time::Instant::now();
        fs.db_conn.update_db();
        let big_toc = time::Instant::now();
        info!(
            "Updated DB in {} seconds",
            big_toc.duration_since(big_tic).as_secs_f64()
        );
    } else {
        info!("Skipped update");
    }
    thread::spawn(move || fs.search_files()).join().unwrap();
    Ok(())
}

/// Lists the USB drives attached to the system.
///
/// # Returns
///
/// A `Result` object containing a vector of strings representing the paths to the USB drives, or an `Err` with an error message if an error occurred.
#[tauri::command]
fn list_usb_drives() -> Result<Vec<String>, String> {

    match pretty_env_logger::try_init() {
        Ok(()) => {
            info!("Logger initialized!");
        }
        Err(err) => {
            warn!("Failed initializing logger: {err}");
        }
    }

    let mut usb_drives = Vec::new();

    if cfg!(target_os = "windows") {
        info!("Trying to retrieve USB drives from Windows OS");
        let system_drive = env::var_os("SystemDrive").unwrap_or_default();
        let drives = env::split_paths(&system_drive);
        let drivez = env::split_paths(&system_drive).collect::<Vec<_>>();
        info!("Drives: {drivez:?}");
        for drive in drives {
            let drive_str = drive.to_str().unwrap();
            info!("Drive found: {drive_str:?}");
            let entries = match fs::read_dir(drive_str) {
                Ok(entries) => entries,
                Err(err) => {
                    return Err(err.to_string());
                }
            };

            for entry in entries {
                info!("Entries: {entry:?}");
                let entry = entry.expect("I couldn't read something inside the directory");
                let path = entry.path();
                if path.is_dir() && path.starts_with(drive_str) {
                    let mut path_buf = PathBuf::new();
                    path_buf.push(path);
                    let drive_type = unsafe {
                        let mut drive = path_buf.to_str().unwrap().encode_utf16().collect::<Vec<u16>>();
                        drive.push(0);
                        fileapi::GetDriveTypeW(drive.as_ptr())
                    };
                    if drive_type == winapi::um::winbase::DRIVE_REMOVABLE {
                        usb_drives.push(path_buf.to_str().unwrap().to_owned());
                    }
                }
            }
        }
    } else if cfg!(target_os = "linux") {
        info!("Trying to retrieve USB drives from Linux OS");
        let entries = match fs::read_dir("/dev") {
            Ok(entries) => entries,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        for entry in entries {
            let entry = entry.expect("I couldn't read something inside the directory");
            let path = entry.path();
            if path.starts_with("/dev/sd") || path.starts_with("/dev/mmc") {
                usb_drives.push(path.to_string_lossy().into_owned());
            }
        }
    }

    Ok(usb_drives)
}