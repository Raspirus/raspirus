use std::{fs::File, io::Write};

use log::{debug, error, warn};

use std::path::PathBuf;

use chrono::Local;

use crate::CONFIG;

use super::yara_scanner::RuleFeedback;

#[derive(Default)]
pub struct FileLog {
    pub file: Option<File>,
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
    /// let log = FileLog::new("log.txt".to_owned());
    /// ```
    pub fn new() -> Result<Self, String> {
        let mut ret = Self::default();
        ret.create_file()?;
        Ok(ret)
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
    /// let log = FileLog::new("log.txt".to_owned());
    /// log.log("abc123".to_owned(), "C:/Users/user/Desktop/file.txt".to_owned());
    /// ```
    pub fn log(&self, file_path: PathBuf, rule_count: usize, descriptions: &[RuleFeedback]) {
        if let Some(mut log_file) = self.file.as_ref() {
            let log_string = format!(
                "[{rule_count}]\t{}\n{}\n",
                file_path.to_string_lossy(),
                descriptions
                    .iter()
                    .map(|description| description.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            let _ = log_file
                .write_all(log_string.as_bytes())
                .map_err(|err| error!("Failed to log: {err}"));
        } else {
            warn!("Log file is none");
        }
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
    /// let mut log = FileLog::new("log.txt".to_owned());
    /// log.create_file("new_log.txt".to_owned());
    /// ```
    pub fn create_file(&mut self) -> Result<(), String> {
        // Create scan log dir
        let log_file_path = CONFIG
            .lock()
            .expect("Failed to lock config")
            .paths
            .clone()
            .ok_or("No paths set. Is config initialized?".to_owned())?
            .logs_scan
            .join(format!("{}.log", Local::now().format("%Y_%m_%d_%H_%M_%S")));

        self.file = Some(
            File::create(&log_file_path)
                .map_err(|err| format!("Failed to create log file: {err}"))?,
        );
        debug!("Created log file at {}", log_file_path.to_string_lossy());
        Ok(())
    }
}
