use directories_next::ProjectDirs;
use log::{error, info, warn, debug};
use std::{fs, path::Path};

use crate::backend::{file_scanner, config_file::Config};

// There are tow equal functions here, one is async and gets called from the GUI to ensure the main thread doesn't stop
// The second one is sync and is called from the CLI. The second one can probably be rewritten

pub async fn start_scanner(
    window: Option<tauri::Window>,
    path: String
) -> Result<String, String> {
    // Define default values, will be overwritten later
    let db_name = "signatures.db";
    let config = Config::new().load().expect("Couldn't load config");
    let mut db_file_str = config.db_location;

    // Basically checks if a db file path has been set in the Config
    if db_file_str.is_empty() {
        // If no path has been set, we check in the default location for one
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
    } else {
        // Else we use the specified file
        if Path::new(&db_file_str).to_owned().exists() && Path::new(&db_file_str).to_owned().is_file() {
            info!("Using specific DB path {}", db_file_str);
        } else {
            info!("Falling back to default DB file (signatures.db)");
            let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
                .expect("Failed to get project directories.");
            let program_dir = project_dirs.data_dir();
            fs::create_dir_all(program_dir).expect("Failed to create program directory.");
            let db_file_path = program_dir.join(db_name);
            db_file_str = db_file_path.to_string_lossy().to_string();
        }
    }
    // Here we create an instance of the scanner, but don't start it yet
    let mut fs = match file_scanner::FileScanner::new(&path, db_file_str.as_str(), window) {
        Ok(fs) => fs,
        Err(err) => {
            error!("{}", err);
            return Err(err.to_string());
        }
    };
    // Finally, before starting the scanner, we check if obfuscated mode has been activated or not
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
    debug!("Dirty files received: {:?}", dirty_files);
    // For the GUI it is important to return JSON 
    Ok(serde_json::to_string(&dirty_files).expect("Error when trying to parse vector to string"))
}

// Same as above, but in synchron (Should be rewritten to better suit the CLI, probably also renamed)
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
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
    } else if Path::new(&db_file_str).to_owned().exists() && Path::new(&db_file_str).to_owned().is_file() {
        info!("Using specific DB path {}", db_file_str);
    } else {
        info!("Falling back to default DB file (signatures.db)");
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
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
    debug!("Dirty files received: {:?}", dirty_files);
    Ok(serde_json::to_string(&dirty_files).expect("Error when trying to parse vector to string"))
}