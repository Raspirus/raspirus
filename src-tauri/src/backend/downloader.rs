use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    sync::mpsc,
    time::Duration,
};

use log::{error, info, trace, warn};
use reqwest::StatusCode;
use tauri::Manager;
use threadpool_rs::threadpool::pool::ThreadPool;

use super::config_file::Config;

pub static MAX_RETRY: usize = 0;
static PARALLEL_DOWNLOADS: usize = 12;
static MAX_TIMEOUT: u64 = 12;

#[derive(Clone, serde::Serialize)]
struct TauriEvent {
    message: String,
}

/// sends given percentage to the frontend
pub fn send(window: &Option<tauri::Window>, event: &str, message: String) {
    if let Some(window) = window {
        trace!("Sending {event}: {message}");
        match window.emit_all(event, message) {
            Ok(_) => {},
            Err(err) => warn!("Failed to send progress to frontend: {err}"),
        }
    }
}

/// calculates progress and sends to frontend if changed. returns new percentage
pub fn calculate_progress(
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

/// Indexes the mirror and checks how many files exist
pub fn index() -> Result<usize, std::io::Error> {
    let config =
        Config::new().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(MAX_TIMEOUT))
        .build()
        .expect("Failed to build client");

    let mut total_files = 0;
    let mut current_try = MAX_RETRY;
    loop {
        let file_url = format!("{}/{:0>5}", config.mirror, total_files);
        trace!("Indexing {file_url}");
        let response = client
            .head(file_url)
            .send()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

        match response.status() {
            StatusCode::OK => total_files += 10,
            StatusCode::NOT_FOUND => break,
            _ => {
                warn!(
                    "Received invalid status {}, trying again...",
                    response.status()
                );
                current_try -= 1;
                if current_try == 0 {
                    warn!("Failed {MAX_RETRY} times, aborting; Check your network?")
                }
            }
        }
    }

    total_files -= 10;
    current_try = MAX_RETRY;

    loop {
        let file_url = format!("{}/{:0>5}", config.mirror, total_files);
        trace!("Indexing {file_url}");
        let response = client
            .head(file_url)
            .send()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        match response.status() {
            StatusCode::OK => total_files += 1,
            StatusCode::NOT_FOUND => break,
            _ => {
                warn!(
                    "Received invalid status {}, trying again...",
                    response.status()
                );
                current_try -= 1;
                if current_try == 0 {
                    warn!("Failed {MAX_RETRY} times, aborting; Check your network?")
                }
            }
        }
    }
    Ok(total_files - 1)
}

/// Downloads all files from the mirror using a threadpool. sends true on tx if file finished downloading
pub fn download_all(total_files: usize, window: &Option<tauri::Window>) -> std::io::Result<()> {
    let start_time = std::time::Instant::now();
    let config =
        Config::new().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    let project_dir = config
        .program_path
        .expect("Failed to get project directories");
    let cache_dir = project_dir.cache_dir().to_owned();

    if cache_dir.exists() {
        fs::remove_dir_all(cache_dir.clone())?;
    }
    fs::create_dir_all(cache_dir.clone())?;

    // frontend channel
    let (tx, rx): (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel();
    // thread channel
    //let ()
    let pool = ThreadPool::new(PARALLEL_DOWNLOADS)?;
    for file_id in 0..=total_files {
        let dir = cache_dir.clone();
        let mirror = config.mirror.clone();
        let tx = tx.clone();
        pool.execute(move || {
            //match 
            let download_path = dir.join(format!("{:0>5}", file_id));
            let file_url = format!("{}/{:0>5}", mirror, file_id);
            match download_file(&download_path, file_url.clone()) {
                Ok(_) => info!(
                    "Downloaded {} to {}",
                    download_path.display(),
                    dir.clone().display()
                ),
                Err(err) => error!("Failed to download {file_url}: {err}"),
            };
            tx.send(true)
                .expect("Download thread failed to send on channel")
        });
    }

    let mut p = 0.0;
    for current in 0..=total_files {
        let _ = rx.recv().expect("Failed to read from channel");
        p = calculate_progress(window, p, current, total_files, "dwld")
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    }

    info!(
        "Downloaded {total_files} files in {}s",
        std::time::Instant::now()
            .duration_since(start_time)
            .as_secs()
    );
    Ok(())
}

/// downloads a single file. used for threading
pub fn download_file(output_name: &Path, file_url: String) -> std::io::Result<()> {
    // checks if output folder exists
    match output_name.parent() {
        Some(parent_dir) => {
            if !parent_dir.exists() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Parent directory does not exist",
                ));
            }
        }
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No parent directory",
            ))
        }
    }
    // deletes output file if exist
    output_name.exists().then(|| fs::remove_file(output_name));

    let mut file = File::create(output_name)?;
    let client = reqwest::blocking::Client::new();

    for current_retry in 0..=MAX_RETRY {
        let response = match client.get(file_url.clone()).send() {
            Ok(response) => response,
            Err(err) => {
                warn!("Failed to download {file_url} on try {current_retry}: {err}");
                continue;
            }
        };

        // if ok we write to file, otherwise we retry
        match response.status() {
            StatusCode::OK => match response.text() {
                Ok(data) => {
                    file.write_all(data.as_bytes())?;
                    return Ok(());
                }
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
        "Could not download file?",
    ))
}
