use std::{
    fs::{self, File},
    io::Write,
};

use directories_next::ProjectDirs;
use log::{error, trace, warn};

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
    pub fn new(fname: String) -> Self {
        let mut ret = FileLog { file: None };
        ret.create_file(fname);
        ret
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
    pub fn log(&self, hash: String, fpath: String) {
        match self.file.as_ref() {
            Some(mut file) => {
                match file.write_all(format!("{hash}\t{fpath}\n").as_bytes()) {
                    Ok(_) => {
                        trace!(
                            "Wrote {hash}\t{fpath} to {:?}",
                            self.file.as_ref().expect("Invalid file reference")
                        )
                    }
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
    pub fn create_file(&mut self, fname: String) {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Logs")
            .expect("Failed to get project directories.");
        let log_dir = project_dirs.data_local_dir().join("logs"); // Create a "logs" subdirectory

        match fs::create_dir_all(&log_dir) {
            Ok(_) => {
                self.file = match File::create(log_dir.join(fname.clone())) {
                    Ok(file) => {
                        trace!(
                            "Created logfile at DIR: {} NAME: {}",
                            log_dir.display(),
                            fname
                        );
                        Some(file)
                    }
                    Err(err) => {
                        error!("Failed creating logfile: {err}");
                        None
                    }
                };
            }
            Err(err) => error!("Failed creating logs folder: {err}"),
        }
    }
}
