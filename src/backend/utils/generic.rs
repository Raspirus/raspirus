use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use log::{debug, info, warn};
use sha2::{Digest, Sha256};
use yara_x::Rules;
use zip::write::SimpleFileOptions;

use crate::{backend::config_file::Config, frontend::iced::ConfigValue};

pub fn get_rules(yar_path: PathBuf) -> Result<Rules, String> {
    // setup rules
    let reader = File::open(yar_path)
        .map_err(|err| format!("Failed to open yar file: {err}. Please update and try again"))?;
    Rules::deserialize_from(reader).map_err(|err| {
        format!("Failed to deserialize yar file: {err}. Please update and try again")
    })
}

/// yields all file paths and the total size of them
pub fn profile_path(path: PathBuf) -> (u64, Vec<PathBuf>) {
    info!("Starting indexing...");
    let mut paths = Vec::new();
    let size = if path.is_dir() {
        // path is folder
        profile_folder(&mut paths, path)
    } else {
        // path is file
        match path.metadata() {
            Ok(metadata) => {
                paths.push(path.clone());
                metadata.len()
            }
            Err(err) => {
                warn!(
                    "Failed to get metadata for {}: {err}",
                    path.to_string_lossy()
                );
                0
            }
        }
    };
    info!("Finished indexing {} files", paths.len());
    (size, paths)
}

/// adds files or files in subfolders to paths and adds their sizes to the total
pub fn profile_folder(paths: &mut Vec<PathBuf>, path: PathBuf) -> u64 {
    let mut size = 0;
    // get all entries in folder
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(err) => {
            warn!(
                "Failed to get entries for {}: {err}",
                path.to_string_lossy()
            );
            return 0;
        }
    };

    for entry in entries {
        // get entry without error
        let entry = match entry {
            Ok(entry) => entry,
            Err(ref err) => {
                warn!("Failed to get entry {:?}: {err}", entry);
                continue;
            }
        };
        // skip symlinks
        if entry.path().is_symlink() {
            continue;
        }

        size += if entry.path().is_dir() {
            profile_folder(paths, entry.path())
        } else {
            match entry.path().metadata() {
                Ok(metadata) => {
                    paths.push(entry.path().clone());
                    metadata.len()
                }
                Err(err) => {
                    warn!(
                        "Failed to get metadata for {}: {err}",
                        entry.path().to_string_lossy()
                    );
                    0
                }
            }
        }
    }
    size
}

/// computes the hash of a file contained in a zip
pub fn hash_in_zip(root_file: File, target_path: PathBuf) -> Result<String, String> {
    let mut zip =
        zip::ZipArchive::new(root_file).map_err(|err| format!("Failed to open archive: {err}"))?;
    let file = zip
        .by_name_seek(&target_path.to_string_lossy())
        .map_err(|err| format!("Could not find file in zip: {err}"))?;

    let mut reader = BufReader::new(file);
    let mut sha256 = Sha256::new();

    loop {
        let mut buffer = [0; 524288];
        let read = reader
            .read(&mut buffer)
            .map_err(|err| format!("Failed to read into buffer: {err}"))?;
        if read == 0 {
            break;
        }
        sha256.update(&buffer[..read]);
    }
    Ok(hex::encode(sha256.finalize()))
}

/// calculates sha256 hash and generates virustotal search link
pub fn generate_virustotal(file: PathBuf) -> Result<String, String> {
    let hash = if !file.exists() {
        let mut hash = String::new();
        // check if inside archive
        let mut path = Path::new("/").to_path_buf();
        for component in file.components() {
            path = path.join(component);
            if let Some(extension) = path.extension() {
                // path is archive
                if crate::SUPPORTED_ARCHIVES.contains(&extension.to_string_lossy().to_string()) {
                    let target = file
                        .strip_prefix(&path)
                        .map_err(|err| format!("Failed to strip path prefix: {err}"))?;
                    hash = hash_in_zip(
                        File::open(&path)
                            .map_err(|err| format!("Failed to open archive: {err}"))?,
                        target.to_path_buf(),
                    )?;
                }
            }
        }
        if hash.is_empty() {
            return Err("File is not accessible".to_string());
        }
        hash
    } else {
        info!("Starting hash compute for {}", file.to_string_lossy());
        let file = File::open(file)
            .map_err(|err| format!("Failed to open file for computing hash: {err}"))?;

        let mut reader = BufReader::new(file);
        let mut sha256 = Sha256::new();

        loop {
            let mut buffer = [0; 524288];
            let read = reader
                .read(&mut buffer)
                .map_err(|err| format!("Failed to read into buffer: {err}"))?;
            if read == 0 {
                break;
            }
            sha256.update(&buffer[..read]);
        }
        hex::encode(sha256.finalize())
    };
    Ok(format!("https://virustotal.com/gui/search/{}", hash))
}

/// updates the global config to what it should be
pub fn update_config(value: ConfigValue) -> Result<Config, String> {
    let mut config = crate::CONFIG
        .lock()
        .map_err(|err| format!("Failed to lock config: {err}"))?;
    match value {
        ConfigValue::MinMatch(min_matches) => config.min_matches = min_matches,
        ConfigValue::MaxMatch(max_matches) => config.max_matches = max_matches,
        ConfigValue::Logging(logging) => config.logging_is_active = logging,
        ConfigValue::MaxThreads(max_threads) => config.max_threads = max_threads,
        ConfigValue::Language(language) => config.language = language,
        ConfigValue::Dark(dark) => config.dark_mode = dark,
        ConfigValue::Scale(scale) => config.scale = scale,
    }
    config.save()?;
    Ok(config.clone())
}

pub fn download_logs(output_file: PathBuf) -> Result<PathBuf, String> {
    let config = crate::CONFIG
        .lock()
        .expect("Failed to lock config")
        .clone()
        .paths
        .ok_or("Paths in config undefined".to_owned())?; 

    info!("Downloading logs to {}", output_file.to_string_lossy());

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    let mut zip =
        zip::ZipWriter::new(File::create(&output_file).expect("Failed to create zip archive"));
    let current_path = config
        .logs_app
        .parent()
        .ok_or("Failed to get app log path")?;

    zip.add_directory_from_path("app", options)
        .map_err(|err| format!("Failed to add ./app to zip: {err}"))?;

    debug!("Compressing {}...", current_path.to_string_lossy());
    for entry in fs::read_dir(current_path)
        .map_err(|err| format!("Failed to read entries for 'app': {err}"))?
    {
        let entry = entry.map_err(|err| format!("Failed to get entry: {err}"))?;

        zip.start_file(
            format!("app/{}", entry.file_name().to_string_lossy()),
            options,
        )
        .map_err(|err| format!("Failed to start file in zip: {err}"))?;
        debug!("Compressing app/{}...", entry.file_name().to_string_lossy());
        zip.write(
            fs::read_to_string(entry.path())
                .map_err(|err| format!("Failed to read file to string: {err}"))?
                .as_bytes(),
        )
        .map_err(|err| format!("Failed to write file in zip: {err}"))?;
    }

    let current_path = config.logs_scan;
    zip.add_directory_from_path("scan", options)
        .map_err(|err| format!("Failed to add ./scan to zip: {err}"))?;

    debug!("Compressing {}...", current_path.to_string_lossy());
    for entry in fs::read_dir(current_path)
        .map_err(|err| format!("Failed to read entries for 'scan': {err}"))?
    {
        let entry = entry.map_err(|err| format!("Failed to get entry: {err}"))?;

        zip.start_file(
            format!("scan/{}", entry.file_name().to_string_lossy()),
            options,
        )
        .map_err(|err| format!("Failed to start file in zip: {err}"))?;
        debug!(
            "Compressing scan/{}...",
            entry.file_name().to_string_lossy()
        );
        zip.write(
            fs::read_to_string(entry.path())
                .map_err(|err| format!("Failed to read file to string: {err}"))?
                .as_bytes(),
        )
        .map_err(|err| format!("Failed to write file in zip: {err}"))?;
    }
    zip.finish()
        .map_err(|err| format!("Failed to finish zip file: {err}"))?;
    info!("Logs have been saved to {}", output_file.to_string_lossy());
    Ok(output_file)
}
