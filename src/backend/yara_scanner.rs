use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::Mutex,
};

use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use threadpool_rs::threadpool::pool::Threadpool;
use tokio::sync::mpsc;
use yara_x::{ScanResults, Scanner};

use crate::{backend::utils::generic::get_rules, CONFIG};

use super::{config_file::Config, file_log::FileLog, utils::generic::profile_path};

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

#[derive(Serialize, Debug, Clone)]
pub struct Skipped {
    pub path: PathBuf,
    pub reason: String,
}

/// collection of pointers for the scan threads
#[derive(Default, Clone)]
pub struct PointerCollection {
    tagged: Arc<Mutex<Vec<TaggedFile>>>,
    skipped: Arc<Mutex<Vec<Skipped>>>,
    analysed: Arc<Mutex<usize>>,
    total: Arc<Mutex<usize>>,
    config: Arc<Config>,
}

pub struct YaraScanner {
    pub progress_channel: Option<Arc<mpsc::Sender<String>>>,
}

impl YaraScanner {
    /// creates a new scanenr and imports the yara rules
    pub fn new(progress_channel: Option<Arc<mpsc::Sender<String>>>) -> Result<Self, String> {
        Ok(Self { progress_channel })
    }

    /// Starts the scanner in the specified location
    pub fn start(&mut self, path: PathBuf) -> Result<(Vec<TaggedFile>, Vec<Skipped>), String> {
        if !path.exists() {
            return Err("Invalid path".to_owned());
        }

        // setup file log
        let file_log = Arc::new(Mutex::new(FileLog::new()?));

        let paths = profile_path(path.clone())
            .map_err(|err| format!("Failed to calculate file tree: {err}"))?;
        let mut pointers = PointerCollection::default();
        pointers.config = Arc::from(CONFIG.lock().expect("Failed to lock config").clone());
        pointers.total = Arc::new(Mutex::new(paths.len()));

        let mut threadpool = Threadpool::new(num_cpus::get());
        for file in paths {
            let pointers_c = pointers.clone();
            let file_log_c = file_log.clone();
            let progress_c = self.progress_channel.as_ref().map(|sender| sender.clone());
            threadpool.execute(move || {
                match Self::scan_file(file.as_path(), file_log_c, progress_c, pointers_c) {
                    Ok(_) => {}
                    Err(err) => error!(
                        "Failed to scan file {}: {err}",
                        file.to_str().unwrap_or_default()
                    ),
                }
            });
        }
        threadpool.join();
        let tagged = pointers
            .tagged
            .lock()
            .map_err(|err| format!("Failed to lock final tagged vec: {err}"))?
            .clone();
        let skipped = pointers
            .skipped
            .lock()
            .map_err(|err| format!("Failed to lock skipped: {err}"))?
            .clone();
        println!("Found tagged files: {tagged:#?}");
        println!("Found skipped files: {skipped:#?}");
        Ok((tagged, skipped))
    }

    fn evaluate_result(
        pointers: &PointerCollection,
        file_log: Arc<Mutex<FileLog>>,
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
        if (rule_count
            >= if pointers.config.min_matches == 0 {
                1
            } else {
                pointers.config.min_matches
            })
            && (pointers.config.max_matches == 0 || rule_count <= pointers.config.max_matches)
        {
            let file_log_locked = file_log
                .lock()
                .map_err(|err| format!("Failed to lock file logger: {err}"))?;
            file_log_locked.log(path.to_path_buf(), rule_count, &descriptions);
            let mut tagged_locked = pointers
                .tagged
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
        progress_channel: Option<Arc<mpsc::Sender<String>>>,
        pointers: PointerCollection,
    ) -> Result<(), String> {
        info!("Scanning {}", path.to_string_lossy());
        let rule_path = pointers
            .config
            .paths
            .clone()
            .ok_or("No paths set. Is config initialized?")?
            .data
            .join(
                CONFIG
                    .lock()
                    .expect("Failed to lock config")
                    .remote_file
                    .clone(),
            );
        info!("Loading rules at {}", rule_path.to_string_lossy());
        let rules = get_rules(rule_path)?;
        let mut scanner = Scanner::new(&rules);
        scanner.max_matches_per_pattern(pointers.config.max_matches);
        match path.extension().unwrap_or_default().to_str() {
            Some("zip") => {
                warn!("Zip files are not supported at the moment and will not be scanned!");
                let mut skipped_locked = pointers
                    .skipped
                    .lock()
                    .map_err(|err| format!("Failed to lock skipped: {err}"))?;
                skipped_locked.push(Skipped {
                    path: path.to_path_buf(),
                    reason: "Zip files unsupported for now".to_owned(),
                });
            }
            None | Some(_) => {
                let result = scanner.scan_file(path).map_err(|err| {
                    let reason = format!("Skipping file {}: {err}", path.to_string_lossy());
                    match pointers.skipped.lock() {
                        Ok(mut skipped_locked) => {
                            skipped_locked.push(Skipped {
                                path: path.to_path_buf(),
                                reason: reason.clone(),
                            });
                            reason
                        }
                        Err(err) => format!("Failed to lock skipped: {err}"),
                    }
                })?;
                Self::evaluate_result(&pointers, file_log, result, path)?;

                // update shared variables
                {
                    let mut analysed_locked = pointers
                        .analysed
                        .lock()
                        .map_err(|err| format!("Failed to lock analysed: {err}"))?;
                    *analysed_locked += 1;
                }
                // send progress to frontend
                Self::progress(&pointers, progress_channel)?;
            }
        }
        Ok(())
    }

    fn progress(
        pointers: &PointerCollection,
        progress_channel: Option<Arc<mpsc::Sender<String>>>,
    ) -> Result<(), String> {
        let analysed_locked = pointers
            .analysed
            .lock()
            .map_err(|err| format!("Failed to lock analysed: {err}"))?;
        let skipped_locked = pointers
            .skipped
            .lock()
            .map_err(|err| format!("Failed to lock skipped: {err}"))?;
        let scanned = *analysed_locked as f64 + skipped_locked.len() as f64;
        let total_locked = pointers
            .total
            .lock()
            .map_err(|err| format!("Failed to lock total: {err}"))?;
        let percentage = (scanned / *total_locked as f64) * 100.0;
        if let Some(channel) = progress_channel {
            channel.send(format!("progress {percentage:.2}"));
            println!("Scan progress: {percentage:.2}%");
        } else {
            println!("Scan progress: {percentage:.2}%");
        }
        Ok(())
    }
}
