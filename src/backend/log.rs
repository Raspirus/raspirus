use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

type Error = crate::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LogLevel {
    Off,
    Info,
    Debug,
    Trace,
}

/// A helper used for logging file scan results to a logfile
pub struct Log {
    pub log_path: PathBuf,
}

impl Log {
    /// Tries to create a new instance of Log, while also creating the log file if it does not
    /// already exist
    pub fn new(log_path: PathBuf) -> Result<Self, Error> {
        if !log_path.exists() {
            fs::create_dir_all(&log_path).map_err(Error::LogIO)?;
        }
        Ok(Self { log_path })
    }

    /// Logs the data of a tagged / skipped file
    pub fn log() -> Result<(), Error> {
        todo!()
    }
}
