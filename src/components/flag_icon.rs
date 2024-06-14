use leptos::*;

#[component]
pub fn FlagIcon(
    mut country_code: String,
) -> impl IntoView {
    if country_code == "en" {
        country_code = "gb".to_string();
    }

    view! {
        <span class=format!("fi fis fiCircle inline-block mr-2 flag-icon-{}", country_code) />
    }
}
