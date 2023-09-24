use std::{
    fs::{self, File},
    io::{BufReader, Error, ErrorKind, Read},
    path::Path,
    process::exit,
    time,
};

use chrono::{DateTime, Local};
use log::{debug, error, info, warn};
use mime_guess::from_path;
use tauri::Manager;
use terminal_size::terminal_size;
use walkdir::WalkDir;
use zip::ZipArchive;
use is_elevated::is_elevated;

use super::{config_file::Config, db_ops::DBOps, file_log::FileLog};

#[derive(Clone, serde::Serialize)]
struct TauriEvent {
    message: String,
}

/// Struct representing a file scanner that is capable of searching through a specified directory and its subdirectories for malicious files.
pub struct FileScanner {
    /// A reference to a `DBOps` object that allows the `FileScanner` to access and manipulate the database.
    pub db_conn: DBOps,
    /// A vector of file paths for files that have been identified as malicious.
    pub dirty_files: Vec<String>,
    /// The file path of the directory that the `FileScanner` should search through.
    pub scanloc: String,
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
}

impl FileScanner {
    // Creates a FileScanner object
    pub fn new(scanloc: &str, db_file: &str, t_win: Option<tauri::Window>) -> Result<Self, Error> {
        //check if the pat that should be scanned exists
        if Path::new(&scanloc).exists() {
            let tmpconf = match DBOps::new(db_file, None) {
                Ok(db_conn) => db_conn,
                Err(err) => {
                    error!("{err}");
                    exit(-1);
                }
            };

            let now: DateTime<Local> = Local::now();
            let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
            let log_str = format!("{}.log", now_str);

            // Add all false positives here
            let mut config = Config::new();
            config = config
                .load()
                .expect("Couldn't load config in the flie scanner");
            let false_pos: Vec<String> = config.ignored_hashes;

            Ok(FileScanner {
                db_conn: tmpconf,
                dirty_files: Vec::new(),
                scanloc: scanloc.to_owned(),
                log: FileLog::new(log_str),
                false_positive: false_pos,
                folder_size: 0,
                scanned_size: 0,
                tauri_window: t_win,
            })
        } else {
            // If the path to be scanned is invalid
            Err(Error::new(ErrorKind::Other, "Invalid Path"))
        }
    }

    pub fn search_files(&mut self, stop_early: bool) -> Result<Vec<String>, String> {
        debug!("Started file searching");
        let scanloc = std::mem::take(&mut self.scanloc);
        debug!("Got scanloc = {}", scanloc);
        let path = Path::new(scanloc.as_str());
        debug!("Path of scanloc = {}", path.display().to_string());

        if path.exists() {
            debug!("Path exists");
            if path.is_file() {
                debug!("Path is file");
                if let Some(mime_type) = from_path(path).first() {
                    if mime_type == "application/zip" {
                        debug!("Path is ZIP file");
                        self.zip_file_search(path, stop_early)
                    } else {
                        debug!("Path is single file");
                        self.scan_single_file(path)
                    }
                } else {
                    error!("Failed to determine the file type: {:?}", path);
                    return Err("Failed to determine mime type".to_string());
                }
            } else if path.is_dir() {
                debug!("Path is folder");
                self.folder_scanner(path, stop_early)
            } else {
                error!(
                    "Given path exists, but is neither a file nor a directory: {:?}",
                    path
                );
                return Err("Given path exists, but is neither a file nor a directory".to_string());
            }
        } else {
            error!("Path to scan not found: {:?}", path);
            return Err("Path to scan not found".to_string());
        }
    }

    fn zip_file_search(
        &mut self,
        zip_path: &Path,
        stop_early: bool,
    ) -> Result<Vec<String>, String> {
        if self.get_folder_size(zip_path).is_err() {
            return Err("Can't get folder size".to_string());
        }

        let file = match File::open(zip_path) {
            Ok(file) => file,
            Err(err) => {
                error!("Can't open zip file: {}", err);
                return Err("Can't open given ZIP file".to_string());
            }
        };

        let mut archive = match ZipArchive::new(file) {
            Ok(archive) => archive,
            Err(err) => {
                error!("Can't open zip archive: {}", err);
                return Err("Can't open ZIP archive".to_string());
            }
        };

        let mut analysed: i128 = 0;
        let mut skipped: i128 = 0;
        let last_percentage: &mut f64 = &mut -1.0;
        let big_tic = time::Instant::now();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).expect("Couldnt get file index");

            if file.is_file() {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).expect("Couldnt read to the end of the file");

                let hash = match self.create_hash_from_buffer(&buffer, file.name()) {
                    Some(hash) => {
                        analysed += 1;
                        hash
                    }
                    None => {
                        skipped += 1;
                        "".to_owned()
                    }
                };

                info!("Retrieved file size = {}", file.size());

                if Self::calculate_progress(self, last_percentage, file.size()).is_err() {
                    error!("Progress calculation is broken");
                    break;
                }
                if let Ok(Some(exists)) = self.db_conn.hash_exists(hash.as_str()) {
                    if exists {
                        self.dirty_files.push(file.name().to_string());
                        if stop_early {
                            warn!("Stopping early at file in ZIP: {:?}", file.name());
                            break;
                        }
                        info!("Found hash {} for file {} in ZIP", hash, file.name());
                        self.log.log(hash, file.name().to_string());
                    }
                } else {
                    error!(
                        "Error checking hash existence for file: {:?} in ZIP",
                        file.name()
                    );
                }
            }
        }

        let big_toc = time::Instant::now();
        info!(
            "=> Analysed: {}, Skipped: {},  Infected: {}, Time: {} seconds",
            analysed,
            skipped,
            self.dirty_files.len(),
            big_toc.duration_since(big_tic).as_secs_f64()
        );
        Ok(self.dirty_files.clone())
    }

    fn scan_single_file(&mut self, path: &Path) -> Result<Vec<String>, String> {
        let hash = self.create_hash(path.to_str().expect("Can't get string of path"));
        if let Some(tauri_win) = &self.tauri_window {
            if tauri_win
                .emit_all(
                    "progress",
                    TauriEvent {
                        message: "100".to_string(),
                    },
                )
                .is_err()
            {
                return Err("Couldn't send progress update to frontend".to_string());
            };
        };

        if let Ok(Some(exists)) = self
            .db_conn
            .hash_exists(hash.clone().unwrap_or("".to_string()).as_str())
        {
            if exists {
                self.dirty_files.push(path.display().to_string());
                info!(
                    "Found hash {:?} for file {}",
                    hash.clone(),
                    path.display().to_string()
                );
                self.log.log(
                    hash.unwrap_or("None".to_string()),
                    path.display().to_string(),
                );
                Ok(self.dirty_files.clone())
            } else {
                Ok(Vec::new())
            }
        } else {
            error!("Error checking hash existence for file: {:?}", path);
            Err("Couldn't check if the hash of the file exists in the DB".to_string())
        }
    }

    pub fn folder_scanner(&mut self, path: &Path, stop_early: bool) -> Result<Vec<String>, String> {
        let mut analysed: i128 = 0;
        let mut skipped: i128 = 0;
        let last_percentage: &mut f64 = &mut -1.0;
        let big_tic = time::Instant::now();
        let folder_size = self.get_folder_size(path);
        if folder_size.is_err() {
            if is_elevated() {
                info!("Program is running with administrative privileges");
            } else {
                info!("Program is running with normal privileges");
            }
            error!("Can't get folder size because of {:?}", folder_size);
            return Err("Can't get folder size: Access is denied".to_string());
        }
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            if (match file.metadata() {
                Ok(md) => md,
                Err(err) => {
                    error!("Failed getting file metadata: {}", err);
                    return Err("Failed getting file metadata.".to_string());
                }
            })
            .is_file()
            {
                let hash = match self.create_hash(&file.path().display().to_string()) {
                    Some(hash) => {
                        analysed += 1;
                        hash
                    }
                    None => {
                        skipped += 1;
                        "".to_owned()
                    }
                };
                if Self::calculate_progress(self, last_percentage, file.metadata().expect("Couldnt get file metadata").len())
                    .is_err()
                {
                    error!("Progress calculation is broken");
                    break;
                }
                if let Ok(Some(exists)) = self.db_conn.hash_exists(hash.as_str()) {
                    if exists {
                        self.dirty_files.push(file.path().display().to_string());
                        if stop_early {
                            warn!("Stopping early at file: {:?}", file.path());
                            break;
                        }
                        info!(
                            "Found hash {} for file {}",
                            hash,
                            file.path().display().to_string()
                        );
                        self.log.log(hash, file.path().display().to_string());
                    }
                } else {
                    // Handle the error case
                    // For example, you can log the error or handle it based on your requirements
                    error!("Error checking hash existence for file: {:?}", file.path());
                }
            }
        }
        let big_toc = time::Instant::now();
        info!(
            "=> Analysed: {}, Skipped: {},  Infected: {}, Time: {} seconds",
            analysed,
            skipped,
            self.dirty_files.len(),
            big_toc.duration_since(big_tic).as_secs_f64()
        );
        Ok(self.dirty_files.clone())
    }

    fn create_hash_from_buffer(&mut self, buffer: &[u8], filename: &str) -> Option<String> {
        let mut context = md5::Context::new();

        for chunk in buffer.chunks(65536) {
            context.consume(chunk);
        }

        let ret = format!("{:x}", context.compute());

        if self.false_positive.contains(&ret) {
            debug!("Skipping Hash: {}", ret);
            return None;
        }

        match terminal_size() {
            Some((width, _)) => {
                match width.0.checked_sub(filename.len() as u16 + 2) {
                    Some(spacing) => {
                        debug!("\n {}{:>width$} ", filename, ret, width = spacing as usize);
                    }
                    None => {}
                };
            }
            None => {}
        }

        Some(ret)
    }

    pub fn create_hash(&mut self, path: &str) -> Option<String> {
        let mut context = md5::Context::new();
        let mut buffer = [0; 65536]; // 64KB

        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                error!("Can't open file: {}", err);
                return None;
            }
        };
        let mut reader = BufReader::new(file);
        let mut it = 0;
        loop {
            let count = match reader.read(&mut buffer) {
                Ok(count) => count,
                Err(err) => {
                    error!("Error while reading: {}", err);
                    return None;
                }
            };

            if count == 0 {
                if it == 0 {
                    debug!("Skipping Empty file: {}", path);
                    return None;
                }
                break;
            }
            context.consume(&buffer[..count]);
            it += 1;
        }
        let ret = format!("{:?}", context.compute());

        if self.false_positive.contains(&ret) {
            debug!("Skipping Hash: {}", ret);
            return None;
        }

        match terminal_size() {
            Some((width, _)) => {
                match width.0.checked_sub(path.len() as u16 + 2) {
                    Some(spacing) => {
                        debug!("\n {}{:>width$} ", path, ret, width = spacing as usize);
                    }
                    None => {}
                };
            }
            None => {}
        };
        Some(ret)
    }

    fn get_folder_size(&mut self, path: &Path) -> Result<u64, std::io::Error> {
        let metadata = fs::metadata(path)?;
        if metadata.is_file() {
            if let Some(mime_type) = from_path(path).first() {
                if mime_type == "application/zip" {
                    let file = File::open(path)?;
                    let mut archive = ZipArchive::new(file)?;

                    let mut total_uncompressed_size: u64 = 0;
                    for i in 0..archive.len() {
                        let entry = archive.by_index(i)?;
                        total_uncompressed_size += entry.size();
                    }
                    info!(
                        "Added file: {} with size: {}",
                        path.to_str().expect("Couldnt convert path to str"),
                        total_uncompressed_size
                    );
                    self.folder_size = total_uncompressed_size;
                    return Ok(total_uncompressed_size);
                } else {
                    info!(
                        "Added file: {} with size: {}",
                        path.to_str().expect("Couldnt convert path to str"),
                        metadata.len()
                    );
                    self.folder_size = metadata.len();
                    return Ok(metadata.len());
                }
            } else {
                error!("Failed to determine the file type: {:?}", path);
                return Err(std::io::Error::last_os_error());
            }
        } else {
            let mut size: u64 = 0;

            for entry in WalkDir::new(path).follow_links(true) {
                let entry = entry?;
                let entry_metadata = entry.metadata()?;
                if entry_metadata.is_file() {
                    size += entry_metadata.len();
                }
            }
            self.folder_size = size;
            Ok(size)
        }
    }

    fn calculate_progress(
        &mut self,
        last_percentage: &mut f64,
        file_size: u64,
    ) -> Result<f64, String> {
        self.scanned_size = self.scanned_size + file_size;
        debug!("Calculated scanned size = {}", self.scanned_size);
        debug!("Calculated folder size = {}", self.folder_size);
        let scanned_percentage =
            (self.scanned_size as f64 / self.folder_size as f64 * 100.0).round();
        // Check if folder is empty, because that would return infinity percentage
        if self.folder_size <= 0 {
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
        if scanned_percentage != *last_percentage {
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
            *last_percentage = scanned_percentage;
        }
        Ok(scanned_percentage)
    }
}
