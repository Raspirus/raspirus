use crate::components::home_button::HomeButton;
use crate::components::virus_card::VirusCard;
use crate::generic::TaggedFile;
use crate::i18n::use_i18n;
use leptonic::components::stack::Stack;
use leptonic::components::prelude::{Collapsibles, OnOpen};
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
    let infected = use_query_map().get_untracked().get("result").cloned();
    let infected_files: Vec<TaggedFile> = serde_json::from_str(&infected.unwrap()).unwrap();

    view! {
        <div>
            <div class="align-middle">
                <HomeButton />
                <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                    {t!(i18n, infected_title)}
                </h1>
            </div>
                <div>
                    <Collapsibles default_on_open=OnOpen::CloseOthers>
                    <Stack spacing=Size::Em(0.6)>
                        {infected_files.into_iter()
                            .map(|file| {
                                view! {
                                    <VirusCard 
                                        file_path=file.path.to_string_lossy().to_string() 
                                        rules_count=file.rule_count 
                                        rule_matches=file.descriptions />
                                }
                            }).collect::<Vec<_>>()
                        }
                    </Stack>
                    </Collapsibles>            
                </div>
            </div>
    }
}
