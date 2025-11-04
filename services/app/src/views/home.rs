use crate::components::HomeMain;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { id: "home-container", HomeMain {} }
    }
}
