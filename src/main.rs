// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::config::Config;
use error::Error;
use log::LevelFilter;
use gtk::prelude::ApplicationExt;
use gtk::{gio, glib};
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk, main_application, RelmApp,
};
use simplelog::TermLogger;
use crate::frontend::app::AppModel;
use crate::globals::APP_ID;

mod backend;
mod error;
mod frontend;
mod globals;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

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
    
    // GTK MAGIC
    gtk::init().unwrap();
    glib::set_application_name(&("RASPIRUS"));
    gtk::Window::set_default_icon_name(APP_ID);
    let app = main_application();

    let mut actions = RelmActionGroup::<AppActionGroup>::new();

    let quit_action = {
        let app = app.clone();
        RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };
    actions.add_action(quit_action);
    actions.register_for_main_application();

    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    let app = RelmApp::from_app(app);
    app.visible_on_activate(false).run::<AppModel>(());
    Ok(())
}
