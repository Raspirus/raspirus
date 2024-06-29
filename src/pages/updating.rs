use futures_util::StreamExt;
use leptonic::components::prelude::ProgressBar;
use leptos::*;
use leptos::leptos_dom::log;
use tauri_wasm::api::core::invoke;
use tauri_wasm::api::event::listen;
use tauri_wasm::Error;
use crate::components::home_button::HomeButton;
use crate::i18n::{t, use_i18n};

#[component]
pub fn Updating() -> impl IntoView {
    let (error_state, setErrorState) = create_signal(false);
    let (completed_state, setCompletedState) = create_signal(false);
    let (progress, setProgress) = create_signal(Some(0.0));
    let (show_progress, setShowProgress) = create_signal(false);
    let (status, setStatus) = create_signal("".to_string());
    let i18n = use_i18n();

    // TODO: It seems like the progress listeners are working, but the modal is not updating
    // In fact it seems like its displayed completely empty

    // Progress listener for the Check state
    spawn_local(async move {
        let mut progress_event = listen::<String>("chck").await.expect("event listen error");
        while let Some(_) = progress_event.next().await {
            log!("Check event received");
            setShowProgress.set(false);
            setStatus.set(t!(i18n, db_update_stage_check)().to_string());
        }
    });

    // Progress listener for the Index state
    spawn_local(async move {
        let mut progress_event = listen::<String>("idx").await.expect("event listen error");
        while let Some(_) = progress_event.next().await {
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
        while let Some(_) = progress_event.next().await {
            log!("Error event received");
            setStatus.set("Error".to_string());
            setShowProgress.set(false);
            setErrorState.set(true);
        }
    });
/*
    spawn_local(async move {
        let return_value: Result<String, Error> = invoke("update_database", &String::new()).await;
        match return_value {
            Ok(_) => {
                log!("Database update successful");
                setCompletedState.set(true);
            }
            Err(e) => {
                log!("Database update failed: {:?}", e);
                setErrorState.set(true);
            }
        }
    });
*/

    view! {
        <div class="h-screen">
            <div class="flex h-full justify-center p-6 text-center">
                <div class="w-full flex justify-center items-center h-full">
                    <div class="w-full">
                        <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                            {move || status.get()}
                        </h1>
                        <div class="flex justify-center">
                            <ProgressBar progress=progress/>
                        </div>
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