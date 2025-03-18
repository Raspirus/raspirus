use thiserror::Error;

use crate::backend::config::Config;

#[derive(Error, Debug)]
#[allow(unused)]
/// Custom error variant
pub enum Error {
    /// Arbitrary error holding just a string
    #[error("Unclassified error: {0}")]
    Other(String),

    /// Holds all types of errors encountered when interacting with the filesystem via config
    #[error("Filesystem interaction failed: {0}")]
    ConfigIOError(std::io::Error),
    /// Thrown when the config fails to be built into a string
    #[error("Failed to deserialize config: {0}")]
    ConfigDeserializationError(serde_json::Error),
    /// Thrown when the config fails to be built from a string
    #[error("Failed to serialize config: {0}")]
    ConfigSerializationError(serde_json::Error),
    /// Thrown when config fails to lock
    #[error("Failed to lock config: {0}")]
    ConfigLockError(String),

    /// File logger IO error
    #[error("Failed to create logfile: {0}")]
    LogIOError(std::io::Error),
    /// File logger log error
    #[error("Failed to log to file: {0}")]
    LogLogError(std::io::Error),
    /// Logger init error
    #[error("Failed to setup logger")]
    LogInitError(log::SetLoggerError),

    /// Thrown when a path given for indexing does not exist
    #[error("File {0} path does not exist")]
    IndexFileNotFoundError(String),
    /// Thrown when user has invalid permissions to access a file
    #[error("File {path} requires elevated permissions: {error}")]
    IndexPermissionError { path: String, error: std::io::Error },
    /// Thrown when given path is a symlink
    #[error("File {0} is symlink")]
    IndexSymlinkError(String),
    /// Thrown when we fail to get the entries of a folder
    #[error("Failed to get entries for {0}")]
    IndexEntriesError(String),
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, Config>>> for Error {
    fn from(value: std::sync::PoisonError<std::sync::MutexGuard<crate::backend::config::Config>>) -> Self {
        Self::ConfigLockError(value.to_string())
    }
}

