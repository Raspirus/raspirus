use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // Amount of hashes in the database
    pub hashes_in_db: u32,
    // Last time and date when the db was successfully updated
    pub last_db_update: String,
    // If we should log information to a file
    pub logging_is_active: bool,
    // Check if we should obfuscate the result
    pub obfuscated_is_active: bool,
    // Automatic updates: Set weekday
    pub db_update_weekday: i32,
    // Automatic update: Set time
    pub db_update_time: String,
    // Location of the .db file
    pub db_location: String,
    // If we should scan direcories instead of files (You can only choose one on the current file picker dialog)
    pub scan_dir: bool,
    // List of hashes that should be ignored during scans
    pub ignored_hashes: Vec<String>,
    // config path
    #[serde(skip)]
    program_path: String,
}

/// The config file simply holds settings of the application that should perists during reboots
/// The entire config is saved to a JSON file and loaded or created on the first start
impl Config {
    pub fn new() -> Result<Self, String> {
        let mut cfg = Config {
            hashes_in_db: 0,
            last_db_update: "Never".to_string(),
            logging_is_active: false,
            obfuscated_is_active: true,
            db_update_weekday: -1,
            db_update_time: "22:00:00".to_string(),
            db_location: "".to_string(),
            scan_dir: true,
            ignored_hashes: Vec::new(),
            program_path: String::new(),
        };
        cfg.set_path()?;
        Ok(cfg)
    }

    /// Finds the suitable path for the current system, creates a subfolder for the app and returns
    /// the path as a normal String
    fn set_path(&mut self) -> Result<(), String> {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        let conf_file_path = program_dir.join("raspirus.config.json");
        self.program_path = conf_file_path
            .to_str()
            .expect("Failed to get config path")
            .to_owned();
        Ok(())
    }

    /// Will save the current configuration to the file
    /// WARNING! If the fields are blank, it will clear the current config
    pub fn save(&self) -> Result<(), String> {
        let file = File::create(self.program_path.clone()).expect("Failed creating path");
        serde_json::to_writer_pretty(file, self).map_err(|err| err.to_string())
    }

    /// Loads the current config and returns it, or creates a new one if there is non yet
    pub fn load(&self) -> Result<Self, String> {
        // Checks if the config file exists, else quickly creates it
        if !Path::new(&self.program_path).exists() {
            self.save()?;
        };

        let mut file = File::open(self.program_path.clone()).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed reading config to string");
        let mut config_from_str: Config = serde_json::from_str(&contents)
            .map_err(|err| err.to_string())
            .expect("Failed deserializing config");
        config_from_str.set_path()?;
        Ok(config_from_str)
    }
}
