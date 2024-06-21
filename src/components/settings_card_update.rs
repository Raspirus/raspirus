use futures_util::StreamExt;
use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;
use leptos::logging::log;
use tauri_wasm::api::event::listen;
use crate::i18n::use_i18n;
use leptos_i18n::t;
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use crate::components::modals::error_modal::ErrorModal;
use crate::components::modals::success_modal::SuccessModal;
use crate::components::modals::update_modal::UpdateModal;

#[component]
pub fn SettingsUpdateCard(
    title: String,
    short_description: String,
    short_description_2: String,
    icon: icondata::Icon,
) -> impl IntoView {
    let (show_modal, setShowModal) = create_signal(false);
    let (show_success_modal, setShowSuccessModal) = create_signal(false);
    let (error_state, setErrorState) = create_signal(false);
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
        setShowModal.set(true);

        spawn_local(async move {
            let return_value: Result<String, Error> = invoke("update_database", &String::new()).await;
            match return_value {
                Ok(_) => {
                    log!("Database update successful");
                    setShowModal.set(false);
                    setShowSuccessModal.set(true);
                }
                Err(e) => {
                    log!("Database update failed: {:?}", e);
                    setShowModal.set(false);
                    setErrorState.set(true);
                }
            }
        })

    };

    view! {
        <div class="flex flex-col m-6 p-2 bg-white rounded-2xl shadow-md">
          <ErrorModal
            show_modal=error_state
            set_show_modal=setErrorState
            title=create_signal("Error".to_string()).0
            body=create_signal("An error occurred while updating the database".to_string()).0
            />
          <SuccessModal
            show_modal=show_success_modal
            set_show_modal=setShowSuccessModal
            title=create_signal("Success".to_string()).0
            body=create_signal("The database was successfully updated".to_string()).0
            />
            <UpdateModal
                show_modal=show_modal
                set_show_modal=setShowModal
                title=status
                progress=progress
                show_progress=show_progress
                is_error_state=error_state
            />
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
                <Button on_press=move |_| handle_button_click()>"Update"</Button>
            </div>
        </div>
    }
}