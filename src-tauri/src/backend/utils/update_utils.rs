use log::{error, info, warn};
use reqwest::StatusCode;
use std::fs::File;
use std::fs::{self, DirEntry};
use std::io::{BufRead, BufReader};
use std::{path::Path, time};

use crate::backend::db_ops::DBOps;
use crate::backend::utils::generic::{clear_cache, send, send_progress, update_config};

use super::generic::get_config;

/// Checks if local is running behind remote. Returns true if remote is newer
pub fn check_update_necessary() -> Result<bool, std::io::Error> {
    // get local timestamp
    let local_timestamp = get_config().last_db_update;
    // fetch remote timestamp
    let remote_timestamp = get_remote_timestamp()?;

    Ok(remote_timestamp != local_timestamp)
}

/// fetches remote timestamp from mirror
pub fn get_remote_timestamp() -> Result<String, std::io::Error> {
    let config = get_config();
    let file_url = format!("{}/timestamp", config.mirror.clone());

    let client = reqwest::blocking::Client::new();
    for current_retry in 0..=crate::MAX_RETRY {
        let response = match client.get(&file_url).send() {
            Ok(response) => response,
            Err(err) => {
                warn!("Failed to download {file_url} on try {current_retry}: {err}");
                continue;
            }
        };

        // if ok we write to file, otherwise we retry
        match response.status() {
            StatusCode::OK => match response.text() {
                Ok(data) => return Ok(data),
                Err(err) => warn!("Failed to download {file_url} on try {current_retry}: {err}"),
            },
            _ => warn!(
                "Failed to download {file_url} on try {current_retry}; Statuscode was {}",
                response.status()
            ),
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::ConnectionAborted,
        "Could not download timestamp",
    ))
}

/// updates if update is necessary
pub fn update(window: Option<tauri::Window>) -> Result<String, String> {
    send(&window, "chck", String::new());
    // if remote is not newer than local we skip
    if !check_update_necessary().map_err(|err| format!("Failed to check for updates: {err}"))? {
        info!("Database already up to date. Skipping...");
        return Ok("100".to_owned());
    }

    info!("Updating database...");
    let data_dir = get_config()
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .data;

    // try to get a usable database path
    let mut config = get_config();
    let db_path = Path::new(&config.db_location);
    let db_file_str = if !config.db_location.is_empty() && db_path.exists() && db_path.is_file() {
        info!("Using specific DB path {}", config.db_location);
        config.db_location.clone()
    } else {
        // if not we use the default path
        data_dir.join(crate::DB_NAME).display().to_string()
    };

    // connect to database
    let mut db_connection = DBOps::new(db_file_str.as_str()).map_err(|err| {
        error!("{err}");
        err.to_string()
    })?;

    // Actually run the update
    let big_tic = time::Instant::now();
    match db_connection.update_db(&window) {
        Ok(res) => {
            // write remote timestamp to config
            let timestamp = get_remote_timestamp().map_err(|err| err.to_string())?;
            config.last_db_update = timestamp;
            update_config(config)?;

            clear_cache().map_err(|err| err.to_string())?;

            let big_toc = time::Instant::now();
            info!(
                "Updated DB in {} seconds",
                big_toc.duration_since(big_tic).as_secs_f64()
            );
            Ok(res.to_string())
        }
        Err(err) => {
            error!("{err}");
            Err(err.to_string())
        }
    }
}

pub fn insert_all(db: &mut DBOps, window: &Option<tauri::Window>) -> Result<(), String> {
    let start_time = std::time::Instant::now();
    let cache_dir = get_config()
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .cache;

    // get all files from a folder
    let entries: Vec<DirEntry> = fs::read_dir(&cache_dir)
        .map_err(|err| err.to_string())?
        .filter_map(Result::ok)
        .collect();

    // read all files line by line into buffer
    let mut p = 0.0;
    let mut i = 0;
    let len = entries.len();
    for file in entries {
        let file = File::open(file.path()).map_err(|err| err.to_string())?;
        let reader = BufReader::new(file);

        // add readers lines to lines vector
        match db.insert_hashes(&reader.lines().map_while(Result::ok).collect()) {
            Ok(_) => {}
            Err(err) => warn!("Error inserting: {err}"),
        }
        i += 1;
        p = send_progress(window, p, i, len, "ins")?;
    }

    info!(
        "Building database took {}s",
        std::time::Instant::now()
            .duration_since(start_time)
            .as_secs_f32()
    );
    Ok(())
}
