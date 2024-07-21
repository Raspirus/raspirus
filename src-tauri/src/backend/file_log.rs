use std::{
    fs::{self, File},
    io::Write,
};

use log::{error, trace, warn};

use super::{utils::generic::get_config, yara_scanner::RuleFeedback};

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
    pub fn new(fname: String) -> Result<Self, String> {
        let mut ret = FileLog { file: None };
        ret.create_file(fname)?;
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
    pub fn log(&self, fpath: String, rule_count: usize, descriptions: &[RuleFeedback]) {
        match self.file.as_ref() {
            Some(mut file) => {
                match file.write_all(
                    format!("[{rule_count}]\t{fpath}\n{}\n", descriptions.iter().map(|description| description.to_string()).collect::<Vec<String>>().join("\n")).as_bytes(),
                ) {
                    Ok(_) => {}
                    Err(err) => error!("Failed loggin: {err}"),
                };
            }
            None => {
                warn!("Logfile invalid!");
            }
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
    pub fn create_file(&mut self, fname: String) -> Result<(), String> {
        // get log directory with Scan subdir
        let log_dir = get_config()
            .paths
            .ok_or("No paths set. Is config initialized?".to_owned())?
            .logs
            .join("scan");

        match fs::create_dir_all(&log_dir) {
            Ok(_) => {
                self.file = Some(match File::create(log_dir.join(fname.clone())) {
                    Ok(file) => {
                        trace!("Created logfile at {}", log_dir.join(fname).display());
                        Ok(file)
                    }
                    Err(err) => Err(format!("Failed creating log file: {err}")),
                }?);
                Ok(())
            }
            Err(err) => Err(format!("Failed creating log folder: {err}")),
        }
    }
}
