use leptos::*;
use leptos::logging::log;
use leptonic::components::progress_bar::ProgressBar;
use leptos_router::{use_query_map, use_navigate};
use tauri_wasm::api::event::listen;
use tauri_wasm::api::core::invoke;
use tauri_wasm::Error;
use futures_util::StreamExt;
use crate::generic::ScannerArgs;

// TODO:
// - Styling
// - If possible, add a "STOP" button to stop the scanning process

#[component]
pub fn Loading() -> impl IntoView {
    let (progress, set_progress) = create_signal(Some(0.0));
    let target = use_query_map().get_untracked().get("target").cloned();
    let navigate = use_navigate();
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

    // We start the scanning process
    spawn_local(async move {
        log!("Starting scanner with target: {:?}", target);
        let result: Result<String, Error> = invoke("start_scanner", &ScannerArgs{path: target.unwrap()}).await;
        match &result {
            Ok(result) => {
                log!("Result: {}", result);
                let infected_files: Vec<String> = serde_json::from_str(&result).unwrap();
                let count = infected_files.len();
                log!("Infected files: {:?}", count);
                if count > 0 {
                    navigate(&format!("/infected?result={}", result), Default::default());
                } else {
                    navigate("/clean", Default::default());
                }
            }
            Err(e) => {
                log!("Error: {:?}", e);
                navigate(&format!("?error={}", e.to_string()), Default::default());
            }
        }
    });



    view! {
        <div>
            <h1>{"Loading Page"}</h1>
            <ProgressBar progress=progress/>
        </div>
    }
}