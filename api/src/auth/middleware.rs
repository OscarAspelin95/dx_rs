use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::auth::auth::{AuthUser, Claims};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

///
pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET missing.");

    // Extract encoded token from header.
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        // Fix error handling later on.
        .ok_or(StatusCode::UNAUTHORIZED)
        .expect("Failed to extract auth token.");

    // Decode token.
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .expect("Failed to authenticate"); // ApiError?

    // Create authenticated user instance.
    let auth_user = AuthUser {
        id: token_data.claims.sub,
        email: token_data.claims.email,
        role: token_data.claims.role,
    };

    // ?
    req.extensions_mut().insert(auth_user);

    // OK to proceed.
    Ok(next.run(req).await)
}

/// Here, we create the actual JWT with jsonwebtoken.
/// We can get user info from either Google or GitHub.
pub fn create_jwt(
    user_id: &str,
    email: &str,
    role: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();

    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
