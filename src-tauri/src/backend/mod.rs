/// config structs and related functions
pub mod config_file;
/// database functions to abstract underlying sql
pub mod db_ops;
/// mirror downloader and related functions
pub mod downloader;
/// logger used by scanner
pub mod file_log;
/// virus scanner utilizing db_ops
pub mod scanner;
/// various specific function collections
pub mod utils;
/// the revised scanner using yara rules
pub mod yara_scanner;
