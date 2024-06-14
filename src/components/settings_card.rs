use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;
use leptos_i18n::t;
use crate::i18n::use_i18n;

#[component]
pub fn SettingsCard(
    title: String,
    short_description: String,
    short_description_2: Option<String>,
    icon: icondata::Icon,
    is_on: Option<ReadSignal<bool>>,
    toggle_function: Option<WriteSignal<bool>>,
    action_function: Option<fn()>,
    action_value: Option<String>
) -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col m-6 p-2 bg-white rounded-2xl shadow-md">
            <div class="flex items-center justify-between mx-4">
                <div class="flex items-center">
                    <Icon icon={icon}
                        class="w-16 h-16 rounded-2xl p-3 border border-maingreen-light text-maingreen-light bg-green-50"
                    />
                    <div class="flex flex-col ml-3">
                        <div class="font-medium">{title}</div>
                        <p class="text-sm text-gray-600 leading-none mt-1">{short_description}</p>
                        {move || if let Some(short_description_2) = short_description_2 {
                                view! {
                                    <p class="text-sm text-gray-600 leading-none mt-1">{short_description_2}</p>
                                }
                            } else {
                                view! {<p />}
                            }
                        }
                    </div>
                </div>
                {move || if let Some(is_on) = is_on {
                        view! {
                            <Button on_press={move || toggle_function.set(!is_on) }
                                class=format!("flex-no-shrink px-5 ml-4 py-2 text-sm shadow-sm font-medium border-2 text-white rounded-full {}",  if is_on.get() { "bg-green-500 border-green-500" } else { "bg-red-500 border-red-500" }) >
                                {if is_on.get() { t!(i18n, settings_on) } else { t!(i18n, settings_off) }}
                            </Button>
                        }
                    } else if let Some(action_function) = action_function {
                        view! {
                            <button onClick={action_function}
                                className="flex-no-shrink px-5 ml-4 py-2 text-sm shadow-sm font-medium border-2 text-white rounded-full bg-blue-500 border-blue-500">
                                {action_value.unwrap_or("Action")}
                            </button>
                        }
                    } else {
                        view! {}
                    }
                }
            </div>
        </div>
    }
}