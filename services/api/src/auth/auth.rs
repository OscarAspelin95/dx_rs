use serde::{Deserialize, Serialize};

/// Json Web Token claims.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // id
    pub email: String,
    pub role: String,
    pub exp: usize, // expiration.
    pub iat: usize, // issued at.
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub role: String,
}

/// What we get from Oauth.
#[derive(Deserialize)]
pub struct OAuthExchange {
    pub code: String,
}

/// User related
#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}
