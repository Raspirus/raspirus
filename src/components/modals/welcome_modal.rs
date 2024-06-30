use leptos::*;
use leptonic::components::modal::{Modal, ModalBody, ModalFooter, ModalHeader, ModalTitle};
use leptonic::components::button::{Button, ButtonColor, ButtonWrapper};
use crate::i18n::use_i18n;
use leptos_i18n::t;


/// Modal that shows a welcome message to the user
/// This modal only shows up in the home page if the database is empty
/// It is meant to greet the user and remind them to update the database
#[component]
pub fn WelcomeModal(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <Modal show_when=show_modal>
            <ModalHeader>
                <ModalTitle>{t!(i18n, welcome_title)}</ModalTitle>
            </ModalHeader>
            <ModalBody>
                <p>{t!(i18n, welcome_text)}</p>
            </ModalBody>
            <ModalFooter>
                <p class="pr-2">{t!(i18n, welcome_footer)}</p>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Info>
                        "Ok"
                    </Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}