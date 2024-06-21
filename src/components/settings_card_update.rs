use futures_util::StreamExt;
use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;
use leptos::logging::log;
use tauri_wasm::api::event::listen;
use crate::i18n::use_i18n;
use leptos_i18n::t;

#[component]
pub fn SettingsUpdateCard(
    title: String,
    short_description: String,
    short_description_2: String,
    icon: icondata::Icon,
) -> impl IntoView {
    let (show_modal, setShowModal) = create_signal(false);
    let (progress, setProgress) = create_signal(0.0);
    let (show_progress, setShowProgress) = create_signal(false);
    let (status, setStatus) = create_signal("".to_string());
    let i18n = use_i18n();

    // Progress listener for the Check state
    spawn_local(async move {
        let mut progress_event = listen::<String>("chck").await.expect("event listen error");
        while let Some(_) = progress_event.next().await {
            setShowProgress.set(false);
            setStatus.set(t!(i18n, db_update_stage_check)().to_string());
        }
    });

    // Progress listener for the Index state
    spawn_local(async move {
        let mut progress_event = listen::<String>("idx").await.expect("event listen error");
        while let Some(_) = progress_event.next().await {
            setShowProgress.set(false);
            setStatus.set(t!(i18n, db_update_stage_index)().to_string());
        }
    });

    // Progress listener for the Download state
    spawn_local(async move {
        let mut progress_event = listen::<String>("dwld").await.expect("event listen error");
        while let Some(event) = progress_event.next().await {
            setStatus.set(t!(i18n, db_update_stage_download)().to_string());
            setShowProgress.set(true);
            setProgress.set(event.payload.parse::<f64>().unwrap_or(0.0));
        }
    });

    // Progress listener for the Install state
    spawn_local(async move {
        let mut progress_event = listen::<String>("ins").await.expect("event listen error");
        while let Some(event) = progress_event.next().await {
            setStatus.set(t!(i18n, db_update_stage_install)().to_string());
            setShowProgress.set(true);
            setProgress.set(event.payload.parse::<f64>().unwrap_or(0.0));
        }
    });

    // Progress listener for the Error state
    spawn_local(async move {
        let mut progress_event = listen::<String>("err").await.expect("event listen error");
        while let Some(_) = progress_event.next().await {
            setStatus.set("Error".to_string());
            setShowProgress.set(false);
        }
    });

    let handle_button_click = move || {
        log!("Button clicked!");
        setShowModal.set(true);

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
                <Button on_press=move |_| handle_button_click()> "Update" </Button>
            </div>
        </div>
    }
}