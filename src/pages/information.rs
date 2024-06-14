use leptonic::components::button::LinkButton;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;
use crate::i18n::use_i18n;
use leptos_i18n::t;
use crate::components::info_card::InfoComp;


#[component]
pub fn Information() -> impl IntoView {
    let i18n = use_i18n();
    let appVersion = env!("CARGO_PKG_VERSION");
    view! {
        <div>
            <div class="align-middle">
                <LinkButton href="/" class="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md">
                    <Icon
                        icon=icondata::AiHomeFilled
                        class="pr-1"
                    />
                    {t!(i18n, back_btn)}
                </LinkButton>
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

            <InfoComp
                title=t!(i18n, app_name)().to_string()
                value=t!(i18n, title)().to_string()
                icon=icondata::TbHexagonLetterR
            />
        <InfoComp
                title=t!(i18n, description)().to_string()
                value=t!(i18n, description_val)().to_string()
                icon=icondata::TbFileDescription
            />
            <InfoComp
                title=t!(i18n, maintainers)().to_string()
                value=t!(i18n, maintainers_val)().to_string()
                icon=icondata::AiUserOutlined
            />
            <InfoComp
                title=t!(i18n, version)().to_string()
                value=appVersion.to_string()
                icon=icondata::OcGitCommitSm
            />
            <InfoComp
                title=t!(i18n, license)().to_string()
                value=t!(i18n, license_val)().to_string()
                icon=icondata::TbLicense
            />

        </div>
    }
}