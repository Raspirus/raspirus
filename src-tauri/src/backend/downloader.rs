use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    sync::{atomic::AtomicBool, mpsc, Arc},
    time::Duration,
};

use log::{error, info, trace, warn};
use reqwest::StatusCode;
use threadpool_rs::threadpool::pool::ThreadPool;

use crate::backend::utils::generic::{clear_cache, send_progress};

use super::utils::generic::get_config;

/// Indexes the mirror and checks how many files exist
pub fn index() -> Result<usize, std::io::Error> {
    let config = get_config();
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(crate::MAX_TIMEOUT))
        .build()
        .expect("Failed to build client");

    let mut total_files = 0;
    let mut current_try = crate::MAX_RETRY;
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
                    warn!(
                        "Failed {} times, aborting; Check your network?",
                        crate::MAX_RETRY
                    )
                }
            }
        }
    }

    total_files -= 10;
    current_try = crate::MAX_RETRY;

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
                    warn!(
                        "Failed {} times, aborting; Check your network?",
                        crate::MAX_RETRY
                    )
                }
            }
        }
    }
    Ok(total_files - 1)
}

/// Downloads all files from the mirror using a threadpool. sends true on tx if file finished downloading
pub fn download_all(total_files: usize, window: &Option<tauri::Window>) -> std::io::Result<()> {
    let start_time = std::time::Instant::now();
    let config = get_config();
    let cache_dir = config
        .paths
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No paths set. Is config initialized?",
        ))?
        .cache;

    clear_cache()?;

    // frontend channel
    let (tx, rx): (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel();
    // terminator
    let should_continue = Arc::new(AtomicBool::new(true));

    let pool = ThreadPool::new(crate::PARALLEL_DOWNLOADS)?;
    for file_id in 0..=total_files {
        let dir = cache_dir.clone();
        let mirror = config.mirror.clone();
        let tx = tx.clone();

        let should_continue_thread = should_continue.clone();

        let cdir = cache_dir.clone();

        pool.execute(move || {
            if should_continue_thread.load(std::sync::atomic::Ordering::Relaxed) {
                let _ = fs::create_dir_all(cdir);

                let download_path = dir.join(format!("{:0>5}", file_id));
                let file_url = format!("{}/{:0>5}", mirror, file_id);
                match download_file(&download_path, file_url.clone()) {
                    Ok(_) => {
                        info!("Downloaded {} to {}", file_url, dir.clone().display());
                        tx.send(true)
                            .expect("Download thread failed to send on channel")
                    }
                    Err(err) => {
                        error!("Failed to download {file_url}: {err}");
                        should_continue_thread.store(false, std::sync::atomic::Ordering::Relaxed);
                        tx.send(false)
                            .expect("Download thread failed to send on channel")
                    }
                };
            }
        });
    }

    let mut previous_progress = 0.0;
    let mut should_update = true;
    for current in 0..=total_files {
        // if we receive false, meaning a thread yielded to an error, we stop updating the progress
        if rx.recv().map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("A download thread failed!: {}", err),
            )
        })? {
            if should_update {
                previous_progress =
                    send_progress(window, previous_progress, current, total_files, "dwld")
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
            }
        } else {
            should_update = false;
        }
    }

    if !should_continue.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "A download thread failed!",
        ));
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
                    "No parent directory",
                ));
            };
        }
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No parent directory",
            ))
        }
    }
    // deletes output file if exist
    output_name.exists().then(|| {
        fs::remove_file(output_name).map_err(|err| warn!("Could not delete output file: {err}"))
    });

    let mut file = File::create(output_name)?;
    let client = reqwest::blocking::Client::new();

    for current_retry in 0..=crate::MAX_RETRY {
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
