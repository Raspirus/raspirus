use chrono::{DateTime, Local, Utc};
use job_scheduler_ng::{Job, JobScheduler};
use log::{error, info};
use std::process::exit;
use std::{path::Path, time};
use std::{fs::File, io::Write, time::Duration};
use tokio::runtime::Runtime;

use crate::backend::config_file::Config;
use crate::backend::db_ops::DBOps;
/// Default name of the database file
static DB_NAME: &str = "signatures.db";

/// Updates the database in async mode, returns a JSON string with the dirty files
/// This is the function getting called from the GUI trough the tauri API
/// It is async to ensure the main thread doesn't stop
pub async fn update_database(window: Option<tauri::Window>) -> Result<String, String> {
    // Loading settings from config file
    let config = Config::new()?;
    let program_dir = config.project_dirs.data;

    let db_file_str = if !config.db_location.is_empty() && Path::new(&config.db_location).to_owned().exists() && Path::new(&config.db_location).to_owned().is_file() {
        info!("Using specific DB path {}", config.db_location);
        config.db_location
    } else {
        // if not we use the default path
        program_dir.join(DB_NAME).to_string_lossy().to_string()
    };

    // Initializing the database connection using the path from above
    let mut db_connection = match DBOps::new(db_file_str.as_str(), window) {
        Ok(db_conn) => db_conn,
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    };

    let big_tic = time::Instant::now();
    // Spawns a thread to update async, else would be sync
    match tokio::task::spawn_blocking(move || match db_connection.update_db() {
        Ok(ok) => ok,
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    })
    .await
    {
        // We await for the database update to finish
        Ok(res) => {
            let big_toc = time::Instant::now();
            info!(
                "Updated DB in {} seconds",
                big_toc.duration_since(big_tic).as_secs_f64()
            );
            // Return the result as JSON
            Ok(serde_json::to_string(&res).unwrap_or_default())
        }
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    }
}

/// Updates the database in sync mode, returns a JSON string with the dirty files
/// This is the function getting called from the CLI
/// It is sync because the CLI is sync, but for the rest it is very similar to the one above
pub fn sync_update_database(window: Option<tauri::Window>) -> Result<String, String> {
    let config = Config::new()?;
    let program_dir = config.project_dirs.data;

    let db_file_str = if !config.db_location.is_empty() && Path::new(&config.db_location).to_owned().exists() && Path::new(&config.db_location).to_owned().is_file() {
        info!("Using specific DB path {}", config.db_location);
        config.db_location
    } else {
        // if not we use the default path
        program_dir.join(DB_NAME).to_string_lossy().to_string()
    };

    let mut db_connection = match DBOps::new(db_file_str.as_str(), window) {
        Ok(db_conn) => db_conn,
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    };

    let big_tic = time::Instant::now();
    // THIS PART CHANGES. Tokio thread removed to make sync
    match db_connection.update_db() {
        Ok(res) => {
            let big_toc = time::Instant::now();
            info!(
                "Updated DB in {} seconds",
                big_toc.duration_since(big_tic).as_secs_f64()
            );
            Ok(serde_json::to_string(&res).unwrap_or_default())
        }
        Err(err) => {
            error!("{err}");
            exit(-1);
        }
    }
}

// Not yet implemented => borked?
#[doc(hidden)]
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
            // Execute the async function using Tokio's Runtime
            let runtime = Runtime::new().expect("Unable to create AutoUpdate Runtime");
            match runtime.block_on(update_database(tauri_win.clone())) {
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

/// Logs the result of the update function to a file
/// This is used by the auto update function to check if the update was successful
fn log_update_res(data: &str, fname: String) -> std::io::Result<()> {
    // Open the file (creates if it doesn't exist)
    let config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
        }
    };
    let mut file = File::create(
        config.project_dirs.logs.update.join(fname),
    )
    .expect("Couldnt open log file");
    // Write the data to the file
    file.write_all(data.as_bytes())?;
    // Flush the buffer to ensure all data is written
    file.flush()
}
