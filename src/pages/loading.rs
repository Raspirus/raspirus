use leptos::*;
use leptos::logging::log;
use leptonic::components::progress_bar::ProgressBar;
use leptos_router::use_query_map;
use tauri_wasm::api::event::listen;
use tauri_wasm::api::core::invoke;
use futures_util::StreamExt;

#[component]
pub fn Loading() -> impl IntoView {
    let (progress, set_progress) = create_signal(Some(0.0));
    let query = use_query_map();
    let target_map = query.get_untracked();
    let target= target_map.get("target");
    log!("Target: {:?}", target);

    spawn_local(async move {
        let mut progress_event = listen::<String>("progress").await.expect("event listen error");
        while let Some(event) = progress_event.next().await {
            let payload : String = event.payload;
            let message = format!("payload: {}", payload);
            log!("Progress: {}", message);
            // Try to convert the payload to a float, if it fails, set the progress to 0
            let progress = payload.parse::<f64>().unwrap_or(0.0);
            set_progress.set(Some(progress));
        }
    });

    spawn_local(async move {
        let mut error_event = listen::<String>("progerror").await.expect("event listen error");
        while let Some(event) = error_event.next().await {
            let payload : String = event.payload;
            let message = format!("payload: {}", payload);
            log!("Error: {}", message);
        }
    });



    view! {
        <div>
            <h1>{"Loading Page"}</h1>
            <ProgressBar progress=progress/>
        </div>
    }
}