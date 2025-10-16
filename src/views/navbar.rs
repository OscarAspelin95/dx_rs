use crate::components::{Navbar, NavbarItem, Separator};
use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
fn NavBarSeparator() -> Element {
    rsx! {
        div { id: "navbar-separator-container",
            Separator { id: "navbar-separator", horizontal: false, decorative: true }
        }
    }
}

#[component]
pub fn MainNavBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { class: "main-nav-bar",
            Navbar { aria_label: "Main Navbav",
                NavbarItem {
                    id: "navbar-item",
                    index: 0usize,
                    value: "home".to_string(),
                    to: Route::Home {},
                    "Home"
                }
                NavBarSeparator {}
                NavbarItem {
                    id: "navbar-item",
                    index: 1usize,
                    value: "blog".to_string(),
                    to: Route::Blog { id: 1 },
                    "Blog"
                }
                NavBarSeparator {}
                NavbarItem {
                    id: "navbar-item",
                    index: 2usize,
                    value: "upload".to_string(),
                    to: Route::Upload {},
                    "Upload"
                }
                NavBarSeparator {}
                NavbarItem {
                    id: "navbar-item",
                    index: 3usize,
                    value: "results".to_string(),
                    to: Route::Results {},
                    "Results"
                }
            }
            Outlet::<Route> {}
        }
    }
}
