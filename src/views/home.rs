use crate::components::Echo;
use crate::components::TestToast;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div { id: "home-container",
            Echo {}
            TestToast {}
        }
    }
}
