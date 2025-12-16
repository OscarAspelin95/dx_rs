use dioxus::prelude::*;
use super::context::use_auth;
use super::types::AuthState;
use crate::Route;

#[component]
pub fn ProtectedRoute(children: Element) -> Element {
    let auth = use_auth();
    let navigator = use_navigator();
    let auth_state = auth.state.read();

    match &*auth_state {
        AuthState::Loading => {
            rsx! {
                div { class: "auth-loading",
                    div { class: "auth-loading-spinner" }
                    p { "Checking authentication..." }
                }
            }
        }
        AuthState::Unauthenticated => {
            // Redirect to login page
            navigator.push(Route::Login {});
            rsx! {}
        }
        AuthState::Authenticated(_) => {
            children
        }
    }
}

