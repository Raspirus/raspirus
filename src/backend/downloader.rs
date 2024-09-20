use std::{
    fs::File,
    io::{copy, BufReader, Read, Write},
    path::PathBuf,
};

#[cfg(target_os = "windows")]
use std::{
    fs,
    process::{Command, Stdio},
};

#[cfg(target_os = "windows")]
use log::debug;

use log::{info, warn};
use serde::{Deserialize, Serialize};
use yara_x::Compiler;

use crate::{CONFIG, MAX_TIMEOUT};

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    zipball_url: String,
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

/// downloads a file from a url to a specific path
async fn download_file(url: &str, path: &PathBuf) -> Result<(), RemoteError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(MAX_TIMEOUT))
        .build()
        .map_err(|err| RemoteError::Other(format!("Failed to build client: {err}")))?;
    let response = client
        .get(url)
        .header("User-Agent", "reqwest")
        .send()
        .await
        .map_err(|err| {
            if err.is_timeout() {
                RemoteError::Offline
            } else {
                RemoteError::Other(err.to_string())
            }
        })?;
    let mut dest = File::create(path)
        .map_err(|err| RemoteError::Other(format!("Failed to create file: {err}")))?;
    let content = response.bytes().await.map_err(|err| {
        if err.is_timeout() {
            RemoteError::Offline
        } else {
            RemoteError::Other(err.to_string())
        }
    })?;
    info!("Downloaded {}mb", content.len() / 1048576);
    copy(&mut content.as_ref(), &mut dest).map_err(|err| RemoteError::Other(err.to_string()))?;
    info!("Downloaded {url} to {}", path.to_string_lossy());
    Ok(())
}

/// updates the currently used yara rules to the latest from the repo
pub async fn update() -> Result<(), RemoteError> {
    // check if online and update necessary
    if !check_update().await? {
        return Ok(());
    }

    let mut config = CONFIG.lock().expect("Failed to lock config").clone();

    let paths = config
        .paths
        .clone()
        .ok_or("No paths set. Is config initialized?".to_owned())
        .map_err(RemoteError::Other)?;

    // cache folder
    let temp = paths.clone().temp;
    // downloaded temporary file name
    let download_path = temp.join("latest.zip");
    // path to compiled yara rules
    let save_path = paths.data.join(crate::DEFAULT_FILE);

    info!("Starting download...");
    let release = get_release().await?;

    download_file(&release.zipball_url, &download_path).await?;

    info!("Building rules. This may take some time...");
    build_rules(download_path, save_path, temp).map_err(RemoteError::Other)?;

    let new_version = get_remote_version().await?;

    CONFIG.lock().expect("Failed to lock config").rules_version.clone_from(&new_version);
    config.rules_version = new_version;
    config.save().map_err(RemoteError::Other)?;
    info!("Updated to {}", &config.rules_version);
    Ok(())
}

/// builds the rules and saves them to the data folder. also runs a powershell script to exlcude
/// the rules from being scanned by windows defender
pub fn build_rules(
    source_zip: PathBuf,
    target_yarac: PathBuf,
    _temp: PathBuf,
) -> Result<(), String> {
    let mut compiler = Compiler::new();
    let mut zip = zip::ZipArchive::new(BufReader::new(
        File::open(source_zip).map_err(|err| format!("Failed to open downloaded zip: {err}"))?,
    ))
    .map_err(|err| format!("Failed to open downloaded zip: {err}"))?;

    for i in 0..zip.len() {
        let mut file = zip
            .by_index(i)
            .map_err(|err| format!("Failed to open file from zip: {err}"))?;

        #[cfg(target_os = "windows")]
        if file.name().ends_with("windows.ps1") {
            info!("Updating windows defender rule...");
            let mut script = String::new();
            file.read_to_string(&mut script)
                .map_err(|err| format!("Failed to read script: {err}"))?;
            let script_file = _temp.join("windows.ps1");
            fs::write(&script_file, script).map_err(|err| {
                format!(
                    "Failed to write script to {}: {err}",
                    script_file.to_string_lossy()
                )
            })?;

            // run the powershell script
            let mut cmd = Command::new("powershell");
            cmd.arg("-ExecutionPolicy")
                .arg("RemoteSigned")
                .arg("-File")
                .arg(script_file)
                .arg(&target_yarac);

            debug!("Running {:?}", cmd);

            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

            let output = cmd
                .output()
                .map_err(|err| format!("Encountered error while running command: {err}"))?;
            debug!("{output:#?}");
            continue;
        }

        if file.name().ends_with(".yar") {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|err| format!("Failed to read rule file {}: {err}", file.name()))?;
            match compiler.add_source(content.as_bytes()) {
                Ok(_) => {}
                Err(_) => warn!("Failed {}", file.name()),
            }
        }
    }

    info!("Building...");
    let rules = compiler.build(); // will take at least 5 billion years
    let mut out = File::create(&target_yarac).map_err(|err| {
        format!(
            "Failed to create yarac at {}: {err}",
            target_yarac.to_string_lossy()
        )
    })?;

    out.write_all(
        &rules
            .serialize()
            .map_err(|err| format!("Failed to serialize rules: {err}"))?,
    )
    .map_err(|err| {
        format!(
            "Failed to write rules to {}: {err}",
            target_yarac.to_string_lossy()
        )
    })?;

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
