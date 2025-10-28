use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::{
    errors::ApiError,
    schema::oauth::{GoogleUserInfo, UserRole},
};
use crate::{schema::oauth::OAuthProvider, state::ConnectionState};

use log::{error, info};

///
use oauth2::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse,
    TokenUrl,
    basic::{BasicClient, BasicTokenType},
};
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client, error::Api};

use crate::auth::auth::{AuthResponse, OAuthExchange, UserInfo};
use crate::auth::middleware::create_jwt;

///

/// Test function for checking authentication.
/// This endpoint is protected upstream to authorization.
pub async fn test_auth(
    State(state): State<ConnectionState>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb.client;

    match db.health().await {
        Ok(()) => info!(""),
        Err(e) => error!("{:?}", e),
    };

    Ok((StatusCode::OK, Json({})))
}

/// The basic flow for OAuth with Google is as follows:
/// * The frontend makes an API request to our "oauth_google_url" endpoint when a user
///     wants to log in with google (e.g., through "log in with google button").
/// * In the API, when we get this request, we establish an OAuth client using our secret
///     google oauth credentials (.env file). We then use it to generate an authorization URL
/// * We send this URL back to the frontend.
/// * The frontend redirects the user’s browser to this url. Upon granting access,
///     Google redirects back to our configured redirect URI with a short-lived authorization code.
/// * In the frontend, we extract this authorization code and make a new
///     request to our "oauth_google_exchange" endpoint.
/// * In this API function, we again establish an OAuth client and
///     use the short-lived authorization code to exchange it for an **access token**,
///     which we then use to fetch user information from google.
/// * After this, we can write the user information to the database. By defining
///     our JWT claims, we can also send back a JWT for the user to use for authentication.
/// * In our protected (authorization required) API routes, the frontend includes the JWT
///     with the request (e.g. in the `Authorization: Bearer <token>` header). If we can
///     validate it with jsonwebtoken, we are good to proceed.
///
/// Basics of refresh tokens (long lived):
/// * Ideally, JWTs should be short lived because they cannot be revoked.
///     JWTs have an expiration date, after which it is invalid.
/// * A solution is to keep track of a refresh token that needs to be stored
///     securely in the frontend and in the database.
/// * The db structure for refresh token could be something like:
///     {token: String, user_id: id, created_at: time, expires_at: time, revoked: bool}
///
/// * When making a API call and getting a TokenExpired error, the frontend can make a call to
///     an API endpoint "/auth/refresh" with the refresh token in the request. We check that:
///     * Is exists in database.
///     * It has not expired (long expiration date).
///     * It is not revoked.
///
/// * If valid, we can generate a new JWT for the user and send back.
/// * We can also possibly generate a new refresh token
/// ----------------------------------------------------------------------------------------------
// Move to schema or similar later on.
#[derive(Deserialize, Serialize, Clone)]
struct DbUser {
    id: String,
    email: String,
    name: Option<String>,
    role: UserRole,
    oauth_provider: OAuthProvider,
    oauth_provider_id: String, // ?
}

impl DbUser {
    fn mock() -> Self {
        Self {
            id: "mock_user_id".into(),
            email: "mock@gmail.com".into(),
            name: Some("mock_name".into()),
            role: UserRole::DefaultUser,
            oauth_provider: OAuthProvider::Google,
            oauth_provider_id: "mock_oauth_id".into(),
        }
    }
}
/// This type is fked up.
type OauthClient = oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    oauth2::StandardTokenIntrospectionResponse<oauth2::EmptyExtraTokenFields, BasicTokenType>,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
>;

/// Connect to Google oauth client.
async fn get_oauth_client(
    google_client_id: String,
    google_client_secret: String,
    google_redirect_url: String,
) -> OauthClient {
    let client_id = ClientId::new(google_client_id);
    let secret = Some(ClientSecret::new(google_client_secret));

    let auth_url = oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".into())
        .expect("Failed to get auth url");

    let token_url = Some(
        TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Failed to get token url"),
    );

    let redirect_uri =
        RedirectUrl::new(google_redirect_url).expect("Failed to generated redirect url.");

    let oauth_client =
        BasicClient::new(client_id, secret, auth_url, token_url).set_redirect_uri(redirect_uri);

    oauth_client
}

///

/// DB script.
// async fn find_or_create_user(
//     db: &Surreal<Client>,
//     email: &str,
//     provider: &str,
//     provider_id: &str,
//     name: Option<&str>,
// ) -> Result<DbUser, String> {
//     // Try to find existing user
//     let mut response = db
//         .query("SELECT * FROM user WHERE email = $email OR (oauth_provider = $provider AND oauth_provider_id = $provider_id) LIMIT 1")
//         .bind(("email", email))
//         .bind(("provider", provider))
//         .bind(("provider_id", provider_id))
//         .await
//         .map_err(|e| format!("Database query failed: {}", e))?;

//     let existing: Option<DbUser> = response
//         .take(0)
//         .map_err(|e| format!("Failed to parse user: {}", e))?;

//     if let Some(user) = existing {
//         Ok(user)
//     } else {
//         // Create new user
//         let mut response = db
//             .query(
//                 "CREATE user CONTENT {
//                 email: $email,
//                 oauth_provider: $provider,
//                 oauth_provider_id: $provider_id,
//                 name: $name,
//                 role: 'user',
//                 created_at: time::now()
//             }",
//             )
//             .bind(("email", email))
//             .bind(("provider", provider))
//             .bind(("provider_id", provider_id))
//             .bind(("name", name.unwrap_or("")))
//             .await
//             .map_err(|e| format!("Failed to create user: {}", e))?;

//         let created: Option<DbUser> = response
//             .take(0)
//             .map_err(|e| format!("Failed to parse created user: {}", e))?;

//         created.ok_or_else(|| "User creation returned no data".to_string())
//     }
// }

///

pub async fn auth_google_callback(
    State(state): State<ConnectionState>,
    Json(payload): Json<OAuthExchange>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let db = state.surrealdb.client;

    match db.health().await {
        Ok(()) => info!("Database healthy!"),
        Err(e) => error!("{:?}", e),
    }

    // Establish OAuth client.
    let oauth_client = get_oauth_client(
        state.environment.google_client_id,
        state.environment.google_client_secret,
        state.environment.google_redirect_url,
    )
    .await;

    // Get authentication token.
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(payload.code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Token exchange failed: {}", e),
            )
        })?;

    // Based on token, we can fetch user info from google.
    // This information can be written to the database.
    let user_info: GoogleUserInfo = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token_result.access_token().secret())
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch user info: {}", e),
            )
        })?
        .json()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse user info: {}", e),
            )
        })?;

    // Write to database...
    // async fn find_or_create_user(
    // For now, we mock.
    let db_user = DbUser::mock();

    // Create JWT
    let token = create_jwt(
        &db_user.id.to_string(),
        &db_user.email,
        &db_user.role.to_string(),
        &state.environment.jwt_secret,
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create JWT: {}", e),
        )
    })?;

    Ok(Json(AuthResponse {
        token,
        user: UserInfo {
            id: db_user.id.to_string(),
            email: db_user.email,
            name: db_user.name,
            role: db_user.role.to_string(),
        },
    }))
}

/// This function takes a request to get a google oauth url
/// and redirects to it to avoid having boilerplate frontend code.
pub async fn auth_google_login(
    State(state): State<ConnectionState>,
) -> Result<impl IntoResponse, ApiError> {
    // Establish OAuth client.
    let oauth_client = get_oauth_client(
        state.environment.google_client_id,
        state.environment.google_client_secret,
        state.environment.google_redirect_url,
    )
    .await;

    let (authorize_url, _) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(vec![
            Scope::new("email".to_string()),
            Scope::new("profile".to_string()),
        ])
        .url();

    Ok(Redirect::to(authorize_url.as_str()))
}
