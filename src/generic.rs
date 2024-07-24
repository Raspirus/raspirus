use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct UsbDevice {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub rules_version: String,
    pub min_matches: usize,
    pub max_matches: usize,
    pub logging_is_active: bool,
    pub scan_dir: bool,
    pub mirror: String,
    pub remote_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsStruct {
    pub logging_is_active: bool,
    pub min_matches: usize,
    pub max_matches: usize,
    pub scan_dir: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsArgs {
    pub contents: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScannerArgs {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaggedFile {
    pub path: PathBuf,
    /// vector of description and rule name
    pub descriptions: Vec<RuleFeedback>,
    pub rule_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleFeedback {
    pub rule_name: String,
    pub rule_description: String,
}
