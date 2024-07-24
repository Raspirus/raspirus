use crate::generic::RuleFeedback;
use leptonic::components::prelude::{
    Chip, ChipColor, Collapsible, CollapsibleBody, CollapsibleHeader,
};
use leptos::*;

#[component]
pub fn VirusCard(
    file_path: String,
    rules_count: usize,
    rule_matches: Vec<RuleFeedback>,
) -> impl IntoView {
    let chip_color = if rules_count > 5 {
        ChipColor::Danger
    } else {
        ChipColor::Warn
    };

    view! {
        <Collapsible>
            <CollapsibleHeader slot>
            <div class="flex justify-between items-center">
                <Chip color={chip_color} class="mr-2">{rules_count}</Chip>
                <div>{file_path}</div>
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
