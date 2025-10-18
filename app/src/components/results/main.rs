use dioxus::prelude::*;

use crate::components::results::Table;

const RESULT_CSS: Asset = asset!("./style.css");

#[component]
pub fn TestResults() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: RESULT_CSS }
        Table {}
    }
}
