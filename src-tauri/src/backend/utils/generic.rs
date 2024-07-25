use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::PathBuf,
    sync::Arc,
};

use log::{debug, info, trace, warn};
use sha2::{Digest, Sha256};
use tauri::Emitter;
use yara_x::Rules;

use crate::backend::config_file::Config;

/*
/// saves the global config
pub fn save_config() -> Result<(), String> {
    CONFIG.with(|config| config.borrow().save())
}

/// updates the global config to new_config and saves
pub fn update_config(new_config: Config) -> Result<(), String> {
    println!("Updating to: {new_config:#?}");
    CONFIG.with(|config| {
        *config.borrow_mut() = Arc::new(new_config);
        save_config()
    })
}


/// returns the config struct
pub fn get_config() -> Config {
    CONFIG.with(|config| {
        let clone = (*config.borrow()).clone();
        println!("Fetching {clone:#?}");
        (*clone).clone()
    })
}
*/

pub fn update_config(new_config: Config) -> Result<(), String> {
    debug!("Saving {new_config:?}");
    new_config.save()
}

pub fn get_config() -> Config {
    debug!("Loading");
    Config::new().expect("Failed to load config")
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
    for entry in fs::read_dir(&path)? {
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
