#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::Path, time, fs};

use backend::file_scanner;
use log::{error, info, warn};

mod backend;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_scanner, list_usb_drives])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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

    fs.search_files();
    Ok(())
}

#[tauri::command]
fn list_usb_drives() -> Result<Vec<String>, String> {
    let mut usb_drives = Vec::new();

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

    Ok(usb_drives)
}