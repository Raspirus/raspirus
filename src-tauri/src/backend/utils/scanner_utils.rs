use log::{debug, error, info, warn};
use std::path::Path;

use crate::backend::{config_file::Config, scanner};

static DB_NAME: &str = "signatures.db";

// There are tow equal functions here, one is async and gets called from the GUI to ensure the main thread doesn't stop
// The second one is sync and is called from the CLI. The second one can probably be rewritten

pub async fn start_scanner(window: Option<tauri::Window>, path: String) -> Result<String, String> {
    // Define default values, will be overwritten later
    let config = Config::new().expect("Failed to load config");
    let project_dir = config.program_path.expect("Failed to get program path");
    let program_dir = project_dir.data_dir();

    // Basically checks if a db file path has been set in the Config
    let db_file_str = if !config.db_location.is_empty()
        && Path::new(&config.db_location).to_owned().exists()
        && Path::new(&config.db_location).to_owned().is_file()
    {
        info!("Using specific DB path {}", config.db_location);
        config.db_location
    } else {
        // if not we use the default path
        program_dir.join(DB_NAME).to_string_lossy().to_string()
    };

    // Here we create an instance of the scanner, but don't start it yet
    let fs = match scanner::Scanner::new(db_file_str.as_str(), window) {
        Ok(fs) => fs,
        Err(err) => {
            error!("{}", err);
            return Err(err.to_string());
        }
    };

    // Finally, before starting the scanner, we check if obfuscated mode has been activated or not
    warn!("Obfuscated mode is: {}", config.obfuscated_is_active);
    let dirty_files = match fs.init(config.obfuscated_is_active, &path) {
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
pub fn sync_start_scanner(window: Option<tauri::Window>, path: String) -> Result<String, String> {
    let config = Config::new()?;
    let project_dir = config.program_path.expect("Failed to get program path");
    let program_dir = project_dir.data_dir();

    let db_file_str = if !config.db_location.is_empty()
        && Path::new(&config.db_location).to_owned().exists()
        && Path::new(&config.db_location).to_owned().is_file()
    {
        info!("Using specific DB path {}", config.db_location);
        config.db_location
    } else {
        program_dir.join(DB_NAME).to_string_lossy().to_string()
    };

    let fs = match scanner::Scanner::new(db_file_str.as_str(), window) {
        Ok(fs) => fs,
        Err(err) => {
            error!("{}", err);
            return Err(err.to_string());
        }
    };

    warn!("Obfuscated mode is: {}", config.obfuscated_is_active);
    let dirty_files = match fs.init(config.obfuscated_is_active, &path) {
        Ok(files) => files,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };
    debug!("Dirty files received: {:?}", dirty_files);
    serde_json::to_string(&dirty_files).map_err(|err| err.to_string())
}
