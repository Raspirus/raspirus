use leptos::*;
use crate::components::home_button::HomeButton;
use crate::i18n::use_i18n;
use leptos_i18n::t;

/// The agreement page.
/// It simply displays a text with the permissions required by the app and
/// a button to go back to the home page.
#[component]
pub fn Agreement() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div>
            <div class="align-middle">
                <HomeButton />
                <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                    {t!(i18n, permissions_title)}
                </h1>
            </div>
                <p class="text-justify text-lg px-16 pb-4">
                    {t!(i18n, permissions_text)}
                </p>
        </div>
    }
}