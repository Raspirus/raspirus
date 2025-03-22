// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use relm4::{gtk::{prelude::ObjectExt, Settings}, RelmApp};
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

    if crate::arguments::get_argument(&crate::arguments::Argument::Update).is_some() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(crate::backend::updater::update())?;
    }

    let app = RelmApp::new("raspirus.app");
    if let Some(settings) = Settings::default() {
        settings.connect_notify_local(Some("gtk-application-prefer-dark-them"), |_settings, _| {
            println!("Dark mode changed");
        });
    }
    app.run::<frontend::main::model::AppModel>(0);

    Ok(())
}
