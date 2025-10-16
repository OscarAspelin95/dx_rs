use crate::components::TestToast;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/results.css");

#[component]
pub fn Results() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div { id: "results-container", TestToast {} }
    }
}
