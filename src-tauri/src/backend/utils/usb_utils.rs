use log::{info, warn};
use std::{env, fs, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UsbDevice {
    name: String,
    path: String,
}

// Lists all the attached USBs for various platforms
pub async fn list_usb_drives() -> Result<String, String> {
    let mut usb_drives = Vec::new();

    // In Linux we look at a specific directory for mounted devices
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        info!("Trying to retrieve USB drives from Unix-like OS");
        let username = match env::var("USER") {
            Ok(val) => val,
            Err(_) => panic!("Could not get current username"),
        };

        let dir_path = format!("/media/{}", username);
        let entries = match fs::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(err) => {
                return Err(err.to_string());
            }
        };

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
    } else if cfg!(target_os = "windows") {
        #[cfg(windows)]
        let mut win_usb_drives = list_usb_windows();
        #[cfg(windows)]
        usb_drives.append(&mut win_usb_drives);
    } else {
        warn!("Not retrieving USBs -> Wrong OS");
    }
    Ok(serde_json::to_string(&usb_drives).unwrap())
}

// In Windows we need to iterate through all possible mount points and see what type of device is mounted
#[cfg(windows)]
fn list_usb_windows() -> Vec<UsbDevice> {
    use std::ffi::{OsStr, OsString};
    use std::iter::once;
    use std::os::windows::prelude::OsStrExt;
    use winapi::um::fileapi::GetDriveTypeW;
    use winapi::um::winbase::DRIVE_REMOVABLE;

    info!("Trying to retrieve USB drives from Windows OS");
    let mut usb_drives = Vec::new();
    let drive_letters: Vec<OsString> = vec![
        OsString::from("A"),
        OsString::from("B"),
        OsString::from("C"),
        OsString::from("D"),
        OsString::from("E"),
        OsString::from("F"),
        OsString::from("G"),
        OsString::from("H"),
        OsString::from("I"),
        OsString::from("J"),
        OsString::from("K"),
        OsString::from("L"),
        OsString::from("M"),
        OsString::from("N"),
        OsString::from("O"),
        OsString::from("P"),
        OsString::from("Q"),
        OsString::from("R"),
        OsString::from("S"),
        OsString::from("T"),
        OsString::from("U"),
        OsString::from("V"),
        OsString::from("W"),
        OsString::from("X"),
        OsString::from("Y"),
        OsString::from("Z"),
    ];
    for letter in drive_letters {
        // We retrieve all possible information to determine if its a removable USB device
        let drive_path = letter.clone().into_string().unwrap() + ":\\";
        let drive_path = Path::new(&drive_path);
        let drive_name = drive_path.file_name().unwrap_or_default();
        let drive_path = drive_path.to_str().unwrap();
        let wide_path = OsStr::new(&drive_path)
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<_>>();
        let drive_type = unsafe { GetDriveTypeW(wide_path.as_ptr()) };

        match fs::metadata(drive_path) {
            Ok(metadata) => {
                if metadata.is_dir() && drive_type == DRIVE_REMOVABLE {
                    info!("Found Drive: {}", drive_path);
                    usb_drives.push(UsbDevice {
                        name: drive_path.to_string() + " " + &drive_name.to_string_lossy(),
                        path: drive_path.to_string(),
                    });
                }
            }
            Err(_) => {}
        }
    }
    return usb_drives;
}