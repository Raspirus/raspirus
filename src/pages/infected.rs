use leptonic::components::stack::Stack;
use leptonic::Size;
use leptos::*;
use leptos_i18n::t;
use leptos_router::use_query_map;
use crate::components::home_button::HomeButton;
use crate::components::virus_card::VirusCard;
use crate::i18n::use_i18n;

// TODO:
// - Styling
// - Fix the text in the VirusCard component

#[component]
pub fn Infected() -> impl IntoView {
    let i18n = use_i18n();
    let infected = use_query_map().get_untracked().get("result").cloned();
    // Convert the string to a vector of strings
    let infected_files: Vec<String> = serde_json::from_str(&infected.unwrap()).unwrap();

    view! {
        <div>
            <div class="align-middle">
                <HomeButton />
                <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                    {t!(i18n, infected_title)}
                </h1>
            </div>
                <div>
                    <Stack spacing=Size::Em(0.6)>
                        {infected_files.into_iter()
                            .map(|file| {
                                view! {
                                    <VirusCard title=file text="This is a virus".to_string() />
                                }
                            }).collect::<Vec<_>>()
                        }
                    </Stack>
                </div>
            </div>
    }
}