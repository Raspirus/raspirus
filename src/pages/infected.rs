use crate::components::home_button::HomeButton;
use crate::components::skipped_card::SkippedCard;
use crate::components::virus_card::VirusCard;
use crate::generic::{Skipped, TaggedFile};
//use crate::generic::{RuleFeedback, TaggedFile};
// use std::path::PathBuf;
use crate::i18n::use_i18n;
use leptonic::components::prelude::{Collapsibles, OnOpen, Separator};
use leptonic::components::stack::Stack;
use leptonic::Size;
use leptos::*;
use leptos_i18n::t;
use leptos_router::use_query_map;

/// Infected page
/// The page that is shown if after the scan the user has infected files
/// It shows a list of all infected files with their path and the signature of the virus
/// The user can further analyze the file by clicking on it
#[component]
pub fn Infected() -> impl IntoView {
    let i18n = use_i18n();
    let infected = use_query_map().get_untracked().get("infected").cloned();
    let infected_files: Vec<TaggedFile> =
        serde_json::from_str(&infected.unwrap_or_default()).unwrap_or_default();
    let skipped = use_query_map().get_untracked().get("skipped").cloned();
    let skipped_files: Vec<Skipped> =
        serde_json::from_str(&skipped.unwrap_or_default()).unwrap_or_default();

    // let infected_files: Vec<TaggedFile> = vec![
    //     TaggedFile {
    //         path: PathBuf::from("/path/to/infected/file1"),
    //         descriptions: vec![
    //             RuleFeedback {
    //                 rule_name: String::from("Malware Detected"),
    //                 rule_description: String::from("This file contains signatures of known malware."),
    //             },
    //             RuleFeedback {
    //                 rule_name: String::from("Suspicious Activity"),
    //                 rule_description: String::from("Suspicious behavior detected in file operations."),
    //             },
    //         ],
    //         rule_count: 2,
    //     },
    //     TaggedFile {
    //         path: PathBuf::from("/path/to/infected/file2"),
    //         descriptions: vec![
    //             RuleFeedback {
    //                 rule_name: String::from("Ransomware Detected"),
    //                 rule_description: String::from("File encryption patterns related to ransomware were detected."),
    //             },
    //         ],
    //         rule_count: 1,
    //     },
    // ];

    view! {
        <div>
            <div class="align-middle">
                <HomeButton />
                <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                    {t!(i18n, infected_title)}
                </h1>
                <button on:click=move |_| {}>"Print PDF"</button>
            </div>
                <h2>"Infected files"</h2>
                <div class="px-2">
                    <Collapsibles default_on_open=OnOpen::CloseOthers>
                    <Stack spacing=Size::Em(0.6)>
                        {infected_files.into_iter()
                            .map(|file| {
                                view! {
                                    <VirusCard
                                        file_path=file.path
                                        rules_count=file.rule_count
                                        rule_matches=file.descriptions />
                                }
                            }).collect::<Vec<_>>()
                        }
                    </Stack>
                    </Collapsibles>
                </div>
                <Separator />
                <h2 class="mt-2">"Skipped files"</h2>
                <div class="px-2">
                <Collapsibles default_on_open=OnOpen::CloseOthers>
                <Stack spacing=Size::Em(0.6)>
                    {skipped_files.into_iter()
                        .map(|file| {
                            view! {
                                <SkippedCard
                                    file_path=file.path
                                    skip_reason=file.reason />
                            }
                        }).collect::<Vec<_>>()
                    }
                </Stack>
                </Collapsibles>
            </div>
            </div>
    }
}
