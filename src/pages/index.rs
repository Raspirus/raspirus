use leptonic::components::button::{Button, LinkButton};
use leptonic::components::icon::Icon;
use leptonic::components::select::Select;
use leptos::*;
use leptonic::prelude::*;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use leptos::wasm_bindgen::JsValue;
use leptos::wasm_bindgen::prelude::wasm_bindgen;
use crate::i18n::use_i18n;
use leptos_i18n::t;
use crate::components::directory_picker_button::DirectoryPickerButton;

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
    let i18n = use_i18n();
    let (scan_target, setScanTarget) = create_signal(String::new());
    let (usb_devices, setUsbDevices) = create_signal(Vec::<String>::new());
    // TODO: The Raspberry Pi option has not been implemented yet
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
        <main class="h-screen">
        <div class="flex justify-start">

          <div class="flex justify-center absolute top-0 right-0">

            <Show when=move || {is_update_available.get()}>
                <LinkButton href="/settings" class="px-2 py-2 border-2 m-2 border-mainred
                  text-white bg-mainred font-medium text-xs leading-tight uppercase rounded">
                    <Icon
                      icon=icondata::FaWrenchSolid
                      class="pr-1"
                    />
                    {t!(i18n, db_update_notif)}
                  </LinkButton>
            </Show>

            <LinkButton href="/settings" class="px-6 py-2 border-2 m-2 border-maingreen
            text-maingreen bg-white font-medium text-xs leading-tight uppercase rounded">
              <Icon
                icon=icondata::OcGearLg
                class="pr-1"
              />
              {t!(i18n, settings)}
            </LinkButton>

          </div>
        </div>

        <div class="flex h-full justify-center p-12 text-center">
          <div class="flex justify-center items-center h-full">
            <div class="w-full">
              <h1 class="font-bold leading-tight text-8xl mt-0 mb-2 text-mainred uppercase">
                {t!(i18n, title)}
              </h1>

              <div class="flex justify-center">
                <Show when=move || {!is_directory.get()}
                    fallback=move || {view! {
                        <div class="m-auto px-3 py-1.5 text-gray-700 bg-white inline-block w-full
                                    border border-solid border-maingreen-light rounded overflow-hidden
                                    max-w-lg max-h-9">
                            {move || scan_target.get()}
                        </div>
                    }}
                    >
                    <Select
                        options=usb_devices.get()
                        search_text_provider=move |o| format!("{o}")
                        render_option=move |o| format!("{o:?}")
                        selected=scan_target
                        set_selected=move |v| setScanTarget.set(v)
                    />
                </Show>

                <DirectoryPickerButton
                    scan_target=setScanTarget
                    can_select_directories=can_select_directories />

                <Button on_press=move |_| update_usb_devices()
                  class="inline-block p-3 ml-1 bg-maingreen rounded shadow-md"
                >
                  <image
                    id="refresh-icon"
                    class="h-full w-4"
                    src="images/refresh.svg"
                    alt="Refresh"
                    width="500"
                    height="500"
                  />
                </Button>
              </div>
              <div class="mt-2">
                <LinkButton href="/information" class="mr-2 inline-block px-7 py-3 border-2
                border-maingreen text-maingreen bg-white font-medium text-sm uppercase rounded"
                >
                  {t!(i18n, info)}
                </LinkButton>
                <LinkButton href="/loading"
                  class="ml-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm uppercase rounded shadow-md"
                >
                  {t!(i18n, start)}
                </LinkButton>
              </div>
            </div>
          </div>
        </div>
      </main>
    }
}
