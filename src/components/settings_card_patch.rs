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
            match file {
                Ok(Some(path)) => {
                    let path_buffer = path.path;
                    let path_string = path_buffer.into_os_string().into_string().unwrap_or_default();
                    log!("Selected file path: {:?}", path_string);

                    // Here we send the path to the backend
                    let result: Result<(usize, usize, usize), Error> = invoke("patch_settings", &SettingsPatchArgs{patchfile: path_string}).await;
                    match result {
                        Ok(result_tuple) => {
                            log!("Patch successful");
                            toasts.push(
                                Toast {
                                    id: Uuid::new_v4(),
                                    created_at: time::OffsetDateTime::now_utc(),
                                    variant: ToastVariant::Success,
                                    header: t!(i18n, add_patch_success).into_view(),
                                    body: format!("{}: {} | {}: {} | {}: {}",
                                        t!(i18n, add_patch_result_inserted)().to_string(), result_tuple.0,
                                        t!(i18n, add_patch_result_removed)().to_string(), result_tuple.1,
                                        t!(i18n, add_patch_result_skipped)().to_string(), result_tuple.2
                                    ).into_view(),
                                    timeout: ToastTimeout::DefaultDelay,
                                }
                            );
                        }
                        Err(e) => {
                            log!("Patch failed: {:?}", e);
                            toasts.push(
                                Toast {
                                    id: Uuid::new_v4(),
                                    created_at: time::OffsetDateTime::now_utc(),
                                    variant: ToastVariant::Error,
                                    header: t!(i18n, add_patch_failed).into_view(),
                                    body: format!("Error: {}", e.to_string()).into_view(),
                                    timeout: ToastTimeout::DefaultDelay,
                                }
                            );
                        }
                    }
                }
                _ => return,
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
