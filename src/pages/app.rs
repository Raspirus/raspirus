use leptonic::components::root::Root;
use leptonic::components::theme::LeptonicTheme;
use leptos::*;
use crate::i18n::provide_i18n_context;
use leptos_router::*;
use crate::pages::{
    index::Index,
    settings::Settings,
    information::Information,
    loading::Loading,
    clean::Clean,
    infected::Infected,
    updating::Updating,
    agreement::Agreement
};


#[component]
pub fn App() -> impl IntoView {
    provide_i18n_context();

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes>
                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/"
                        view=Index
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/settings"
                        view=Settings
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/information"
                        view=Information
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/loading"
                        view=Loading
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/clean"
                        view=Clean
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/infected"
                        view=Infected
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/agreement"
                        view=Agreement
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />

                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/update"
                        view=Updating
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />

                    <StaticRoute
                        mode=StaticMode::Incremental
                        path="/*any"
                        view=|| view! { <h1>"Not Found"</h1> }
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />
                </Routes>
            </Router>
        </Root>
    }
}