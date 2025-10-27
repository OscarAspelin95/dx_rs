use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GoogleUserInfo {
    pub email: String,
    pub name: Option<String>,
    pub sub: String, // id.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OAuthProvider {
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "github")]
    GitHub,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UserRole {
    #[serde(rename = "default_user")]
    DefaultUser,
}

impl UserRole {
    pub fn to_string(&self) -> String {
        match self {
            UserRole::DefaultUser => "default_user".into(),
        }
    }
}
