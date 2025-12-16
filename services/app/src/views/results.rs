use crate::auth::ProtectedRoute;
use crate::components::TestResults;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/results.css");

#[component]
pub fn Results() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        ProtectedRoute {
            div { id: "results-container", TestResults {} }
        }
    }
}
