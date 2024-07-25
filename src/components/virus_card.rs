use std::path::PathBuf;

use crate::generic::RuleFeedback;
use leptonic::{components::prelude::{
    Button, Chip, ChipColor, Collapsible, CollapsibleBody, CollapsibleHeader
}, prelude::icondata::{self}};
use leptos::*;
use leptonic::components::prelude::Icon;

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

    view! {
        <Collapsible>
            <CollapsibleHeader slot class="w-full">
            <div class="flex w-full">
                <Chip color={chip_color} class="mr-2">{rules_count}</Chip>
                <div>{display_path}</div>
                <Button class="ml-auto mr-2" on_press=move |_| {}>
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
