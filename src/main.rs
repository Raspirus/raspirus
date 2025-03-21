// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config::Config;
use log::LevelFilter;
use simplelog::TermLogger;

mod arguments;
mod backend;
mod error;
mod frontend;
mod globals;

type Error = crate::error::Error;

fn main() -> Result<(), Error> {
    // init global variables
    crate::globals::get_mut_config().lock()?.load()?;
    crate::globals::get_application_log();

    // capture log level or fall back to info
    let level_filter = std::env::var("RUST_LOG")
        .unwrap_or(format!("{:?}", crate::globals::get_loglevel()))
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);

    // init logger with possibly inserted loglevel
    TermLogger::init(
        level_filter,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )
    .map_err(Error::LogInit)?;

    dbg!(crate::globals::get_loglevel());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(crate::backend::updater::update())?;

    Ok(())
}
