use crate::components::home_button::HomeButton;
use crate::components::modals::{error_modal::ErrorModal, success_modal::SuccessModal};
use futures_util::StreamExt;
use leptonic::components::prelude::ProgressBar;
use leptos::leptos_dom::log;
use leptos::*;
use tauri_wasm::api::core::invoke;
use tauri_wasm::api::event::listen;
use tauri_wasm::Error;

use crate::i18n::{t, use_i18n};

/// The Updating page component.
/// This component is responsible for updating the database. It listens for events from the backend
/// and updates the UI accordingly. When opened, it will trigger the `update_database` command.
/// Depending on the events received, it will show a progress bar, success modal or error modal.
#[component]
pub fn Updating() -> impl IntoView {
    let (completed_state, setCompletedState) = create_signal(false);
    let (show_success_modal, setShowSuccessModal) = create_signal(false);
    let (show_error_modal, setShowErrorModal) = create_signal(false);
    let (progress, setProgress) = create_signal(Some(0.0));
    let (show_progress, setShowProgress) = create_signal(false);
    let (status, setStatus) = create_signal(String::new());
    let (error_message, setErrorMessage) = create_signal(String::new());
    let i18n = use_i18n();

    // Progress listener for the Check state
    spawn_local(async move {
        let mut progress_event = listen::<String>("chck").await.expect("event listen error");
        while progress_event.next().await.is_some() {
            log!("Check event received");
            setShowProgress.set(false);
            setStatus.set(t!(i18n, db_update_stage_check)().to_string());
        }
    });

    // Progress listener for the Index state
    spawn_local(async move {
        let mut progress_event = listen::<String>("idx").await.expect("event listen error");
        while progress_event.next().await.is_some() {
            log!("Index event received");
            setShowProgress.set(false);
            setStatus.set(t!(i18n, db_update_stage_index)().to_string());
        }
    });

    // Progress listener for the Download state
    spawn_local(async move {
        let mut progress_event = listen::<String>("dwld").await.expect("event listen error");
        while let Some(event) = progress_event.next().await {
            log!("Download event received with payload: {}", event.payload);
            setStatus.set(t!(i18n, db_update_stage_download)().to_string());
            setShowProgress.set(true);
            setProgress.set(Option::from(event.payload.parse::<f64>().unwrap_or(0.0)));
        }
    });

    // Progress listener for the Install state
    spawn_local(async move {
        let mut progress_event = listen::<String>("ins").await.expect("event listen error");
        while let Some(event) = progress_event.next().await {
            log!("Install event received with payload: {}", event.payload);
            setStatus.set(t!(i18n, db_update_stage_install)().to_string());
            setShowProgress.set(true);
            setProgress.set(Option::from(event.payload.parse::<f64>().unwrap_or(0.0)));
        }
    });

    // Progress listener for the Error state
    spawn_local(async move {
        let mut progress_event = listen::<String>("err").await.expect("event listen error");
        while let Some(event) = progress_event.next().await {
            log!("Error event received: {}", event.payload);
            setStatus.set(t!(i18n, update_db_failed)().to_string());
            setErrorMessage.set(event.payload);
            setShowProgress.set(false);
            setCompletedState.set(true);
            setShowErrorModal.set(true);
        }
    });

    spawn_local(async move {
        let return_value: Result<String, Error> = invoke("update", &String::new()).await;
        match return_value {
            Ok(_) => {
                log!("Database update successful");
                setCompletedState.set(true);
                setShowSuccessModal.set(true);
            }
            Err(e) => {
                log!("Database update failed: {:?}", e);
                setErrorMessage.set(e.to_string());
                setShowErrorModal.set(true);
            }
        }
    });

    view! {
        <div class="h-screen">
            <SuccessModal
                show_modal=show_success_modal
                set_show_modal=setShowSuccessModal
                title=create_signal(t!(i18n, update_db_completed)().to_string()).0
                body=create_signal(t!(i18n, update_db_completed_val)().to_string()).0
            />
            <ErrorModal
                show_modal=show_error_modal
                set_show_modal=setShowErrorModal
                title=create_signal(t!(i18n, update_db_failed)().to_string()).0
                body=error_message
            />

            <div class="flex h-full justify-center p-6 text-center">
                <div class="w-full flex justify-center items-center h-full">
                    <div class="w-full">
                        <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                            {move || status.get()}
                        </h1>
                        <Show when=move || show_progress.get()>
                            <div class="flex justify-center">
                                <ProgressBar progress=progress/>
                            </div>
                        </Show>

                        <Show when=move || completed_state.get()>
                            <div class="pt-6">
                                <HomeButton />
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}
