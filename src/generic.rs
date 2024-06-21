use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UsbDevice {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    // Saves hash count after update in order to avoid having to recount
    pub hash_count: u32,
    // Last time and date when the db was successfully updated. Is a number we need to format
    pub last_db_update: String,
    // If we should log information to a file
    pub logging_is_active: bool,
    // Check if we should obfuscate the result
    pub obfuscated_is_active: bool,
    // Location of the .db file
    pub db_location: String,
    // If we should scan directories instead of files (You can only choose one on the current file picker dialog)
    pub scan_dir: bool,
    // mirror to folder with hash files for update
    pub mirror: String,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsPatchArgs {
    pub patchfile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsStruct {
    pub logging_is_active: bool,
    pub db_location: String,
    pub scan_dir: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsArgs {
    pub contents: String
}

// A function to convert a big integer to a date string in the format of "DD-MM-YYYY HH:MM:SS"
pub fn int_to_date_string(date_int: i64) -> String {
    let date_time = DateTime::from_timestamp(date_int, 0);
    date_time.unwrap_or_default().format("%d-%m-%Y %H:%M:%S").to_string()
}