use chrono::{DateTime, Local, Utc};
use directories_next::ProjectDirs;
use job_scheduler_ng::{Job, JobScheduler};
use log::{error, info};
use std::process::exit;
use std::{fs, path::Path, time};
use std::{fs::File, io::Write, time::Duration};
use tokio::runtime::Runtime;

use crate::backend::config_file::Config;
use crate::backend::db_ops::DBOps;

// Updates the database async (Very similar to the scanner_utils.rs setup)
pub async fn update_database(window: Option<tauri::Window>) -> Result<String, String> {
    let db_name = "signatures.db";
    let config = Config::new()?.load()?;
    let mut db_file_str = config.db_location;

    if db_file_str.is_empty() {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
    } else if Path::new(&db_file_str).to_owned().exists()
        && Path::new(&db_file_str).to_owned().is_file()
    {
        info!("Using specific DB path {}", db_file_str);
    } else {
        info!("Falling back to default DB file (signatures.db)");
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
    }

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

// Almost identical to above
pub fn sync_update_database(window: Option<tauri::Window>) -> Result<String, String> {
    let db_name = "signatures.db";
    let config = Config::new()?.load()?;
    let mut db_file_str = config.db_location;

    if db_file_str.is_empty() {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
    } else if Path::new(&db_file_str).to_owned().exists()
        && Path::new(&db_file_str).to_owned().is_file()
    {
        info!("Using specific DB path {}", db_file_str);
    } else {
        info!("Falling back to default DB file (signatures.db)");
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Data")
            .expect("Failed to get project directories.");
        let program_dir = project_dirs.data_dir();
        fs::create_dir_all(program_dir).expect("Failed to create program directory.");
        let db_file_path = program_dir.join(db_name);
        db_file_str = db_file_path.to_string_lossy().to_string();
    }

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

// Simply logs the database update result to a file
fn log_update_res(data: &str, fname: String) -> std::io::Result<()> {
    let project_dirs =
        ProjectDirs::from("com", "Raspirus", "Logs").expect("Failed to get project directories.");
    let log_dir = project_dirs.data_local_dir().join("updates");
    // Open the file (creates if it doesn't exist)
    let mut file = File::create(log_dir.join(fname)).expect("Couldnt open log file");
    // Write the data to the file
    file.write_all(data.as_bytes())?;
    // Flush the buffer to ensure all data is written
    file.flush()?;
    Ok(())
}
