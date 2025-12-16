use dioxus::prelude::*;
use super::server_fns::{logout as server_logout, validate_token};
use super::types::{AuthState, User};

#[allow(dead_code)]
const AUTH_TOKEN_KEY: &str = "auth_token";

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub state: Signal<AuthState>,
}

impl AuthContext {
    pub fn init() -> Self {
        let mut state = use_signal(|| AuthState::Loading);

        // Check authentication status on initialization
        use_effect(move || {
            spawn(async move {
                // Try to get token from localStorage
                let token = get_stored_token();
                
                match token {
                    Some(t) if !t.is_empty() => {
                        // Validate token with server
                        match validate_token(t).await {
                            Ok(Some(user)) => state.set(AuthState::Authenticated(user)),
                            Ok(None) => {
                                // Invalid token, clear it
                                clear_stored_token();
                                state.set(AuthState::Unauthenticated);
                            }
                            Err(_) => {
                                clear_stored_token();
                                state.set(AuthState::Unauthenticated);
                            }
                        }
                    }
                    _ => state.set(AuthState::Unauthenticated),
                }
            });
        });

        Self { state }
    }

    #[allow(dead_code)]
    pub fn current_user(&self) -> Option<User> {
        match &*self.state.read() {
            AuthState::Authenticated(user) => Some(user.clone()),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn is_authenticated(&self) -> bool {
        matches!(&*self.state.read(), AuthState::Authenticated(_))
    }

    #[allow(dead_code)]
    pub fn is_loading(&self) -> bool {
        matches!(&*self.state.read(), AuthState::Loading)
    }

    pub fn set_user_with_token(&self, user: User, token: String) {
        store_token(&token);
        let mut state = self.state;
        state.set(AuthState::Authenticated(user));
    }

    #[allow(dead_code)]
    pub fn clear_user(&self) {
        clear_stored_token();
        let mut state = self.state;
        state.set(AuthState::Unauthenticated);
    }

    pub async fn logout(&self) {
        let _ = server_logout().await;
        clear_stored_token();
        let mut state = self.state;
        state.set(AuthState::Unauthenticated);
    }
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
}

// Token storage helpers using web_sys for localStorage
#[cfg(feature = "web")]
fn get_stored_token() -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(AUTH_TOKEN_KEY)
        .ok()?
}

#[cfg(feature = "web")]
fn store_token(token: &str) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item(AUTH_TOKEN_KEY, token);
    }
}

#[cfg(feature = "web")]
fn clear_stored_token() {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.remove_item(AUTH_TOKEN_KEY);
    }
}

// Non-web implementations (for server-side rendering)
#[cfg(not(feature = "web"))]
fn get_stored_token() -> Option<String> {
    None
}

#[cfg(not(feature = "web"))]
fn store_token(_token: &str) {}

#[cfg(not(feature = "web"))]
fn clear_stored_token() {}

