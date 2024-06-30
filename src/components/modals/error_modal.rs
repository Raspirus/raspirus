use leptonic::components::alert::{Alert, AlertContent, AlertTitle, AlertVariant};
use leptos::*;
use leptonic::components::modal::Modal;
use leptonic::components::button::{Button, ButtonColor, ButtonWrapper};

#[component]
pub fn ErrorModal (
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    title: ReadSignal<String>,
    body: ReadSignal<String>
) -> impl IntoView {

    view! {
        <Modal show_when=show_modal>
            <Alert variant=AlertVariant::Danger>
                    <AlertTitle slot>{title}</AlertTitle>
                    <AlertContent slot>{body}</AlertContent>
                </Alert>
            <ButtonWrapper>
                <Button on_press=move |_| set_show_modal.set(false) color=ButtonColor::Warn>"Ok"</Button>
            </ButtonWrapper>
        </Modal>
    }

}