use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UsbDevice {
    name: String,
    path: String,
}

// Lists all the attached USBs for various platforms
pub async fn list_usb_drives() -> Result<String, String> {
    let mut usb_drives: Vec<UsbDevice> = Vec::new();

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        info!("Trying to retrieve USB drives from Unix-like OS");

        let options = lfs_core::ReadOptions::default();
        let mut mounts = lfs_core::read_mounts(&options).unwrap();
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
                    entry
                        .clone()
                        .fs_label
                        .unwrap_or_else(|| panic!("Broken fs label for usb {entry:?}")),
                    entry
                        .clone()
                        .disk
                        .unwrap_or_else(|| panic!("Broken disk data for usb {entry:?}"))
                        .name
                ),
                path: entry
                    .info
                    .mount_point
                    .to_str()
                    .unwrap_or_else(|| panic!("Broken mount point for usb {entry:?}"))
                    .to_owned(),
            });
        }
    }

    #[cfg(target_os = "windows")]
    {
        let mut win_usb_drives = list_usb_windows();
        usb_drives.append(&mut win_usb_drives);
    }

    #[cfg(all(
        not(target_os = "windows"),
        not(target_os = "linux"),
        not(target_os = "macos")
    ))]
    warn!("Not retrieving USBs -> Wrong OS");

    Ok(serde_json::to_string(&usb_drives).expect("Couldnt convert usb drives to a Serde string"))
}

// In Windows we need to iterate through all possible mount points and see what type of device is mounted
#[cfg(windows)]
fn list_usb_windows() -> Vec<UsbDevice> {
    use std::ffi::OsStr;
    use std::fs;
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
    usb_drives
}
