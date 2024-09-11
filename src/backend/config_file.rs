use directories_next::{ProjectDirs, UserDirs};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

use crate::APPLICATION_LOG;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Config {
    /// contains the config version
    #[serde(default = "default_config")]
    pub config_version: String,
    /// Last time and date when the db was successfully updated
    pub rules_version: String,
    /// lower and upper threshhold for flagging
    pub min_matches: usize,
    pub max_matches: usize,
    /// If we should log information to a file
    pub logging_is_active: bool,
    /// mirror to folder with hashfiles for update
    pub mirror: String,
    /// the filename of the compiled yara rules on remote
    pub remote_file: String,
    /// various paths in an effort to unify them. are folders expected to be used later
    #[serde(skip)]
    pub paths: Option<Paths>,
}

fn default_config() -> String {
    String::from("0")
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub data: PathBuf,
    pub config: PathBuf,
    pub logs_scan: PathBuf,
    pub logs_app: PathBuf,
    pub downloads: PathBuf,
}

/// Struct for which fields the frontend has access to
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFrontend {
    pub logging_is_active: Option<bool>,
    pub scan_dir: Option<bool>,
    pub min_matches: Option<usize>,
    pub max_matches: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_version: crate::CONFIG_VERSION.to_owned(),
            rules_version: "None".to_owned(),
            logging_is_active: true,
            min_matches: crate::DEFAULT_MIN_MATCHES,
            max_matches: crate::DEFAULT_MAX_MATCHES,
            mirror: crate::DEFAULT_MIRROR.to_owned(),
            remote_file: crate::DEFAULT_FILE.to_owned(),
            paths: None,
        }
    }
}

/// The config file simply holds settings of the application that should perists during reboots
/// The entire config is saved to a JSON file and loaded or created on the first start
/// Default config gets created, then we try to load. If load fails we return default
impl Config {
    /// Finds the suitable path for the current system, creates a subfolder for the app and returns
    /// the path as a normal String
    fn set_paths(&mut self) -> Result<(), String> {
        //let downloads = directories_next::UserDirs::download_dir(&self);
        //let downloads = dirs::download_dir().unwrap_or(Path::new(".").to_path_buf());
        let downloads = UserDirs::new()
            .ok_or("Could not determine user directories".to_owned())?
            .download_dir()
            .ok_or("Could not get download dir".to_owned())?
            .to_path_buf();

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let dirs = ProjectDirs::from("com", "Raspirus", "Raspirus")
            .ok_or("Failed to get projectdir".to_owned())?;
        #[cfg(target_os = "windows")]
        let dirs = ProjectDirs::from("com", "Raspirus", "")
            .ok_or("Failed to get projectdir".to_owned())?;

        // RoamingData under windows
        let data = dirs.data_dir().to_owned();
        let logs = data.to_owned().join("logs");

        let logs_scan = logs.join("scan");
        let mut logs_app = logs.join("application");

        // LocalData under windows
        let config = dirs.config_dir().to_owned();

        fs::create_dir_all(&data).map_err(|err| format!("Failed to create data dir: {err}"))?;
        fs::create_dir_all(&logs_scan)
            .map_err(|err| format!("Failed to create scan log dir: {err}"))?;
        fs::create_dir_all(&logs_app)
            .map_err(|err| format!("Failed to create application log dir: {err}"))?;
        fs::create_dir_all(&config).map_err(|err| format!("Failed to create config dir: {err}"))?;

        // add launch timestamp to app log path
        logs_app = logs_app.join(APPLICATION_LOG.clone());

        self.paths = Some(Paths {
            data,
            config,
            logs_scan,
            logs_app,
            downloads,
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
    pub fn new() -> Result<Self, String> {
        // fetch default config path
        let mut tmp = Config::default();
        tmp.set_paths()?;
        let path = tmp
            .paths
            .clone()
            .ok_or("Could not get config path".to_owned())?
            .config
            .join(crate::CONFIG_FILENAME);
        // Checks if the config file exists, else quickly creates it
        if !path.exists() {
            tmp.save()?;
        };

        // now we load the config
        let mut file = File::open(path).map_err(|err| err.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| format!("Failed to read config to string: {err}"))?;
        let mut config_from_str: Config = serde_json::from_str(&contents)
            .map_err(|err| err.to_string())
            .map_err(|err| format!("Failed deserializing config: {err}"))?;
        config_from_str.set_paths()?;
        config_from_str.update_config()?;
        Ok(config_from_str)
    }

    /// checks if the config version is the expected one
    fn update_config(&self) -> Result<Self, String> {
        if self.config_version != crate::CONFIG_VERSION {
            info!(
                "Updating config from {} to {}",
                self.config_version,
                crate::CONFIG_VERSION
            );
            let mut config = Config::default();
            config.set_paths()?;
            config.save()?;
            Ok(config)
        } else {
            Ok(self.clone())
        }
    }
}
