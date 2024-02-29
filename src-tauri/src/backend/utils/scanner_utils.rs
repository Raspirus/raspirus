use log::{debug, info, warn};
use std::path::Path;

use crate::backend::{scanner::Scanner, utils::generic::get_config};

// Starts the filescanner
pub fn start_scanner(window: Option<tauri::Window>, path: String) -> Result<String, String> {
    let config = get_config();
    let data_dir = config
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .data;

    let db_path = Path::new(&config.db_location);
    let db_file_str = if !config.db_location.is_empty() && db_path.exists() && db_path.is_file() {
        info!("Using specific DB path {}", config.db_location);
        config.db_location
    } else {
        data_dir.join(crate::DB_NAME).display().to_string()
    };

    let fs = Scanner::new(db_file_str.as_str(), window)?;
    warn!("Obfuscated mode is: {}", config.obfuscated_is_active);
    let dirty_files = fs.init(config.obfuscated_is_active, &path)?;
    debug!("Dirty files received: {:?}", dirty_files);
    serde_json::to_string(&dirty_files).map_err(|err| err.to_string())
}
