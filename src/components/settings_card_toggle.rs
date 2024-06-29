use leptonic::components::icon::Icon;
use leptonic::components::toggle::Toggle;
use leptonic::prelude::icondata;
use leptos::*;

// format!("flex-no-shrink px-5 ml-4 py-2 text-sm shadow-sm font-medium border-2 text-white rounded-full {}", button_class)


#[component]
pub fn SettingsToggleCard(
    title: String,
    short_description: String,
    short_description_2: Option<String>,
    icon: icondata::Icon,
    is_on: ReadSignal<bool>,
    toggle_function: WriteSignal<bool>,
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
                        {match short_description_2 {
                            Some(short_description_2) => {
                                view! {
                                    <p class="text-sm text-gray-600 leading-none mt-1">{short_description_2}</p>
                                }
                            }
                            None => {
                                view! {<p></p>}
                            }
                        }}
                    </div>
                </div>
                <Toggle class="flex-no-shrink ml-4 py-2"
                    state=is_on set_state=toggle_function />
            </div>
        </div>
    }
}
