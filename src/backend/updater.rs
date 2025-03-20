use std::{
    fs::{self, File},
    io::{copy, BufReader, Read, Write},
    path::PathBuf,
};

use log::{info, warn};
use serde::Deserialize;

use crate::{error::Error, globals::get_config};

#[derive(Deserialize)]
struct Release {
    published_at: String,
    zipball_url: String,
}

/// Updates the local rules build if necessary
pub async fn update() -> Result<(), Error> {
    let earlier = std::time::Instant::now();

    if !check_update().await? {
        info!("Already up to date");
        Err(Error::RemoteAlreadyUpdated)?
    }

    // build downloaded file
    build(download().await?)?;

    info!(
        "Successfully updated to {:?} in {}s",
        get_local_datetime()?,
        std::time::Instant::now().duration_since(earlier).as_secs()
    );
    Ok(())
}

/// Checks remote if there is a newer version available
async fn check_update() -> Result<bool, Error> {
    info!("Checking for new remote version...");
    let remote_url = get_config().lock()?.clone().remote_url;

    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(crate::globals::TIMEOUT))
        .build()
        .map_err(Error::RemoteClientBuild)?;

    // fetch release json
    let release = match client
        .get(remote_url)
        .header("User-Agent", "raspirus-reqwest")
        .send()
        .await
    {
        Ok(data) => data
            .json::<Release>()
            .await
            .map_err(Error::RemoteSerialize)?,
        Err(err) => Err(if err.is_request() || err.is_timeout() {
            Error::RemoteOffline(err)
        } else {
            Error::RemoteUndefined
        })?,
    };

    let remote_datetime = release
        .published_at
        .parse::<chrono::DateTime<chrono::Utc>>()
        .map_err(Error::RemoteTime)?;
    Ok(remote_datetime > get_local_datetime()?.unwrap_or(chrono::DateTime::default()))
}

/// Fetches the latest local yarac datetime
pub fn get_local_datetime() -> Result<Option<chrono::DateTime<chrono::Utc>>, Error> {
    let data_path = get_config().lock()?.clone().get_paths()?.data;
    let mut timestamps = Vec::new();
    for entry in fs::read_dir(data_path.join("yara_c")).map_err(Error::RemoteLocalIO)? {
        let mut entry = entry.map_err(Error::RemoteLocalIO)?.path();
        entry.set_extension("");
        if let Some(file_name) = entry.file_name() {
            timestamps.push(file_name.to_owned());
        }
    }

    // convert all timestamp strings to datetime
    let parsed_timestamps = timestamps
        .iter()
        // filter out none variants
        .filter_map(|timestamp| {
            // convert osstring to timestamp or none
            timestamp.to_str().and_then(|time| {
                chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H-%M-%SZ")
                    .map(|naive_datetime| naive_datetime.and_utc())
                    .ok()
            })
        })
        .collect::<Vec<chrono::DateTime<chrono::Utc>>>();

    // return newest datetime
    Ok(parsed_timestamps.iter().max().copied())
}

/// Downloads the latest release, returning the path to the downloaded file
async fn download() -> Result<PathBuf, Error> {
    let config = get_config().lock()?.clone();
    let remote_url = &config.remote_url;
    let temp_path = config.get_paths()?.temp;

    let earlier = std::time::Instant::now();
    info!("Found updates; Dowloading...");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(crate::globals::TIMEOUT))
        .build()
        .map_err(Error::RemoteClientBuild)?;

    // fetch release
    let release = match client
        .get(remote_url)
        .header("User-Agent", "raspirus-reqwest")
        .send()
        .await
    {
        Ok(data) => data
            .json::<Release>()
            .await
            .map_err(Error::RemoteSerialize)?,
        Err(err) => Err(if err.is_request() || err.is_timeout() {
            Error::RemoteOffline(err)
        } else {
            Error::RemoteUndefined
        })?,
    };

    // start fetching zipball
    let file = match client
        .get(&release.zipball_url)
        .header("User-Agent", "raspirus-reqwest")
        .send()
        .await
    {
        Ok(file) => file,
        Err(err) => Err(if err.is_request() || err.is_timeout() {
            Error::RemoteOffline(err)
        } else {
            Error::RemoteUndefined
        })?,
    };

    let content = file.bytes().await.map_err(|err| {
        if err.is_request() || err.is_timeout() {
            Error::RemoteOffline(err)
        } else {
            Error::RemoteUndefined
        }
    })?;

    // create path in data/published_at
    let dest = temp_path.join(release.published_at.replace(":", "-"));

    // copy downloaded content to destination file
    copy(
        &mut content.as_ref(),
        &mut File::create(&dest).map_err(Error::RemoteLocalIO)?,
    )
    .map_err(Error::RemoteLocalIO)?;

    info!(
        "Downloaded {}mb from {} in {}s",
        content.len() / 1048576,
        release.zipball_url,
        std::time::Instant::now().duration_since(earlier).as_secs()
    );
    Ok(dest)
}

/// Unpacks and builds the fetched files
fn build(archive: PathBuf) -> Result<(), Error> {
    let paths = get_config().lock()?.clone().get_paths()?;

    let mut output_filename = PathBuf::from(archive.file_name().ok_or(Error::BuilderIO(
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Archive path did not have a file name",
        ),
    ))?);
    output_filename.set_extension("yarac");

    // create path in data/published_at
    let target_path = paths.data.join("yara_c").join(&output_filename);

    let earlier = std::time::Instant::now();

    let mut zip = zip::ZipArchive::new(BufReader::new(
        File::open(archive).map_err(Error::BuilderIO)?,
    ))
    .map_err(Error::BuilderArchive)?;

    let mut compiler = yara_x::Compiler::new();

    info!("Adding rules...");

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).map_err(Error::BuilderArchive)?;

        // windows specific powershell script execution
        #[cfg(target_os = "windows")]
        if file.name().ends_with("windows.ps1") {
            info!("Updating windows defender rule...");
            let mut script = String::new();
            file.read_to_string(&mut script).map_err(Error::BuilderIO)?;
            // save powershell script to cache
            let script_file = paths.temp.join("windows.ps1");
            // copy content to new file
            fs::write(&script_file, script).map_err(Error::BuilderIO)?;
            info!("Running {}...", script_file.display());

            // run the powershell script
            let mut cmd = std::process::Command::new("powershell");
            cmd.arg("-ExecutionPolicy")
                .arg("Bypass")
                .arg("-File")
                .arg(script_file)
                .arg(&paths.data);

            log::debug!("Running {:?}", cmd);

            cmd.stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped());
            log::debug!("{:?}", cmd.output().map_err(Error::BuilderPowershell)?);

            continue;
        }

        if file.name().ends_with(".yar") {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(Error::BuilderIO)?;
            match compiler.add_source(content.as_bytes()) {
                Ok(_) => {}
                Err(_) => warn!("Failed to add {}", file.name()),
            }
        }
    }

    info!("Starting rule build; This might take some time...");
    let rules = compiler.build(); // will take at least 5 billion years
    let mut out = File::create(target_path).map_err(Error::BuilderIO)?;

    out.write_all(&rules.serialize().map_err(Error::BuilderSerialization)?)
        .map_err(Error::BuilderIO)?;

    info!(
        "Built rules in {}s",
        std::time::Instant::now().duration_since(earlier).as_secs()
    );
    Ok(())
}
