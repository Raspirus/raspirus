use leptos::*;
use leptos_i18n::t;
use crate::components::home_button::HomeButton;
use crate::i18n::use_i18n;

// TODO: Styling

#[component]
pub fn Clean() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div>
            <div class="align-middle">
                <HomeButton />
                <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                    {t!(i18n, info_title)}
                </h1>
            </div>
                <img
                    src="/images/success_image.png"
                    alt="Success"
                    className="max-w-[30%]"
                    width="500"
                    height="500"
                />
            </div>
    }
}