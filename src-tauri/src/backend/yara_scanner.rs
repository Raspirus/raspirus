use std::{
    fs::{self, File}, os::unix::fs::MetadataExt, path::{Path, PathBuf}
};

use chrono::{DateTime, Local};
use log::{debug, info, warn};
use serde::Serialize;
use yara_x::{Rules, ScanResults, Scanner};

use crate::backend::utils::generic::get_config;

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
    last_percentage: f32,
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
            last_percentage: 0.0,
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

    fn get_rules(&self) -> Result<Rules, String> {
        let yar_path = get_config()
            .paths
            .ok_or("No paths set. Is config initialized?")?
            .data
            .join(get_config().remote_file);
        // setup rules
        let reader = File::open(yar_path)
            .map_err(|err| format!("Failed to load yar file: {}", err.to_string()))?;
        Rules::deserialize_from(reader)
            .map_err(|err| format!("Failed to deserialize yar file: {}", err.to_string()))

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
        if rule_count > 0 {
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
        let rules = &self.get_rules()?;
        let mut scanner = Scanner::new(&rules);
        match path.extension().unwrap_or_default().to_str() {
            Some("zip") => {
                warn!("Zip files are not supported at the moment and will nto be scanned!");
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
