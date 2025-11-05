use crate::components::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger, Separator};
use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const BINARY: Asset = asset!("/assets/icons/binary.svg");

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
                NavbarNav { id: "navbar-nav", index: 2usize,
                    NavbarTrigger { id: "navbar-trigger", "Upload" }
                    NavbarContent { id: "navbar-content",
                        NavbarItem {
                            index: 0usize,
                            value: "fastq".to_string(),
                            to: Route::Upload {},
                            div { id: "navchoice",
                                "Fastq"
                                img { id: "binary-file", src: BINARY }
                            }
                        }
                        NavbarItem {
                            index: 1usize,
                            value: "fasta".to_string(),
                            to: Route::Upload {},
                            div { id: "navchoice",
                                "Fasta"
                                span { class: "material-symbols-outlined", "text_snippet" }
                            }
                        }
                        NavbarItem {
                            index: 2usize,
                            value: "database".to_string(),
                            to: Route::Upload {},
                            div { id: "navchoice",
                                "Database"
                                span { class: "material-symbols-outlined", "storage" }
                            }
                        }

                    }
                }
                NavBarSeparator {}
                NavbarItem {
                    id: "navbar-item",
                    index: 3usize,
                    value: "results".to_string(),
                    to: Route::Results {},
                    "Results"
                }
                NavBarSeparator {}
                NavbarItem {
                    id: "navbar-item",
                    index: 4usize,
                    value: "todo".to_string(),
                    to: Route::ToDo {},
                    "ToDo"
                }
            }
            Outlet::<Route> {}
        }
    }
}
