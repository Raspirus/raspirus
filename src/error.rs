use thiserror::Error;

#[derive(Error, Debug)]
/// Custom error variant
pub enum Error {
    /// Thrown when an invalid argument was supplied
    #[error("Invalid argument {0}; Try --help")]
    InvalidArgument(String),

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
    /// File logger bufwriter log error
    #[error("Failed to lock writer for logging {0}")]
    LogLock(String),

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
    /// Thrown when scanner cannot open rule file
    #[error("Failed to open rule file: {0}")]
    ScannerRuleLoad(std::io::Error),
    /// Thrown when the rules cannot be deserialized
    #[error("Failed to deserialize rules: {0}")]
    ScannerRuleDeserialize(yara_x::errors::SerializationError),
    /// Thrown when a file has been deleted between getting indexed and scanned
    #[error("File to be scanned is no longer present: {0}")]
    ScannerFileNotFound(std::path::PathBuf),
    /// Thrown when scanner fails to scan file
    #[error("Failed to scan file: {0}")]
    ScannerScan(yara_x::errors::ScanError),
    /// Thrown when the metadata fetch for a file fails
    #[error("Failed to get metadata for file: {0}")]
    ScannerIOError(std::io::Error),
    
    /// Thrown when the watchdog fails
    #[error("Watchdog failed to receive update: {0}")]
    WatchdogRecv(std::sync::mpsc::RecvError),
    #[error("Failed to send update to watchdog: {0}")]
    WatchdogSend(String)
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, crate::backend::config::Config>>> for Error {
    fn from(
        value: std::sync::PoisonError<std::sync::MutexGuard<crate::backend::config::Config>>,
    ) -> Self {
        Self::ConfigLock(value.to_string())
    }
}
