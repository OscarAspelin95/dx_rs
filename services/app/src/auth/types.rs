use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AuthState {
    Loading,
    Authenticated(User),
    Unauthenticated,
}

#[cfg(feature = "server")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DbUser {
    pub id: surrealdb::sql::Thing,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub oauth_provider: String,
    pub oauth_provider_id: String,
}

#[cfg(feature = "server")]
impl From<DbUser> for User {
    fn from(db_user: DbUser) -> Self {
        User {
            id: db_user.id.to_string(),
            email: db_user.email,
            name: db_user.name,
            role: db_user.role,
        }
    }
}

