use std::sync::Arc;

use log::{trace, warn};
use tauri::Manager;

use crate::{backend::config_file::Config, CONFIG};

#[derive(Clone, serde::Serialize)]
struct TauriEvent {
    message: String,
}

/// loads the global config
pub fn load_config() -> Result<(), String> {
    CONFIG.with(|config| {
        *config.borrow_mut() = Arc::new(Config::new()?);
        Ok(())
    })
}

#[allow(unused)]
/// saves the global config
pub fn save_config() -> Result<(), String>{
    CONFIG.with(|config| {
        config.borrow().save()
    })
}

/// updates the global config to new_config and saves
pub fn update_config(new_config: Config) -> Result<(), String> {
    CONFIG.with(|config| {
        *config.borrow_mut() = Arc::new(new_config);
        (*config.borrow_mut()).save()
    })
}

/// returns the config struct
pub fn get_config() -> Config {
    CONFIG.with(|config| {
        // magic to get to the inner value
        let refcell = config.clone();
        let arc = refcell.borrow_mut();
        let mut cloned_arc = Arc::clone(&arc);
        Arc::make_mut(&mut cloned_arc).to_owned()
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