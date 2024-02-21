use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    process::exit,
    time,
};

use chrono::{DateTime, Local};
use log::{debug, error, info, trace, warn};
use tauri::Manager;
use zip::ZipArchive;

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
        let tmpconf = match DBOps::new(db_file) {
            Ok(db_conn) => db_conn,
            Err(err) => {
                error!("{err}");
                exit(-1);
            }
        };

        let now: DateTime<Local> = Local::now();
        let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
        let log_str = format!("{}.log", now_str);

        let config = get_config();
        // Add all false positives here
        let false_positive: Vec<String> = config.ignored_hashes;

        Ok(Scanner {
            db_conn: tmpconf,
            dirty_files: Vec::new(),
            log: FileLog::new(log_str),
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
        self.folder_size = match Self::size(path) {
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
            match Self::scan_folder(&mut self, path, early_stop) {
                Ok(_) => {}
                Err(err) => {
                    error!("Failed to scan folder: {err}");
                    return Err(String::from("Failed to scan folder"));
                }
            }
        } else {
            match Self::scan_file(&mut self, path, early_stop) {
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

    // gets the unpacked size of a zip file
    fn size_zip(path: &Path) -> Result<u64, std::io::Error> {
        trace!("Calculating zip: {}", path.display());
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut archive_size = 0;

        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            archive_size += file.size();
        }
        Ok(archive_size)
    }

    // gets the size of a file
    fn size_file(path: &Path) -> Result<u64, std::io::Error> {
        trace!("Calculating file: {}", path.display());
        Ok(
            match path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
            {
                "zip" => Self::size_zip(path)?,
                _ => File::open(path)?.metadata()?.len(),
            },
        )
    }

    // gets the size of a folder and its contents
    fn size_folder(path: &Path) -> Result<u64, std::io::Error> {
        trace!("Calculating folder: {}", path.display());
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

    // gets the size for a given path
    fn size(path: &Path) -> Result<u64, String> {
        if path.is_dir() {
            match Self::size_folder(path) {
                Ok(size) => Ok(size),
                Err(err) => {
                    warn!("Failed to get folder size for scanning: {err}");
                    Err(String::from("Failed to get folder size for scanning"))
                }
            }
        } else {
            match Self::size_file(path) {
                Ok(size) => Ok(size),
                Err(err) => {
                    warn!("Failed to get file size for scanning: {err}");
                    Ok(0)
                }
            }
        }
    }

    // scans a folder. returns true if infected
    pub fn scan_folder(&mut self, path: &Path, early_stop: bool) -> Result<bool, String> {
        debug!("Entering {}", path.to_str().unwrap_or_default());
        let mut found = false;

        let entries = fs::read_dir(path).unwrap();
        for entry in entries {
            let entry_path = match entry {
                Ok(entry) => entry.path(),
                Err(err) => {
                    error!("Failed to get folder entry: {err}");
                    self.skipped += 1;
                    continue;
                }
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
                    }
                    Err(err) => {
                        error!(
                            "Encountered error while handling folder {}: {err}",
                            entry_path.to_str().unwrap_or_default()
                        );
                        continue;
                    }
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
                    }
                    Err(err) => {
                        error!(
                            "Encountered error while handling file {}: {err}",
                            entry_path.to_str().unwrap_or_default()
                        );
                        continue;
                    }
                }
            }
        }

        Ok(found)
    }

    // scans a file. returns true if infected
    pub fn scan_file(&mut self, path: &Path, early_stop: bool) -> Result<bool, String> {
        debug!("Scanning {}", path.to_str().unwrap_or_default());
        let mut found = false;
        match path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
        {
            "zip" => {
                let file = match File::open(path) {
                    Ok(file) => file,
                    Err(err) => {
                        error!("Failed to get file: {err}");
                        self.skipped += 1;
                        return Ok(false);
                    }
                };

                let mut archive = match ZipArchive::new(file) {
                    Ok(archive) => archive,
                    Err(err) => {
                        error!("Failed to open archive: {err}");
                        self.skipped += 1;
                        return Err(String::from("Failed to open archive"));
                    }
                };

                for i in 0..archive.len() {
                    let file = match archive.by_index(i) {
                        Ok(file) => file,
                        Err(err) => {
                            error!("Failed to get file from archive: {err}");
                            self.skipped += 1;
                            continue;
                        }
                    };

                    if file.is_file() {
                        if Self::calculate_progress(self, file.size()).is_err() {
                            error!("Progress calculation is broken");
                        }

                        let hash = match Self::compute_hash(file) {
                            Ok(hash) => hash,
                            Err(err) => {
                                error!(
                                    "Encountered error while computing hash for {}: {err}",
                                    path.to_str().unwrap_or_default()
                                );
                                self.skipped += 1;
                                return Err(String::from("Encountered error while computing hash"));
                            }
                        };

                        if self.false_positive.contains(&hash) {
                            info!("Found false postitive! Skipping...");
                            self.skipped += 1;
                            continue;
                        }

                        match self.db_conn.hash_exists(&hash) {
                            Ok(exists) => {
                                if exists {
                                    info!("Hash {hash} found");
                                    self.dirty_files.push(
                                        path.file_name()
                                            .unwrap_or_default()
                                            .to_str()
                                            .unwrap_or_default()
                                            .to_owned(),
                                    );
                                    self.log
                                        .log(hash, path.to_str().unwrap_or_default().to_owned());
                                    found = true;
                                    if early_stop {
                                        return Ok(found);
                                    }
                                }
                                self.analysed += 1;
                            }
                            Err(err) => {
                                self.skipped += 1;
                                error!(
                                    "Could not retrieve hash from db for file {}: {err}",
                                    path.file_name()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default()
                                )
                            }
                        }
                    }
                }
            }
            _ => {
                let file = match File::open(path) {
                    Ok(file) => file,
                    Err(err) => {
                        error!("Failed to get file: {err}");
                        self.skipped += 1;
                        return Err(String::from("Failed to get file"));
                    }
                };

                if Self::calculate_progress(
                    self,
                    fs::metadata(path).expect("Failed to get file size").len(),
                )
                .is_err()
                {
                    error!("Progress calculation is broken");
                }

                let hash = match Self::compute_hash(file) {
                    Ok(hash) => hash,
                    Err(err) => {
                        error!(
                            "Encountered error while computing hash for {}: {err}",
                            path.to_str().unwrap_or_default()
                        );
                        self.skipped += 1;
                        return Err(String::from("Encountered error while computing hash"));
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
                            self.dirty_files.push(
                                path.file_name()
                                    .unwrap_or_default()
                                    .to_str()
                                    .unwrap_or_default()
                                    .to_owned(),
                            );
                            self.log
                                .log(hash, path.to_str().unwrap_or_default().to_owned());
                            found = true;
                        }
                        self.analysed += 1;
                    }
                    Err(err) => {
                        self.skipped += 1;
                        error!(
                            "Could not retrieve hash from db for file {}: {err}",
                            path.file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default()
                        )
                    }
                }
            }
        }
        Ok(found)
    }

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

    fn calculate_progress(&mut self, file_size: u64) -> Result<f64, String> {
        self.scanned_size += file_size;
        let scanned_percentage =
            (self.scanned_size as f64 / self.folder_size as f64 * 100.0).round();
        // Check if folder is empty, because that would return infinity percentage
        if self.folder_size == 0 {
            if let Some(tauri_win) = &self.tauri_window {
                if tauri_win
                    .emit_all(
                        "progerror",
                        TauriEvent {
                            message: "Calculated foldersize is 0".to_string(),
                        },
                    )
                    .is_err()
                {
                    return Err("Couldn't send progress update to frontend".to_string());
                }
            }
            return Err("Calculated foldersize is 0".to_string());
        }
        info!("Scanned: {}%", scanned_percentage);
        if scanned_percentage != self.last_percentage {
            if let Some(tauri_win) = &self.tauri_window {
                if tauri_win
                    .emit_all(
                        "progress",
                        TauriEvent {
                            message: scanned_percentage.to_string(),
                        },
                    )
                    .is_err()
                {
                    return Err("Couldn't send progress update to frontend".to_string());
                };
            }
            self.last_percentage = scanned_percentage;
        }
        Ok(scanned_percentage)
    }
}
