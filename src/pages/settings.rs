use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::components::toast::{Toast, Toasts, ToastTimeout, ToastVariant};
use leptos::*;
use crate::i18n::use_i18n;
use leptos_i18n::t;
use leptonic::prelude::icondata;
use leptos::leptos_dom::error;
use leptos::logging::log;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use uuid::Uuid;
use crate::components::home_button::HomeButton;
use crate::components::settings_card_toggle::SettingsToggleCard;
use crate::generic::Config;

// TODO:
// - Styling
// - Add Update, CustomDB, PatchFile, DownloadLogs component
// - Remove the obfuscated mode
// - Add a "Reset to Default" button
// - Add a "Save" button
// - Add confirmation and update popups

#[derive(Serialize, Deserialize)]
struct SettingsArgs {
    logging_is_active: bool,
    db_location: String,
    scan_dir: bool,
    mirror: String,
}

#[component]
pub fn Settings() -> impl IntoView {
    let i18n = use_i18n();
    let (logging, setLogging) = create_signal(false);
    let (use_db_path, setUseDbPath) = create_signal(false);
    let (custom_db_path, setCustomDbPath) = create_signal(String::new());
    let (scan_dir, setScanDir) = create_signal(false);
    let (hash_count, setHashCount) = create_signal(0);
    let (updated_date, setUpdatedDate) = create_signal(String::new());
    let (mirror, setMirror) = create_signal(String::new());
    let (changes_detected, setChangesDetected) = create_signal(false);
    let navigate = use_navigate();
    let toasts = expect_context::<Toasts>();

    let logging_watch = watch(
        move || logging.get(),
        move |_, _, _| {setChangesDetected.set(true)},
        false
    );
    let custom_db_watch = watch(
        move || custom_db_path.get(),
        move |_, _, _| {setChangesDetected.set(true)},
        false
    );
    let scandir_watch = watch(
        move || scan_dir.get(),
        move |_, _, _| {setChangesDetected.set(true)},
        false
    );
    let mirror_watch = watch(
        move || mirror.get(),
        move |_, _, _| {setChangesDetected.set(true)},
        false
    );

    spawn_local(async move {
        let config: Result<String, Error> = invoke("load_config_fe", &String::new()).await;
        match config {
            Ok(config_string) => {
                let config: Config = serde_json::from_str(&config_string).unwrap();
                setLogging.set(config.logging_is_active);
                setUseDbPath.set(!config.db_location.is_empty());
                setCustomDbPath.set(config.db_location);
                setScanDir.set(config.scan_dir);
                setHashCount.set(config.hash_count);
                setUpdatedDate.set(config.last_db_update);
                setMirror.set(config.mirror);
            }
            Err(e) => {
                error!("Error loading config: {:?}", e);
            }
        }
    });

    let save_settings = move || {
        // The only things that can change are:
        // - logging_is_active
        // - db_location
        // - scan_dir
        // - mirror
        let args = SettingsArgs {
            logging_is_active: logging.get(),
            db_location: custom_db_path.get(),
            scan_dir: scan_dir.get(),
            mirror: mirror.get(),
        };
        spawn_local(async move {
            let result: Result<String, Error> = invoke("save_config_fe", &args).await;
            match result {
                Ok(_) => {
                    log!("Settings saved");
                    setChangesDetected.set(false);
                }
                Err(e) => {
                    error!("Error saving settings: {:?}", e);
                }
            }
        });
    };

    let navigate_home = move || {
        if changes_detected.get() {
            toasts.push(
                Toast {
                    id: Uuid::new_v4(),
                    created_at: time::OffsetDateTime::now_utc(),
                    variant: ToastVariant::Error,
                    header: "Changes not saved".into_view(),
                    body: "You have unsaved changes!".into_view(),
                    timeout: ToastTimeout::DefaultDelay,
                }
            )
        } else {
            // We stop the watch effects
            logging_watch();
            custom_db_watch();
            scandir_watch();
            mirror_watch();
            // We navigate back to the home page
            navigate("/", Default::default());
        }
    };

    // TODO: Add Update, CustomDB, PatchFile, DownloadLogs component
    // TODO: Move the Mirror status to the info page

    view! {
        <div>
            <div class="align-middle">
                        <Button on_press=move |_| navigate_home() class="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md">
                            <Icon icon=icondata::AiHomeFilled class="pr-1" />
                            {t!(i18n, back_btn)}
                        </Button>
                <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                  {t!(i18n, settings_title)}
                </h1>
            </div>


      <SettingsToggleCard
        title=t!(i18n, activate_logs)().to_string()
        short_description=t!(i18n, activate_logs_val)().to_string()
        short_description_2=None
        icon=icondata::FaFileLinesRegular
        is_on=logging
        toggle_function=setLogging
      />

      <SettingsToggleCard
        title=t!(i18n, file_dialog_opt)().to_string()
        short_description=t!(i18n, file_dialog_opt_val)().to_string()
        short_description_2=Option::from(t!(i18n, file_dialog_opt_val2)().to_string())
        icon=icondata::AiFileZipOutlined
        is_on=scan_dir
        toggle_function=setScanDir
      />

        <Show when=move || {changes_detected.get()}>
            <div>
                <Button on_press=move |_| save_settings() class="inline-block align-middle px-6 py-2.5 m-2 bg-maingreen text-white font-medium text-xs leading-tight uppercase rounded shadow-md">
                    <Icon icon=icondata::IoSave class="pr-1" />
                    {"Save".to_string()}
                </Button>
            </div>
        </Show>

      </div>
    }
}