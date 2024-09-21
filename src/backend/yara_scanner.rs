use iced::futures::{channel::mpsc, SinkExt};
use log::{error, info, trace};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Arc;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::Mutex,
};
use threadpool_rs::threadpool::pool::Threadpool;
use yara_x::{Rules, ScanResults, Scanner};

use crate::frontend::iced::Worker;
use crate::{backend::utils::generic::get_rules, frontend::iced::Message, CONFIG};

use super::{config_file::Config, file_log::FileLog, utils::generic::profile_path};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TaggedFile {
    pub path: PathBuf,
    /// vector of description and rule name
    pub descriptions: Vec<RuleFeedback>,
    pub rule_count: usize,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RuleFeedback {
    pub rule_name: String,
    pub rule_description: String,
}

impl Display for RuleFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.rule_name, self.rule_description)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Skipped {
    pub path: PathBuf,
    pub reason: String,
}

/// collection of pointers for the scan threads
#[derive(Clone)]
pub struct PointerCollection {
    tagged: Arc<Mutex<Vec<TaggedFile>>>,
    skipped: Arc<Mutex<Vec<Skipped>>>,
    analysed: Arc<Mutex<u64>>,
    total: Arc<Mutex<u64>>,
    config: Arc<Config>,
}

impl PointerCollection {
    fn new(path_len: u64) -> Self {
        Self {
            tagged: Arc::new(Mutex::new(Vec::new())),
            skipped: Arc::new(Mutex::new(Vec::new())),
            analysed: Arc::new(Mutex::new(0)),
            total: Arc::new(Mutex::new(path_len)),
            config: Arc::from(CONFIG.lock().expect("Failed to lock config").clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct YaraScanner {
    /// given to threads to send their progress when done
    pub progress_sender: Option<Arc<Mutex<mpsc::Sender<Worker>>>>,
    /// current path being scanned
    pub path: Option<PathBuf>,
}

impl YaraScanner {
    /// creates a new scanenr and imports the yara rules
    pub fn new() -> Self {
        Self {
            progress_sender: None,
            path: None,
        }
    }

    /// sets the path to scan
    pub fn set_path(&self, path: PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Err("Invalid path".to_owned());
        }

        let mut scanner = self.clone();
        scanner.path = Some(path);
        Ok(scanner)
    }

    /// Starts the scanner in the specified location
    pub fn start(&self) -> Result<(Vec<TaggedFile>, Vec<Skipped>, PathBuf), String> {
        let start_time = std::time::Instant::now();
        let path = match &self.path {
            Some(path) => path,
            None => return Err("No path set".to_owned()),
        };

        let yarac = CONFIG
            .lock()
            .expect("Failed to lock config")
            .paths
            .clone()
            .expect("Paths not set. Is config initialized?")
            .data
            .join(crate::DEFAULT_FILE);

        // check if rules load and cache them for threads
        let rules = Arc::new(get_rules(yarac)?);

        // setup file log
        let file_log = Arc::new(Mutex::new(FileLog::new()?));

        let paths = profile_path(path.to_path_buf())
            .map_err(|err| format!("Failed to calculate file tree: {err}"))?;
        let paths_count = paths.1.len();
        let pointers = PointerCollection::new(paths.0);

        let mut threadpool =
            Threadpool::new(CONFIG.lock().expect("Failed to lock config").max_threads);
        for file in paths.1 {
            let pointers_c = pointers.clone();
            let file_log_c = file_log.clone();
            let progress_c = self.progress_sender.clone().ok_or("Channel not set")?;
            let rules_c = rules.clone();
            threadpool.execute(move || {
                let scan_result = || async {
                    Self::scan_file(file.as_path(), file_log_c, progress_c, pointers_c, rules_c)
                        .await
                };
                match futures::executor::block_on(scan_result()) {
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

        let logger = file_log.lock().expect("Failed to lock logger");
        info!(
            "Scanned {paths_count} files in {:.2}s",
            std::time::Instant::now()
                .duration_since(start_time)
                .as_secs_f32()
        );
        // return tagged and skipped files aswell as path to the scan log
        Ok((tagged, skipped, logger.log_path.clone()))
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
            let tagged_file = TaggedFile {
                path: path.to_path_buf(),
                descriptions,
                rule_count,
            };
            let mut file_log_locked = file_log
                .lock()
                .map_err(|err| format!("Failed to lock file logger: {err}"))?;
            file_log_locked.log(&tagged_file);
            let mut tagged_locked = pointers
                .tagged
                .lock()
                .map_err(|err| format!("Failed to lock tagged: {err}"))?;
            tagged_locked.push(tagged_file)
        }
        Ok(())
    }

    /// thread function to scan a singular file
    async fn scan_file(
        path: &Path,
        file_log: Arc<Mutex<FileLog>>,
        progress_channel: Arc<Mutex<mpsc::Sender<Worker>>>,
        pointers: PointerCollection,
        rules: Arc<Rules>,
    ) -> Result<(), String> {
        info!("Scanning {}", path.to_string_lossy());
        let mut scanner = Scanner::new(&rules);
        scanner.max_matches_per_pattern(pointers.config.max_matches);

        // check if archive format is supported
        let archive = if let Some(extension) = path.extension().unwrap_or_default().to_str() {
            crate::SUPPORTED_ARCHIVES.contains(&extension.to_owned())
        } else {
            false
        };

        let size = path
            .metadata()
            .map_err(|err| format!("Failed to fetch file metadata: {err}"))?
            .len();

        if archive {
            // open zip file
            let mut zip = match zip::ZipArchive::new(BufReader::new(match File::open(path) {
                Ok(file) => file,
                Err(err) => {
                    let err = format!("Could not open archive: {err}");
                    pointers
                        .skipped
                        .lock()
                        .map_err(|err| format!("Failed to lock skipped: {err}"))?
                        .push(Skipped {
                            path: path.to_path_buf(),
                            reason: err.clone(),
                        });
                    return Err(err);
                }
            })) {
                Ok(zip) => zip,
                Err(err) => {
                    let err = format!("Could not open archive: {err}");
                    pointers
                        .skipped
                        .lock()
                        .map_err(|err| format!("Failed to lock skipped: {err}"))?
                        .push(Skipped {
                            path: path.to_path_buf(),
                            reason: err.clone(),
                        });
                    return Err(err);
                }
            };

            for i in 0..zip.len() {
                let size = zip.len();
                let mut file = match zip.by_index(i) {
                    Ok(file) => file,
                    Err(err) => {
                        pointers
                            .skipped
                            .lock()
                            .map_err(|err| format!("Failed to lock skipped: {err}"))?
                            .push(Skipped {
                                path: path.to_path_buf(),
                                reason: format!("Could not get file in zip: {err}"),
                            });
                        continue;
                    }
                };

                let path = path
                    .to_path_buf()
                    .join(file.enclosed_name().unwrap_or(Path::new("").to_path_buf()));

                if let Some(extension) = path.extension() {
                    if crate::SUPPORTED_ARCHIVES.contains(&extension.to_string_lossy().to_string())
                    {
                        pointers
                            .skipped
                            .lock()
                            .map_err(|err| format!("Failed to lock skipped: {err}"))?
                            .push(Skipped {
                                path: path.to_path_buf(),
                                reason: format!("Nested archives unsupported currently"),
                            });
                        continue;
                    }
                }

                // compressed file is larger than threshhold
                if file.size() > crate::MAX_ZIP_FILE_SIZE {
                    pointers
                        .skipped
                        .lock()
                        .map_err(|err| format!("Failed to lock skipped: {err}"))?
                        .push(Skipped {
                            path,
                            reason: "File in zip exceeds size threshhold".to_owned(),
                        });
                    continue;
                }

                info!(
                    "Scanning {}/{} archived file {}",
                    i + 1,
                    size + 1,
                    path.to_string_lossy()
                );
                let mut content = Vec::new();
                match file.read_to_end(&mut content).map_err(|err| {
                    format!(
                        "Failed to read zipped file {}: {err}",
                        path.to_string_lossy()
                    )
                }) {
                    Ok(_) => {}
                    Err(reason) => {
                        pointers
                            .skipped
                            .lock()
                            .map_err(|err| format!("Failed to lock skipped: {err}"))?
                            .push(Skipped { path, reason });
                        continue;
                    }
                };

                let result = match scanner
                    .scan(&content)
                    .map_err(|err| format!("Could not scan file {}: {err}", path.to_string_lossy()))
                {
                    Ok(result) => result,
                    Err(reason) => {
                        pointers
                            .skipped
                            .lock()
                            .map_err(|err| format!("Failed to lock skipped: {err}"))?
                            .push(Skipped { path, reason });
                        continue;
                    }
                };
                Self::evaluate_result(&pointers, file_log.clone(), result, &path)?;
                // update shared variables
                {
                    let mut analysed_locked = pointers
                        .analysed
                        .lock()
                        .map_err(|err| format!("Failed to lock analysed: {err}"))?;
                    *analysed_locked += file.compressed_size();
                    // send progress to frontend
                }
                Self::progress(&pointers, progress_channel.clone()).await?;
            }
        } else {
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
                *analysed_locked += size;
                // send progress to frontend
            }
            Self::progress(&pointers, progress_channel).await?;
        }
        Ok(())
    }

    async fn progress(
        pointers: &PointerCollection,
        progress_channel: Arc<Mutex<mpsc::Sender<Worker>>>,
    ) -> Result<(), String> {
        let analysed_locked = pointers
            .analysed
            .lock()
            .map_err(|err| format!("Failed to lock analysed: {err}"))?
            .clone();
        let skipped_locked = pointers
            .skipped
            .lock()
            .map_err(|err| format!("Failed to lock skipped: {err}"))?
            .clone();
        let scanned = analysed_locked + skipped_locked.len() as u64;
        let total_locked = pointers
            .total
            .lock()
            .map_err(|err| format!("Failed to lock total: {err}"))?
            .clone();
        let percentage = (scanned as f32 / total_locked as f32) * 100.0;

        progress_channel
            .lock()
            .map_err(|err| format!("Failed to lock progress sender: {err}"))?
            .send(Worker::Message {
                message: Message::ScanPercentage { percentage },
            })
            .await
            .expect("Failed to send progress");
        trace!("Scan progress: {percentage:.2}%");
        Ok(())
    }
}
