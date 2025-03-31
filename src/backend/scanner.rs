use std::{
    collections::VecDeque,
    fmt::Display,
    fs::File,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, Mutex},
};

use log::{debug, info, trace, warn};

use crate::globals::{get_max_matches, get_min_matches};

use super::log::Log;

type Error = crate::Error;

#[derive(Clone)]
pub enum NotableFile {
    Skip(Skip),
    Flag(Flag),
}

/// Holds a flagged files path and the rules, which flagged it
#[derive(Clone)]
pub struct Flag {
    pub path: PathBuf,
    pub rules: Vec<String>,
}

/// Holds a skipped files path and the reason for it to be skipped
#[derive(Clone)]
pub struct Skip {
    pub path: PathBuf,
    pub reason: String,
}

impl Display for NotableFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotableFile::Skip(skip) => {
                write!(f, "[skipped]\t{}\t{}", skip.path.display(), skip.reason)
            }
            NotableFile::Flag(flag) => write!(
                f,
                "[flagged]\t{}\t{}",
                flag.path.display(),
                flag.rules.join("\t")
            ),
        }
    }
}

/// Holds all the paths of files, which are contained in a root path
#[derive(Default)]
struct Index {
    /// All paths indexes
    pub paths: Vec<PathBuf>,
    /// Total size in bytes
    pub total_size: usize,
}

impl Index {
    /// Creates a new struct which contains every child file of root
    fn new(root: PathBuf) -> Result<Self, Error> {
        let mut indexed = Self::default();
        if root.is_dir() {
            indexed.index_folder(&root)?;
        }

        if root.is_file() {
            indexed.index_file(&root)?;
        }

        Ok(indexed)
    }

    /// Tries to add a file path to the path list
    fn index_file(&mut self, root: &Path) -> Result<(), Error> {
        // checks if file exists
        if !root.exists() {
            Err(Error::IndexFileNotFound(root.display().to_string()))?
        }

        // checks if permissions suffice, and if file is is_symlink
        let metadata = root.metadata().map_err(|error| Error::IndexPermission {
            path: root.display().to_string(),
            error,
        })?;

        if metadata.is_symlink() {
            Err(Error::IndexSymlink(root.display().to_string()))?
        }

        self.total_size += metadata.len() as usize;
        self.paths.push(root.to_path_buf());
        Ok(())
    }

    /// Tries to add all of a folders subfolders / files to the list of paths
    fn index_folder(&mut self, root: &Path) -> Result<(), Error> {
        // go through all children of a folder
        for entry in
            std::fs::read_dir(root).map_err(|_| Error::IndexEntries(root.display().to_string()))?
        {
            // if we can fetch entry, save its path, otherwise log and skip
            let root = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    warn!("Failed to get entry: {err:?}; Skipping...");
                    continue;
                }
            }
            .path();

            // if entry is file, index it, otherwise index it as folder, or, if neither applies,
            // skip
            if root.is_file() {
                match self.index_file(&root) {
                    Ok(_) => trace!("Indexed {}", root.display()),
                    Err(err) => warn!("{err}; Skipping..."),
                }
            } else if !root.is_symlink() {
                match self.index_folder(&root) {
                    Ok(_) => trace!("Indexed {}", root.display()),
                    Err(err) => warn!("{err}; Skipping..."),
                }
            } else {
                warn!("{} is a symlink; Skipping...", root.display());
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Pointers {
    pub log: Arc<Log>,
    pub noted_files: Arc<Mutex<Vec<NotableFile>>>,
    pub total_size: Arc<usize>,
    pub rules: Arc<yara_x::Rules>,
    pub channel: Arc<mpsc::Sender<Option<Processing>>>,
}

/// Holds a path to a file and what status it has now entered
#[derive(Debug, Clone)]
struct Processing {
    pub path: PathBuf,
    pub status: Status,
}

#[derive(Debug, Clone)]
enum Status {
    // if error is encountered while processing file
    Error(Arc<Error>, Option<usize>),
    // if file successfully completes scanning
    Completed(usize),
    // if file is now being processed
    Started,
}

impl Processing {
    pub fn start(path: &PathBuf) -> Self {
        Self {
            path: path.to_owned(),
            status: Status::Started,
        }
    }

    pub fn error(&mut self, error: Error, size: Option<usize>) {
        self.status = Status::Error(Arc::new(error), size);
    }

    pub fn completed(&mut self, size: usize) {
        self.status = Status::Completed(size)
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Status::Error(error, _) => "[Err]",
                Status::Completed(_) => "[OK]",
                Status::Started => "[..]",
            }
        )
    }
}

impl Pointers {
    fn new(
        log: Log,
        total_size: usize,
        rules: yara_x::Rules,
        channel: mpsc::Sender<Option<Processing>>,
    ) -> Self {
        Self {
            log: Arc::new(log),
            noted_files: Arc::new(Mutex::new(Vec::new())),
            total_size: Arc::new(total_size),
            rules: Arc::new(rules),
            channel: Arc::new(channel),
        }
    }
}

/// Starts the scan with the current indexed files
pub async fn start(root: PathBuf) -> Result<(), Error> {
    info!("Indexing path...");
    let indexed = Index::new(root)?;

    info!("Preparing rules...");
    let rules = load_rules().await?;

    info!("Preparing scan log...");
    let log = Log::new()?;

    info!("Starting scan...");
    let mut threadpool = threadpool_rs::Threadpool::new(crate::globals::get_max_threads());
    let (sender, receiver) = mpsc::channel();

    let pointers = Pointers::new(log, indexed.total_size, rules, sender);
    let watchdog_handle = std::thread::spawn(move || watchdog(receiver, indexed.total_size));

    for path in indexed.paths {
        let pointers_c = pointers.clone();
        threadpool.execute(move || {
            let _ = scan(pointers_c, path).map_err(|err| warn!("Scan job failed: {err}"));
        });
    }
    threadpool.join();
    pointers
        .channel
        .send(None)
        .map_err(|err| Error::WatchdogSend(err.to_string()))?;
    watchdog_handle.join().unwrap()?;

    Ok(())
}

/// Loads the latest rules, or tries to update
async fn load_rules() -> Result<yara_x::Rules, Error> {
    let local_rules = if let Some(date_time) = crate::backend::updater::get_local_datetime()? {
        date_time
    } else {
        crate::backend::updater::update().await?;
        match crate::backend::updater::get_local_datetime()? {
            Some(datetime) => datetime,
            None => Err(Error::ScannerNoRules)?,
        }
    };

    let rule_path = crate::globals::get_ro_config()?
        .get_paths()?
        .data
        .join("yara_c")
        .join(local_rules.format("%Y-%m-%dT%H-%M-%SZ.yarac").to_string());

    debug!("Attempting to load rules at {}", rule_path.display());
    let rule_file = File::open(rule_path).map_err(Error::ScannerRuleLoad)?;
    yara_x::Rules::deserialize_from(rule_file).map_err(Error::ScannerRuleDeserialize)
}

/// Attempts to scan a specific file
fn scan(pointers: Pointers, path: PathBuf) -> Result<(), Error> {
    trace!("Notifying watchdog of {} being started", path.display());
    let mut processing = Processing::start(&path);
    send_update(&pointers.channel, processing.clone())?;

    debug!("Scanning {}...", path.display());
    let mut scanner = yara_x::Scanner::new(&pointers.rules);
    scanner.max_matches_per_pattern(get_max_matches());

    if !path.exists() {
        processing.error(Error::ScannerFileNotFound(path.clone()), None);
        send_update(&pointers.channel, processing.clone())?;
        return Ok(());
    }

    let size = match path.metadata().map_err(Error::ScannerIOError) {
        Ok(metadata) => metadata,
        Err(err) => {
            processing.error(err, None);
            send_update(&pointers.channel, processing)?;
            return Ok(());
        }
    }
    .len() as usize;

    // collect results from yara scanner
    let results = match scanner.scan_file(&path) {
        Ok(results) => results,
        Err(err) => {
            skip(
                &pointers,
                Skip {
                    path: path.clone(),
                    reason: err.to_string(),
                },
            )?;
            processing.error(Error::ScannerScan(err), Some(size));
            send_update(&pointers.channel, processing)?;
            return Ok(());
        }
    };

    // check if rule count qualifies to be marked as notable file
    if results.matching_rules().len() > get_min_matches() {
        pointers.log.log(NotableFile::Flag(Flag {
            path,
            rules: Vec::new(),
        }))?;
    }

    processing.completed(size);
    send_update(&pointers.channel, processing)
}

fn send_update(
    sender: &Arc<mpsc::Sender<Option<Processing>>>,
    processing: Processing,
) -> Result<(), Error> {
    sender
        .send(Some(processing))
        .map_err(|err| Error::WatchdogSend(err.to_string()))
}

fn watchdog(receiver: mpsc::Receiver<Option<Processing>>, total_size: usize) -> Result<(), Error> {
    let display_limit = crate::globals::get_max_threads();
    let mut scanned_size = 0;
    let mut running = VecDeque::new();
    let mut completed = VecDeque::new();
    while let Some(processing) = receiver.recv().map_err(Error::WatchdogRecv)? {
        match processing.status {
            Status::Completed(size) | Status::Error(_, Some(size)) => {
                // move from running to completed
                running.retain(|(path, _)| *path != processing.path);
                completed.push_back((processing.path, processing.status));
                // trim completed list to have proper length
                (completed.len() > display_limit).then(|| completed.pop_front());
                scanned_size += size
            }
            Status::Started | Status::Error(_, None) => {
                running.push_back((processing.path, processing.status))
            }
        }

        // print current queue
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{:.2}%", (scanned_size as f64 / total_size as f64) * 100.0);
        let mut merged = Vec::new();
        merged.extend(&completed);
        merged.extend(&running);
        merged
            .iter()
            .for_each(|elem| println!("{} {}", elem.1, elem.0.display()));
    }
    debug!("Stopping watchdog");
    Ok(())
}

/// Adds a skipped file to the pointer array
fn skip(pointers: &Pointers, skip: Skip) -> Result<(), Error> {
    pointers
        .noted_files
        .lock()
        .map_err(|err| Error::ScannerLock(err.to_string()))?
        .push(NotableFile::Skip(skip.clone()));
    pointers.log.log(NotableFile::Skip(skip))?;
    Ok(())
}
