use dioxus::prelude::*;
use crate::auth::{get_google_auth_url, use_auth, AuthState};
use crate::components::Button;
use crate::Route;

const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Login() -> Element {
    let auth = use_auth();
    let navigator = use_navigator();
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    // Redirect if already authenticated
    use_effect(move || {
        if auth.is_authenticated() {
            navigator.push(Route::Home {});
        }
    });

    let handle_google_login = move |_| {
        spawn(async move {
            loading.set(true);
            error.set(None);

            match get_google_auth_url().await {
                Ok(url) => {
                    // Redirect to Google OAuth using navigator
                    navigator.push(NavigationTarget::<Route>::External(url));
                }
                Err(e) => {
                    error.set(Some(format!("Failed to initiate login: {}", e)));
                    loading.set(false);
                }
            }
        });
    };

    // Don't show login form if still checking auth status
    if matches!(&*auth.state.read(), AuthState::Loading) {
        return rsx! {
            div { class: "login-container",
                p { "Loading..." }
            }
        };
    }

    rsx! {
        document::Link { rel: "stylesheet", href: LOGIN_CSS }
        div { class: "login-container",
            div { class: "login-card",
                div { class: "login-header",
                    h1 { "Welcome to µBiome" }
                    p { "Sign in to upload and analyze your genomic samples" }
                }

                if let Some(err) = error.read().as_ref() {
                    div { class: "login-error",
                        span { class: "material-symbols-outlined", "error" }
                        span { "{err}" }
                    }
                }

                div { class: "login-actions",
                    Button {
                        id: "google-login-button",
                        disabled: loading(),
                        onclick: handle_google_login,
                        if loading() {
                            span { class: "button-loading", "Redirecting..." }
                        } else {
                            img {
                                src: "https://www.gstatic.com/firebasejs/ui/2.0.0/images/auth/google.svg",
                                alt: "Google",
                                class: "google-icon",
                            }
                            span { "Sign in with Google" }
                        }
                    }
                }

                div { class: "login-footer",
                    p { "By signing in, you agree to our terms of service." }
                }
            }
        }
    }
}

