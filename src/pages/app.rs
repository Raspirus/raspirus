use crate::i18n::provide_i18n_context;
use crate::pages::{
    agreement::Agreement, clean::Clean, index::Index, infected::Infected, information::Information,
    loading::Loading, settings::Settings,
};
use leptonic::components::root::Root;
use leptonic::components::theme::LeptonicTheme;
use leptos::*;
use leptos_router::*;

/// Defines the routes of the application. We use the `StaticRoute` component to define the routes.
/// The `StaticRoute` component is a wrapper around the `Route` component that provides a static
/// way to define the routes of the application. This is useful when we want to define the routes
/// of the application in a static way, without the need to use the `Route` component.
///
/// This is also the entry point of the application. The `App` component is the root component of
/// the application. It defines the routes of the application and the components that will be
/// rendered when the user navigates to a specific route.
#[component]
pub fn App() -> impl IntoView {
    // We only need to call this once, it will provide the i18n context to the application
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
                        path="/*any"
                        view=|| view! { <h1>"Not Found"</h1> <p>"Please report this issue"</p> }
                        static_params=|| Box::pin(async { StaticParamsMap::default() })
                    />
                </Routes>
            </Router>
        </Root>
    }
}
