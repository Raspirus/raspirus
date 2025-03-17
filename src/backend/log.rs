use serde::{Deserialize, Serialize};
use std::{path::PathBuf, fs};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LogLevel {
    Off,
    Info,
    Debug,
    Trace
}

/// A helper used for logging file scan results to a logfile
pub struct Log {
    pub log_path: PathBuf,
}

impl Log {
    /// Tries to create a new instance of Log, while also creating the log file if it does not
    /// already exist
    pub fn new(log_path: PathBuf) -> Result<Self, String> {
        if !log_path.exists() {
            fs::create_dir_all(&log_path).map_err(|err| format!("Failed to create log file: {err:?}"))?;
        }
        Ok(Self { log_path })
    }

    pub fn log() -> Result<(), String> {
        todo!()
    }
}
