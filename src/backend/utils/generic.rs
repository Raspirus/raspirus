use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::PathBuf,
};

use log::info;
use sha2::{Digest, Sha256};
use yara_x::Rules;

use crate::frontend::iced::ConfigValue;

pub fn get_rules(yar_path: PathBuf) -> Result<Rules, String> {
    // setup rules
    let reader = File::open(yar_path).map_err(|err| format!("Failed to load yar file: {err}"))?;
    Rules::deserialize_from(reader).map_err(|err| format!("Failed to deserialize yar file: {err}"))
}

/// yields all file paths and the total size of them
pub fn profile_path(path: PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    info!("Starting indexing...");
    let mut paths = Vec::new();
    if path.is_dir() {
        profile_folder(&mut paths, path)?;
    } else {
        paths.push(path);
    }
    info!("Finished indexing {} files", paths.len());
    Ok(paths)
}

/// adds files or files in subfolders to paths and adds their sizes to the total
pub fn profile_folder(paths: &mut Vec<PathBuf>, path: PathBuf) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            profile_folder(paths, entry.path())?;
        } else {
            paths.push(entry.path().clone());
        }
    }
    Ok(())
}

/// calculates sha256 hash and generates virustotal search link
pub fn generate_virustotal(file: PathBuf) -> Result<String, String> {
    info!("Starting hash compute for {}", file.to_string_lossy());
    let file =
        File::open(file).map_err(|err| format!("Failed to open file for computing hash: {err}"))?;

    let mut reader = BufReader::new(file);
    let mut sha256 = Sha256::new();

    loop {
        let mut buffer = [0; 524288];
        let read = reader
            .read(&mut buffer)
            .map_err(|err| format!("Failed to read into buffer: {err}"))?;
        if read == 0 {
            break;
        }
        sha256.update(&buffer[..read]);
    }
    let result = sha256.finalize();
    Ok(format!(
        "https://virustotal.com/gui/search/{}",
        hex::encode(result)
    ))
}

/// updates the global config to what it should be
pub fn update_config(value: ConfigValue) -> Result<(), String> {
    let mut config = crate::CONFIG.lock().map_err(|err| format!("Failed to lock config: {err}"))?;
    match value {
        ConfigValue::MinMatch(min_matches) => config.min_matches = min_matches,
        ConfigValue::MaxMatch(max_matches) => config.max_matches = max_matches,
        ConfigValue::Logging(logging) => config.logging_is_active = logging,
    }
    config.save()?;
    Ok(())
}
