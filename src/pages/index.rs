use leptonic::components::button::{Button, LinkButton};
use leptonic::components::icon::Icon;
use leptonic::components::select::Select;
use leptonic::components::theme::{LeptonicTheme, ThemeToggle};
use leptos::*;
use leptos::logging::{log, error};
use leptonic::prelude::*;
use serde::{Deserialize, Serialize};
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use crate::i18n::use_i18n;
use leptos_i18n::t;
use crate::components::{
    directory_picker_button::DirectoryPickerButton,
    language_switch::LanguageSwitch,
};
use crate::generic::{Config, UsbDevice};

#[component]
pub fn Index() -> impl IntoView {
    let i18n = use_i18n();
    let (scan_target, setScanTarget) = create_signal(String::new());
    let (usb_devices, setUsbDevices) = create_signal(Vec::<String>::new());
    // Flag to indicate if the file picker can select files or directories
    let (can_select_directories, setCanSelectDirectories) = create_signal(true);
    let (is_update_available, setIsUpdateAvailable) = create_signal(false);

    // We have to call a couple invoke commands to set the initial state of the app
    spawn_local(async move {
        // First we load the config
        let config: Result<String, Error> = invoke("load_config_fe", &String::new()).await;
        match config {
            Ok(config_string) => {
                let config: Config = serde_json::from_str(&config_string).unwrap();
                // We set the flag to indicate if the file picker can select directories
                setCanSelectDirectories.set(config.scan_dir);
            }
            Err(e) => {
                error!("Error loading config: {:?}", e);
            }
        }
    });

    // We check if there is an update available
    spawn_local(async move {
        let update_available: Result<bool, Error> = invoke("check_update", &String::new()).await;
        match update_available {
            Ok(update_available) => {
                setIsUpdateAvailable.set(update_available);
            }
            Err(e) => {
                error!("Error checking for update: {:?}", e);
            }
        }
    });

    let update_usb_devices = move || {
        spawn_local(async move {
            let usb_devices: Result<String, Error> = invoke("list_usb_drives", &String::new()).await;
            match usb_devices {
                Ok(usb_devices_string) => {
                    // We will get a JSON string with the USB devices
                    let usb_devices_list: Vec<UsbDevice> = serde_json::from_str(&usb_devices_string).unwrap();
                    // Then we create a vector with the names of the USB devices
                    let usb_names: Vec<String> = usb_devices_list.iter().map(|d| d.name.clone()).collect();
                    setUsbDevices.set(usb_names);
                }
                Err(e) => {
                    error!("Error listing USB devices: {:?}", e);
                }
            }
        });
    };


    view! {
        <main class="h-screen">
        <div class="flex justify-start">
            <LanguageSwitch />
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
            <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>

          </div>
        </div>

        <div class="flex h-full justify-center p-12 text-center">
          <div class="flex justify-center items-center h-full">
            <div class="w-full">
              <h1 class="font-bold leading-tight text-8xl mt-0 mb-2 text-mainred uppercase">
                {t!(i18n, title)}
              </h1>

              <div class="flex justify-center">
                // TODO: The select below is not being updated with the new USB devices
                <Select
                        options=usb_devices
                        search_text_provider=move |o| format!("{o}")
                        render_option=move |o| format!("{o:?}")
                        selected=scan_target
                        set_selected=move |v| setScanTarget.set(v)
                    />

                <DirectoryPickerButton
                    scan_target=setScanTarget
                    can_select_directories=can_select_directories />

                <Button on_press=move |_| update_usb_devices()
                  class="inline-block p-3 ml-1 bg-maingreen rounded shadow-md"
                >
                    <Icon icon=icondata::TbReload class="h-full w-4" />
                </Button>
              </div>
              <div class="mt-2">
                <LinkButton href="/information" class="mr-2 inline-block px-7 py-3 border-2
                border-maingreen text-maingreen bg-white font-medium text-sm uppercase rounded"
                >
                  {t!(i18n, info)}
                </LinkButton>
                <LinkButton href= move || format!("/loading?target={}", scan_target.get())
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
