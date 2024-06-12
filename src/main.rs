use leptos::*;
use leptos_router::*;

mod pages;
use pages::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}