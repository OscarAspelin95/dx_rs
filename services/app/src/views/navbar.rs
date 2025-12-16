use crate::auth::{use_auth, AuthState};
use crate::components::{Button, Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger, Separator};
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
fn AuthSection() -> Element {
    let auth = use_auth();
    let auth_state = auth.state.read();

    match &*auth_state {
        AuthState::Loading => {
            rsx! {
                div { class: "auth-section auth-loading",
                    span { "..." }
                }
            }
        }
        AuthState::Unauthenticated => {
            rsx! {
                div { class: "auth-section",
                    Link { to: Route::Login {}, class: "nav-login-link",
                        span { class: "material-symbols-outlined", "login" }
                        span { "Sign In" }
                    }
                }
            }
        }
        AuthState::Authenticated(user) => {
            let user_email = user.email.clone();
            rsx! {
                div { class: "auth-section auth-user",
                    span { class: "user-email", "{user_email}" }
                    Button {
                        id: "logout-button",
                        "data-style": "ghost",
                        onclick: move |_| {
                            spawn(async move {
                                auth.logout().await;
                            });
                        },
                        span { class: "material-symbols-outlined", "logout" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn MainNavBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { class: "main-nav-bar",
            Navbar { aria_label: "Main Navbar",
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

                // Auth section (right side)
                div { class: "navbar-spacer" }
                AuthSection {}
            }
            Outlet::<Route> {}
        }
    }
}
