use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    time,
};

use chrono::{DateTime, Local};
use log::{debug, error, info, warn};
use tauri::Manager;
use zip::ZipArchive;

use crate::backend::utils::generic::size;

use super::{db_ops::DBOps, file_log::FileLog, utils::generic::get_config};

#[derive(Clone, serde::Serialize)]
struct TauriEvent {
    message: String,
}

/// Struct representing a file scanner that is capable of searching through a specified directory and its subdirectories for malicious files.
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

    analysed: u64,
    skipped: u64,
}

impl Scanner {
    // Creates a FileScanner object
    pub fn new(db_file: &str, t_win: Option<tauri::Window>) -> Result<Self, String> {
        // checks if database exists
        let tmpconf = DBOps::new(db_file).map_err(|err| err.to_string())?;

        let now: DateTime<Local> = Local::now();
        let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
        let log_str = format!("{}.log", now_str);

        // Add all false positives here
        let false_positive: Vec<String> = get_config().ignored_hashes;

        Ok(Scanner {
            db_conn: tmpconf,
            dirty_files: Vec::new(),
            log: FileLog::new(log_str)?,
            false_positive,
            folder_size: 0,
            scanned_size: 0,
            tauri_window: t_win,
            last_percentage: 0.0,
            analysed: 0,
            skipped: 0,
        })
    }

    // initializes a scan
    pub fn init(mut self, early_stop: bool, path_str: &str) -> Result<Vec<String>, String> {
        let path = Path::new(&path_str);
        if !path.exists() {
            return Err("Invalid Path".to_owned());
        }
        let big_tic = time::Instant::now();
        self.folder_size = match size(path) {
            Ok(size) => size,
            Err(err) => {
                error!("Failed to calculate folder size: {err}");
                return Err(String::from(
                    "Failed to calculate folder size (Do you have permission?)",
                ));
            }
        };

        debug!("Started scanning {}...", path_str);
        if path.is_dir() {
            match self.scan_folder(path, early_stop) {
                Ok(_) => {}
                Err(err) => {
                    error!("Failed to scan folder: {err}");
                    return Err(String::from("Failed to scan folder"));
                }
            }
        } else {
            match self.scan_file(path, early_stop) {
                Ok(_) => {}
                Err(err) => {
                    error!("Failed to scan file: {err}");
                    return Err(String::from("Failed to scan file"));
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
        Ok(self.dirty_files.clone())
    }

    // scans a folder. returns true if infected
    pub fn scan_folder(&mut self, path: &Path, early_stop: bool) -> Result<bool, String> {
        debug!("Entering {}", path.to_str().unwrap_or_default());
        let mut found_total = false;

        let entries = fs::read_dir(path).map_err(|err| err.to_string())?;

        for entry in entries {
            // check if entry is readable
            let entry = match entry.map_err(|err| err.to_string()) {
                Ok(entry) => entry,
                Err(err) => {
                    warn!("Failed to get entry: {err}");
                    self.skipped += 1;
                    continue;
                }
            };
            let entry_path = entry.path();
            // if this returns something, it means we found something and are returning early
            if (match entry_path {
                // scan directory
                _ if entry_path.is_dir() => match self.scan_folder(&entry_path, early_stop) {
                    Ok(found) => {
                        self.analysed += 1;
                        // if we find something, we set found_total to true
                        found.then(|| found_total = true);
                        // if we find something and early_stop is true, we return something, otherwise not
                        (found && early_stop).then(|| Some(()))
                    }
                    Err(err) => {
                        self.skipped += 1;
                        error!("Error for folder {}: {err}", entry_path.display());
                        continue;
                    }
                },
                // scan file
                _ if entry_path.is_file() => match self.scan_file(&entry_path, early_stop) {
                    Ok(found) => {
                        self.analysed += 1;
                        // if we find something, we set found_total to true
                        found.then(|| found_total = true);
                        // if we find something and early_stop is true, we return something, otherwise not
                        (found && early_stop).then(|| Some(()))
                    }
                    Err(err) => {
                        self.skipped += 1;
                        error!("Error for file {}: {err}", entry_path.display());
                        continue;
                    }
                },
                // undefined path target
                _ => {
                    warn!("{} is not a file or folder? Skipping", entry_path.display());
                    self.skipped += 1;
                    continue;
                }
            })
            .is_some()
            {
                break;
            }
        }

        Ok(found_total)
    }

    // scans a file. returns true if infected
    pub fn scan_file(&mut self, path: &Path, early_stop: bool) -> Result<bool, String> {
        debug!("Scanning file: {}", path.display());
        let mut found = false;
        match path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
        {
            // zip files
            "zip" => {
                let file = File::open(path).map_err(|err| format!("Failed to get file: {err}"))?;

                let mut archive = ZipArchive::new(file)
                    .map_err(|err| format!("Failed top open archive: {err}"))?;

                for i in 0..archive.len() {
                    let mut file = archive
                        .by_index(i)
                        .map_err(|err| format!("Failed to get file from archive: {err}"))?;

                    // if file is not actually a file we skip
                    if !file.is_file() {
                        continue;
                    }

                    // compute hash
                    let hash = Scanner::compute_hash(&mut file).map_err(|err| {
                        format!(
                            "Encountered error while computing hash for {}: {err}",
                            path.display()
                        )
                    })?;

                    // update percentage
                    self.last_percentage =
                        self.calculate_progress(file.size()).unwrap_or_else(|err| {
                            error!("{err}");
                            self.last_percentage
                        });

                    if self.false_positive.contains(&hash) {
                        info!("Found false postitive! Skipping...");
                        self.skipped += 1;
                        continue;
                    }

                    if self
                        .db_conn
                        .hash_exists(&hash)
                        .map_err(|err| format!("Failed to retrieve hash from db: {err}"))?
                    {
                        info!("Hash {hash} found");
                        // mark file path as infected
                        self.dirty_files.push(
                            path.file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default()
                                .to_owned(),
                        );
                        // log found file
                        self.log.log(hash, path.display().to_string());

                        found = true;
                        if early_stop {
                            return Ok(found);
                        }
                    }
                }
            }
            // other files
            _ => {
                let mut file =
                    File::open(path).map_err(|err| format!("Failed to get file: {err}"))?;

                let _ = self
                    .calculate_progress(
                        fs::metadata(path)
                            .map_err(|_| "Failed to get file size".to_owned())?
                            .len(),
                    )
                    .map_err(|err| warn!("Failed to calculate progress: {err}"));

                let hash = Scanner::compute_hash(&mut file).map_err(|err| {
                    format!(
                        "Encountered error while computing hash for {}: {err}",
                        path.display()
                    )
                })?;

                if self.false_positive.contains(&hash) {
                    info!("Found false postitive! Skipping...");
                    self.skipped += 1;
                    return Ok(found);
                }

                if self
                    .db_conn
                    .hash_exists(&hash)
                    .map_err(|err| format!("Failed to retrieve hash from db: {err}"))?
                {
                    info!("Hash {hash} found");
                    // mark file path as infected
                    self.dirty_files.push(
                        path.file_name()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_owned(),
                    );
                    // log found file
                    self.log.log(hash, path.display().to_string());

                    found = true;
                    if early_stop {
                        return Ok(found);
                    }
                }
            }
        }
        Ok(found)
    }

    fn compute_hash(file: &mut impl Read) -> Result<String, std::io::Error> {
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

    fn calculate_progress(&mut self, file_size: u64) -> Result<f64, String> {
        self.scanned_size += file_size;
        // Check if folder is empty, because that would return infinity percentage
        if self.folder_size == 0 {
            // return error and send to frontend, if it exists
            if let Some(tauri_window) = &self.tauri_window {
                tauri_window
                    .emit_all(
                        "progerror",
                        TauriEvent {
                            message: "Calculated foldersize is 0".to_string(),
                        },
                    )
                    .map_err(|err| format!("Could not send progress to frontend: {err}"))?;
            }
            return Err("Calculated foldersize is 0".to_string());
        }

        let scanned_percentage =
            (self.scanned_size as f64 / self.folder_size as f64 * 100.0).round();

        info!("Scanned: {}%", scanned_percentage);
        // nothing changed
        if scanned_percentage == self.last_percentage {
            return Ok(scanned_percentage);
        }

        // if there is a window, send percentage, otherwise ignore
        if let Some(tauri_window) = &self.tauri_window {
            tauri_window
                .emit_all(
                    "progress",
                    TauriEvent {
                        message: scanned_percentage.to_string(),
                    },
                )
                .map_err(|err| format!("Could not send progress to frontend: {err}"))?;
        }
        Ok(scanned_percentage)
    }
}
