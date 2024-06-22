use leptonic::components::select::Select;
use leptos::*;
use crate::components::flag_icon::FlagIcon;
use crate::i18n::{Locale, use_i18n};


#[component]
pub fn LanguageSwitch() -> impl IntoView {
    let i18n = use_i18n();
    // We need to get the current locale from the i18n context as a string
    let current_locale = i18n.get_locale();
    let current_locale_string = match {
        current_locale
    } {
        Locale::en => "EN".to_string(),
        Locale::de => "DE".to_string(),
        Locale::it => "IT".to_string(),
    };
    let (current_locale_string, _) = create_signal(current_locale_string);
    // We want to create a vector of locales that are available in the i18n context
    // but the i18n context does not provide a method to get all available locales
    // it only provides an enum that represents the available locales,
    // so we have to create a vector of locales manually
    let available_locales = vec!["EN".to_string(), "DE".to_string(), "IT".to_string()];

    // Now we need a function that converts the string to a Locale enum and sets the locale
    // in the i18n context
    let set_locale = move |v: String| {
        let locale: Locale = match {
            v.to_uppercase().as_str()
        } {
            "EN" => Locale::en,
            "DE" => Locale::de,
            "IT" => Locale::it,
            _ => Locale::en,
        };
        i18n.set_locale(locale);
    };

    // TODO: When changing the locale, the page should be reloaded because the translations
    // icons are not updated

    view! {
        <div class="absolute top-0 left-0 m-2">
            <Select
                class="uppercase bg-white w-fit"
                options=available_locales
                search_text_provider=move |o| format!("{o}")
                render_option=move |o| view! {
                    <FlagIcon country_code={o} />
                }
                selected=current_locale_string
                set_selected=move |v| set_locale(v)
            />
        </div>
}

}
