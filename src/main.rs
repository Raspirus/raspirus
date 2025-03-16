// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use relm4::RelmApp;

mod frontend;
mod tests;

fn main() -> Result<(), String> {
    let app = RelmApp::new("raspirus.app");
    app.run::<frontend::main::model::AppModel>(0);
    Ok(())
}
