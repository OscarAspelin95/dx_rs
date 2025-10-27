use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::auth::auth::{AuthUser, Claims};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

/// The basic steps here are:
/// * Get the token from the request header.
/// * Decode token into claims (this means we can get user id, email, whatever).
/// * Add authenticated user info to the request on forwarding it to endpoint.
pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Can put in connectionstate or similar.
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

    // Define authenticated user instance.
    let auth_user = AuthUser {
        id: token_data.claims.sub,
        email: token_data.claims.email,
        role: token_data.claims.role,
    };

    // Insert into request we'll forward.
    req.extensions_mut().insert(auth_user);

    // OK to proceed (with user info added).
    Ok(next.run(req).await)
}

/// Here, we create the actual JWT with jsonwebtoken.
/// We can get user info from either Google or GitHub.
/// NOTE - we could make a temp test API endpoint for testing this.
/// both encoding and decoding with mock data to make sure it works.
pub fn create_jwt(
    user_id: &str,
    email: &str,
    role: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();

    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(), // user id.
        email: email.to_string(),
        role: role.to_string(),
        exp,                           // expiration.
        iat: now.timestamp() as usize, // issued at.
    };

    // Actual encoding.
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
