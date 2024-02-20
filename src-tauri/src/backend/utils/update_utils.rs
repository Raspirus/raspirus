use log::{error, info, warn};
use reqwest::StatusCode;
use std::fs::{self, DirEntry};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::{path::Path, time};

use crate::backend::config_file::Config;
use crate::backend::db_ops::DBOps;
use crate::backend::downloader::{calculate_progress, send};

static DB_NAME: &str = "signatures.db";

/// Checks if local is running behind remote. Returns true if remote is newer
pub fn check_update_necessary() -> Result<bool, std::io::Error> {
    let config =
        Config::new().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    // get local timestamp
    let local_timestamp = config.last_db_update;

    // fetch remote timestamp
    let remote_timestamp = get_remote_timestamp()?;

    Ok(remote_timestamp != local_timestamp)
}

/// fetches remote timestamp from mirror
pub fn get_remote_timestamp() -> Result<String, std::io::Error> {
    let config =
        Config::new().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    let file_url = format!("{}/timestamp", config.mirror.clone());

    let client = reqwest::blocking::Client::new();
    for current_retry in 0..=crate::backend::downloader::MAX_RETRY {
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
    if !check_update_necessary().map_err(|err| err.to_string())? {
        info!("Database already up to date. Skipping...");
        return Ok("100".to_owned());
    }

    info!("Updating database...");
    let mut config = Config::new()?;
    let project_dir = match config.program_path.as_ref() {
        Some(project_dir) => Ok(project_dir),
        None => Err(String::from("Failed to get project directories.")),
    }?;
    let program_dir = project_dir.data_dir();

    // try to get a usable database path
    let db_file_str = if !config.db_location.is_empty()
        && Path::new(&config.db_location).to_owned().exists()
        && Path::new(&config.db_location).to_owned().is_file()
    {
        info!("Using specific DB path {}", config.db_location);
        config.db_location.clone()
    } else {
        // if not we use the default path
        program_dir.join(DB_NAME).to_string_lossy().to_string()
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
            config.save().map_err(|err| err.to_string())?;

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
    let config = Config::new()?;
    let project_dir = match config.program_path.as_ref() {
        Some(project_dir) => Ok(project_dir),
        None => Err(String::from("Failed to get project directories.")),
    }?;
    let cache_dir = project_dir.cache_dir();

    // get all files from a folder
    let entries: Vec<DirEntry> = fs::read_dir(cache_dir)
        .map_err(|err| err.to_string())?
        .filter_map(Result::ok)
        .collect();

    // read all files line by line into buffer
    let mut p = 0.0;
    let mut i = 0;
    let len = entries.len();
    for file in entries {
        let mut lines: Vec<String> = Vec::new();
        let file = File::open(file.path()).map_err(|err| err.to_string())?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .map_while(Result::ok)
            .for_each(|line| lines.push(line));

        match db.insert_hashes(&lines) {
            Ok(_) => {}
            Err(err) => warn!("Error inserting: {err}"),
        }
        i += 1;
        p = calculate_progress(window, p, i, len, "ins")?;
    }

    info!(
        "Building database took {}s",
        std::time::Instant::now()
            .duration_since(start_time)
            .as_secs_f32()
    );

    info!("Clearing cache...");
    let _ = fs::remove_dir_all(cache_dir);
    Ok(())
}
