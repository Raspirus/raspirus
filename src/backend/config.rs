use std::{fs, path::PathBuf};

use super::log::LogLevel;
use directories_next::ProjectDirs;
use log::{info, warn};
use serde::{Deserialize, Serialize};

type Error = crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
/// Holds the config values for the application
pub struct Config {
    /// Holds the config version. If there is a mismatch, the config will be reset to default
    pub config_version: usize,
    /// Remote mirror with git api structure for fetching yara rules
    pub remote_url: String,
    /// Minimum matches for a file to get flagged
    pub min_matches: usize,
    /// Maximum matches after which a file stops gaining additional flags. 0 to disable
    pub max_matches: usize,
    /// Threads used for parallel scanning
    pub max_threads: usize,
    /// Loglevel used for logging
    pub logging: LogLevel,
    /// Language used to display text
    pub language: String,
    #[serde(skip)]
    /// Paths holding various paths needed for ordinary execution
    pub paths: Option<Paths>,
}

impl Default for Config {
    /// Fills the config with a bunch of predefined values
    fn default() -> Self {
        Self {
            config_version: crate::globals::CONFIG_VERSION,
            remote_url: crate::globals::DEFAULT_REMOTE_URL.to_owned(),
            min_matches: 1,
            max_matches: 0,
            max_threads: num_cpus::get(),
            logging: crate::globals::DEFAULT_LOG_LEVEL.clone(),
            language: crate::globals::DEFAULT_LANGUAGE.to_owned(),
            paths: None,
        }
    }
}

#[derive(Debug, Clone)]
/// Holds paths necessary for application execution
pub struct Paths {
    /// %appdata%\Roaming under windows || ~/.local/share under linux
    pub data: PathBuf,
    /// %appdata%\Local under windows || ~/.cache under linux
    pub temp: PathBuf,
    /// %appdata%\Roaming under windows || ~/.config under linux
    pub config: PathBuf,
    pub logs_scan: PathBuf,
    pub logs_app: PathBuf,
}

impl Paths {
    /// Creates a new instance of Paths
    pub fn identify() -> Result<Self, Error> {
        #[cfg(not(target_os = "windows"))]
        let dirs = ProjectDirs::from("org", "raspirus", "raspirus")
            .ok_or("Failed to get projectdir".to_owned())
            .map_err(|err| {
                Error::ConfigIO(std::io::Error::new(std::io::ErrorKind::NotFound, err))
            })?;
        #[cfg(target_os = "windows")]
        let dirs = ProjectDirs::from("org", "raspirus", "")
            .ok_or("Failed to get projectdir".to_owned())
            .map_err(|err| {
                Error::ConfigIO(std::io::Error::new(std::io::ErrorKind::NotFound, err))
            })?;

        // data folders
        let data = dirs.data_dir().to_owned();
        let logs = data.to_owned().join("logs");
        let temp = dirs.cache_dir().to_path_buf();

        // log folders
        let logs_scan = logs.join("scan");
        let mut logs_app = logs.join("application");

        // config folder location
        let config = dirs.config_dir().to_owned();

        // create necessary folders
        fs::create_dir_all(&data).map_err(Error::ConfigIO)?;
        fs::create_dir_all(data.join("yara_c")).map_err(Error::ConfigIO)?;
        fs::create_dir_all(&logs_scan).map_err(Error::ConfigIO)?;
        // remove old cache
        let _ = fs::remove_dir(&temp);
        fs::create_dir_all(&temp).map_err(Error::ConfigIO)?;
        fs::create_dir_all(&logs_app).map_err(Error::ConfigIO)?;
        fs::create_dir_all(&config).map_err(Error::ConfigIO)?;

        // add launch timestamp to app log path
        logs_app = logs_app.join(crate::globals::get_application_log());

        Ok(Paths {
            data,
            config,
            logs_scan,
            logs_app,
            temp,
        })
    }
}

impl Config {
    /// Returns either the paths contained in the config, or tries to create a new instance
    pub fn get_paths(&self) -> Result<Paths, Error> {
        Ok(match &self.paths {
            Some(paths) => paths.clone(),
            None => Paths::identify()?,
        })
    }

    /// Try to modify config with loaded values; This can also be used to populate the default
    /// config struct
    pub fn load(&mut self) -> Result<(), Error> {
        // if config folder does not exist, create it
        let paths = self.get_paths()?;
        let config_folder_path = paths.config.clone();
        if !config_folder_path.exists() {
            fs::create_dir_all(&config_folder_path).map_err(Error::ConfigIO)?;
        }

        // add config file name to config folder path
        let config_file_path = config_folder_path.join(crate::globals::CONFIG_FILE_NAME);

        // read config to string in order to serialize it using serde
        let config_string = match fs::read_to_string(&config_file_path) {
            Ok(config_string) => config_string,
            // write default config to file if nothing can be read
            Err(err) => {
                info!("Could not read config file: {err:?}; Attempting to create one");
                let default_config = Config::default();
                default_config.save()?;
                serde_json::to_string(&default_config).map_err(Error::ConfigDeserialization)?
            }
        };

        // serialize config from loaded string
        let mut config =
            serde_json::from_str::<Config>(&config_string).map_err(Error::ConfigDeserialization)?;

        // check if loaded config version equals current version, otherwise revert to default
        if config.config_version != crate::globals::CONFIG_VERSION {
            warn!(
                "Config version did not match ({} != {}). Restoring defaults...",
                config.config_version,
                crate::globals::CONFIG_VERSION
            );
            config = Config::default();
            config.paths = Some(paths);
            config.save()?;
        } else {
            config.paths = Some(paths);
        }

        self.config_version = config.config_version;
        self.min_matches = config.min_matches;
        self.max_matches = config.max_matches;
        self.max_threads = config.max_threads;
        self.logging = config.logging;
        self.language = config.language;
        self.paths = config.paths;

        Ok(())
    }

    /// Writes the current config struct to the file system
    pub fn save(&self) -> Result<(), Error> {
        let paths = self.get_paths()?;
        let config_string =
            serde_json::to_string_pretty(&self).map_err(Error::ConfigSerialization)?;

        fs::write(
            paths.config.join(crate::globals::CONFIG_FILE_NAME),
            &config_string,
        )
        .map_err(Error::ConfigIO)?;

        Ok(())
    }
}
