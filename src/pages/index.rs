use leptonic::components::button::{Button, LinkButton};
use leptonic::components::icon::Icon;
use leptonic::components::link::Link;
use leptonic::components::prelude::{Toast, Toasts, ToastTimeout, ToastVariant};
use leptonic::components::select::Select;
use leptos::*;
use leptos::logging::{log, error};
use leptonic::prelude::*;
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use crate::i18n::use_i18n;
use leptos_i18n::t;
use leptos_router::{use_navigate, use_query_map};
use uuid::Uuid;
use crate::components::{
    directory_picker_button::DirectoryPickerButton,
    language_switch::LanguageSwitch,
};
use crate::components::modals::error_modal::ErrorModal;
use crate::components::modals::welcome_modal::WelcomeModal;
use crate::generic::{Config, UsbDevice};

// TODO:
// - Styling

#[component]
pub fn Index() -> impl IntoView {
    let i18n = use_i18n();
    let error = use_query_map().get_untracked().get("error").cloned();
    let (error_title, setErrorTitle) = create_signal(String::new());
    let (error_message, setErrorMessage) = create_signal(String::new());
    let (scan_target, setScanTarget) = create_signal(String::new());
    let (usb_devices, setUsbDevices) = create_signal(Vec::<String>::new());
    // Flag to indicate if the file picker can select files or directories
    let (can_select_directories, setCanSelectDirectories) = create_signal(true);
    let (is_update_available, setIsUpdateAvailable) = create_signal(false);
    let (show_error_modal, setShowErrorModal) = create_signal(false);
    let (show_welcome_modal, setShowWelcomeModal) = create_signal(false);
    let toasts = expect_context::<Toasts>();

    let navigate = use_navigate();

    // If the error is not empty, we show the error modal
    if let Some(error) = error {
        log!("Index Error: {:?}", error);
        setErrorTitle.set("Unexpected Error".to_string());
        setErrorMessage.set(error);
        setShowErrorModal.set(true);
        // Then we remove the error from the query params
        navigate("/", Default::default());
    }

    // We have to call a couple invoke commands to set the initial state of the app
    spawn_local(async move {
        // First we load the config
        let config: Result<String, Error> = invoke("load_config_fe", &String::new()).await;
        match config {
            Ok(config_string) => {
                let config: Config = serde_json::from_str(&config_string).unwrap();
                // We set the flag to indicate if the file picker can select directories
                setCanSelectDirectories.set(config.scan_dir);
                if config.hash_count <= 0 {
                    setShowWelcomeModal.set(true);
                }
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
                    toasts.push(
                        Toast {
                            id: Uuid::new_v4(),
                            created_at: time::OffsetDateTime::now_utc(),
                            variant: ToastVariant::Info,
                            header: "UI updated".into_view(),
                            body: "USB drives reloaded".into_view(),
                            timeout: ToastTimeout::DefaultDelay,
                        }
                    );
                }
                Err(e) => {
                    error!("Error listing USB devices: {:?}", e);
                    toasts.push(
                        Toast {
                            id: Uuid::new_v4(),
                            created_at: time::OffsetDateTime::now_utc(),
                            variant: ToastVariant::Error,
                            header: "Error listing USB devices".into_view(),
                            body: format!("Error: {}", e.to_string()).into_view(),
                            timeout: ToastTimeout::DefaultDelay,
                        }
                    );
                }
            }
            // We also reset the selected target
            setScanTarget.set(String::new());
        });
    };

    // A function that programmatically navigates to the loading page if the selected target is not empty
    let navigate_to_loading = move || {
        let target = scan_target.get();
        if !target.is_empty() {
            navigate(&format!("/loading?target={}", target), Default::default());
        } else {
            toasts.push(
                Toast {
                    id: Uuid::new_v4(),
                    created_at: time::OffsetDateTime::now_utc(),
                    variant: ToastVariant::Warn,
                    header: "No target selected".into_view(),
                    body: "Please select something to scan".into_view(),
                    timeout: ToastTimeout::DefaultDelay,
                }
            );
        }
    };


    view! {
        <main class="h-screen">
        <div class="flex justify-start">
            <LanguageSwitch />
            <ErrorModal
                show_modal=show_error_modal
                set_show_modal=setShowErrorModal
                title=error_title
                body=error_message
                />
            <WelcomeModal
                show_modal=show_welcome_modal
                set_show_modal=setShowWelcomeModal
                />
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
                <Button on_press=move |_| navigate_to_loading()
                  class="ml-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm uppercase rounded shadow-md"
                >
                  {t!(i18n, start)}
                </Button>
              </div>
        <p>"By using this app, you accept our " <Link href="/agreement">"Terms and Conditions."</Link></p>
            </div>
          </div>
        </div>
      </main>
    }
}
