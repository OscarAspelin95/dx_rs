use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub role: String,
}
