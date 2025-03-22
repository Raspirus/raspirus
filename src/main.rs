// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

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

    dbg!(crate::globals::get_loglevel());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if crate::arguments::get_argument(&crate::arguments::Argument::Update).is_some() {
        rt.block_on(crate::backend::updater::update())?;
    }

    if let Some(arg) = crate::arguments::get_argument(&crate::arguments::Argument::Scan(None)) {
        match arg {
            arguments::Argument::Scan(Some(path)) => {
                rt.block_on(crate::backend::scanner::start(PathBuf::from(path)))?
            }
            _ => todo!(),
        }
    }

    if crate::arguments::get_argument(&crate::arguments::Argument::NoGUI).is_none() {
        relm4::gtk::init();

        if let Some(settings) = relm4::gtk::Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(true);
        }

        let app = RelmApp::new("raspirus.app");
        app.run::<frontend::main::model::AppModel>(0);
    }
    Ok(())
}
