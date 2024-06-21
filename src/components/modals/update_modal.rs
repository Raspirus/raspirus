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
    is_error_state: ReadSignal<bool>,
) -> impl IntoView {

    view! {
    <Modal
        show_when=show_modal
        on_escape=move || ()
        on_backdrop_interaction=move || ()
        >
        <ModalHeader><ModalTitle>{title.get()}</ModalTitle></ModalHeader>
        <ModalBody>{if show_progress.get() {format!("{}%", progress.get().to_string())} else {"".to_string()}}</ModalBody>
        <ModalFooter>
        <Show when=move || is_error_state.get()>
            <ButtonWrapper>
                <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Secondary>"Return"</Button>
            </ButtonWrapper>
        </Show>
        </ModalFooter>
    </Modal>
    }
}