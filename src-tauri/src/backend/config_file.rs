use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
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
    // program_path
    #[serde(skip)]
    pub program_path: Option<ProjectDirs>,
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
            program_path: None,
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
        cfg.set_program_path()?;
        cfg.load()?;
        Ok(cfg)
    }

    /// Finds the suitable path for the current system, creates a subfolder for the app and returns
    /// the path as a normal String
    fn set_program_path(&mut self) -> Result<(), String> {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        self.program_path = Some(project_dirs);
        Ok(())
    }

    // OS compliant config path
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
    pub fn save(&mut self) -> Result<(), String> {
        if !Path::new(&Self::get_config_path()).exists() {
            fs::create_dir_all(
                Path::new(&Self::get_config_path())
                    .parent()
                    .expect("Path creation failed"),
            )
            .expect("Failed creating config file");
        }

        let file = File::create(Self::get_config_path()).expect("Failed to open config file");
        serde_json::to_writer_pretty(file, self).map_err(|err| err.to_string())
    }

    /// Loads the current config and returns it, or creates a new one if there is none yet
    pub fn load(&mut self) -> Result<(), String> {
        // Checks if the config file exists, else quickly creates it
        if !Path::new(&Self::get_config_path()).exists() {
            self.save()?;
        };

        let mut file = File::open(Self::get_config_path()).map_err(|err| err.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| format!("Failed to read config to string: {err}"))?;
        let mut config_from_str: Config = serde_json::from_str(&contents)
            .map_err(|err| err.to_string())
            .map_err(|err| format!("Failed deserializing config: {err}"))?;
        config_from_str.set_program_path()?;
        *self = config_from_str;
        Ok(())
    }
}
