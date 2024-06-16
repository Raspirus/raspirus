use std::path::PathBuf;
use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;
use leptos::logging::log;
use tauri_wasm::plugin::dialog::FileDialogBuilder;


#[component]
pub fn DirectoryPickerButton(
    scan_target: WriteSignal<String>,
    can_select_directories: ReadSignal<bool>
) -> impl IntoView {
    let handle_button_click = move || {
        spawn_local(async move {
            let path_buffer: PathBuf;
            if can_select_directories.get() {
                let folder = FileDialogBuilder::new().pick_folder().await;
                log!("Selected folder: {:?}", folder);
                path_buffer = folder.expect("Folder selection error")
                    .expect("Path conversion error");
            } else {
                let file = FileDialogBuilder::new().pick_file().await;
                log!("Selected file: {:?}", file);
                // It returns a FileResponse object, which contains the file path and the file name
                path_buffer = file.expect("File selection error")
                    .expect("Path conversion error").path;
            }
            let path_string = path_buffer.into_os_string().into_string().unwrap_or_default();
            scan_target.set(path_string);
        });
    };

    view! {
        <Button on_press=move |_| handle_button_click()
            class="ml-1 inline-block p-3 bg-orange-400 rounded shadow-md">
            <Icon icon=icondata::FaFolderSolid class="h-full w-4" />
        </Button>
    }
}
