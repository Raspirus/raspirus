use std::{
    fs::{self}, os::unix::fs::MetadataExt, path::{Path, PathBuf}
};

use chrono::{DateTime, Local};
use log::{debug, warn};
use serde::Serialize;
use yara_x::{ScanResults, Scanner};

use crate::backend::utils::generic::{get_config, get_rules};

use super::{file_log::FileLog, utils::generic::{send, size}};

#[derive(Serialize, Clone)]
pub struct TaggedFile {
    pub path: PathBuf,
    pub descriptions: Vec<String>,
    pub rule_count: usize,
}

pub struct YaraScanner {
    pub tags: Vec<TaggedFile>,
    size: u64,
    last_size: u64,
    file_log: FileLog,
    tauri_window: Option<tauri::Window>,
    analysed: u64,
    skipped: u64,
}

impl YaraScanner {
    /// creates a new scanenr and imports the yara rules
    pub fn new(tauri_window: Option<tauri::Window>) -> Result<Self, String> {        
        // setup file log
        let now: DateTime<Local> = Local::now();
        let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
        let log_str = format!("{}.log", now_str);

        Ok(Self {
            tags: Vec::new(),
            tauri_window,
            size: 0,
            last_size: 0,
            file_log: FileLog::new(log_str)?,
            analysed: 0,
            skipped: 0,
        })
    }

    /// Starts the scanner in the specified location
    pub fn start(&mut self, path: PathBuf) -> Result<(), String> {
        if !path.exists() {
            return Err("Invalid path".to_owned());
        }

        self.size = size(&path)?;

        self.scan_file(&path.clone())?;
        Ok(())
    }
 
    fn evaluate_result(&mut self, result: ScanResults, path: &Path) {
        let matching = result.matching_rules();
        let rule_count = matching.count();
        let descriptions = result
            .matching_rules()
            .map(|rule| rule.metadata().into_json())
            .map(|m| {
                match m.get("description") {
                    Some(description) => {
                        description.as_str().unwrap_or("No description set").to_owned()
                    },
                    None => "No description set".to_owned(),
                }
            }).collect::<Vec<String>>();
        if rule_count > get_config().min_matches {
            self.file_log.log(path.to_str().unwrap_or_default().to_owned(), rule_count, &descriptions);
            self.tags.push(TaggedFile {
                path: path.to_path_buf(),
                descriptions,
                rule_count,
            })
        }
    }

    /// scans a file
    fn scan_file(&mut self, path: &Path) -> Result<(), String> {
        if path.is_dir() {
            return self.scan_folder(path)
        }
        debug!("File: {}", path.to_str().unwrap_or_default());
        let rules = get_rules()?;
        let mut scanner = Scanner::new(&rules);
        scanner.max_matches_per_pattern(get_config().max_matches);
        match path.extension().unwrap_or_default().to_str() {
            Some("zip") => {
                warn!("Zip files are not supported at the moment and will nto be scanned!");
                self.skipped += 1;
                /*
                let file = File::open(path).map_err(|err| format!("Failed to open zip file: {err}"))?;
                let mut archive = ZipArchive::new(file).map_err(|err| format!("Failed to create zip: {err}"))?;
                for i in 0..archive.len() {
                    let mut content = archive.by_index(i).map_err(|err| format!("Failed to get file in zip: {err}"))?;
                    if !content.is_file() {
                        continue;
                    }
                    let buffer = content.bytes
                    self.evaluate_result(scanner.scan())
                }
                */
            }
            None | Some(_) => {
                let result = scanner
                    .scan_file(path)
                    .map_err(|err| format!("Failed to scan file: {err}"))?;
                self.evaluate_result(result, path);
                self.last_size += path.metadata().map_err(|err| format!("Failed to get metadata: {err}"))?.size();
                self.analysed += 1;
                self.progress();
            }
        }
        Ok(())
    }

    /// scans a folder
    fn scan_folder(&mut self, path: &Path) -> Result<(), String> {
        debug!("Folder: {}", path.to_str().unwrap_or_default());
        for entry in fs::read_dir(path).map_err(|err| err.to_string())? {
            let entry_path = entry.map_err(|err| err.to_string())?.path();
            if entry_path.is_dir() {
                self.scan_file(&entry_path)?;
            } else {
                self.scan_file(&entry_path)?;
            }
        }
        Ok(())
    }

    fn progress(&self) {
        let percentage = ((self.last_size as f64 / self.size as f64) * 100.0).round();
        if self.tauri_window.is_some() {
            send(&self.tauri_window, "progress", format!("{percentage}%"));
            println!("Scan progress: {percentage}%");
        } else {
            println!("Scan progress: {percentage}%");
        }
    }
}
