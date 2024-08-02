use leptonic::components::button::{Button, ButtonColor, ButtonWrapper};
use leptonic::components::modal::{Modal, ModalBody, ModalFooter, ModalHeader, ModalTitle};
use leptos::leptos_dom::{error, log};
use leptos::*;
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;

/// UpdateModal component
/// A modal that starts an update of the yara rules database when opened and self-closes when
/// the update is finished. There is no interaction with this modal
#[component]
pub fn UpdateModal(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
) -> impl IntoView {
    let (is_completed, setIsCompleted) = create_signal(false);
    let (message, setMessage) = create_signal(String::new());

    watch(
        move || show_modal.get(),
        move |_, _, _| {
            spawn_local(async move {
                log!("UPDATE STARTED");
                let return_value: Result<(), Error> = invoke("update", &String::new()).await;
                match return_value {
                    Ok(_) => {
                        log!("Database update successful");
                        setMessage.set("Update completed".to_string());
                        setIsCompleted.set(true);
                    }
                    Err(e) => {
                        error!("Database update failed: {:?}", e);
                        setMessage.set(e.to_string());
                        setIsCompleted.set(true);
                    }
                }
            })
        },
        false,
    );

    view! {
        <Modal show_when=show_modal>
        <ModalHeader><ModalTitle>{"Updater"}</ModalTitle></ModalHeader>
        <ModalBody>{move ||
            if message.get().is_empty() {
            "Updating, please wait...".to_string() }
            else {message.get()} }
        </ModalBody>
        <Show when=move || { is_completed.get() }>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Info>"Done"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Show>
    </Modal>
    }
}
