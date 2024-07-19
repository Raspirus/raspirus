use std::{
    fs::{self, File}, path::PathBuf, sync::Arc, usize
};

use log::{trace, warn};
use tauri::Emitter;
use yara_x::Rules;

use crate::{backend::config_file::Config, CONFIG};

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
    let reader = File::open(yar_path)
        .map_err(|err| format!("Failed to load yar file: {}", err.to_string()))?;
    Rules::deserialize_from(reader)
        .map_err(|err| format!("Failed to deserialize yar file: {}", err.to_string()))
}

/// clears the cache directory
pub fn clear_cache() -> std::io::Result<()> {
    trace!("Clearing caches...");
    let cache_dir = get_config()
        .paths
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No paths set. Is config initialized?",
        ))?
        .cache;

    cache_dir
        .exists()
        .then(|| fs::remove_dir_all(cache_dir))
        .unwrap_or(Ok(()))?;
    Ok(())
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
pub fn profile_folder(paths: &mut Vec<PathBuf>, size: &mut usize, path: PathBuf) -> Result<(), std::io::Error> {
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
pub fn profile_file(paths: &mut Vec<PathBuf>, size: &mut usize, path: PathBuf) -> Result<(), std::io::Error> {
    *size += path.metadata()?.size() as usize;
    paths.push(path);
    Ok(())
}
