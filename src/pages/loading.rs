use leptos::*;
use leptonic::components::progress_bar::ProgressBar;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, callback: &JsValue) -> JsValue;
}

// TODO: The above function provided by Tauri won't work, as the original synatx is like this:
/*
const handleProgress = (event) => {
      //console.log("Progress: ", event.payload.message);
      setProgress(event.payload.message);
    };
    // Backend can also send error instead of the progress
    const handleProgressErr = (event) => {
      console.error(error);
      localStorage.setItem("errorOccurred", 'true');
      // Returns to the Home page with an error statements that will be displayed there
      router.push({
        pathname: '/',
        query: { scanner_error: event.payload.message }
      })
    }

    // Starts listening for incoming signals emited from the backend
    const startListening = async () => {
      await listen('progress', handleProgress);
      await listen('progerror', handleProgressErr);
    };

    startListening();

    // Clean up function to remove the event listener when the component unmounts
    return () => {
      removeEventListener('progress', handleProgress);
      removeEventListener('progerror', handleProgressErr);
    };
 */

#[component]
pub fn Loading() -> impl IntoView {
    let (progress, set_progress) = create_signal(Some(0.0));

    view! {
        <div>
            <h1>{"Loading Page"}</h1>
            <ProgressBar progress=progress/>
        </div>
    }
}