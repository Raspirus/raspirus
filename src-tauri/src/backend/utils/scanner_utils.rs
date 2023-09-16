use directories_next::ProjectDirs;
use log::{error, info, warn};
use std::{fs, path::Path};

use crate::backend::{file_scanner, config_file::Config};

pub async fn start_scanner(
    window: Option<tauri::Window>,
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

    let mut fs = match file_scanner::FileScanner::new(&path, db_file_str.as_str(), window) {
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


pub fn sync_start_scanner(
    window: Option<tauri::Window>,
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

    let mut fs = match file_scanner::FileScanner::new(&path, db_file_str.as_str(), window) {
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