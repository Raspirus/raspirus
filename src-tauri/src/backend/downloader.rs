use std::{fs::File, io::copy};

use log::{error, info};
use serde::Deserialize;

use super::utils::generic::get_config;

#[derive(Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

async fn get_release(repo: &str) -> Result<Release, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "request")
        .send()
        .await
        .map_err(|err| err.to_string())?
        .json::<Release>()
        .await
        .map_err(|err| err.to_string())?;
    Ok(response)
}

async fn download_file(url: &str, path: &str) -> Result<(), String> {
    let response = reqwest::get(url).await.map_err(|err| err.to_string())?;
    let mut dest = File::create(path).map_err(|err| err.to_string())?;
    let content = response.bytes().await.map_err(|err| err.to_string())?;
    copy(&mut content.as_ref(), &mut dest).map_err(|err| err.to_string())?;
    Ok(())
}

/// updates the currently used yara rules to the latest from the repo
pub async fn update() -> Result<(), String> {
    if !check_update()? {
        return Ok(());
    }
    let config = get_config();
    let repo = "";
    let asset_name = "";
    let download = config
        .paths
        .ok_or("No paths set. Is config initialized?".to_owned())?
        .data
        .join(asset_name);

    let release = get_release(repo).await.map_err(|err| err.to_string())?;
    if let Some(asset) = release.assets.iter().find(|&a| a.name == asset_name) {
        download_file(&asset.browser_download_url, &asset_name).await.map_err(|err| err.to_string())?;
        info!(
            "Downloaded: {} from {}",
            asset.name, asset.browser_download_url
        );
    } else {
        error!("Asset not found");
    }
    Ok(())
}

pub fn check_update() -> Result<bool, String> {
    Ok(true)
}
