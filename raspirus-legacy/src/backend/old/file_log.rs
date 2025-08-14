use std::{fs::File, io::Write, sync::Arc};

use log::{debug, error};

use std::path::PathBuf;

use chrono::Local;

use crate::CONFIG;

use super::yara_scanner::TaggedFile;

pub struct FileLog {
    pub file: Arc<File>,
    pub log_path: PathBuf,
}

/// A struct for creating and writing to a log file.
impl FileLog {
    /// Creates a new `FileLog` struct and attempts to create a new file with the specified name.
    ///
    /// # Arguments
    ///
    /// * `fname` - A string representing the name of the file to create.
    ///
    /// # Example
    ///
    /// ```
    /// let log = FileLog::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, String> {
        let values = Self::create_file()?;
        Ok(Self {
            file: Arc::new(values.0),
            log_path: values.1,
        })
    }

    /// Appends the specified `hash` and `fpath` to the log file.
    ///
    /// # Arguments
    ///
    /// * `hash` - A string representing the hash to log.
    /// * `fpath` - A string representing the file path to log.
    ///
    /// # Example
    ///
    /// ```
    /// let log = FileLog::new().unwrap();
    /// ```
    pub fn log(&mut self, file: &TaggedFile) {
        let log_string = format!(
            "[{}]\t{}\n{}\n",
            file.rule_count,
            file.path.to_string_lossy(),
            file.descriptions
                .iter()
                .map(|description| description.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
        let _ = self
            .file
            .write_all(log_string.as_bytes())
            .map_err(|err| error!("Failed to log: {err}"));
    }

    /// Creates a new file with the specified name and attempts to create a logs folder if it doesn't already exist.
    ///
    /// # Arguments
    ///
    /// * `fname` - A string representing the name of the file to create.
    ///
    /// # Example
    ///
    /// ```
    /// let mut log = FileLog::new().unwrap();
    /// log.create_file("new_log.txt".to_owned());
    /// ```
    fn create_file() -> Result<(File, PathBuf), String> {
        // Create scan log dir
        let log_file_path = CONFIG
            .lock()
            .expect("Failed to lock config")
            .paths
            .clone()
            .ok_or("No paths set. Is config initialized?".to_owned())?
            .logs_scan
            .join(format!("{}.log", Local::now().format("%Y_%m_%d_%H_%M_%S")));
        debug!("Created log file at {}", log_file_path.to_string_lossy());

        Ok((
            File::create(&log_file_path)
                .map_err(|err| format!("Failed to create log file: {err}"))?,
            log_file_path,
        ))
    }
}
