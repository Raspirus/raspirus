use leptonic::components::alert::{Alert, AlertContent, AlertTitle, AlertVariant};
use leptos::*;
use leptonic::components::modal::Modal;
use leptonic::components::button::{Button, ButtonColor, ButtonWrapper};


/// A modal that shows a success message
/// Similar to the error modal, but with a different color
#[component]
pub fn SuccessModal (
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    title: ReadSignal<String>,
    body: ReadSignal<String>
) -> impl IntoView {

    view! {
        <Modal show_when=show_modal>
            <Alert variant=AlertVariant::Success>
                    <AlertTitle slot>{move || title.get()}</AlertTitle>
                    <AlertContent slot>{move || body.get()}</AlertContent>
                </Alert>
            <ButtonWrapper>
                <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Danger>"Ok"</Button>
            </ButtonWrapper>
        </Modal>
    }

}