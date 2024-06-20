use leptos::*;
use leptos_i18n::t;
use crate::components::home_button::HomeButton;
use crate::i18n::use_i18n;

// TODO: Styling

#[component]
pub fn Clean() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex items-center justify-center h-screen flex-col">
                <h1 class="text-center mb-10 pt-4 font-medium text-5xl text-maingreen">
                    {t!(i18n, clean_title)}
                </h1>
                <img
                    src="/images/success_image.png"
                    alt="Success"
                    className="max-w-[30%]"
                    width="500"
                    height="500"
                />
                <HomeButton />
            </div>
    }
}