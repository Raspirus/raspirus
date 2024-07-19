use crate::components::modals::{error_modal::ErrorModal, welcome_modal::WelcomeModal};
use crate::components::{
    directory_picker_button::DirectoryPickerButton, language_switch::LanguageSwitch,
};
use crate::generic::{Config, UsbDevice};
use crate::i18n::use_i18n;
use leptonic::{components::prelude::*, prelude::*};
use leptos::logging::{error, log, warn};
use leptos::*;
use leptos_i18n::t;
use leptos_router::{use_navigate, use_query_map};
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use uuid::Uuid;

/// The Index - main page
/// It is the actual main page and can be called with parameters.
/// It will show the main page with the main buttons. It also loads the settings and checks for updates.
/// When the error parameter is passed, it will show the error modal.
/// The welcome modal is shown every time the hash count is 0.
#[component]
pub fn Index() -> impl IntoView {
    let i18n = use_i18n();
    // Check if we have an error in the query params
    let error = use_query_map().get_untracked().get("error").cloned();
    // Data for the error modal
    let (error_title, setErrorTitle) = create_signal(String::new());
    let (error_message, setErrorMessage) = create_signal(String::new());
    // Defines the scan target, default is None
    let (scan_target, setScanTarget) = create_signal(Option::<String>::None);
    let (usb_devices, setUsbDevices) = create_signal(Vec::<String>::new());
    // Flag to indicate if the file picker can select a single file or directories
    let (can_select_directories, setCanSelectDirectories) = create_signal(true);
    let (is_update_available, setIsUpdateAvailable) = create_signal(false);
    // Show modals
    let (show_error_modal, setShowErrorModal) = create_signal(false);
    let (show_welcome_modal, setShowWelcomeModal) = create_signal(false);
    let toasts = expect_context::<Toasts>();
    let navigate = use_navigate();

    // If the error is not empty, we show the error modal
    if let Some(error) = error {
        error!("Unexpected error: {:?}", error);
        setErrorTitle.set(t!(i18n, unexpected_error)().to_string());
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
                if config.hash_count == 0 {
                    setShowWelcomeModal.set(true);
                }
                log!("Config loaded")
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
                log!("Update checked");
            }
            Err(e) => {
                error!("Error checking for update: {:?}", e);
            }
        }
    });

    let update_usb_devices = move || {
        spawn_local(async move {
            // Retrieve a list of attached USB drives from the backend
            let usb_devices: Result<String, Error> =
                invoke("list_usb_drives", &String::new()).await;
            match usb_devices {
                Ok(usb_devices_string) => {
                    // We will get a JSON string with the USB devices
                    let usb_devices_list: Vec<UsbDevice> =
                        serde_json::from_str(&usb_devices_string).unwrap();
                    // Then we create a vector with the names of the USB devices
                    let usb_names: Vec<String> =
                        usb_devices_list.iter().map(|d| d.name.clone()).collect();
                    setUsbDevices.set(usb_names);
                    toasts.push(Toast {
                        id: Uuid::new_v4(),
                        created_at: time::OffsetDateTime::now_utc(),
                        variant: ToastVariant::Info,
                        header: t!(i18n, ui_update_title).into_view(),
                        body: t!(i18n, ui_update_text).into_view(),
                        timeout: ToastTimeout::DefaultDelay,
                    });
                    log!("USB devices updated");
                }
                Err(e) => {
                    error!("Error listing USB devices: {:?}", e);
                    toasts.push(Toast {
                        id: Uuid::new_v4(),
                        created_at: time::OffsetDateTime::now_utc(),
                        variant: ToastVariant::Error,
                        header: t!(i18n, usb_list_error).into_view(),
                        body: format!("Error: {:?}", e).into_view(),
                        timeout: ToastTimeout::DefaultDelay,
                    });
                    error!("Error listing USB devices: {:?}", e);
                }
            }
            // We also reset the selected target
            setScanTarget.set(None);
        });
    };

    // A function that programmatically navigates to the loading page if the selected target is not empty
    let navigate_to_loading = move || {
        let target = scan_target.get();
        if target.is_some() && !target.clone().unwrap_or_default().is_empty() {
            log!("Target selected, navigating to loading page");
            navigate(
                &format!("/loading?target={}", target.unwrap_or_default()),
                Default::default(),
            );
        } else {
            warn!("No target selected, showing error modal");
            toasts.push(Toast {
                id: Uuid::new_v4(),
                created_at: time::OffsetDateTime::now_utc(),
                variant: ToastVariant::Warn,
                header: t!(i18n, target_selection_error).into_view(),
                body: t!(i18n, target_selection_error_msg).into_view(),
                timeout: ToastTimeout::DefaultDelay,
            });
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
                <LinkButton href="/settings" class="px-2 py-2 m-2 font-medium text-sm leading-tight uppercase">
                    <Icon
                      icon=icondata::FaWrenchSolid
                      class="pr-1"
                      style="font-size: 1.25rem;"
                    />
                    {t!(i18n, db_update_notif)}
                  </LinkButton>
            </Show>

            <LinkButton href="/settings" variant=ButtonVariant::Outlined
                class="px-6 py-2 m-2 font-medium text-sm leading-tight uppercase">
              <Icon
                icon=icondata::OcGearLg
                style="font-size: 1.25rem;"
              />
              {t!(i18n, settings)}
            </LinkButton>
          </div>
        </div>

        <div class="flex h-full justify-center p-12 text-center">
          <div class="flex justify-center items-center h-full">
            <div class="w-full">
              <h1 class="font-bold leading-tight text-8xl mt-0 mb-2 text-mainred uppercase">
                {t!(i18n, app_title)}
              </h1>

              <div class="flex justify-center">
                <OptionalSelect
                        options=usb_devices
                        search_text_provider=move |o| o
                        render_option=move |o| format!("{o:?}")
                        selected=scan_target
                        set_selected=move |v| setScanTarget.set(v)
                        allow_deselect=true
                    />

                <DirectoryPickerButton
                    scan_target=setScanTarget
                    can_select_directories=can_select_directories />

                <Button on_press=move |_| update_usb_devices()
                    color=ButtonColor::Info
                  class="inline-block p-3 ml-1 shadow-md"
                >
                    <Icon icon=icondata::TbReload class="h-full w-4" />
                </Button>
              </div>
              <div class="mt-2">
                <LinkButton href="/information" variant=ButtonVariant::Outlined
                    class="mr-2 inline-block px-7 py-3 font-medium text-sm uppercase"
                >
                  {t!(i18n, info)}
                </LinkButton>
                <Button on_press=move |_| navigate_to_loading() color=ButtonColor::Primary
                  class="ml-2 inline-block px-7 py-3 font-medium text-sm uppercase shadow-md"
                >
                  {t!(i18n, start)}
                </Button>
              </div>
                <p class="mt-5">{t!(i18n, terms_part_1)} <Link href="/agreement">{t!(i18n, terms_part_2)}</Link></p>
            </div>
          </div>
        </div>
      </main>
    }
}
