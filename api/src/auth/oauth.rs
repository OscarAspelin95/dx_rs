use axum::{Json, extract::State, http::StatusCode};
use oauth2::{
    AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
    basic::{BasicClient, BasicTokenType},
};
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::auth::middleware::create_jwt;
use crate::{
    auth::auth::{AuthResponse, OAuthExchange, UserInfo},
    state::ConnectionState,
};

#[derive(Deserialize)]
struct GoogleUserInfo {
    email: String,
    name: Option<String>,
    sub: String, // id.
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
    fn to_string(&self) -> String {
        match self {
            UserRole::DefaultUser => "default_user".into(),
        }
    }
}

// Struct for reading and writing user to database.
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

type OauthClient = oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    oauth2::StandardTokenIntrospectionResponse<oauth2::EmptyExtraTokenFields, BasicTokenType>,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
>;

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

pub async fn google_oauth_exchange(
    State(state): State<ConnectionState>,
    Json(payload): Json<OAuthExchange>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // Establish OAuth client.
    let oauth_client = get_oauth_client(
        state.google_client_id,
        state.google_client_secret,
        state.google_redirect_url,
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
    // For now, we mock.
    let db_user = DbUser::mock();

    // Create JWT
    let token = create_jwt(
        &db_user.id.to_string(),
        &db_user.email,
        &db_user.role.to_string(),
        &state.jwt_secret,
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
