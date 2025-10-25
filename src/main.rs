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
mod cli;
mod error;
mod frontend;
mod globals;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn main() -> Result<(), Error> {
    // Parse CLI arguments
    let args = cli::CliArgs::parse_args();

    // init global variables
    crate::globals::get_config().lock()?.load()?;
    crate::globals::get_application_log();

    // Determine log level: CLI args take precedence
    let level_filter = if args.nolog || args.json {
        // Suppress logs for JSON output to ensure clean JSON
        LevelFilter::Off
    } else if args.debug {
        LevelFilter::Debug
    } else {
        std::env::var("RUST_LOG")
            .unwrap_or("Info".to_owned())
            .parse::<LevelFilter>()
            .unwrap_or(LevelFilter::Info)
    };

    // init logger with possibly inserted loglevel
    TermLogger::init(
        level_filter,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )
    .map_err(Error::LogInit)?;

    // Load config and apply CLI overrides
    let mut config = Config::default();
    config.load()?;

    // Apply thread limit override from CLI if specified
    if let Some(threads) = args.threads {
        config.max_threads = threads;
        log::info!("Thread limit overridden to: {}", threads);
    }

    // Handle CLI actions (update, scan) before launching GUI
    if args.update {
        log::info!("Running update from CLI...");
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| Error::Other(format!("Failed to build tokio runtime: {}", e)))?;

        match rt.block_on(crate::backend::updater::update()) {
            Ok(_) => log::info!("Update completed successfully"),
            Err(Error::RemoteAlreadyUpdated) => log::info!("Already up to date"),
            Err(e) => log::error!("Update failed: {}", e),
        }

        // If nogui is set, exit after update
        if args.nogui {
            return Ok(());
        }
    }

    if let Some(ref scan_path) = args.scan {
        log::info!("Running scan from CLI on: {}", scan_path.display());
        
        match crate::backend::scanner::start(scan_path.clone()) {
            Ok(results) => {
                if args.json {
                    // Output detailed scan results as JSON
                    match serde_json::to_string_pretty(&results) {
                        Ok(json) => println!("{}", json),
                        Err(e) => eprintln!("{{\"status\": \"error\", \"error\": \"Failed to serialize results: {}\" }}", e),
                    }
                } else {
                    log::info!("Scan completed successfully");
                    log::info!("Total files scanned: {}", results.total_files);
                    log::info!("Total size: {} bytes", results.total_size);
                    log::info!("Notable files: {}", results.notable_files.len());
                }
            },
            Err(e) => {
                if args.json {
                    println!("{{\"status\": \"error\", \"error\": \"{}\" }}", e);
                } else {
                    log::error!("Scan failed: {}", e);
                }
            }
        }

        // If nogui is set, exit after scan
        if args.nogui {
            return Ok(());
        }
    }

    // Exit early if nogui is set and no actions were performed
    if args.nogui && !args.has_action() {
        log::warn!("--nogui specified without any action (--update or --scan). Exiting.");
        return Ok(());
    }

    // Launch GUI only if not suppressed
    if !args.nogui {
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
        
        // Create initialization data for AppModel
        let app_init = crate::frontend::app::AppInit {
            fullscreen: args.fullscreen,
        };
        
        app.visible_on_activate(false).run::<AppModel>(app_init);
    }
    
    Ok(())
}
