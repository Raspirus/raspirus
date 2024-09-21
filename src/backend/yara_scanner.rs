use iced::{
    futures::{channel::mpsc, SinkExt, Stream},
    stream::try_channel,
};
use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::Mutex,
};
use threadpool_rs::threadpool::pool::Threadpool;
use yara_x::{ScanResults, Scanner};

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
    analysed: Arc<Mutex<usize>>,
    total: Arc<Mutex<usize>>,
    config: Arc<Config>,
}

impl PointerCollection {
    fn new(path_len: usize) -> Self {
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
    pub progress_sender: Arc<Mutex<mpsc::Sender<Message>>>,
    /// given to the caller to read progress
    pub progress_output: Arc<Mutex<mpsc::Receiver<Message>>>,
    /// current path being scanned
    pub path: Option<PathBuf>,
}

pub enum Progress {
    Percentage(f32),
    Done((Vec<TaggedFile>, Vec<Skipped>, PathBuf)),
}

impl YaraScanner {
    /// creates a new scanenr and imports the yara rules
    pub fn new() -> Self {
        let channel = mpsc::channel(100);
        Self {
            progress_sender: Arc::new(Mutex::new(channel.0)),
            progress_output: Arc::new(Mutex::new(channel.1)),
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
    pub fn start(&self) -> impl Stream<Item = Result<Message, String>> {
        let path = self.path.clone();

        let yarac = CONFIG
            .lock()
            .expect("Failed to lock config")
            .paths
            .clone()
            .expect("Paths not set. Is config initialized?")
            .data
            .join(crate::DEFAULT_FILE);

        try_channel(100, move |mut output| async move {
            let path = path.ok_or("No path set")?;

            // check if rules can load
            get_rules(yarac)?;

            // setup file log
            let file_log = Arc::new(Mutex::new(FileLog::new()?));

            let paths = profile_path(path.to_path_buf())
                .map_err(|err| format!("Failed to calculate file tree: {err}"))?;
            let pointers = PointerCollection::new(paths.len());

            let mut threadpool =
                Threadpool::new(CONFIG.lock().expect("Failed to lock config").max_threads);
            for file in paths {
                let pointers_c = pointers.clone();
                let file_log_c = file_log.clone();
                let progress_c = Arc::new(Mutex::new(output.clone()));
                threadpool.execute(move || {
                    let scan_result = || async {
                        Self::scan_file(file.as_path(), file_log_c, progress_c, pointers_c).await
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

            let log = file_log
                .lock()
                .expect("Failed to lock logger")
                .log_path
                .clone();
            // return tagged and skipped files aswell as path to the scan log
            output.send(Message::ScanComplete {
                tagged: tagged.iter().map(|tag| (tag.clone(), false)).collect(),
                skipped: skipped.iter().map(|skip| (skip.clone(), false)).collect(),
                log,
            });
            Ok(())
        })
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
        progress_channel: Arc<Mutex<mpsc::Sender<Message>>>,
        pointers: PointerCollection,
    ) -> Result<(), String> {
        info!("Scanning {}", path.to_string_lossy());
        let rule_path = pointers
            .config
            .paths
            .clone()
            .ok_or("No paths set. Is config initialized?")?
            .data
            .join(crate::DEFAULT_FILE);
        trace!("Loading rules at {}", rule_path.to_string_lossy());
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
                Self::progress(&pointers, progress_channel).await?;
            }
        }
        Ok(())
    }

    async fn progress(
        pointers: &PointerCollection,
        progress_channel: Arc<Mutex<mpsc::Sender<Message>>>,
    ) -> Result<(), String> {
        let analysed_locked = pointers
            .analysed
            .lock()
            .map_err(|err| format!("Failed to lock analysed: {err}"))?;
        let skipped_locked = pointers
            .skipped
            .lock()
            .map_err(|err| format!("Failed to lock skipped: {err}"))?;
        let scanned = *analysed_locked + skipped_locked.len();
        let total_locked = pointers
            .total
            .lock()
            .map_err(|err| format!("Failed to lock total: {err}"))?;
        let percentage = (scanned as f32 / *total_locked as f32) * 100.0;

        progress_channel
            .lock()
            .map_err(|err| format!("Failed to lock progress sender: {err}"))?
            .send(Message::ScanPercentage { percentage });
        trace!("Scan progress: {percentage:.2}%");
        Ok(())
    }
}
