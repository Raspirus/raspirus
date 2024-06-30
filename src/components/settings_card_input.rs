use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::components::prelude::ButtonColor;
use leptonic::prelude::icondata;
use leptos::*;
use leptos::logging::log;
use tauri_wasm::plugin::dialog::FileDialogBuilder;
use crate::i18n::use_i18n;
use leptos_i18n::t;


/// A card where the user can select a file from the file system
/// The file path is then stored in the provided signal for further processing. This
/// makes the selection of a file a reusable component.
#[component]
pub fn SettingsInputCard(
    title: String,
    short_description: String,
    short_description_2: String,
    icon: icondata::Icon,
    set_value: WriteSignal<String>,
) -> impl IntoView {
    let i18n = use_i18n();

    let handle_button_click = move || {
        spawn_local(async move {
            let file = FileDialogBuilder::new().pick_file().await;
            log!("Selected file: {:?}", file);
            // Same as with the folder, if the file is ok, we parse the path, else we don't do anything
            if let Ok(Some(path)) = file {
                let path_string = path.path.into_os_string().into_string().unwrap_or_default();
                set_value.set(path_string);
            }
        });
    };

    view! {
        <div class="flex flex-col m-6 p-2 bg-white rounded-2xl shadow-md">
            <div class="flex items-center justify-between mx-4">
                <div class="flex items-center">
                    <Icon icon=icon
                        class="w-16 h-16 rounded-2xl p-3 border border-maingreen-light text-maingreen-light bg-green-50"
                    />
                    <div class="flex flex-col ml-3">
                        <div class="font-medium">{title}</div>
                        <p class="text-sm text-gray-600 leading-none mt-1">{short_description}</p>
                        <p class="text-sm text-gray-600 leading-none mt-1">{short_description_2}</p>
                    </div>
                </div>
                <Button on_press=move |_| handle_button_click() color=ButtonColor::Info>{t!(i18n, custom_db_action)}</Button>
            </div>
        </div>
    }
}
