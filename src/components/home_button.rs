use leptonic::components::button::LinkButton;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;
use leptos_i18n::t;
use crate::i18n::use_i18n;


/// Home button component
/// Is a button that redirects to the home page of the website. Used for convenience.
#[component]
pub fn HomeButton() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <LinkButton href="/" class="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md">
            <Icon icon=icondata::AiHomeFilled class="pr-1" />
            {t!(i18n, back_btn)}
        </LinkButton>
    }
}