use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

/// The config file simply holds settings of the application that should perists during reboots
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Amount of hashes in the database
    pub hashes_in_db: u32,
    /// Last time and date when the db was successfully updated
    pub last_db_update: String,
    /// If we should log information to a file
    pub logging_is_active: bool,
    /// Check if we should obfuscate the result
    pub obfuscated_is_active: bool,
    /// Automatic updates: Set weekday
    pub db_update_weekday: i32,
    /// Automatic update: Set time
    pub db_update_time: String,
    /// Location of the .db file
    pub db_location: String,
    /// If we should scan direcories instead of files (You can only choose one on the current file picker dialog)
    pub scan_dir: bool,
    /// List of hashes that should be ignored during scans
    pub ignored_hashes: Vec<String>,
    /// stores the different dirs as a struct
    #[serde(skip)]
    pub project_dirs: Dirs
}

/// Stores directories as a struct for easier access
#[derive(Debug, Default)]
pub struct Dirs {
    pub logs: Logs,
    pub data: Box<PathBuf>,
}

/// Stores the different log paths as a struct for easier access
#[derive(Debug, Default)]
pub struct Logs {
    pub main: Box<PathBuf>,
    pub scan: Box<PathBuf>,
    pub update: Box<PathBuf>,
}

/// The config file simply holds settings of the application that should perists during reboots
/// The entire config is saved to a JSON file and loaded or created on the first start
impl Config {
    pub fn new() -> Result<Self, String> {
        // creates instance of new config
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
            project_dirs: Dirs::default()
        };
        cfg.set_program_path()?;
        cfg.load()?;
        Ok(cfg)
    }

    /// Finds the suitable path for the current system, creates a subfolder for the app
    fn set_program_path(&mut self) -> Result<(), String> {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Raspirus")
            .expect("Failed to get project directories.");
        
        self.project_dirs = Dirs {
            logs: Logs {
                main: Box::new(project_dirs.data_local_dir().join("logs").join("main")),
                scan: Box::new(project_dirs.data_local_dir().join("logs").join("scan")),
                update: Box::new(project_dirs.data_local_dir().join("logs").join("update")),
            },
            data: Box::new(project_dirs.data_dir().join("data")),
        };
        Ok(())
    }

    /// Creates the directories for the logs and data
    /// If they already exist, it will do nothing
    pub fn create_dirs(&self) -> std::io::Result<()> {
        // create logs
        fs::create_dir_all(self.project_dirs.logs.main.as_path())?;
        fs::create_dir_all(self.project_dirs.logs.scan.as_path())?;
        fs::create_dir_all(self.project_dirs.logs.update.as_path())?;
        // create data folders
        fs::create_dir_all(self.project_dirs.data.as_path())?;
        Ok(())
    }

    /// OS compliant config path getter
    pub fn get_config_path() -> String {
        ProjectDirs::from("com", "Raspirus", "Raspirus")
            .expect("Failed to get project directories")
            .config_dir()
            .join("raspirus.config.json")
            .to_str()
            .expect("Failed to get config path")
            .to_owned()
    }

    /// Will save the current configuration to the file
    /// WARNING! If the fields are blank, it will clear the current config
    /// Since the user is not supposed to edit the config file, this should not be a problem
    pub fn save(&mut self) -> Result<(), String> {
        if !Path::new(&Self::get_config_path()).exists() {
            fs::create_dir_all(
                Path::new(&Self::get_config_path())
                    .parent()
                    .expect("Path creation failed"))
            .expect("Failed creating config file");
        }
        
        let file = File::create(Self::get_config_path()).expect("Failed to open config file");
        serde_json::to_writer_pretty(file, self).map_err(|err| err.to_string())
    }

    /// Loads the current config and returns it, or creates a new one if there is non yet
    pub fn load(&mut self) -> Result<(), String> {
        // Checks if the config file exists, else quickly creates it
        if !Path::new(&Self::get_config_path()).exists() {
            self.save()?;
        };

        let mut file = File::open(Self::get_config_path()).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed reading config to string");
        let mut config_from_str: Config = serde_json::from_str(&contents)
            .map_err(|err| err.to_string())
            .expect("Failed deserializing config");
        config_from_str.set_program_path()?;
        *self = config_from_str;
        Ok(())
    }
}
