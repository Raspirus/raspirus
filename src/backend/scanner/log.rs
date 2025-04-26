use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use super::structs::NotableFile;

type Error = crate::Error;

/// A helper used for logging file scan results to a logfile
pub struct Log {
    pub log_path: PathBuf,
    writer: Arc<Mutex<BufWriter<std::fs::File>>>,
}

impl Log {
    /// Tries to create a new instance of Log, while also creating the log file with current time
    pub fn new() -> Result<Self, Error> {
        let log_path = crate::globals::get_ro_config()?.get_paths()?.logs_scan;

        if !log_path.exists() {
            fs::create_dir_all(&log_path).map_err(Error::LogIO)?;
        }

        // open the file for appending
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(
                log_path.join(
                    chrono::Utc::now()
                        .format("%Y-%m-%dT%H:%M:%SZ.log")
                        .to_string(),
                ),
            )
            .map_err(Error::LogIO)?;

        let writer = Arc::new(Mutex::new(BufWriter::new(file)));
        Ok(Self { log_path, writer })
    }

    /// Logs the data of a tagged / skipped file
    pub fn log(&self, file: NotableFile) -> Result<(), Error> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|err| Error::LogLock(err.to_string()))?;
        writeln!(writer, "{}", file).map_err(Error::LogLog)
    }
}
