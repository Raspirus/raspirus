use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::components::prelude::ButtonColor;
use leptonic::prelude::icondata;
use leptos::logging::log;
use leptos::*;
use std::path::PathBuf;
use tauri_wasm::plugin::dialog::FileDialogBuilder;

/// A button that opens a file dialog to select a directory or file
/// and sets the selected path as the scan target. We open the dialog through the WebAssembly API
#[component]
pub fn DirectoryPickerButton(
    scan_target: WriteSignal<Option<String>>,
    can_select_directories: ReadSignal<bool>,
) -> impl IntoView {
    let handle_button_click = move || {
        spawn_local(async move {
            let path_buffer: PathBuf;
            if can_select_directories.get() {
                let folder = FileDialogBuilder::new().pick_folder().await;
                log!("Selected folder: {:?}", folder);
                // If the folder is ok, we parse the path, else we just don't do anything
                match folder {
                    Ok(Some(path)) => {
                        path_buffer = path;
                    }
                    _ => return,
                }
            } else {
                let file = FileDialogBuilder::new().pick_file().await;
                log!("Selected file: {:?}", file);
                // Same as with the folder, if the file is ok, we parse the path, else we don't do anything
                match file {
                    Ok(Some(path)) => {
                        path_buffer = path.path;
                    }
                    _ => return,
                }
            }
            let path_string = path_buffer
                .into_os_string()
                .into_string()
                .unwrap_or_default()
                .replace("\"", "") // Remove double quotes
                .replace("'", ""); // Remove single quotes
            log!("Selected formatted target: {}", path_string.clone());
            scan_target.set(Option::from(path_string));
        });
    };

    view! {
        <Button on_press=move |_| handle_button_click()
            color=ButtonColor::Warn
            class="ml-1 inline-block p-3 bg-orange-400 rounded shadow-md">
            <Icon icon=icondata::FaFolderSolid class="h-full w-4" />
        </Button>
    }
}
