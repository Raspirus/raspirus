use leptonic::prelude::icondata;
use leptos::*;
use crate::i18n::use_i18n;
use leptos_i18n::t;
use crate::components::home_button::HomeButton;
use crate::components::info_card::InfoCard;

// TODO:
// - Styling
// - Add status of the mirror website (Online/Offline)
// - Add Agreement and Privacy Policy Links (?)

#[component]
pub fn Information() -> impl IntoView {
    let i18n = use_i18n();
    let appVersion = env!("CARGO_PKG_VERSION");
    view! {
        <div>
            <div class="align-middle">
                <HomeButton />
                <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                    {t!(i18n, info_title)}
                </h1>
            </div>

            <img
                src="./images/banner.png"
                alt="Banner image"
                class="max-w-[90%] mx-auto rounded-xl shadow-md"
                width="1856"
                height="1024"
                />

            <InfoCard
                title=t!(i18n, app_name)().to_string()
                value=t!(i18n, title)().to_string()
                icon=icondata::TbHexagonLetterR
            />
        <InfoCard
                title=t!(i18n, description)().to_string()
                value=t!(i18n, description_val)().to_string()
                icon=icondata::TbFileDescription
            />
            <InfoCard
                title=t!(i18n, maintainers)().to_string()
                value=t!(i18n, maintainers_val)().to_string()
                icon=icondata::AiUserOutlined
            />
            <InfoCard
                title=t!(i18n, version)().to_string()
                value=appVersion.to_string()
                icon=icondata::OcGitCommitSm
            />
            <InfoCard
                title=t!(i18n, license)().to_string()
                value=t!(i18n, license_val)().to_string()
                icon=icondata::TbLicense
            />

        </div>
    }
}