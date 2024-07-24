use crate::generic::ScannerArgs;
use crate::i18n::use_i18n;
use futures_util::StreamExt;
use leptonic::components::progress_bar::ProgressBar;
use leptos::logging::log;
use leptos::*;
use leptos_i18n::t;
use leptos_router::{use_navigate, use_query_map};
use tauri_wasm::api::core::invoke;
use tauri_wasm::api::event::listen;
use tauri_wasm::Error;

/// The Scanning page is responsible for starting the scanning process and displaying the progress
/// of the scan. It listens for progress events and updates the progress bar accordingly.
/// If the scan is successful and infected files are found, it navigates to the infected page.
/// If the scan is successful and no infected files are found, it navigates to the clean page.
/// If the scan fails, it navigates back to the home page with an error message.
#[component]
pub fn Scanning() -> impl IntoView {
    let (progress, set_progress) = create_signal(Some(0.0));
    let target = use_query_map().get_untracked().get("target").cloned();
    let navigate = use_navigate();
    let i18n = use_i18n();
    log!("Target: {:?}", target);

    spawn_local(async move {
        let mut progress_event = listen::<String>("progress")
            .await
            .expect("event listen error");
        while let Some(event) = progress_event.next().await {
            let payload: String = event.payload;
            let message = format!("payload: {}", payload);
            log!("Progress: {}", message);
            // Try to convert the payload to a float, if it fails, set the progress to 0
            let progress = payload.parse::<f64>().unwrap_or(0.0);
            set_progress.set(Some(progress));
        }
    });

    spawn_local(async move {
        let mut error_event = listen::<String>("progerror")
            .await
            .expect("event listen error");
        while let Some(event) = error_event.next().await {
            let payload: String = event.payload;
            let message = format!("payload: {}", payload);
            log!("Error: {}", message);
        }
    });

    // We start the scanning process
    spawn_local(async move {
        log!("Starting scanner with target: {:?}", target.clone().unwrap());
        let result: Result<String, Error> = invoke(
            "start_scanner",
            &ScannerArgs {
                path: target.unwrap(),
            },
        )
        .await;
        match &result {
            Ok(result) => {
                log!("Result: {}", result);
                let infected_files: Vec<String> = serde_json::from_str(result).unwrap();
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
                navigate(&format!("?error={:?}", e), Default::default());
            }
        }
    });

    view! {
        <div class="h-screen">
            <div class="flex h-full justify-center p-6 text-center">
                <div class="w-full flex justify-center items-center h-full">
                    <div class="w-full">
                        <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                            {t!(i18n, loading_text)}
                        </h1>
                        <div class="flex justify-center">
                            <ProgressBar progress=progress/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
