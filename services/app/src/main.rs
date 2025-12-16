use dioxus::prelude::*;
use dotenv::dotenv;
mod auth;
mod components;
mod route;
mod views;

use auth::AuthContext;
use route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

const GOOGLE_ICONS_LINK: &str =
    "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined";

const PRIMITIVES_CSS: Asset = asset!("/assets/styling/dx-components-theme.css");

fn main() {
    dotenv().ok();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize auth context at the root level
    let auth_context = AuthContext::init();
    use_context_provider(|| auth_context);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: PRIMITIVES_CSS }
        document::Link { rel: "stylesheet", href: GOOGLE_ICONS_LINK }

        Router::<Route> {}
    }
}
