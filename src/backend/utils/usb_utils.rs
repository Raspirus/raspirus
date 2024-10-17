use std::{fmt::Display, path::PathBuf};

#[cfg(not(target_os = "windows"))]
use log::debug;
#[cfg(target_os = "windows")]
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct UsbDevice {
    pub name: String,
    pub path: PathBuf,
}

impl Display for UsbDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.path.to_string_lossy(), self.name)
    }
}

// Lists all the attached USBs for various platforms
pub fn list_usb_drives() -> Result<Vec<UsbDevice>, String> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    return list_usb_linux();
    #[cfg(target_os = "windows")]
    return list_usb_windows();
    #[cfg(all(
        not(target_os = "windows"),
        not(target_os = "linux"),
        not(target_os = "macos")
    ))]
    Err("Not retrieving USBs -> Unsupported OS. Please open an issue on Github".to_owned());
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn list_usb_linux() -> Result<Vec<UsbDevice>, String> {
    let mut usb_drives: Vec<UsbDevice> = Vec::new();

    debug!("Trying to retrieve USB drives from Unix-like OS");

    let options = lfs_core::ReadOptions::default();
    let mut mounts = lfs_core::read_mounts(&options)
        .map_err(|err| format!("Failed to list usb drives: {err}"))?;
    // filter not ok / non removable drives
    mounts.retain(|m| {
        m.stats.is_ok()
            && match m.disk.clone() {
                Some(disk) => disk.removable.unwrap_or_default(),
                None => false,
            }
    });

    for entry in mounts {
        usb_drives.push(UsbDevice {
            name: format!(
                "{} ({})",
                entry.clone().fs_label.unwrap_or("No Label".to_owned()),
                entry
                    .clone()
                    .disk
                    .unwrap_or_else(|| panic!("Broken disk data for usb {entry:?}"))
                    .name
            ),
            path: entry.info.mount_point,
        });
    }
    debug!("Found: {usb_drives:?}");
    Ok(usb_drives)
}

// In Windows we need to iterate through all possible mount points and see what type of device is mounted
#[cfg(target_os = "windows")]
fn list_usb_windows() -> Result<Vec<UsbDevice>, String> {
    use std::ffi::OsStr;
    use std::fs;
    use std::iter::once;
    use std::os::windows::prelude::OsStrExt;
    use winapi::um::fileapi::GetDriveTypeW;
    use winapi::um::winbase::DRIVE_REMOVABLE;

    debug!("Trying to retrieve USB drives from Windows OS");
    let mut usb_drives = Vec::new();
    for letter in 'A'..='Z' {
        // We retrieve all possible information to determine if its a removable USB device
        let drive_path = letter.clone().to_string() + ":\\"; // C:\\
        let drive_path_os = OsStr::new(std::path::Path::new(&drive_path));

        let wide_path = drive_path_os
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<_>>();

        let drive_type = unsafe { GetDriveTypeW(wide_path.as_ptr()) };

        if let Ok(metadata) = fs::metadata(drive_path.clone()) {
            if metadata.is_dir() && drive_type == DRIVE_REMOVABLE {
                debug!("Found Drive: {}", drive_path);
                usb_drives.push(UsbDevice {
                    name: letter.to_string(),
                    path: drive_path.to_string().into(),
                });
            }
        }
    }
    debug!("Found: {usb_drives:?}");
    Ok(usb_drives)
}
