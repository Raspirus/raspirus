use std::{fs::File, io::copy, path::PathBuf};

use log::{error, info};
use serde::Deserialize;

use crate::backend::utils::generic::update_config;

use super::utils::generic::get_config;

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

async fn get_release() -> Result<Release, String> {
    let url = get_config().mirror;
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await
        .map_err(|err| format!("Failed to send release request: {err}"))?
        .json::<Release>()
        .await
        .map_err(|err| format!("Failed to serialize json from release: {err}"))?;
    Ok(response)
}

async fn get_remote_version() -> Result<String, String> {
    let config = get_config();
    let url = config.mirror;
    let client = reqwest::Client::new();
    println!("{url}");
    let remote = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await
        .map_err(|err| format!("Failed to send version request: {err}"))?
        .json::<Release>()
        .await
        .map_err(|err| format!("Failed to serialize json from version: {err}"))?
        .tag_name;
    Ok(remote)
}

async fn download_file(url: &str, path: PathBuf) -> Result<(), String> {
    let response = reqwest::get(url).await.map_err(|err| err.to_string())?;
    let mut dest = File::create(path).map_err(|err| err.to_string())?;
    let content = response.bytes().await.map_err(|err| err.to_string())?;
    copy(&mut content.as_ref(), &mut dest).map_err(|err| err.to_string())?;
    Ok(())
}

/// updates the currently used yara rules to the latest from the repo
pub async fn update() -> Result<(), String> {
    if !check_update().await? {
        return Ok(());
    }
    let config = get_config();
    let download_path = config
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .data
        .join(get_config().remote_file);

    info!("Starting download...");
    let release = get_release().await.map_err(|err| err.to_string())?;
    if let Some(asset) = release
        .assets
        .iter()
        .find(|&a| a.name == config.remote_file)
    {
        download_file(&asset.browser_download_url, download_path)
            .await
            .map_err(|err| err.to_string())?;
        info!(
            "Downloaded: {} from {}",
            asset.name, asset.browser_download_url
        );
    } else {
        error!("Asset not found");
    }
    let mut config = get_config();
    config.rules_version = get_remote_version().await?;
    info!("Updated to {}", &config.rules_version);
    update_config(config)?;
    Ok(())
}

/// returns true if remote is different from local
pub async fn check_update() -> Result<bool, String> {
    let current_version = get_config().rules_version;
    Ok(current_version != get_remote_version().await?)
}
