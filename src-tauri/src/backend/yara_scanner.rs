use std::{fs::File, path::PathBuf};

use chrono::{DateTime, Local};
use log::{debug, warn};
use yara_x::Rules;

use crate::backend::utils::generic::get_config;

use super::file_log::FileLog;

pub struct TaggedFile {
    pub path: PathBuf,
    pub reason: String,
    pub confidence: f32,
}

pub struct YaraScanner {
    pub rules: Rules,
    pub tags: Vec<TaggedFile>,
    file_log: FileLog,
    tauri_window: Option<tauri::Window>,
    last_percentage: f32,
    analysed: u64,
    skipped: u64,
}

impl YaraScanner {
    /// creates a new scanenr and imports the yara rules
    pub fn new(tauri_window: Option<tauri::Window>) -> Result<Self, String> {
        let yar_path = get_config().yar_location;
        // setup rules
        let reader = File::open(yar_path)
            .map_err(|err| format!("Failed to load yar file: {}", err.to_string()))?;
        let rules = Rules::deserialize_from(reader)
            .map_err(|err| format!("Failed to deserialize yar file: {}", err.to_string()))?;

        // setup file log
        let now: DateTime<Local> = Local::now();
        let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
        let log_str = format!("{}.log", now_str);

        Ok(Self {
            rules,
            tags: Vec::new(),
            tauri_window,
            file_log: FileLog::new(log_str)?,
            last_percentage: 0.0,
            analysed: 0,
            skipped: 0,
        })
    }

    /// Starts the scanner in the specified location
    pub fn start(&self, path: PathBuf) -> Result<String, String> {
        let config = get_config();
        let data_dir = config
            .paths
            .ok_or("No paths set. Is config initialized?".to_owned())?
            .data;

        if !path.exists() {
            return Err("Invalid path".to_owned());
        }
        if path.is_dir() {
            Self::scan_folder();
        } else {
            Self::scan_file();
        }

        //let fs = Scanner::new(db_file_str.as_str(), window)?;
        warn!("Obfuscated mode is: {}", config.obfuscated_is_active);
        let dirty_files = 0; //fs.init(config.obfuscated_is_active, &path)?;
        debug!("Dirty files received: {:?}", dirty_files);
        serde_json::to_string(&dirty_files).map_err(|err| err.to_string())
    }

    /// scans a file
    fn scan_file() -> Result<(), String> {
        Ok(())
    }

    /// scans a folder
    fn scan_folder() -> Result<(), String> {
        Ok(())
    }
}
