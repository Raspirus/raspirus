use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;
use leptos::leptos_dom::error;
use leptos::logging::log;
use leptos_i18n::t;
use leptos_router::use_navigate;
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;

use crate::components::settings_card_download::SettingsDownloadCard;
use crate::components::settings_card_input::SettingsInputCard;
use crate::components::settings_card_patch::SettingsPatchCard;
use crate::components::settings_card_toggle::SettingsToggleCard;
use crate::components::settings_card_update::SettingsUpdateCard;
use crate::generic::{Config, int_to_date_string, SettingsArgs, SettingsStruct};
use crate::i18n::use_i18n;


#[component]
pub fn Settings() -> impl IntoView {
    let i18n = use_i18n();
    let (logging, setLogging) = create_signal(false);
    let (use_db_path, setUseDbPath) = create_signal(false);
    let (custom_db_path, setCustomDbPath) = create_signal(String::new());
    let (scan_dir, setScanDir) = create_signal(false);
    let (hash_count, setHashCount) = create_signal(0);
    let (updated_date, setUpdatedDate) = create_signal(String::new());
    let navigate = use_navigate();
    let formatted_updated_date = int_to_date_string((move || updated_date.get().parse::<i64>().unwrap_or_default())());

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
            }
            Err(e) => {
                error!("Error loading config: {:?}", e);
            }
        }
    });

    let navigate_home = move || {
        let settings_struct = SettingsStruct {
            logging_is_active: logging.get(),
            db_location: custom_db_path.get(),
            scan_dir: scan_dir.get(),
        };
        spawn_local(async move {
            // We need to convert the args to a string in order to send it to the backend
            let args = SettingsArgs {
                contents: serde_json::to_string(&settings_struct).unwrap()
            };
            log!("Args: {:?}", args);

            let result: Result<(), Error> = invoke("save_config_fe", &args).await;
            match result {
                Ok(_) => {
                    log!("Settings saved");
                }
                Err(e) => {
                    error!("Error saving settings: {:?}", e);
                }
            }
        });
        navigate("/", Default::default());
    };

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

        <SettingsUpdateCard
                title=t!(i18n, update_db)().to_string()
                short_description=t!(i18n, update_db_val)().to_string()
                short_description_2=format!("{}: {} | {}: {}",
                        t!(i18n, update_db_1)(), (move || hash_count.get())(),
                        t!(i18n, update_db_2)(), formatted_updated_date)
                icon=icondata::IoCloudDownload
            />

      <SettingsToggleCard
        title=t!(i18n, activate_logs)().to_string()
        short_description=t!(i18n, activate_logs_val)().to_string()
        short_description_2=None
        icon=icondata::FaFileLinesRegular
        is_on=logging
        toggle_function=setLogging
      />

        <SettingsInputCard
            title=t!(i18n, custom_db)().to_string()
            short_description=t!(i18n, custom_db_val)().to_string()
            short_description_2=if use_db_path.get() {format!("{}: {}", t!(i18n, custom_db_1)(), custom_db_path.get())} else {t!(i18n, custom_db_2)().to_string()}
            icon=icondata::FaDatabaseSolid
            set_value=setCustomDbPath
        />

      <SettingsToggleCard
        title=t!(i18n, file_dialog_opt)().to_string()
        short_description=t!(i18n, file_dialog_opt_val)().to_string()
        short_description_2=Option::from(t!(i18n, file_dialog_opt_val2)().to_string())
        icon=icondata::LuFolderSync
        is_on=scan_dir
        toggle_function=setScanDir
      />

        <SettingsPatchCard
            title=t!(i18n, add_patch_title)().to_string()
            short_description=t!(i18n, add_patch_desc)().to_string()
            short_description_2=t!(i18n, add_patch_desc2)().to_string()
            icon=icondata::BsDatabaseFillGear
        />

        <SettingsDownloadCard
            title=t!(i18n, download_logs)().to_string()
            short_description=t!(i18n, download_logs_desc)().to_string()
            icon=icondata::TbFileDownload
        />

      </div>
    }
}