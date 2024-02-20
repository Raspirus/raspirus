use chrono::{DateTime, Local, Utc};
use directories_next::ProjectDirs;
use job_scheduler_ng::{Job, JobScheduler};
use log::{error, info, warn};
use reqwest::StatusCode;
use std::fs::{self, DirEntry};
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::{fs::File, io::Write, time::Duration};
use std::{path::Path, time};

use crate::backend::config_file::Config;
use crate::backend::db_ops::DBOps;
use crate::backend::downloader::{calculate_progress, send_progress};

static DB_NAME: &str = "signatures.db";

/// Checks if local is running behind remote. Returns true if remote is newer
pub fn check_update_necessary() -> Result<bool, std::io::Error> {
    let config =
        Config::new().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    // get local timestamp
    let local_timestamp = match config.last_db_update.as_str() {
        "Never" => "0".to_owned(),
        timestamp => timestamp.to_owned(),
    }
    .parse::<u128>()
    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;

    // fetch remote timestamp
    let remote_timestamp = get_remote_timestamp()?
        .parse::<u128>()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;

    Ok(remote_timestamp > local_timestamp)
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
    send_progress(&window, String::from("Checking for updates..."));
    // if remote is not newer than local we skip
    if !check_update_necessary().map_err(|err| err.to_string())? {
        info!("Database already up to date. Skipping...");
        return Ok("100".to_owned());
    }

    info!("Updating database...");
    let mut config = Config::new()?;
    let project_dir = config
        .program_path
        .as_ref()
        .expect("Failed to get project directories.");
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
    let mut db_connection = match DBOps::new(db_file_str.as_str()) {
        Ok(db_conn) => db_conn,
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    };

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
            exit(-1);
        }
    }
}

pub fn insert_all(db: &mut DBOps, window: &Option<tauri::Window>) -> Result<(), String> {
    let start_time = std::time::Instant::now();
    let config = Config::new()?;
    let project_dir = config
        .program_path
        .as_ref()
        .expect("Failed to get project directories.");
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
        p = calculate_progress(window, p, i, len, "Inserting...".to_owned())?;
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

// TODO

// Not yet implemented => borked?
pub async fn auto_update_scheduler(tauri_win: Option<tauri::Window>, hour: i32, weekday: i32) {
    // ISSUE: Needs to restart app to apply new update schedule

    // In cron, the time is in 24h format, while the weekday starts at 0 = sunday and 6 = saturday
    let mut scheduler = JobScheduler::new();

    // Construct the cron-like syntax using the given hour and weekday
    let cron_schedule = format!("0 {} * * {}", hour, weekday);

    scheduler.add(Job::new(
        cron_schedule.parse().expect("Given CronSyntax is invalid"),
        move || {
            // Check current time and use it for the name of the update logs file
            let now: DateTime<Local> = Local::now();
            let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
            let log_str = format!("{}.log", now_str);
            // Write to logs that the update function has started
            let message = format!("{} DB update executed\n", Utc::now());
            log_update_res(&message, log_str.clone())
                .expect("Failed to write update logs to file.");
            match update(tauri_win.clone()) {
                Ok(result) => {
                    let message = format!("{} DB update finished\n", Utc::now());
                    log_update_res(&message, log_str.clone())
                        .expect("Failed to write update logs to file.");
                    info!("AutoUpdate finished with: {}", result);
                }
                Err(error) => {
                    let message = format!("{} DB update error {}\n", Utc::now(), error);
                    log_update_res(&message, log_str.clone())
                        .expect("Failed to write update logs to file.");
                    error!("AutoUpdate failed with: {}", error)
                }
            };
        },
    ));

    // Block the main thread to keep the program running until terminated
    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}

// Simply logs the database update result to a file
fn log_update_res(data: &str, fname: String) -> std::io::Result<()> {
    // Open the file (creates if it doesn't exist)
    let mut file = File::create(
        ProjectDirs::from("com", "Raspirus", "Logs")
            .expect("Failed to get project directories.")
            .data_local_dir()
            .join("updates")
            .join(fname),
    )
    .expect("Couldnt open log file");
    // Write the data to the file
    file.write_all(data.as_bytes())?;
    // Flush the buffer to ensure all data is written
    file.flush()
}
