#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::Path, time, fs, thread, env};
use backend::file_scanner;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

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
fn list_usb_drives() -> Result<String, String> {

    match pretty_env_logger::try_init() {
        Ok(()) => {
            info!("Logger initialized!");
        }
        Err(err) => {
            warn!("Failed initializing logger: {err}");
        }
    }

    let mut usb_drives = Vec::new();

    if cfg!(target_os = "linux") {
        info!("Trying to retrieve USB drives from Linux OS");
        // WARNING! Username pi is hardcoded here!
        let entries = match fs::read_dir("/media/pi") {
            Ok(entries) => entries,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        for entry in entries {
            let entry = entry.expect("I couldn't read something inside the directory");
            let path = entry.path();

            usb_drives.push(UsbDevice {
                name: entry.file_name().into_string().expect("File name is strange"), 
                path: path.as_path().to_str().expect("Path is strange").to_string()
            });
        }
    } else {
        warn!("Not retrieving USBs -> Wrong OS");
    }
    Ok(serde_json::to_string(&usb_drives).unwrap())
}