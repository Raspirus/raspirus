use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::Mutex,
};

use chrono::{DateTime, Local};
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use threadpool_rs::threadpool::pool::ThreadPool;
use yara_x::{ScanResults, Scanner};

use crate::backend::utils::generic::{get_config, get_rules};

use super::{
    file_log::FileLog,
    utils::generic::{profile_path, send},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaggedFile {
    pub path: PathBuf,
    /// vector of description and rule name
    pub descriptions: Vec<RuleFeedback>,
    pub rule_count: usize,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RuleFeedback {
    pub rule_name: String,
    pub rule_description: String,
}

impl Display for RuleFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.rule_name, self.rule_description)
    }
}

/// collection of pointers for the scan threads
#[derive(Default, Clone)]
pub struct PointerCollection {
    tagged: Arc<Mutex<Vec<TaggedFile>>>,
    analysed: Arc<Mutex<usize>>,
    skipped: Arc<Mutex<usize>>,
    scanned_size: Arc<Mutex<usize>>,
}

pub struct YaraScanner {
    pub tauri_window: Option<Arc<tauri::Window>>,
}

impl YaraScanner {
    /// creates a new scanenr and imports the yara rules
    pub fn new(tauri_window: Option<Arc<tauri::Window>>) -> Result<Self, String> {
        Ok(Self { tauri_window })
    }

    /// Starts the scanner in the specified location
    pub fn start(&mut self, path: PathBuf) -> Result<Vec<TaggedFile>, String> {
        if !path.exists() {
            return Err("Invalid path".to_owned());
        }

        // setup file log
        let now: DateTime<Local> = Local::now();
        let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
        let log_str = format!("{}.log", now_str);
        let file_log = Arc::new(Mutex::new(FileLog::new(log_str)?));

        let profiled = profile_path(path.clone())
            .map_err(|err| format!("Failed to calculate file tree: {err}"))?;
        let total_size = profiled.1;
        let paths = profiled.0;
        let pointers = PointerCollection::default();

        let threadpool = ThreadPool::new(num_cpus::get())
            .map_err(|err| format!("Failed to create threadpool: {err}"))?;
        for file in paths {
            let pointers_c = pointers.clone();
            let file_log_c = file_log.clone();
            let tauri_window_c = self.tauri_window.as_ref().map(|window| window.clone());
            threadpool.execute(move || {
                match Self::scan_file(
                    file.as_path(),
                    file_log_c,
                    tauri_window_c,
                    total_size,
                    pointers_c,
                ) {
                    Ok(_) => {}
                    Err(err) => error!(
                        "Failed to scan file {}: {err}",
                        file.to_str().unwrap_or_default()
                    ),
                }
            });
        }
        drop(threadpool);
        let tagged = pointers
            .tagged
            .lock()
            .map_err(|err| format!("Failed to lock final tagged vec: {err}"))?
            .clone();
        println!("Found tagged files: {:#?}", tagged);
        Ok(tagged)
    }

    fn evaluate_result(
        file_log: Arc<Mutex<FileLog>>,
        tagged: Arc<Mutex<Vec<TaggedFile>>>,
        result: ScanResults,
        path: &Path,
    ) -> Result<(), String> {
        let matching = result.matching_rules();
        let rule_count = matching.count();
        let descriptions = result
            .matching_rules()
            .map(|rule| (rule.metadata().into_json(), rule))
            .map(|m| RuleFeedback {
                rule_description: match m.0.get("description") {
                    Some(description) => description
                        .as_str()
                        .unwrap_or("No description set")
                        .to_owned(),
                    None => "No description set".to_owned(),
                },
                rule_name: m.1.identifier().to_owned(),
            })
            .collect::<Vec<RuleFeedback>>();
        if rule_count > get_config().min_matches {
            let file_log_locked = file_log
                .lock()
                .map_err(|err| format!("Failed to lock file logger: {err}"))?;
            file_log_locked.log(
                path.to_str().unwrap_or_default().to_owned(),
                rule_count,
                &descriptions,
            );
            let mut tagged_locked = tagged
                .lock()
                .map_err(|err| format!("Failed to lock tagged: {err}"))?;
            tagged_locked.push(TaggedFile {
                path: path.to_path_buf(),
                descriptions,
                rule_count,
            })
        }
        Ok(())
    }

    /// thread function to scan a singular file
    fn scan_file(
        path: &Path,
        file_log: Arc<Mutex<FileLog>>,
        tauri_window: Option<Arc<tauri::Window>>,
        total_size: usize,
        pointers: PointerCollection,
    ) -> Result<(), String> {
        let rules = get_rules()?;
        let mut scanner = Scanner::new(&rules);
        scanner.max_matches_per_pattern(get_config().max_matches);
        match path.extension().unwrap_or_default().to_str() {
            Some("zip") => {
                warn!("Zip files are not supported at the moment and will not be scanned!");
                let mut skipped_locked = pointers
                    .skipped
                    .lock()
                    .map_err(|err| format!("Failed to lock skipped: {err}"))?;
                *skipped_locked += 1;
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
                Self::evaluate_result(file_log, pointers.tagged, result, path)?;

                // update shared variables
                {
                    let mut scanned_size_locked = pointers
                        .scanned_size
                        .lock()
                        .map_err(|err| format!("Failed to lock scanned size: {err}"))?;
                    *scanned_size_locked += path
                        .metadata()
                        .map_err(|err| format!("Failed to get metadata: {err}"))?
                        .len() as usize;
                    let mut analysed_locked = pointers
                        .analysed
                        .lock()
                        .map_err(|err| format!("Failed to lock analysed: {err}"))?;
                    *analysed_locked += 1;
                }
                // send progress to frontend
                Self::progress(pointers.scanned_size, total_size, tauri_window)?;
            }
        }
        Ok(())
    }

    fn progress(
        scanned_size: Arc<Mutex<usize>>,
        total_size: usize,
        tauri_window: Option<Arc<tauri::Window>>,
    ) -> Result<(), String> {
        let scanned_size_locked = scanned_size
            .lock()
            .map_err(|err| format!("Failed to lock scanned size: {err}"))?;
        let scanned = *scanned_size_locked as f64;
        let percentage = (scanned / total_size as f64) * 100.0;
        if tauri_window.is_some() {
            send(&tauri_window, "progress", format!("{percentage:.2}%"));
            println!("Scan progress: {percentage:.2}%");
        } else {
            println!("Scan progress: {percentage:.2}%");
        }
        Ok(())
    }
}
