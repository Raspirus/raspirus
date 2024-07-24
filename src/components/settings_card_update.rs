use crate::components::modals::update_modal::UpdateModal;
use crate::i18n::use_i18n;
use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::components::prelude::ButtonColor;
use leptonic::prelude::icondata;
use leptos::*;
use leptos_i18n::t;

/// A card that displays the current version of the database and allows the user to update it.
/// The update does not happen in this component, but is triggered by the user clicking the button.
/// The update happens on a separate page where the user is redirected to.
#[component]
pub fn SettingsUpdateCard(
    title: String,
    short_description: String,
    short_description_2: ReadSignal<String>,
    icon: icondata::Icon,
) -> impl IntoView {
    let i18n = use_i18n();
    let (show_updater, setShowUpdater) = create_signal(false);

    view! {
        <UpdateModal
        show_modal=show_updater
        set_show_modal=setShowUpdater
    />

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
                        <span>{t!(i18n, update_db_1)().to_string()}</span>": "
                        <span>{move || short_description_2.get()}</span>
                        </p>
                    </div>
                </div>
                <Button on_press=move |_| {setShowUpdater.set(true)} color=ButtonColor::Info>{t!(i18n, update_db_action)}</Button>
            </div>
        </div>
    }
}
