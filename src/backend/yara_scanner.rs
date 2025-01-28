//use iced::futures::{channel::mpsc, SinkExt};
use log::{error, info};
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

use crate::{backend::utils::generic::get_rules, CONFIG};

use super::{config_file::Config, file_log::FileLog};

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
    config: Arc<Config>,
}

impl PointerCollection {
    fn new() -> Self {
        Self {
            tagged: Arc::new(Mutex::new(Vec::new())),
            skipped: Arc::new(Mutex::new(Vec::new())),
            config: Arc::from(CONFIG.lock().expect("Failed to lock config").clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct YaraScanner {}

impl YaraScanner {
    /// creates a new scanenr and imports the yara rules
    pub fn new() -> Self {
        Self {}
    }

    /// Starts the scanner in the specified location
    pub fn start(
        &self,
        //channel: mpsc::Sender<u64>,
        mut paths: Vec<PathBuf>,
    ) -> Result<(Vec<TaggedFile>, Vec<Skipped>, usize, PathBuf), String> {
        let start_time = std::time::Instant::now();

        let yarac = CONFIG
            .lock()
            .expect("Failed to lock config")
            .paths
            .clone()
            .expect("Paths not set. Is config initialized?")
            .data
            .join(crate::DEFAULT_FILE);

        info!("Preparing rules. This might take a second...");
        // check if rules load and cache them for threads
        let rules = Arc::new(get_rules(yarac)?);
        info!("Rules loaded!");

        // setup file log with timestamp of scan start
        let mut file_log = FileLog::new()?;

        let pointers = PointerCollection::new();

        let paths_count = paths.len();

        let mut threadpool =
            Threadpool::new(CONFIG.lock().expect("Failed to lock config").max_threads);

        for _ in 0..paths_count {
            let file = match paths.pop() {
                Some(path) => path,
                None => break,
            };
            let pointers_c = pointers.clone();
            //let progress_c = channel.clone();
            let rules_c = rules.clone();
            threadpool.execute(move || {
                match futures::executor::block_on(Self::scan_file(
                    file.as_path(),
                    //progress_c,
                    pointers_c,
                    rules_c,
                )) {
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

        info!("Writing tagged files to log file...");
        for tagged in &tagged {
            file_log.log(tagged);
        }

        info!(
            "Scanned {paths_count} files in {:.2}s",
            std::time::Instant::now()
                .duration_since(start_time)
                .as_secs_f32()
        );
        // return tagged and skipped files aswell as path to the scan log
        Ok((tagged, skipped, paths_count, file_log.log_path.clone()))
    }

    fn evaluate_result(
        pointers: &PointerCollection,
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
            {
                let mut tagged_locked = pointers
                    .tagged
                    .lock()
                    .map_err(|err| format!("Failed to lock tagged: {err}"))?;
                tagged_locked.push(tagged_file)
            }
        }
        Ok(())
    }

    /// thread function to scan a singular file
    async fn scan_file(
        path: &Path,
        //progress_channel: mpsc::Sender<u64>,
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

        // size of entire file
        /*
        let size = path
            .metadata()
            .map_err(|err| format!("Failed to fetch file metadata: {err}"))?
            .len();
*/

        if archive {
            // open zip file
            let mut zip = match zip::ZipArchive::new(BufReader::new(match File::open(path) {
                Ok(file) => file,
                Err(err) => {
                    Self::skip(
                        &pointers.skipped,
                        Skipped {
                            path: path.to_path_buf(),
                            reason: format!("Could not open archive: {err}"),
                        },
                    )?;
                    error!(
                        "Could not open archive file {}: {err}",
                        path.to_string_lossy()
                    );
                    // we skip archive entirely since we cannot open it
                    //Self::progress(size, progress_channel).await?;
                    return Ok(());
                }
            })) {
                Ok(zip) => zip,
                Err(err) => {
                    Self::skip(
                        &pointers.skipped,
                        Skipped {
                            path: path.to_path_buf(),
                            reason: format!("Could not open archive: {err}"),
                        },
                    )?;
                    error!("Could not open archive {}: {err}", path.to_string_lossy());
                    //Self::progress(size, progress_channel).await?;
                    return Ok(());
                }
            };

            let length = zip.len();
            for i in 0..length {
                // try to fetch file from zip
                let mut file = match zip.by_index(i) {
                    Ok(file) => file,
                    Err(err) => {
                        Self::skip(
                            &pointers.skipped,
                            Skipped {
                                path: path.to_path_buf(),
                                reason: format!("Could not get file in zip: {err}"),
                            },
                        )?;
                        //Self::progress(size, progress_channel.clone()).await?;
                        continue;
                    }
                };
                //let size = file.compressed_size();

                let path = path
                    .to_path_buf()
                    .join(file.enclosed_name().unwrap_or(Path::new("").to_path_buf()));

                // found archive inside current archive -> skip
                if let Some(extension) = path.extension() {
                    if crate::SUPPORTED_ARCHIVES.contains(&extension.to_string_lossy().to_string())
                    {
                        Self::skip(
                            &pointers.skipped,
                            Skipped {
                                path: path.to_path_buf(),
                                reason: "Nested archives unsupported currently".to_owned(),
                            },
                        )?;
                        //Self::progress(size, progress_channel.clone()).await?;
                        continue;
                    }
                }

                // compressed file is larger than threshhold
                if file.size() > crate::MAX_ZIP_FILE_SIZE {
                    Self::skip(
                        &pointers.skipped,
                        Skipped {
                            path,
                            reason: "File in zip exceeds size threshhold".to_owned(),
                        },
                    )?;
                    //Self::progress(size, progress_channel.clone()).await?;
                    continue;
                }

                info!(
                    "{:<20} archived file {}",
                    format!("Scanning {}/{}", i + 1, length),
                    path.to_string_lossy()
                );
                let mut content = Vec::new();
                // try to read content into byte vector
                match file.read_to_end(&mut content).map_err(|err| {
                    format!(
                        "Failed to read zipped file {}: {err}",
                        path.to_string_lossy()
                    )
                }) {
                    Ok(_) => {}
                    Err(reason) => {
                        Self::skip(&pointers.skipped, Skipped { path, reason })?;
                        //Self::progress(size, progress_channel.clone()).await?;
                        continue;
                    }
                };

                let result = match scanner
                    .scan(&content)
                    .map_err(|err| format!("Could not scan file {}: {err}", path.to_string_lossy()))
                {
                    Ok(result) => result,
                    Err(reason) => {
                        Self::skip(&pointers.skipped, Skipped { path, reason })?;
                        //Self::progress(size, progress_channel.clone()).await?;
                        continue;
                    }
                };
                Self::evaluate_result(&pointers, result, &path)?;
                // send size progress to frontend
                //Self::progress(size, progress_channel.clone()).await?;
            }
        } else {
            match scanner.scan_file(path) {
                Err(err) => {
                    let reason = format!("Skipping file {}: {err}", path.to_string_lossy());
                    Self::skip(
                        &pointers.skipped,
                        Skipped {
                            path: path.to_path_buf(),
                            reason: reason.clone(),
                        },
                    )?;
                    //Self::progress(size, progress_channel.clone()).await?;
                    return Ok(());
                }
                Ok(result) => {
                    Self::evaluate_result(&pointers, result, path)?;
                    // send progress to frontend
                    //Self::progress(size, progress_channel.clone()).await?;
                }
            }
        }
        Ok(())
    }

    fn skip(skipped: &Arc<Mutex<Vec<Skipped>>>, file: Skipped) -> Result<(), String> {
        skipped
            .lock()
            .map_err(|err| format!("Failed to lock skipped: {err}"))?
            .push(file);
        Ok(())
    }

    /*
    async fn progress(size: u64, mut progress_channel: mpsc::Sender<u64>) -> Result<(), String> {
        progress_channel
            .send(size)
            .await
            .map_err(|err| format!("Failed to send size to frontend: {err}"))?;
        Ok(())
    }
    */
}
