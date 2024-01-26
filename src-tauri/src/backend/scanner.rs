use std::{
    fs::{self, File},
    path::Path,
    time, io::Read,
};

use chrono::{DateTime, Local};
use log::{debug, error, info, warn};
use tauri::Manager;
use zip::ZipArchive;

use super::{config_file::Config, db_ops::DBOps, file_log::FileLog};

/// Struct for sending events to the frontend
#[derive(Clone, serde::Serialize)]
struct TauriEvent {
    message: String,
}

/// Struct representing a file scanner that is capable of searching through a specified directory 
/// and its subdirectories for malicious files.
pub struct Scanner {
    /// A reference to a `DBOps` object that allows the `FileScanner` to access and manipulate the database.
    pub db_conn: DBOps,
    /// A vector of file paths for files that have been identified as malicious.
    pub dirty_files: Vec<String>,
    /// A `FileLog` object that the `FileScanner` can use to log information about the search process.
    pub log: FileLog,
    /// An array containing false positives MD5 Hashes
    pub false_positive: Vec<String>,
    /// Defines the scanning size in bytes
    folder_size: u64,
    /// Amount scanned in bytes
    scanned_size: u64,
    /// Tauri window for events
    tauri_window: Option<tauri::Window>,
    /// last percentage
    last_percentage: f64,
    // Number of files analysed
    analysed: u64,
    // Number of files skipped
    skipped: u64,
}

/// Implementation of the `FileScanner` struct.
impl Scanner {
    // Creates a Scanner object
    pub fn new(db_file: &str, t_win: Option<tauri::Window>) -> Result<Self, String> {
        // Initialize database. We need this to compare hashes
        let tmpconf = match DBOps::new(db_file, None) {
            Ok(db_conn) => db_conn,
            Err(err) => {
                return Err(format!("Failed to initialize database: {err}"));
            }
        };
        // We initialize the log file here
        let now: DateTime<Local> = Local::now();
        let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
        let log_str = format!("{}.log", now_str);

        let config = Config::new()?;
        // We retrieve signatures that should be ignored from the config file
        let false_positive: Vec<String> = config.ignored_hashes;

        // We return the scanner object
        Ok(Scanner {
            db_conn: tmpconf,
            dirty_files: Vec::new(),
            log: FileLog::new(log_str).expect("Failed to initialize scan logger"),
            false_positive,
            folder_size: 0,
            scanned_size: 0,
            tauri_window: t_win,
            last_percentage: 0.0,
            analysed: 0,
            skipped: 0,
        })
    }

    /// Initializes the scanner and starts the scanning process.
    /// Returns a vector of strings containing the paths of the files that have been identified as malicious.
    pub fn init(mut self, early_stop: bool, path_str: &str) -> Result<Vec<String>, String> {
        // Check the path we need to scan
        let path = Path::new(&path_str);
        if !path.exists() {
            return Err("Invalid Path".to_owned())
        }
        let big_tic = time::Instant::now();
        // Calculate the size of the folder. We need this to calculate the progress
        // and to determine if we have the permission to scan the folder
        self.folder_size = match Self::size(path) {
            Ok(size) => size,
            Err(err) => {
                error!("Failed to calculate folder size: {err}");
                return Err(String::from("Failed to calculate folder size (Do you have permission?)"));
            }
        };
        
        debug!("Started scanning {}...", path_str);
        if path.is_dir() {
            // We scan the folder and all its subfolders
            match Self::scan_folder(&mut self, path, early_stop) {
                Ok(_) => {},
                Err(err) => {
                    error!("Failed to scan folder: {err}");
                    return Err(String::from("Failed to scan folder"))
                }
            }
        } else {
            // We scan the single file, if it is a zip file we scan all its contents
            match Self::scan_file(&mut self, path, early_stop) {
                Ok(_) => {},
                Err(err) => {
                    error!("Failed to scan file: {err}");
                    return Err(String::from("Failed to scan file"))
                }
            }
        }
        let big_toc = time::Instant::now();
        info!(
            "=> Analysed: {}, Skipped: {},  Infected: {}, Time: {} seconds",
            self.analysed,
            self.skipped,
            self.dirty_files.len(),
            big_toc.duration_since(big_tic).as_secs_f64()
        );
        // We return the vector of infected files
        Ok(self.dirty_files.clone())
    }
    
    /// Gets the size of a zip file without extracting it (for security reasons)
    fn size_zip(path: &Path) -> Result<u64, std::io::Error> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut archive_size = 0;
        
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            archive_size += file.size();
        }
        Ok(archive_size)
    }
    
    /// Gets the size of a file. If it is a zip file, it calls the size_zip function
    /// otherwise it just returns the size of the file
    fn size_file(path: &Path) -> Result<u64, std::io::Error> {
        debug!("Calculating {}", path.to_str().unwrap_or_default());
        Ok(match path.extension().unwrap_or_default().to_str().unwrap_or_default() {
            "zip" => Self::size_zip(path)?,
            _ => File::open(path)?.metadata()?.len(),
        })
    }
    
    /// Gets the size of a folder. It recursively calls itself for each subfolder
    /// and calls size_file for each file in the folder
    fn size_folder(path: &Path) -> Result<u64, std::io::Error> {
        debug!("Entering {}", path.to_str().unwrap_or_default());
        let mut size = 0;
        
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry_path = entry?.path();
        
            if entry_path.is_dir() {
                size += Self::size_folder(&entry_path)?;
            } else if entry_path.is_file() {
                size += Self::size_file(&entry_path)?;
            }
        }
        
        Ok(size)
    }
    
    /// Gets the size of a file or folder and returns it
    /// It automatically detects if it is a file or a folder
    fn size(path: &Path) -> Result<u64, String> {
        Ok(if path.is_dir() {
            match Self::size_folder(path) {
                Ok(size) => size,
                Err(err) => {
                    warn!("Failed to get folder size for scanning: {err}");
                    return Err(String::from("Failed to get folder size for scanning"))
                }
            }
        } else {
            match Self::size_file(path) {
                Ok(size) => size,
                Err(err) => {
                    warn!("Failed to get file size for scanning: {err}");
                    return Ok(0)
                }
            }
        })
    }

    /// Scans a folder and all its subfolders. Returns true if infected
    /// It recursively calls itself for each subfolder and calls scan_file for each file in the folder
    /// It also calculates the progress and sends it to the frontend
    /// If early_stop is true, it stops scanning as soon as it finds a malicious file
    pub fn scan_folder(&mut self, path: &Path, early_stop: bool) -> Result<bool, String> {
        debug!("Entering {}", path.to_str().unwrap_or_default());
        let mut found = false;

        // Check all entries in the folder
        let entries = fs::read_dir(path).unwrap();
        for entry in entries {
            let entry_path = match entry {
                Ok(entry) => entry.path(),
                Err(err) => {
                    error!("Failed to get folder entry: {err}");
                    self.skipped += 1;
                    continue;
                },
            };
        
            if entry_path.is_dir() {
                match self.scan_folder(Path::new(&entry_path), early_stop) {
                    Ok(res) => {
                        if res {
                            found = true;
                            if early_stop {
                                return Ok(true);
                            }
                        }
                    },
                    Err(err) => {
                        error!("Encountered error while handling folder {}: {err}", entry_path.to_str().unwrap_or_default());
                        continue;
                    },
                };
            } else if entry_path.is_file() {
                match self.scan_file(Path::new(&entry_path), early_stop) {
                    Ok(res) => {
                        if res {
                            found = true;
                            if early_stop {
                                return Ok(true);
                            }
                        }
                    },
                    Err(err) => {
                        error!("Encountered error while handling file {}: {err}", entry_path.to_str().unwrap_or_default());
                        continue;
                    },
                }
            }
        }

        Ok(found)
    }

    /// Scans a file. Returns true if infected
    /// It can also scan zip files and all their contents
    pub fn scan_file(&mut self, path: &Path, early_stop: bool) -> Result<bool, String> {
        debug!("Scanning {}", path.to_str().unwrap_or_default());
        let mut found = false;
        // We determine if the file is a zip file or not by checking its extension
        // We could extend this in the future to support more archive formats
        match path.extension().unwrap_or_default().to_str().unwrap_or_default() {
            "zip" => {
                let file = match File::open(path) {
                    Ok(file) => file,
                    Err(err) => {
                        error!("Failed to get file: {err}");
                        self.skipped += 1;
                        return Ok(false);
                    }
                };
                // Open the zip archive
                let mut archive = match ZipArchive::new(file) {
                    Ok(archive) => archive,
                    Err(err) => {
                        error!("Failed to open archive: {err}");
                        self.skipped += 1;
                        return Err(String::from("Failed to open archive"));
                    }
                };
                // Iterate through all files in the archive
                for i in 0..archive.len() {
                    let file = match archive.by_index(i) {
                        Ok(file) => file,
                        Err(err) => {
                            error!("Failed to get file from archive: {err}");
                            self.skipped += 1;
                            continue;
                        }
                    };
                    // Calculate the progress to send it to the frontend
                    if file.is_file() {
                        if Self::calculate_progress(self, file.size()).is_err() {
                            error!("Progress calculation is broken");
                        }
                        // Create the hash of the file
                        let hash = match Self::compute_hash(file) {
                            Ok(hash) => hash,
                            Err(err) => {
                                error!("Encountered error while computing hash for {}: {err}", path.to_str().unwrap_or_default());
                                self.skipped += 1;
                                return Err(String::from("Encountered error while computing hash"))
                            }
                        };
                        // Check if the hash is a false positive (should be ignored)
                        if self.false_positive.contains(&hash) {
                            info!("Found false postitive! Skipping...");
                            self.skipped += 1;
                            continue;
                        }
                        // Check if the hash is in the database
                        match self.db_conn.hash_exists(&hash) {
                            Ok(exists) => {
                                if exists {
                                    // If it is in the database, we log it and add it to the list of infected files
                                    info!("Hash {hash} found");
                                    self.dirty_files.push(path.file_name().unwrap_or_default().to_str().unwrap_or_default().to_owned());
                                    self.log.log(hash, path.to_str().unwrap_or_default().to_owned());
                                    found = true;
                                    if early_stop {
                                        return Ok(found);
                                    }
                                }
                                self.analysed += 1;
                            },
                            Err(err) => {
                                self.skipped += 1;
                                error!("Could not retrieve hash from db for file {}: {err}", path.file_name().unwrap_or_default().to_str().unwrap_or_default())
                            },
                        }
                    }
                }
            }
            _ => {
                // For any other file, we just calculate the hash and check if it is in the database
                // Since the underscore stands for "any other file extension", it may also be a rar file or something else
                let file = match File::open(path) {
                    Ok(file) => file,
                    Err(err) => {
                        error!("Failed to get file: {err}");
                        self.skipped += 1;
                        return Err(String::from("Failed to get file"));
                    }
                };
                // Again, we calculate the progress to send it to the frontend
                if Self::calculate_progress(self, fs::metadata(path).expect("Failed to get file size").len()).is_err() {
                    error!("Progress calculation is broken");
                }

                let hash = match Self::compute_hash(file) {
                    Ok(hash) => hash,
                    Err(err) => {
                        error!("Encountered error while computing hash for {}: {err}", path.to_str().unwrap_or_default());
                        self.skipped += 1;
                        return Err(String::from("Encountered error while computing hash"))
                    }
                };

                if self.false_positive.contains(&hash) {
                    info!("Found false postitive! Skipping...");
                    self.skipped += 1;
                    return Ok(found);
                }

                match self.db_conn.hash_exists(&hash) {
                    Ok(exists) => {
                        if exists {
                            info!("Hash {hash} found");
                            self.dirty_files.push(path.file_name().unwrap_or_default().to_str().unwrap_or_default().to_owned());
                            self.log.log(hash, path.to_str().unwrap_or_default().to_owned());
                            found = true;
                        }
                        self.analysed += 1;
                    },
                    Err(err) => {
                        self.skipped += 1;
                        error!("Could not retrieve hash from db for file {}: {err}", path.file_name().unwrap_or_default().to_str().unwrap_or_default())
                    },
                }
            },
        }
        // Return if the file is infected or not
        Ok(found)
    }


    /// Computes the MD5 hash of a file and returns it as a string
    fn compute_hash(mut file: impl Read) -> Result<String, std::io::Error> {
        let mut hasher = md5::Context::new();

        let mut buffer = [0; 1024];
        while let Ok(size) = file.read(&mut buffer) {
            if size == 0 {
                break;
            }
            hasher.consume(&buffer[..size]);
        }
        Ok(format!("{:x}", hasher.compute()))
    }

    /// Calculates the progress of the scanning process and sends it to the frontend
    /// It returns an error if it fails to send the progress to the frontend
    /// It uses the scanned_size and folder_size variables to calculate the progress
    /// It also checks if the folder is empty, because that would return infinity percentage
    fn calculate_progress(
        &mut self,
        file_size: u64,
    ) -> Result<f64, String> {
        self.scanned_size += file_size;
        let scanned_percentage = (self.scanned_size as f64 / self.folder_size as f64 * 100.0).round();
        // Check if folder is empty, because that would return infinity percentage
        if self.folder_size == 0 {
            if let Some(tauri_win) = &self.tauri_window {
                if tauri_win.emit_all("progerror", TauriEvent { message: "Calculated foldersize is 0".to_string(),},).is_err() {
                    return Err("Couldn't send progress update to frontend".to_string());
                }
            }
            return Err("Calculated foldersize is 0".to_string());
        }
        info!("Scanned: {}%", scanned_percentage);
        if scanned_percentage != self.last_percentage {
            if let Some(tauri_win) = &self.tauri_window {
                if tauri_win.emit_all("progress",TauriEvent { message: scanned_percentage.to_string(),},).is_err() {
                    return Err("Couldn't send progress update to frontend".to_string());
                };
            }
            self.last_percentage = scanned_percentage;
        }
        Ok(scanned_percentage)
    }
}