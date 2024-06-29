use leptonic::components::alert::{Alert, AlertContent, AlertTitle, AlertVariant};
use leptos::*;
use leptonic::components::modal::Modal;
use leptonic::components::button::{Button, ButtonColor, ButtonWrapper};

#[component]
pub fn SuccessModal (
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    title: String,
    body: String
) -> impl IntoView {

    view! {
        <Modal show_when=show_modal>
            <Alert variant=AlertVariant::Success>
                    <AlertTitle slot>|| move title</AlertTitle>
                    <AlertContent slot>|| move body</AlertContent>
                </Alert>
            <ButtonWrapper>
                <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Secondary>"Ok"</Button>
            </ButtonWrapper>
        </Modal>
    }

}