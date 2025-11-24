// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::VecDeque,
    io::Write,
    path::PathBuf,
    sync::mpsc,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand,
    cursor::{MoveToColumn, MoveToRow, MoveUp, RestorePosition, SavePosition},
    terminal::{Clear, ClearType, size},
};
use log::{LevelFilter, error, info};
use raspirus_backend::scanner::structs::{MetaMessage, ScannerMessage, Status};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

type Error = raspirus_backend::Error;

fn main() -> Result<(), Error> {
    // init global variables
    raspirus_backend::globals::get_mut_config().lock()?.load()?;
    raspirus_backend::globals::get_application_log_filename();

    // fetch arguments
    let parser = raspirus_backend::globals::get_parser()?.lock()?.clone();

    // handle help command
    if raspirus_backend::globals::get_argument((Some('h'), Some("help")))?.is_some() {
        println!("{}", parser);
        return Ok(());
    }

    // capture loglevel from environment variable, argument or config
    let level_filter = std::env::var("RUST_LOG")
        .unwrap_or(format!("{:?}", raspirus_backend::globals::get_loglevel()?))
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);

    // initialize term logging
    TermLogger::init(
        level_filter,
        ConfigBuilder::new()
            .add_filter_ignore_str("cranelift_codegen")
            .add_filter_ignore_str("wasmtime")
            .add_filter_ignore_str("walrus")
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .map_err(Error::LogInit)?;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // runs the update task
    if raspirus_backend::globals::get_argument((Some('u'), Some("update")))?.is_some() {
        rt.block_on(raspirus_backend::scanner::updater::update())?;
    }

    // runs a scan task
    if let Some(raspirus_backend::cli::ArgumentValue::String(Some(scan_path))) =
        raspirus_backend::globals::get_argument((Some('s'), Some("scan")))?
    {
        let channel = mpsc::channel();
        let watchdog_handle = std::thread::spawn(|| tui_watchdog(channel.1));
        rt.block_on(raspirus_backend::scanner::scanner::start(
            PathBuf::from(scan_path),
            channel.0,
        ))?;

        watchdog_handle
            .join()
            .map_err(|_| Error::WatchdogJoinError)??;
    }

    // early abort conditions in which case no gui should open
    if raspirus_backend::globals::get_argument((Some('n'), Some("nogui")))?.is_some()
        || raspirus_backend::globals::get_argument((Some('s'), Some("scan")))?.is_some()
    {
        return Ok(());
    }

    Ok(())
}

/// terminal output for cli scans
pub fn tui_watchdog(channel: mpsc::Receiver<ScannerMessage>) -> Result<(), Error> {
    let mut total_size = 0;
    let mut completed = 0;
    let mut current_paths = VecDeque::with_capacity(raspirus_backend::globals::get_max_threads()?);

    // output relevant data
    let mut stdout = std::io::stdout();
    let debounce_duration = Duration::from_millis(100);
    let mut last_print = Instant::now() - debounce_duration;

    // determine terminal output size
    let (cols, _) = size().unwrap_or((100, 100));

    loop {
        // receive events from backend
        match channel.recv().map_err(Error::WatchdogRecv)? {
            ScannerMessage::Meta(meta_message) => match meta_message {
                // scan is done, abort loop
                MetaMessage::ScanFinish => {
                    info!("Scan completed");
                    break;
                }
                // scan encountered unrecoverable error
                MetaMessage::ScanError(error) => return Err(error),
                // scan started; received total size
                MetaMessage::ScanStart(size) => total_size = size,
            },
            ScannerMessage::Processing(value) => match value.status {
                Status::Error(error, _) => error!(
                    "Encountered error {error} while scanning file {}",
                    value.path.to_str().unwrap_or("Undefined")
                ),
                Status::Completed(size) => {
                    completed += size;
                    // terminal output debounced
                    if last_print.elapsed() >= debounce_duration {
                        stdout
                            .execute(RestorePosition)
                            .map_err(Error::TerminalIOError)?;
                        stdout
                            .execute(Clear(ClearType::FromCursorDown))
                            .map_err(Error::TerminalIOError)?;
                        let paths = current_paths
                            .iter()
                            .map(|path: &PathBuf| {
                                path.file_name()
                                    .unwrap_or_default()
                                    .to_str()
                                    .unwrap_or_default()
                            })
                            .collect::<Vec<&str>>()
                            .join(", ");
                        todo!("Fix terminal output");
                        write!(
                            stdout,
                            "{}\nProgress {}%",
                            paths,
                            completed.saturating_mul(100) / total_size
                        )
                        .map_err(Error::TerminalIOError)?;
                        last_print = Instant::now();
                        stdout.flush().map_err(Error::TerminalIOError)?;
                    }
                    // remove from current list
                    current_paths.retain(|path| path != &value.path);
                }
                Status::Started => current_paths.push_back(value.path),
            },
        }
    }
    Ok(())
}
