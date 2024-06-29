use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::components::prelude::{ButtonColor, Toast, Toasts, ToastTimeout, ToastVariant};
use leptonic::prelude::icondata;
use leptos::*;
use leptos::logging::log;
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use uuid::Uuid;
use crate::i18n::use_i18n;
use leptos_i18n::t;

#[component]
pub fn SettingsDownloadCard(
    title: String,
    short_description: String,
    icon: icondata::Icon,
) -> impl IntoView {
    let toasts = expect_context::<Toasts>();
    let i18n = use_i18n();

    let handle_button_click = move || {
        spawn_local(async move {
            let output: Result<String, Error> = invoke("download_logs", &String::new()).await;
            match output {
                Ok(output) => {
                    log!("Output: {}", output);
                    toasts.push(
                        Toast {
                            id: Uuid::new_v4(),
                            created_at: time::OffsetDateTime::now_utc(),
                            variant: ToastVariant::Success,
                            header: t!(i18n, logs_download_dialog).into_view(),
                            body: format!("{} {}", t!(i18n, logs_download_dialog_text)().to_string(), output).into_view(),
                            timeout: ToastTimeout::DefaultDelay,
                        }
                    );
                }
                Err(e) => {
                    log!("Error: {:?}", e);
                    toasts.push(
                        Toast {
                            id: Uuid::new_v4(),
                            created_at: time::OffsetDateTime::now_utc(),
                            variant: ToastVariant::Error,
                            header: "Download failed".into_view(),
                            body: format!("Error: {}", e.to_string()).into_view(),
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
                    </div>
                </div>
                <Button on_press=move |_| handle_button_click() color=ButtonColor::Info> "Download" </Button>
            </div>
        </div>
    }
}
