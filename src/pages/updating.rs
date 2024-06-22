use futures_util::StreamExt;
use leptonic::components::skeleton::Skeleton;
use leptonic::components::stack::Stack;
use leptonic::Size;
use leptos::*;
use leptos::leptos_dom::log;
use tauri_wasm::api::core::invoke;
use tauri_wasm::api::event::listen;
use tauri_wasm::Error;
use crate::i18n::{t, use_i18n};

#[component]
pub fn Updating() -> impl IntoView {
    let (error_state, setErrorState) = create_signal(false);
    let (completed_state, setCompletedState) = create_signal(false);
    let (progress, setProgress) = create_signal(0.0);
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
            setProgress.set(event.payload.parse::<f64>().unwrap_or(0.0));
        }
    });

    // Progress listener for the Install state
    spawn_local(async move {
        let mut progress_event = listen::<String>("ins").await.expect("event listen error");
        while let Some(event) = progress_event.next().await {
            log!("Install event received with payload: {}", event.payload);
            setStatus.set(t!(i18n, db_update_stage_install)().to_string());
            setShowProgress.set(true);
            setProgress.set(event.payload.parse::<f64>().unwrap_or(0.0));
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

    let handle_button_click = move || {
        log!("Button clicked!");

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
        })

    };


    view! {
        <Stack spacing=Size::Em(0.6)>
            <Skeleton animated=false>"Item 1"</Skeleton>
            <Skeleton animated=false>"Item 2"</Skeleton>
            <Skeleton animated=false>"Item 3"</Skeleton>
        </Stack>
    }
}