use std::{
    fs::{self, File},
    path::Path,
    sync::Arc,
};

use log::{trace, warn};
use tauri::Manager;
use zip::ZipArchive;

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
pub fn send(window: &Option<tauri::Window>, event: &str, message: String) {
    if let Some(window) = window {
        trace!("Sending {event}: {message}");
        match window.emit_all(event, message) {
            Ok(_) => {}
            Err(err) => warn!("Failed to send progress to frontend: {err}"),
        }
    }
}

/// calculates progress and sends to frontend if changed. returns new percentage
pub fn send_progress(
    window: &Option<tauri::Window>,
    last_percentage: f32,
    current: usize,
    total: usize,
    event: &str,
) -> Result<f32, String> {
    let new_percentage = ((current as f32 / total as f32) * 100.0).round();
    // if percentage has not changed return new percentage
    if new_percentage == last_percentage {
        return Ok(new_percentage);
    }

    send(window, event, format!("{new_percentage}"));
    Ok(new_percentage)
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

/// gets the unpacked size of a zip file
pub fn size_zip(path: &Path) -> Result<u64, std::io::Error> {
    trace!("Calculating zip: {}", path.display());
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut archive_size = 0;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        archive_size += file.size();
    }
    Ok(archive_size)
}

// gets the size of a file
pub fn size_file(path: &Path) -> Result<u64, std::io::Error> {
    trace!("Calculating file: {}", path.display());
    Ok(
        match path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
        {
            "zip" => size_zip(path)?,
            _ => File::open(path)?.metadata()?.len(),
        },
    )
}

// gets the size of a folder and its contents
pub fn size_folder(path: &Path) -> Result<u64, std::io::Error> {
    trace!("Calculating folder: {}", path.display());
    let mut size = 0;

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry_path = entry?.path();

        if entry_path.is_dir() {
            size += size_folder(&entry_path)?;
        } else if entry_path.is_file() {
            size += size_file(&entry_path)?;
        }
    }

    Ok(size)
}

// gets the size for a given path
pub fn size(path: &Path) -> Result<u64, String> {
    if path.is_dir() {
        match size_folder(path) {
            Ok(size) => Ok(size),
            Err(err) => {
                warn!("Failed to get folder size for scanning: {err}");
                Err(String::from("Failed to get folder size for scanning"))
            }
        }
    } else {
        match size_file(path) {
            Ok(size) => Ok(size),
            Err(err) => {
                warn!("Failed to get file size for scanning: {err}");
                Ok(0)
            }
        }
    }
}
