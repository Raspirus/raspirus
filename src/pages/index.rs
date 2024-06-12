use leptos::*;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use leptos::wasm_bindgen::JsValue;
use leptos::wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Debug)]
struct UsbDevice {
    name: String,
    path: String,
}

#[component]
pub fn Index() -> impl IntoView {
    let (scan_target, setScanTarget) = create_signal(String::new());
    let (usb_devices, setUsbDevices) = create_signal(Vec::<String>::new());
    let (is_raspberrypi, setIsRaspberrypi) = create_signal(false);
    // Flag to indicate if the selected target is a directory
    let (is_directory, setIsDirectory) = create_signal(false);
    // Flag to indicate if the file picker can select files or directories
    let (can_select_directories, setCanSelectDirectories) = create_signal(false);
    let (is_update_available, setIsUpdateAvailable) = create_signal(false);
    let (hash_count, setHashCount) = create_signal(0);

//    let update_selection = move |ev: &T| {
//        let target = event_target_value(ev);
//        setScanTarget.set(target);
//    };

    let update_usb_devices = move || {
        spawn_local(async move {
            let usb_devices = match invoke("list_usb_drives", JsValue::NULL).await.as_string() {
                Some(data) => {
                    // Log the received data
                    debug!("Received USB devices: {}", data);
                    let devices: Vec<UsbDevice> = serde_json::from_str(&data).unwrap();
                    devices.iter().map(|d| d.name.clone()).collect()
                }
                None => {
                    error!("Failed to receive USB devices");
                    vec![]
                }
            };
            setUsbDevices.set(usb_devices);
        });
    };


    view! {
        <div>
            <h1>{"Hello, World!"}</h1>
            <p>{"This is a simple example of a Leptos app."}</p>
        </div>
    }
}