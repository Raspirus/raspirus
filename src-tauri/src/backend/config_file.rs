use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Config {
    // Amount of hashes in the database
    pub hashes_in_db: u32,
    // Last time and date when the db was successfully updated
    pub last_db_update: String,
    // If we should log information to a file
    pub logging_is_active: bool,
    // Check if we should obfuscate the result
    pub obfuscated_is_active: bool,
    // Location of the .db file
    pub db_location: String,
    // If we should scan direcories instead of files (You can only choose one on the current file picker dialog)
    pub scan_dir: bool,
    // List of hashes that should be ignored during scans
    pub ignored_hashes: Vec<String>,
    // mirror to folder with hashfiles for update
    pub mirror: String,
    // various paths in an effort to unify them. are folders expected to be used later
    #[serde(skip)]
    pub paths: Option<Paths>,
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub data: PathBuf,
    pub config: PathBuf,
    pub logs: PathBuf,
    pub cache: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hashes_in_db: 0,
            last_db_update: "Never".to_string(),
            logging_is_active: false,
            obfuscated_is_active: true,
            db_location: "".to_string(),
            scan_dir: true,
            ignored_hashes: Vec::new(),
            mirror: "https://raw.githubusercontent.com/Raspirus/signatures/main/hashes".to_string(),
            paths: None,
        }
    }
}

/// The config file simply holds settings of the application that should perists during reboots
/// The entire config is saved to a JSON file and loaded or created on the first start
/// Default config gets created, then we try to load. If load fails we return default
impl Config {
    pub fn new() -> Result<Self, String> {
        // creates instance of new config
        let mut cfg = Config::default();
        cfg.set_paths()?;
        cfg.load()?;
        Ok(cfg)
    }

    /// Finds the suitable path for the current system, creates a subfolder for the app and returns
    /// the path as a normal String
    fn set_paths(&mut self) -> Result<(), String> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let dirs =
            ProjectDirs::from("com", "Raspirus", "Raspirus").ok_or("Failed to get projectdir".to_owned())?;
        #[cfg(target_os = "windows")]
        let dirs =
            ProjectDirs::from("com", "Raspirus", "").ok_or("Failed to get projectdir".to_owned())?;
            
        // RoamingData
        let data = dirs.data_dir().to_owned();
        let logs = data.to_owned().join("logs");

        // LocalData
        let config = dirs.config_dir().to_owned();
        let cache = dirs.cache_dir().to_owned();

        // create all paths
        if !data.exists() {
            fs::create_dir_all(&data).map_err(|err| err.to_string())?;
        }

        if !logs.exists() {
            fs::create_dir_all(&logs).map_err(|err| err.to_string())?;
        }

        if !config.exists() {
            fs::create_dir_all(&config).map_err(|err| err.to_string())?;
        }

        if !cache.exists() {
            fs::create_dir_all(&cache).map_err(|err| err.to_string())?;
        }

        self.paths = Some(Paths {
            data,
            config,
            logs,
            cache,
        });
        Ok(())
    }

    /// Will save the current configuration to the file
    /// WARNING! If the fields are blank, it will clear the current config
    pub fn save(&self) -> Result<(), String> {
        let path = self
            .paths
            .clone()
            .ok_or("Could not get config path".to_owned())?
            .config;
        if !path.exists() {
            fs::create_dir_all(&path)
                .map_err(|err| format!("Failed to create config file: {err}"))?;
        }

        let file = File::create(path.join(crate::CONFIG_FILENAME))
            .map_err(|err| format!("Failed to write config file: {err}"))?;
        serde_json::to_writer_pretty(file, self).map_err(|err| err.to_string())
    }

    /// Loads the current config and returns it, or creates a new one if there is none yet
    pub fn load(&mut self) -> Result<(), String> {
        let path = self
            .paths
            .clone()
            .ok_or("Could not get config path".to_owned())?
            .config
            .join(crate::CONFIG_FILENAME);
        // Checks if the config file exists, else quickly creates it
        if !path.exists() {
            self.save()?;
        };

        let mut file =
            File::open(path).map_err(|err| err.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| format!("Failed to read config to string: {err}"))?;
        let mut config_from_str: Config = serde_json::from_str(&contents)
            .map_err(|err| err.to_string())
            .map_err(|err| format!("Failed deserializing config: {err}"))?;

        config_from_str.set_paths()?;
        *self = config_from_str;
        Ok(())
    }
}
