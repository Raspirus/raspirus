use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::components::prelude::ButtonColor;
use leptonic::components::toast::{Toast, Toasts, ToastTimeout, ToastVariant};
use leptonic::prelude::icondata;
use leptos::*;
use leptos::logging::log;
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use crate::generic::SettingsPatchArgs;
use tauri_wasm::plugin::dialog::FileDialogBuilder;
use uuid::Uuid;
use crate::i18n::use_i18n;
use leptos_i18n::t;


/// A card that allows the user to patch the database with a file
/// The card opens a file picker where the user can choose a file they want to patch the database
/// with. The file is then sent to the backend where it is processed and the database is updated.
/// The card displays a success or error message in a toast.
#[component]
pub fn SettingsPatchCard(
    title: String,
    short_description: String,
    short_description_2: String,
    icon: icondata::Icon,
) -> impl IntoView {
    let i18n = use_i18n();
    let toasts = expect_context::<Toasts>();

    let handle_button_click = move || {
        spawn_local(async move {
            let file = FileDialogBuilder::new().pick_file().await;
            log!("Selected file: {:?}", file);
            // Same as with the folder, if the file is ok, we parse the path, else we don't do anything
            if let Ok(Some(path)) = file {
                let path_string = path.path.into_os_string().into_string().unwrap_or_default();
                log!("Selected file path: {:?}", path_string);

                // Here we send the path to the backend
                let result: Result<(usize, usize, usize), Error> = invoke("patch_settings", &SettingsPatchArgs{patchfile: path_string}).await;
                if let Ok(result_tuple) = result {
                    log!("Patch successful");
                    toasts.push(
                        Toast {
                            id: Uuid::new_v4(),
                            created_at: time::OffsetDateTime::now_utc(),
                            variant: ToastVariant::Success,
                            header: t!(i18n, add_patch_success).into_view(),
                            body: format!("{}: {} | {}: {} | {}: {}",
                                          t!(i18n, add_patch_result_inserted)(), result_tuple.0,
                                          t!(i18n, add_patch_result_removed)(), result_tuple.1,
                                          t!(i18n, add_patch_result_skipped)(), result_tuple.2
                            ).into_view(),
                            timeout: ToastTimeout::DefaultDelay,
                        }
                    );
                } else if let Err(e) = result {
                    log!("Patch failed: {:?}", e);
                    toasts.push(
                        Toast {
                            id: Uuid::new_v4(),
                            created_at: time::OffsetDateTime::now_utc(),
                            variant: ToastVariant::Error,
                            header: t!(i18n, add_patch_failed).into_view(),
                            body: format!("Error: {:?}", e).into_view(),
                            timeout: ToastTimeout::DefaultDelay,
                        }
                    );
                }
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
                <Button on_press=move |_| handle_button_click() color=ButtonColor::Info> {t!(i18n, add_patch_action)} </Button>
            </div>
        </div>
    }
}
