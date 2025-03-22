// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use arguments::{get_argument, Argument};
use log::LevelFilter;
use relm4::RelmApp;
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

    if let Some(Argument::Invalid(Some(invalid))) = get_argument(&Argument::Invalid(None)) {
        Err(Error::InvalidArgument(invalid))?
    }

    //if let Some(Argument::Help) = get_argument(&Argument::Help) {}

    // capture log level or fall back to info
    let level_filter = std::env::var("RUST_LOG")
        .unwrap_or(format!("{:?}", crate::globals::get_loglevel()))
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);

    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_ignore_str("cranelift_codegen")
        .add_filter_ignore_str("wasmtime")
        .build();

    // init logger with possibly inserted loglevel
    TermLogger::init(
        level_filter,
        log_config,
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )
    .map_err(Error::LogInit)?;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if get_argument(&Argument::Update).is_some() {
        rt.block_on(crate::backend::updater::update())?;
    }

    if let Some(Argument::Scan(Some(path))) = get_argument(&Argument::Scan(None)) {
        rt.block_on(crate::backend::scanner::start(PathBuf::from(path)))?
    }

    if get_argument(&Argument::NoGUI).is_none() {
        let app = RelmApp::new("raspirus.app");
        app.run::<frontend::main::model::AppModel>(0);
    }
    Ok(())
}
