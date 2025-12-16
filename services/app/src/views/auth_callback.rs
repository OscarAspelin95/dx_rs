use dioxus::prelude::*;
use crate::auth::{exchange_oauth_code, use_auth};
use crate::Route;

const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn AuthCallback(code: String) -> Element {
    let auth = use_auth();
    let navigator = use_navigator();
    let mut error = use_signal(|| None::<String>);
    let mut processing = use_signal(|| true);

    // Process the OAuth callback on mount
    use_effect(move || {
        let code = code.clone();
        spawn(async move {
            if code.is_empty() {
                error.set(Some("No authorization code received".to_string()));
                processing.set(false);
                return;
            }

            match exchange_oauth_code(code).await {
                Ok(auth_response) => {
                    // Store the token and set user
                    auth.set_user_with_token(auth_response.user, auth_response.token);
                    navigator.push(Route::Home {});
                }
                Err(e) => {
                    error.set(Some(format!("Authentication failed: {}", e)));
                    processing.set(false);
                }
            }
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: LOGIN_CSS }
        div { class: "auth-callback-container",
            if let Some(err) = error.read().as_ref() {
                div { class: "auth-callback-error",
                    span { class: "material-symbols-outlined error-icon", "error" }
                    h2 { "Authentication Failed" }
                    p { "{err}" }
                    Link { to: Route::Login {}, class: "retry-link",
                        "Try again"
                    }
                }
            } else if processing() {
                div { class: "auth-callback-loading",
                    div { class: "auth-loading-spinner" }
                    h2 { "Signing you in..." }
                    p { "Please wait while we complete the authentication." }
                }
            }
        }
    }
}

