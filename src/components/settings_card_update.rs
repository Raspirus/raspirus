use leptonic::components::button::LinkButton;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;

#[component]
pub fn SettingsUpdateCard(
    title: String,
    short_description: String,
    short_description_2: String,
    icon: icondata::Icon,
) -> impl IntoView {

    view! {
        <div class="flex flex-col m-6 p-2 bg-white rounded-2xl shadow-md">
            <div class="flex items-center justify-between mx-4">
                <div class="flex items-center">
                    <Icon icon=icon
                        class="w-16 h-16 rounded-2xl p-3 border border-maingreen-light text-maingreen-light bg-green-50"
                    />
                    <div class="flex flex-col ml-3">
                        <div class="font-medium">{title}</div>
                        <p class="text-sm text-gray-600 leading-none mt-1">{short_description}</p>
                        <p class="text-sm text-gray-600 leading-none mt-1">{short_description_2}</p>
                    </div>
                </div>
                <LinkButton href="/update">"Update"</LinkButton>
            </div>
        </div>
    }
}