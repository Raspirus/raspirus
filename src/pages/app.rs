use leptos::*;
use leptos::wasm_bindgen::JsValue;
use leptos::wasm_bindgen::prelude::wasm_bindgen;
use leptos_router::*;
use crate::pages::{
    index::Index,
    settings::Settings,
    information::Information,
    loading::Loading,
    clean::Clean,
    infected::Infected,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
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
                    path="/*any"
                    view=|| view! { <h1>"Not Found"</h1> }
                    static_params=|| Box::pin(async { StaticParamsMap::default() })
                />
            </Routes>
        </Router>
    }
}