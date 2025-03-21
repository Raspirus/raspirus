// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config::Config;
use error::Error;
use log::LevelFilter;
use relm4::RelmApp;
use simplelog::TermLogger;

mod backend;
mod error;
mod frontend;
mod globals;

fn main() -> Result<(), Error> {
    // init global variables
    crate::globals::get_config().lock()?.load()?;
    crate::globals::get_application_log();

    // capture log level or fall back to info
    let level_filter = std::env::var("RUST_LOG")
        .unwrap_or("Info".to_owned())
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

    /*
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(crate::backend::updater::update())?;
     */

    // let scanner = Scanner::new(PathBuf::from("/home/gamingguy003/Downloads/"))?;
    let mut config = Config::default();
    config.load()?;

    //crate::backend::scanner::start(PathBuf::from("/home/gamingguy003/.cache"))?;

    let app = RelmApp::new("raspirus.app");
    app.run::<frontend::main::model::AppModel>(0);
    Ok(())
}
