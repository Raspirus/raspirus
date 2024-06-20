use leptos::*;
use leptonic::components::modal::{Modal, ModalBody, ModalFooter, ModalHeader, ModalTitle};
use leptonic::components::button::{Button, ButtonColor, ButtonWrapper};

#[component]
pub fn SuccessModal(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    title: String,
    body: String
) -> impl IntoView {

    view! {
        <Modal show_when=show_modal>
            <ModalHeader>
                <ModalTitle>
                    "Hello"
                </ModalTitle>
            </ModalHeader>
            <ModalBody>
                "This ia a simple modal."
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }

}