use leptos::*;
use crate::i18n::use_i18n;
use leptos_i18n::t;
use leptonic::prelude::icondata;
use crate::components::home_button::HomeButton;
use crate::components::settings_card_toggle::SettingsToggleCard;

// TODO:
// - Styling
// - Add Update, CustomDB, PatchFile, DownloadLogs component
// - Move the Mirror status to the info page
// - Remove the obfuscated mode
// - Add a "Reset to Default" button
// - Add a "Save" button
// - Add confirmation and update popups

#[component]
pub fn Settings() -> impl IntoView {
    let i18n = use_i18n();
    let (logging, setLogging) = create_signal(false);
    let (obfuscated, setObfuscated) = create_signal(false);
    let (use_db_path, setUseDbPath) = create_signal(false);
    let (custom_db_path, setCustomDbPath) = create_signal(String::new());
    let (scan_dir, setScanDir) = create_signal(false);
    let (hash_count, setHashCount) = create_signal(0);
    let (updated_date, setUpdatedDate) = create_signal(String::new());
    let (mirror, setMirror) = create_signal(String::new);

    // TODO: Add Update, CustomDB, PatchFile, DownloadLogs component
    // TODO: Move the Mirror status to the info page

    view! {
        <div>
            <div class="align-middle">
                <HomeButton />
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
        title=t!(i18n, obfuscated_mode)().to_string()
        short_description=t!(i18n, obfuscated_mode_val)().to_string()
        short_description_2=None
        icon=icondata::BiHideRegular
        is_on=obfuscated
        toggle_function=setObfuscated
      />

      <SettingsToggleCard
        title=t!(i18n, file_dialog_opt)().to_string()
        short_description=t!(i18n, file_dialog_opt_val)().to_string()
        short_description_2=Option::from(t!(i18n, file_dialog_opt_val2)().to_string())
        icon=icondata::AiFileZipOutlined
        is_on=scan_dir
        toggle_function=setScanDir
      />

      </div>
    }
}