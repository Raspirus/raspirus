use leptos::*;


/// A small helper component to display a flag icon based on the country code.
/// The country code is expected to be a two-letter ISO 3166-1 alpha-2 code.
/// If the country code is "en", it will be replaced with "gb" to display the UK flag.
#[component]
pub fn FlagIcon(
    country_code: String,
) -> impl IntoView {
    let mut flag_icon = country_code.clone();
    flag_icon = flag_icon.to_lowercase();
    if flag_icon == "en" {
        flag_icon = "gb".to_string();
    }

    view! {
        <span class=format!("fi fis fiCircle inline-block mr-2 fi-{}", flag_icon) />
    }
}
