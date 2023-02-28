#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use backend::db_ops::DBOps;
use backend::file_scanner;
use log::{error, info, warn };
use serde::{Deserialize, Serialize};
use std::ffi::{OsString, OsStr};
use std::iter::once;
use std::process::exit;
use std::{env, fs, path::Path, time};

#[cfg(windows)]
use std::os::windows::prelude::OsStrExt;
#[cfg(windows)]
use winapi::um::fileapi::GetDriveTypeW;
#[cfg(windows)]
use winapi::um::winbase::DRIVE_REMOVABLE;

mod backend;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_scanner, list_usb_drives, update_database])
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
async fn start_scanner(path: String, dbfile: Option<String>, obfuscated: bool) -> Result<String, String> {
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

    let dirty_files = match fs.search_files(obfuscated) {
        Ok(files) => files,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };
    Ok(serde_json::to_string(&dirty_files).unwrap())

}

#[tauri::command]
fn update_database(db_file: Option<String>) {
    let mut use_db = "signatures.db".to_owned();
    match db_file {
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

    let mut db_connection = match DBOps::new(&use_db) {
        Ok(db_conn) => db_conn,
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    };

    let big_tic = time::Instant::now();
    db_connection.update_db();
    let big_toc = time::Instant::now();
    info!(
        "Updated DB in {} seconds",
        big_toc.duration_since(big_tic).as_secs_f64()
    );
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
        info!("Trying to retrieve USB drives from Windows OS");
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

            #[cfg(windows)]
            let wide_path = OsStr::new(&drive_path).encode_wide().chain(once(0)).collect::<Vec<_>>();
            #[cfg(windows)]
            let drive_type = unsafe { GetDriveTypeW(wide_path.as_ptr()) };

            match fs::metadata(drive_path) {
                Ok(metadata) => {
                    #[cfg(windows)]
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
    } else {
        warn!("Not retrieving USBs -> Wrong OS");
    }
    Ok(serde_json::to_string(&usb_drives).unwrap())
}
