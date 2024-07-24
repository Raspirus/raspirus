use leptonic::components::button::{Button, ButtonColor, ButtonWrapper};
use leptonic::components::modal::{Modal, ModalBody, ModalFooter, ModalHeader, ModalTitle};
use leptos::*;
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use leptos::leptos_dom::{log, error};

/// UpdateModal component
/// A modal that starts an update of the yara rules database when opened and self-closes when
/// the update is finished. There is no interaction with this modal
#[component]
pub fn UpdateModal (
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>
) -> impl IntoView {
    let (is_completed, setIsCompleted) = create_signal(false);
    let (error_message, setErrorMessage) = create_signal(String::new());

    spawn_local(async move {
        log!("UPDATE STARTED");
        let return_value: Result<(), Error> = invoke("update", &String::new()).await;
        match return_value {
            Ok(_) => {
                log!("Database update successful");
                setIsCompleted.set(true);
            }
            Err(e) => {
                error!("Database update failed: {:?}", e);
                setErrorMessage.set(e.to_string());
                setIsCompleted.set(true);
            }
        }
    });


    view! {
        <Modal show_when=show_modal>
        <ModalHeader><ModalTitle>{"Updater"}</ModalTitle></ModalHeader>
        <ModalBody>{
            if error_message.get().is_empty() { 
            "Updating, please wait...".to_string() } 
            else {error_message.get()} }
        </ModalBody>
        <Show when=move || { is_completed.get() }>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Secondary>"Done"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Show>
    </Modal>
    }
}
