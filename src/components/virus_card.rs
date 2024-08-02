use crate::generic::{FileLookupArgs, RuleFeedback};
use leptonic::components::prelude::Icon;
use leptonic::{
    components::prelude::{
        Button, Chip, ChipColor, Collapsible, CollapsibleBody, CollapsibleHeader,
    },
    prelude::icondata::{self},
};
use leptos::logging::log;
use leptos::*;
use leptos_router::NavigateOptions;
use std::path::PathBuf;
use tauri_wasm::{api::core::invoke, Error};

#[component]
pub fn VirusCard(
    file_path: PathBuf,
    rules_count: usize,
    rule_matches: Vec<RuleFeedback>,
) -> impl IntoView {
    let chip_color = if rules_count > 5 {
        ChipColor::Danger
    } else {
        ChipColor::Warn
    };
    let string_lossy = file_path.to_string_lossy();
    let display_path = string_lossy.to_string();
    let navigate = leptos_router::use_navigate();

    // A function that gathers the file signature from the backend and then sends
    // it to VirusTotal for further analysis
    let analyze_file = move || {
        spawn_local(async move {
            let file_signature: Result<String, Error> = invoke(
                "lookup_file",
                &FileLookupArgs {
                    file: file_path.clone(),
                },
            )
            .await;
            match file_signature {
                Ok(file_signature) => {
                    log!("File signature: {}", file_signature);
                    navigate(
                        &format!("https://www.virustotal.com/gui/search/{}", file_signature),
                        NavigateOptions {
                            resolve: false,
                            replace: false,
                            scroll: true,
                            state: Default::default(),
                        },
                    );
                }
                Err(e) => {
                    log!("Error: {:?}", e);
                }
            }
        });
    };

    view! {
        <Collapsible>
            <CollapsibleHeader slot class="w-full">
            <div class="flex w-full">
                <Chip color={chip_color} class="mr-2">{rules_count}</Chip>
                <div>{display_path}</div>
                <Button on_press=move |_| { /* TODO: Add function call */ } class="ml-auto mr-2">
                    <Icon icon=icondata::SiVirustotal />
                </Button>
            </div>
            </CollapsibleHeader>
            <CollapsibleBody slot>
            <div class="flex flex-col w-full">
            {
                rule_matches.iter().enumerate().map(|(index, rule_match)| {
                    view! {
                        <>
                            <div class="flex flex-col my-2">
                                <p class="font-medium">{rule_match.rule_name.clone()}</p>
                                <p class="text-sm text-gray-600 leading-none mt-1">{rule_match.rule_description.clone()}</p>
                            </div>
                            <Show when=move || index < rules_count - 1 >
                                <hr class="my-2 w-full"/>
                            </Show>
                        </>
                    }
                }).collect::<Vec<_>>()
            }
            </div>
        </CollapsibleBody>
        </Collapsible>
    }
}
