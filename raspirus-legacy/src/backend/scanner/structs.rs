use std::{
    fmt::Display,
    path::PathBuf,
    sync::{mpsc, Arc, Mutex},
};

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

/// A collection of pointers meant to be shared between scan threads
#[derive(Clone)]
pub struct Pointers {
    pub log: Arc<Log>,
    pub noted_files: Arc<Mutex<Vec<NotableFile>>>,
    pub rules: Arc<yara_x::Rules>,
    pub channel: Arc<mpsc::Sender<Option<Processing>>>,
}

impl Pointers {
    pub fn new(log: Log, rules: yara_x::Rules, channel: mpsc::Sender<Option<Processing>>) -> Self {
        Self {
            log: Arc::new(log),
            noted_files: Arc::new(Mutex::new(Vec::new())),
            rules: Arc::new(rules),
            channel: Arc::new(channel),
        }
    }
}

/// Holds a path to a file and what status it has now entered
#[derive(Debug, Clone)]
pub struct Processing {
    pub path: PathBuf,
    pub status: Status,
}

/// Status update message sent by scan threads
#[derive(Debug, Clone)]
pub enum Status {
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
                Status::Error(error, _) => format!("[Err] {error}"),
                Status::Completed(_) => "[OK]".to_owned(),
                Status::Started => "[..]".to_owned(),
            }
        )
    }
}
