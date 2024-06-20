use leptos::*;
use leptos_i18n::t;
use crate::components::home_button::HomeButton;
use crate::i18n::use_i18n;

// TODO:
// - Styling
// - Add component to visualize each malware in the infected list
// - Catch the infected list from the query and create a list of malware components
// - Do some basic error handling with Alerts, not pop-ups
//      - Then the user can redirect manually to the home page

#[component]
pub fn Infected() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex items-center justify-center h-screen flex-col">
                <h1 class="text-center mb-10 pt-4 font-medium text-5xl text-maingreen">
                    {t!(i18n, infected_title)}
                </h1>
                <img
                    src="/images/failure_image.png"
                    alt="Failure"
                    className="max-w-[30%]"
                    width="500"
                    height="500"
                />
                <HomeButton />
            </div>
    }
}