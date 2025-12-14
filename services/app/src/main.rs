use dioxus::prelude::*;
mod components;
mod route;
mod views;

use route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

const GOOGLE_ICONS_LINK: &str =
    "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined";

// This is the base css for all primitive components. However, sometimes this is
// not enough so we can style additionally with CSS id attributes.
const PRIMITIVES_CSS: Asset = asset!("/assets/styling/dx-components-theme.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: PRIMITIVES_CSS }
        document::Link { rel: "stylesheet", href: GOOGLE_ICONS_LINK }

        // Routing for main App.
        Router::<Route> {}
    }
}
