use super::types::User;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use {
    super::types::{DbUser, GoogleUserInfo},
    jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header as JwtHeader, Validation},
    oauth2::{
        basic::BasicClient, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
        Scope, TokenResponse, TokenUrl,
    },
    serde::{Deserialize, Serialize},
    surrealdb::{engine::remote::ws::Client, engine::remote::ws::Ws, opt::auth::Root, Surreal},
    tokio::time::Duration,
    tracing::info,
};

#[cfg(feature = "server")]
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    role: String,
    exp: usize,
    iat: usize,
}

#[cfg(feature = "server")]
async fn connect_db(max_retries: usize) -> Result<Surreal<Client>, ServerFnError> {
    let mut retries: usize = 0;

    let endpoint = std::env::var("SURREALDB_ENDPOINT")
        .map_err(|_| ServerFnError::new("SURREALDB_ENDPOINT not set"))?;

    let db: Surreal<Client> = loop {
        info!("Attempting to connect to SurrealDB at {}...", endpoint);

        if retries > max_retries {
            return Err(ServerFnError::new(
                "Failed to connect to SurrealDB after max retries",
            ));
        }

        match Surreal::new::<Ws>(&endpoint).await {
            Ok(db) => {
                info!("Connected successfully after {} retries", retries);
                break db;
            }
            Err(e) => {
                info!("Connection failed: {:?}", e);
                retries += 1;
                let wait_time: u64 = 5 * retries as u64;
                info!("Retrying in {} seconds...", wait_time);
                tokio::time::sleep(Duration::from_secs(wait_time)).await;
            }
        }
    };

    let username =
        std::env::var("ROOT_USERNAME").map_err(|_| ServerFnError::new("ROOT_USERNAME not set"))?;
    let password =
        std::env::var("ROOT_PASSWORD").map_err(|_| ServerFnError::new("ROOT_PASSWORD not set"))?;

    db.signin(Root {
        username: &username,
        password: &password,
    })
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to sign in: {}", e)))?;

    let namespace = std::env::var("SURREALDB_NAMESPACE")
        .map_err(|_| ServerFnError::new("SURREALDB_NAMESPACE not set"))?;
    let dbname = std::env::var("SURREALDB_DBNAME")
        .map_err(|_| ServerFnError::new("SURREALDB_DBNAME not set"))?;

    db.use_ns(&namespace)
        .use_db(&dbname)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to use namespace/db: {}", e)))?;

    db.health()
        .await
        .map_err(|e| ServerFnError::new(format!("DB health check failed: {}", e)))?;

    Ok(db)
}

#[cfg(feature = "server")]
fn get_oauth_client() -> Result<BasicClient, ServerFnError> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| ServerFnError::new("GOOGLE_CLIENT_ID not set"))?;
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .map_err(|_| ServerFnError::new("GOOGLE_CLIENT_SECRET not set"))?;
    let redirect_url = std::env::var("GOOGLE_REDIRECT_URL")
        .map_err(|_| ServerFnError::new("GOOGLE_REDIRECT_URL not set"))?;

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .map_err(|e| ServerFnError::new(e.to_string()))?,
        Some(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        ),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url).map_err(|e| ServerFnError::new(e.to_string()))?,
    );

    Ok(client)
}

#[cfg(feature = "server")]
fn create_jwt_token(user: &User, secret: &str) -> Result<String, ServerFnError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        iat: now,
        exp: now + (60 * 60 * 24 * 7), // 7 days
    };

    encode(
        &JwtHeader::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ServerFnError::new(e.to_string()))
}

#[cfg(feature = "server")]
fn verify_jwt_token(token: &str, secret: &str) -> Result<Claims, ServerFnError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| ServerFnError::new(format!("Invalid token: {}", e)))?;

    Ok(token_data.claims)
}

#[cfg(feature = "server")]
async fn find_or_create_user(
    email: String,
    name: Option<String>,
    oauth_provider_id: String,
) -> Result<User, ServerFnError> {
    let db = connect_db(3).await?;

    let mut result = db
        .query(
            "SELECT * FROM users WHERE email = $email OR
             (oauth_provider = 'google' AND oauth_provider_id = $provider_id)
             LIMIT 1",
        )
        .bind(("email", email.clone()))
        .bind(("provider_id", oauth_provider_id.clone()))
        .await
        .map_err(|e| ServerFnError::new(format!("DB query failed: {}", e)))?;

    let existing: Option<DbUser> = result
        .take(0)
        .map_err(|e| ServerFnError::new(format!("Failed to parse user: {}", e)))?;

    if let Some(db_user) = existing {
        return Ok(db_user.into());
    }

    let new_user: Option<DbUser> = db
        .create("users")
        .content(serde_json::json!({
            "email": email,
            "name": name,
            "role": "user",
            "oauth_provider": "google",
            "oauth_provider_id": oauth_provider_id,
        }))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create user: {}", e)))?;

    new_user
        .map(|u| u.into())
        .ok_or_else(|| ServerFnError::new("Failed to create user: no data returned"))
}

#[server]
pub async fn get_google_auth_url() -> Result<String, ServerFnError> {
    let client = get_oauth_client()?;

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    Ok(auth_url.to_string())
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

#[server]
pub async fn exchange_oauth_code(code: String) -> Result<AuthResponse, ServerFnError> {
    use oauth2::reqwest::async_http_client;

    if code.is_empty() {
        return Err(ServerFnError::new("Authorization code is required"));
    }

    let jwt_secret =
        std::env::var("JWT_SECRET").map_err(|_| ServerFnError::new("JWT_SECRET not set"))?;

    let client = get_oauth_client()?;

    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .map_err(|e| ServerFnError::new(format!("Token exchange failed: {}", e)))?;

    let user_info_response = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token_result.access_token().secret())
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch user info: {}", e)))?;

    let response_text = user_info_response
        .text()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read user info: {}", e)))?;

    let google_user: GoogleUserInfo = serde_json::from_str(&response_text)
        .map_err(|e| ServerFnError::new(format!("Failed to parse user info: {}", e)))?;

    if google_user.email.is_empty() {
        return Err(ServerFnError::new("Google did not return an email address"));
    }

    let user = find_or_create_user(
        google_user.email.clone(),
        google_user.name.clone(),
        google_user.id.clone(),
    )
    .await?;

    let token = create_jwt_token(&user, &jwt_secret)?;

    Ok(AuthResponse { user, token })
}

#[server]
pub async fn validate_token(token: String) -> Result<Option<User>, ServerFnError> {
    let jwt_secret =
        std::env::var("JWT_SECRET").map_err(|_| ServerFnError::new("JWT_SECRET not set"))?;

    if token.is_empty() {
        return Ok(None);
    }

    let claims = match verify_jwt_token(&token, &jwt_secret) {
        Ok(c) => c,
        Err(_) => return Ok(None),
    };

    Ok(Some(User {
        id: claims.sub,
        email: claims.email,
        name: None,
        role: claims.role,
    }))
}

#[server]
pub async fn get_current_user() -> Result<Option<User>, ServerFnError> {
    Ok(None)
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    Ok(())
}
