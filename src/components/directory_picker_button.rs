use leptonic::components::button::Button;
use leptonic::components::icon::Icon;
use leptonic::prelude::icondata;
use leptos::*;

#[component]
pub fn DirectoryPickerButton(
    scan_target: WriteSignal<String>,
    can_select_directories: ReadSignal<bool>
) -> impl IntoView {
    let handle_button_click = move || {
        // Do nothing for now
        // TODO: Implement the file picker
    };

    view! {
        <Button on_press=move |_| handle_button_click() class="ml-1 inline-block p-3 \
                bg-orange-400 rounded shadow-md">
            <Icon icon=icondata::FaFolderSolid class="h-full w-4" />
        </Button>
    }
}
