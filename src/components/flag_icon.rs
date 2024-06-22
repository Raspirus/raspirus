use leptos::*;

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
