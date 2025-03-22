use std::{
    fmt::Display,
    fs::File,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use log::{debug, info, trace, warn};

use super::{config::Config, log::Log};

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
    pub config: Arc<Config>,
    pub log: Arc<Log>,
    pub noted_files: Arc<Mutex<Vec<NotableFile>>>,
    pub total_size: Arc<usize>,
    pub rules: Arc<yara_x::Rules>,
}

impl Pointers {
    fn new(config: Config, log: Log, total_size: usize, rules: yara_x::Rules) -> Self {
        Self {
            config: Arc::new(config),
            log: Arc::new(log),
            noted_files: Arc::new(Mutex::new(Vec::new())),
            total_size: Arc::new(total_size),
            rules: Arc::new(rules),
        }
    }
}

/// Starts the scan with the current indexed files
pub async fn start(root: PathBuf) -> Result<(), Error> {
    let config = crate::globals::get_ro_config()?;

    info!("Indexing path...");
    let indexed = Index::new(root)?;

    info!("Preparing rules...");
    let rules = load_rules().await?;

    info!("Preparing scan log...");
    let log = Log::new()?;

    info!("Starting scan...");
    let mut threadpool = threadpool_rs::Threadpool::new(config.max_threads);
    let pointers = Pointers::new(config, log, indexed.total_size, rules);
    for path in indexed.paths {
        let pointers_c = pointers.clone();
        threadpool.execute(move || {
            let _ = scan(pointers_c, path).map_err(|err| warn!("Scan job failed: {err}"));
        });
    }
    threadpool.join();

    Ok(())
}

/// Loads the latest rules, or tries to update
async fn load_rules() -> Result<yara_x::Rules, Error> {
    let local_rules = if let Some(date_time) = crate::backend::updater::get_local_datetime()? {
        dbg!(&date_time);
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
    debug!("Scanning {}...", path.display());
    let mut scanner = yara_x::Scanner::new(&pointers.rules);
    scanner.max_matches_per_pattern(pointers.config.max_matches);

    let results = scanner.scan_file(&path).map_err(Error::ScannerScan)?;
    if results.matching_rules().len() > 0 {
        dbg!(path);
        for out in results.module_outputs() {
            dbg!(out);
        }
    }
    Ok(())
}

/// Adds a skipped file to the pointer array
fn skip(pointers: &Pointers, skip: Skip) -> Result<(), Error> {
    pointers
        .noted_files
        .lock()
        .map_err(|err| Error::ScannerLock(err.to_string()))?
        .push(NotableFile::Skip(skip));
    Ok(())
}
