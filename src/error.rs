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
    ConfigIO(std::io::Error),
    /// Thrown when the config fails to be built into a string
    #[error("Failed to deserialize config: {0}")]
    ConfigDeserialization(serde_json::Error),
    /// Thrown when the config fails to be built from a string
    #[error("Failed to serialize config: {0}")]
    ConfigSerialization(serde_json::Error),
    /// Thrown when config fails to lock
    #[error("Failed to lock config: {0}")]
    ConfigLock(String),

    /// File logger IO error
    #[error("Failed to create logfile: {0}")]
    LogIO(std::io::Error),
    /// File logger log error
    #[error("Failed to log to file: {0}")]
    LogLog(std::io::Error),
    /// Logger init error
    #[error("Failed to setup logger")]
    LogInit(log::SetLoggerError),

    /// Thrown when a path given for indexing does not exist
    #[error("File {0} path does not exist")]
    IndexFileNotFound(String),
    /// Thrown when user has invalid permissions to access a file
    #[error("File {path} requires elevated permissions: {error}")]
    IndexPermission { path: String, error: std::io::Error },
    /// Thrown when given path is a symlink
    #[error("File {0} is symlink")]
    IndexSymlink(String),
    /// Thrown when we fail to get the entries of a folder
    #[error("Failed to get entries for {0}")]
    IndexEntries(String),

    /// Thrown when request fails to to being offline
    #[error("Failed to fetch remote resource: {0}")]
    RemoteOffline(reqwest::Error),
    /// Thrown when fetched data cannot be serialized. Most likely api limit has been reached
    #[error("Failed to serialize release: {0}")]
    RemoteSerialize(reqwest::Error),
    /// Thrown when something else breaks for whatever reason
    #[error("Failed to get latest remote")]
    RemoteUndefined,
    /// Thrown when the client fails to build
    #[error("Failed to build client: {0}")]
    RemoteClientBuild(reqwest::Error),
    /// Thrown when local yara_c folder cannot be read, or files cannot be accessed
    #[error("Failed to find local yarac file: {0}")]
    RemoteLocalIO(std::io::Error),
    /// Thrown when chrono fails to serialize a datetime from a string
    #[error("Failed to create datetime: {0}")]
    RemoteTime(chrono::ParseError),
    /// Thrown when rules are already updated
    #[error("Already up to date")]
    RemoteAlreadyUpdated,

    /// Thrown when builder cannot read archive
    #[error("Builder failed to manipulate archive: {0}")]
    BuilderArchive(zip::result::ZipError),
    /// Thrown when builder fails io access
    #[error("Builder filesystem interaction failed: {0}")]
    BuilderIO(std::io::Error),
    #[cfg(target_os = "windows")]
    /// Thrown when the powershell script, designed to stop defender from breaking our rules fails
    #[error("Failed to run powershell script for windows defender")]
    BuilderPowershell(std::io::Error),
    /// Thrown when the built rules fail to serialize to a string
    #[error("Failed to serialize freshly built rules: {0}")]
    BuilderSerialization(yara_x::errors::SerializationError),

    /// Thrown when accessing the notable files vec fails
    #[error("Could not lock collection vector: {0}")]
    ScannerLock(String),
    /// Thrown when there are no rules to load, even after attempting an update
    #[error("Could not load rules even after an attempted update")]
    ScannerNoRules,
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, Config>>> for Error {
    fn from(
        value: std::sync::PoisonError<std::sync::MutexGuard<crate::backend::config::Config>>,
    ) -> Self {
        Self::ConfigLock(value.to_string())
    }
}
