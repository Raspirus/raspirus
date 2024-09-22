use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Read},
    path::{Path, PathBuf},
};

use log::{debug, info, warn};
use printpdf::PdfDocument;
use sha2::{Digest, Sha256};
use yara_x::Rules;

use crate::frontend::iced::ConfigValue;

pub fn get_rules(yar_path: PathBuf) -> Result<Rules, String> {
    // setup rules
    let reader = File::open(yar_path).map_err(|err| format!("Failed to load yar file: {err}"))?;
    Rules::deserialize_from(reader).map_err(|err| format!("Failed to deserialize yar file: {err}"))
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
pub fn update_config(value: ConfigValue) -> Result<(), String> {
    let mut config = crate::CONFIG
        .lock()
        .map_err(|err| format!("Failed to lock config: {err}"))?;
    match value {
        ConfigValue::MinMatch(min_matches) => config.min_matches = min_matches,
        ConfigValue::MaxMatch(max_matches) => config.max_matches = max_matches,
        ConfigValue::Logging(logging) => config.logging_is_active = logging,
        ConfigValue::MaxThreads(max_threads) => config.max_threads = max_threads,
    }
    config.save()?;
    Ok(())
}

static PAGE_HEIGHT: f32 = 297.0;
static PAGE_WIDTH: f32 = 210.0;
static FONT_SIZE: f32 = 14.0;

pub fn create_pdf(log_file: PathBuf) -> Result<PathBuf, String> {
    // fetch filename from log path
    let file_name = log_file.file_name().unwrap_or_default().to_string_lossy();
    let timestamp = file_name.trim_end_matches(".log");
    // fetch downloads directory
    let downloads_folder = crate::CONFIG
        .lock()
        .map_err(|err| format!("Failed to lock config: {err}"))?
        .paths
        .clone()
        .ok_or_else(|| "No paths?".to_string())?
        .downloads;
    info!("Creating pdf report for {}", log_file.to_string_lossy());

    let log =
        File::open(log_file.clone()).map_err(|err| format!("Failed to open logfile: {err}"))?;

    // create pdf file
    let (doc, page1, layer1) = PdfDocument::new(
        format!("Log {timestamp}",),
        printpdf::Mm(PAGE_WIDTH),
        printpdf::Mm(PAGE_HEIGHT),
        "Layer 1",
    );

    // set font
    let font = doc
        .add_builtin_font(printpdf::BuiltinFont::Helvetica)
        .map_err(|err| format!("Failed to add font: {err}"))?;
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    current_layer.begin_text_section();
    current_layer.set_font(&font, FONT_SIZE);

    current_layer.set_text_cursor(printpdf::Mm(10.0), printpdf::Mm(PAGE_HEIGHT - 10.0));
    current_layer.set_line_height(FONT_SIZE + 2.0);

    current_layer.write_text(format!("Log {timestamp}"), &font);
    current_layer.add_line_break();
    current_layer.add_line_break();

    // weird code with magic numbers but what can you do /shrug
    let pt_in_mm = (FONT_SIZE + 2.0) * 0.3537778;
    let max_lines_page = ((PAGE_HEIGHT - 20.0) / pt_in_mm) as usize;
    debug!("Determined lines per page should be {max_lines_page}");

    for (num, line) in std::io::BufReader::new(log).lines().enumerate() {
        if (num + 2) % max_lines_page == 0 && num > 0 {
            // cleanup old page
            current_layer.end_text_section();

            debug!("Page end reached, creating new page at line {num}");
            let (page, layer) = doc.add_page(
                printpdf::Mm(PAGE_WIDTH),
                printpdf::Mm(PAGE_HEIGHT),
                "Layer 1",
            );

            // prepare new page
            current_layer = doc.get_page(page).get_layer(layer);
            current_layer.begin_text_section();
            current_layer.set_font(&font, 14.0);

            current_layer.set_text_cursor(printpdf::Mm(10.0), printpdf::Mm(287.0));
            current_layer.set_line_height(14.0);
        }
        match line {
            Ok(line) => {
                current_layer.write_text(line, &font);
                current_layer.add_line_break();
            }
            Err(err) => return Err(format!("Could not read log file line: {err}")),
        }
    }
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(
        File::create(downloads_folder.join(format!("{timestamp}.pdf")))
            .map_err(|err| format!("Failed to open file for writing: {err}"))?,
    ))
    .map_err(|err| format!("Failed to save pdf: {err}"))?;
    Ok(downloads_folder.join(format!("{timestamp}.pdf")))
}
