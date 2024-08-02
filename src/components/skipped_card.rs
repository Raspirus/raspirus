use leptonic::components::prelude::{
    Chip, ChipColor, Collapsible, CollapsibleBody, CollapsibleHeader,
};
use leptos::*;
use std::path::PathBuf;

#[component]
pub fn SkippedCard(file_path: PathBuf, skip_reason: String) -> impl IntoView {
    let string_lossy = file_path.to_string_lossy();
    let display_path = string_lossy.to_string();

    view! {
        <Collapsible>
            <CollapsibleHeader slot class="w-full">
            <div class="flex w-full">
                <Chip color=ChipColor::Warn class="mr-2">"SKIP"</Chip>
                <div>{display_path}</div>
            </div>
            </CollapsibleHeader>
            <CollapsibleBody slot>
                <div class="flex flex-col w-full">
                    <div class="flex flex-col my-2">
                        <p class="font-medium">"Skipped"</p>
                        <p class="text-sm text-gray-600 leading-none mt-1">{skip_reason}</p>
                    </div>
                </div>
        </CollapsibleBody>
        </Collapsible>
    }
}
