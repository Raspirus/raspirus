use std::{path::PathBuf, str::FromStr};

use log::{debug, error, info, warn};

use crate::backend::{downloader, yara_scanner::YaraScanner};

#[cfg(all(not(debug_assertions), windows))]
pub fn remove_windows_console() {
    unsafe {
        windows_sys::Win32::System::Console::FreeConsole();
    }
}

// If a command is not yet implemented
pub fn not_implemented(app: tauri::AppHandle) {
    warn!("Function not implemented yet");
    app.exit(2);
}

// Starts the GUI without attaching a CLI
pub fn cli_gui(app: tauri::AppHandle) -> Result<(), tauri::Error> {
    debug!("Showing GUI...");
    #[cfg(all(not(debug_assertions), windows))]
    remove_windows_console();
    tauri::WebviewWindowBuilder::new(
        &app,
        "raspirus",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("Raspirus")
    .inner_size(800., 480.)
    .resizable(true)
    .build()?;
    debug!("This won't show on Windows release builds");
    Ok(())
}

// Starts the scanner on the CLI
pub fn cli_scanner(app: tauri::AppHandle, data: String) {
    let unescaped_str = data.replace("\\n", "\n").replace("\\t", "\t");
    debug!("Data provided: {}", unescaped_str);
    let mut scanner = match YaraScanner::new(None) {
        Ok(scanner) => scanner,
        Err(err) => {
            error!("Failed to initialize scanner: {err}");
            app.exit(-1);
            return;
        }
    };
    let path = match PathBuf::from_str(&unescaped_str) {
        Ok(path) => path,
        Err(err) => {
            error!("Failed to create path from arguments: {err}");
            app.exit(-1);
            return;
        }
    };
    match scanner.start(path) {
        Ok(res) => {
            info!("Result: {res:#?}");
            app.exit(0);
        }
        Err(err) => {
            error!("Error: {err}");
            app.exit(-1);
        }
    }
}

// Updates over the CLI
pub fn cli_update(app: tauri::AppHandle) {
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(err) => {
            error!("Failed to create tokio runtime: {err}");
            app.exit(-1);
            return;
        }
    };
    rt.block_on(async {
        match downloader::update().await {
            Ok(_) => {
                info!("Successfully updated");
                app.exit(0);
            }
            Err(err) => {
                error!(
                    "Error: {}",
                    match err {
                        downloader::RemoteError::Offline => "Offline".to_owned(),
                        downloader::RemoteError::Other(err) => err,
                    }
                );
                app.exit(-1);
            }
        }
    })
}
