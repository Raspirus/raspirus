// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use backend::scanner::{self, Scanner};
use log::LevelFilter;
use relm4::RelmApp;
use simplelog::{Config, TermLogger};

mod backend;
mod frontend;
mod globals;

fn main() -> Result<(), String> {
    //let time = chrono::NaiveDateTime::parse_from_str("2024-09-20T19:50:20Z", "%Y-%m-%dT%H:%M:%SZ");
    //dbg!(time);

    // init logger with possibly inserted loglevel
    let log_level = std::env::var("RUST_LOG").unwrap_or("Info".to_owned());
    let level_filter = log_level
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);
    TermLogger::init(
        level_filter,
        Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )
    .map_err(|err| format!("Failed to initialize termlogger: {err:?}"))?;

    let scanner = Scanner::new(PathBuf::from("/home/gamingguy003/Games"))?;

    let app = RelmApp::new("raspirus.app");
    app.run::<frontend::main::model::AppModel>(0);
    Ok(())
}
