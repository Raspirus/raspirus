use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;
use log::info;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;


/*
    const { open } = await import('@tauri-apps/plugin-dialog');
    // Set user selection restrictions
    const selected = await open({
      directory: scanDirectory,
      multiple: false,
      defaultPath: "/",
    })
    if (selected === null) {
      // No dir selected
    } else {
      onSelectDirectory(selected);
    }
  }

*/

#[component]
pub fn DirectoryPickerButton(
    scan_target: WriteSignal<String>,
    can_select_directories: ReadSignal<bool>
) -> impl IntoView {
    let handle_button_click = move || {
        info!("DirectoryPickerButton clicked");
    };

    view! {
        <Button on_press=move |_| handle_button_click()
            class="ml-1 inline-block p-3 bg-orange-400 rounded shadow-md">
            <Icon icon=icondata::FaFolderSolid class="h-full w-4" />
        </Button>
    }
}
