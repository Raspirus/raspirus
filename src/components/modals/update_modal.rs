use leptonic::components::button::{Button, ButtonColor, ButtonWrapper};
use leptonic::components::modal::{Modal, ModalBody, ModalFooter, ModalHeader, ModalTitle};
use leptos::*;

#[component]
pub fn UpdateModal(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    title: ReadSignal<String>,
    progress: ReadSignal<f64>,
    show_progress: ReadSignal<bool>,
) -> impl IntoView {

    view! {
    <Modal show_when=show_modal>
        <ModalHeader><ModalTitle>{title.get()}</ModalTitle></ModalHeader>
        <ModalBody>{format!("{}%", if show_progress.get() {progress.get().to_string()} else {"".to_string()})}</ModalBody>
        <ModalFooter>
            <ButtonWrapper>
        // TODO: Only show button when an error state occurs
        // Add constraints to lock the user in the pop-up, not allowing him to leave until completed or error
        // Add a "STOP" button to stop the process
                <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
            </ButtonWrapper>
        </ModalFooter>
    </Modal>
    }
}