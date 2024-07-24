use leptonic::components::icon::Icon;
use leptonic::components::prelude::NumberInput;
use leptonic::prelude::icondata;
use leptos::*;

/// A simple toggle card for settings
/// It is the most common setting and allows user to set something to on or off
#[component]
pub fn SettingsNumInputCard(
    title: String,
    short_description: String,
    icon: icondata::Icon,
    min_input: (ReadSignal<f64>, WriteSignal<f64>),
    max_input: (ReadSignal<f64>, WriteSignal<f64>),
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
                        <p class="text-sm text-gray-600 leading-none mt-1">
                            "Setting the value to 0 ignores the limit"
                        </p>
                    </div>
                </div>
                <div>
                    <NumberInput class="ml-4 mb-4"
                        min=0.0 step=1.0
                        get=max_input.0
                        set=max_input.1
                    />
                    <NumberInput class="ml-4"
                        min=0.0 step=1.0
                        get=min_input.0
                        set=min_input.1
                    />
                </div>
            </div>
        </div>
    }
}
