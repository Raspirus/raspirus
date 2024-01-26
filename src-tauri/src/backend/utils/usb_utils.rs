use log::info;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct UsbDevice {
    name: String,
    path: String,
}

// NOTE: We have two functions here, one for Windows and one for Unix-like OS
// This is because Windows doesn't have a /run/media/username folder, so we need to iterate through all possible mount points

/// Lists all USB drives connected to the computer
/// Primarily works for Unix-like OS, the function for Windows is below
/// Returns a JSON string with the USB drives for the GUI, the CLI doesn't use this function
pub async fn list_usb_drives() -> Result<String, String> {
    let mut usb_drives: Vec<UsbDevice> = Vec::new();

    #[cfg(any(target_os= "linux", target_os = "macos"))]
    {
        info!("Trying to retrieve USB drives from Unix-like OS");
        // Retrieve the username here
        let username = match std::env::var("USER") {
            Ok(val) => val,
            Err(_) => panic!("Could not get current username"),
        };
        // Check for any folders inside /run/media/username
        let dir_path = format!("/run/media/{}", username);
        let entries = match fs::read_dir(dir_path) {
            // Assume each entry is a USB drive
            Ok(entries) => {
                entries
            },
            Err(err) => {
                return Err(format!("{err}"));
            }
        };

        // Iterate through all entries and add them to the vector as a UsbDevice struct
        for entry in entries {
            let entry = entry.expect("I couldn't read something inside the directory");
            let path = entry.path();

            usb_drives.push(UsbDevice {
                name: entry
                    .file_name()
                    .into_string()
                    .expect("File name is strange"),
                path: path
                    .as_path()
                    .to_str()
                    .expect("Path is strange")
                    .to_string(),
            });
        }
    }
    // If the OS is Windows, we call the function below
    #[cfg(target_os = "windows")]
    {
        let mut win_usb_drives = list_usb_windows();
        usb_drives.append(&mut win_usb_drives);
    }
    // If the OS is not Windows, Linux or MacOS, we warn the user
    #[cfg(all(not(target_os = "windows"), not(target_os = "linux"), not(target_os = "macos")))]
    warn!("Not retrieving USBs -> Wrong OS");

    Ok(serde_json::to_string(&usb_drives).expect("Couldnt convert usb drives to a Serde string"))
}

/// In Windows we need to iterate through all possible mount points and see what type of device is mounted
/// Basically we check from A to Z if there is a removable drive mounted
/// Its not the best solution, but it works
#[cfg(windows)]
fn list_usb_windows() -> Vec<UsbDevice> {
    // We need to use the WinAPI to check if the drive is removable, but on LInux it causes problems
    // So we only import it if we are on Windows
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::prelude::OsStrExt;
    use winapi::um::fileapi::GetDriveTypeW;
    use winapi::um::winbase::DRIVE_REMOVABLE;

    info!("Trying to retrieve USB drives from Windows OS");
    let mut usb_drives = Vec::new();
    for letter in 'A'..='Z' {
        // We retrieve all possible information to determine if its a removable USB device
        let drive_path = letter.clone().to_string() + ":\\";
        let drive_path = std::path::Path::new(&drive_path);
        let drive_name = drive_path.file_name().unwrap_or_default();
        let drive_path = drive_path.to_str().unwrap();
        let wide_path = OsStr::new(&drive_path)
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<_>>();
        let drive_type = unsafe { GetDriveTypeW(wide_path.as_ptr()) };

        // If the drive is a removable drive, we add it to the vector
        if let Ok(metadata) = fs::metadata(drive_path) {
            if metadata.is_dir() && drive_type == DRIVE_REMOVABLE {
                info!("Found Drive: {}", drive_path);
                usb_drives.push(UsbDevice {
                    name: drive_path.to_string() + " " + &drive_name.to_string_lossy(),
                    path: drive_path.to_string(),
                });
            }
        }
    }
    // Return the vector
    usb_drives
}
