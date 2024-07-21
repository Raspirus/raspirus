use crate::components::flag_icon::FlagIcon;
use crate::i18n::{use_i18n, Locale};
use leptonic::components::select::Select;
use leptos::*;

/// This component is a language switcher that allows the user to change the language of the app.
/// It uses the i18n context to set the locale of the app. The language switcher is a select
/// component that contains the available locales in the i18n context. We have to create a vector
/// of locales manually because the i18n context does not provide a method to get all available
/// locales. The language switcher also displays a flag icon for each locale.
#[component]
pub fn LanguageSwitch() -> impl IntoView {
    let i18n = use_i18n();
    let (current_selected_locale, setCurrentSelectedLocale) = create_signal(String::new());
    // We need to get the current locale from the i18n context as a string
    let current_locale = i18n.get_locale();
    let current_locale_string = match current_locale {
        Locale::en => "EN".to_string(),
        Locale::de => "DE".to_string(),
        Locale::it => "IT".to_string(),
    };
    setCurrentSelectedLocale.set(current_locale_string);
    // We want to create a vector of locales that are available in the i18n context
    // but the i18n context does not provide a method to get all available locales
    // it only provides an enum that represents the available locales,
    // so we have to create a vector of locales manually
    let available_locales = vec!["EN".to_string(), "DE".to_string(), "IT".to_string()];

    // Now we need a function that converts the string to a Locale enum and sets the locale
    // in the i18n context
    let set_locale = move |v: String| {
        let locale: Locale = match v.to_uppercase().as_str() {
            "EN" => Locale::en,
            "DE" => Locale::de,
            "IT" => Locale::it,
            _ => Locale::en,
        };
        i18n.set_locale(locale);
        setCurrentSelectedLocale.set(v);
    };

    view! {
            <div class="absolute top-0 left-0 m-2">
                <Select
                    class="uppercase bg-white w-fit"
                    options=available_locales
                    search_text_provider=move |o: String| o
                    render_option=move |o: String| view! {
                        <div class="flex">
                            <FlagIcon country_code={o.clone()} />
                            <p class="upper">{o}</p>
                        </div>
                    }
                    selected=current_selected_locale
                    set_selected=set_locale
                />
            </div>
    }
}
