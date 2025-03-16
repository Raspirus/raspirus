// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use relm4::RelmApp;

mod backend;
mod frontend;
mod globals;

fn main() -> Result<(), String> {
    //let time = chrono::NaiveDateTime::parse_from_str("2024-09-20T19:50:20Z", "%Y-%m-%dT%H:%M:%SZ");
    //dbg!(time);

    let app = RelmApp::new("raspirus.app");
    app.run::<frontend::main::model::AppModel>(0);
    Ok(())
}
