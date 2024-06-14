use leptos::*;

mod pages;
mod components;

use pages::app::App;

leptos_i18n::load_locales!();

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}