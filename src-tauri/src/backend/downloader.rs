use std::{fs::File, io::copy, path::PathBuf};

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{CONFIG, MAX_TIMEOUT};

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Serialize)]
pub enum RemoteError {
    Offline,
    Other(String),
}

async fn get_release() -> Result<Release, RemoteError> {
    let url = CONFIG.lock().expect("Failed to lock config").mirror.clone();
    let client = reqwest::Client::new();
    let response = match client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await
    {
        Ok(data) => data.json::<Release>().await.map_err(|err| {
            RemoteError::Other(format!("Failed to serialize json from release: {err}"))
        })?,
        Err(err) => {
            return Err(if err.is_timeout() || err.is_request() {
                RemoteError::Offline
            } else {
                RemoteError::Other(format!("Failed to retrieve version: {err}"))
            })
        }
    };
    Ok(response)
}

async fn get_remote_version() -> Result<String, RemoteError> {
    let config = CONFIG.lock().expect("Failed to lock config").clone();
    let url = config.mirror;
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(MAX_TIMEOUT))
        .build()
        .map_err(|err| RemoteError::Other(format!("Failed to build client: {err}")))?;
    let remote = match client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await
    {
        Ok(data) => {
            data.json::<Release>()
                .await
                .map_err(|err| {
                    RemoteError::Other(format!("Failed to serialize json from version: {err}"))
                })?
                .tag_name
        }
        Err(err) => {
            return Err(if err.is_timeout() || err.is_connect() {
                RemoteError::Offline
            } else {
                RemoteError::Other(format!("Failed to retrieve version: {err}"))
            })
        }
    };
    Ok(remote)
}

async fn download_file(url: &str, path: PathBuf) -> Result<(), RemoteError> {
    let response = reqwest::get(url).await.map_err(|err| {
        if err.is_timeout() {
            RemoteError::Offline
        } else {
            RemoteError::Other(err.to_string())
        }
    })?;
    info!(
        "Starting download of {}mb",
        response.content_length().unwrap_or_default() / 1048576
    );
    let mut dest = File::create(&path)
        .map_err(|err| RemoteError::Other(format!("Failed to create file: {err}")))?;
    let content = response.bytes().await.map_err(|err| {
        if err.is_timeout() {
            RemoteError::Offline
        } else {
            RemoteError::Other(err.to_string())
        }
    })?;
    copy(&mut content.as_ref(), &mut dest).map_err(|err| RemoteError::Other(err.to_string()))?;
    info!("Downloaded to {}", path.to_string_lossy());
    Ok(())
}

/// updates the currently used yara rules to the latest from the repo
pub async fn update() -> Result<(), RemoteError> {
    // check if online and update necessary
    if !check_update().await? {
        return Ok(());
    }

    let config = CONFIG.lock().expect("Failed to lock config").clone();
    let download_path = config
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())
        .map_err(|err| RemoteError::Other(err))?
        .data
        .join(config.remote_file.clone());

    info!("Starting download...");
    let release = get_release().await?;
    if let Some(asset) = release
        .assets
        .iter()
        .find(|&a| a.name == config.remote_file)
    {
        download_file(&asset.browser_download_url, download_path).await?;
        info!(
            "Downloaded: {} from {}",
            asset.name, asset.browser_download_url
        );
    } else {
        error!("Asset not found");
    }
    CONFIG.lock().expect("Failed to lock config").rules_version = get_remote_version().await?;
    let config = CONFIG.lock().expect("Failed to lock config");
    config.save().map_err(|err| RemoteError::Other(err))?;
    info!("Updated to {}", &config.rules_version);
    Ok(())
}

/// returns true or false if update is available, otherwise the remote error enum
pub async fn check_update() -> Result<bool, RemoteError> {
    let current_version = CONFIG
        .lock()
        .expect("Failed to lock config")
        .rules_version
        .clone();
    Ok(current_version != get_remote_version().await?)
}
