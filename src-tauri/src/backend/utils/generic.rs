use std::{
    fs::{self, File}, io::{BufReader, Read}, path::PathBuf, sync::Arc
};

use log::{info, trace, warn};
use sha2::{Digest, Sha256};
use tauri::Emitter;
use yara_x::Rules;

use crate::{
    backend::{config_file::Config, yara_scanner::TaggedFile},
    CONFIG,
};

#[allow(unused)]
/// saves the global config
pub fn save_config() -> Result<(), String> {
    CONFIG.with(|config| config.borrow().save())
}

/// updates the global config to new_config and saves
pub fn update_config(new_config: Config) -> Result<(), String> {
    CONFIG.with(|config| {
        *config.borrow_mut() = Arc::new(new_config);
        save_config()
    })
}

/// returns the config struct
pub fn get_config() -> Config {
    CONFIG.with(|config| {
        let clone = (*config.borrow()).clone();
        (*clone).clone()
    })
}

/// sends given percentage to the frontend
pub fn send(window: &Option<Arc<tauri::Window>>, event: &str, message: String) {
    if let Some(window) = window {
        trace!("Sending {event}: {message}");
        match window.emit(event, message) {
            Ok(_) => {}
            Err(err) => warn!("Failed to send progress to frontend: {err}"),
        }
    }
}

pub fn get_rules() -> Result<Rules, String> {
    let yar_path = get_config()
        .paths
        .ok_or("No paths set. Is config initialized?")?
        .data
        .join(get_config().remote_file);
    // setup rules
    let reader = File::open(yar_path).map_err(|err| format!("Failed to load yar file: {err}"))?;
    Rules::deserialize_from(reader).map_err(|err| format!("Failed to deserialize yar file: {err}"))
}

/// yields all file paths and the total size of them
pub fn profile_path(path: PathBuf) -> Result<(Vec<PathBuf>, usize), std::io::Error> {
    let mut paths = Vec::new();
    let mut size = 0;
    if path.is_dir() {
        profile_folder(&mut paths, &mut size, path)?;
    } else {
        profile_file(&mut paths, &mut size, path)?;
    }
    Ok((paths, size))
}

/// adds files or files in subfolders to paths and adds their sizes to the total
pub fn profile_folder(
    paths: &mut Vec<PathBuf>,
    size: &mut usize,
    path: PathBuf,
) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            profile_folder(paths, size, entry.path())?;
        } else {
            profile_file(paths, size, entry.path())?;
        }
    }
    Ok(())
}

/// adds file to paths and adds its size to the total
pub fn profile_file(
    paths: &mut Vec<PathBuf>,
    size: &mut usize,
    path: PathBuf,
) -> Result<(), std::io::Error> {
    *size += path.metadata()?.len() as usize;
    paths.push(path);
    Ok(())
}

/// calculates sha256 hash and generates virustotal search link
pub fn generate_virustotal(file: TaggedFile) -> Result<String, String> {
    info!("Starting hash compute for {}", file.path.to_string_lossy());
    let file = File::open(file.path).map_err(|err| format!("Failed to open file for computing hash: {err}"))?;
    
    let mut reader = BufReader::new(file);
    let mut sha256 = Sha256::new();

    loop {
        let mut buffer = [0; 524288];
        let read = reader.read(&mut buffer).map_err(|err| format!("Failed to read into buffer: {err}"))?;
        if read == 0 {
            break;
        }
        sha256.update(&buffer[..read]);
    }
    let result = sha256.finalize();
    Ok(format!("https://virustotal.com/gui/search/{}", hex::encode(result)))
}
