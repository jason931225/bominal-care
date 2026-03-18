// =============================================================================
// OAuth Authentication — Kakao, Naver, Google
// Routes: /api/auth/oauth/{provider}/{start,callback}
// =============================================================================

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EndpointNotSet, EndpointSet,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenUrl, TokenResponse,
    basic::{BasicClient, BasicTokenResponse},
};
use oauth2::reqwest as oauth2_reqwest;
use serde::Deserialize;
use reqwest;
use tower_sessions::Session;
use uuid::Uuid;

use crate::AppState;
use super::extractor::{AuthUser, SESSION_USER_KEY};

const OAUTH_CSRF_KEY: &str = "oauth_csrf";
const OAUTH_PKCE_KEY: &str = "oauth_pkce";
const OAUTH_PROVIDER_KEY: &str = "oauth_provider";

// ---------------------------------------------------------------------------
// Provider configuration
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct OAuthProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OAuthProviders {
    pub kakao: Option<OAuthProviderConfig>,
    pub naver: Option<OAuthProviderConfig>,
    pub google: Option<OAuthProviderConfig>,
}

impl OAuthProviders {
    pub fn from_env() -> Self {
        Self {
            kakao: Self::load_provider(
                "KAKAO",
                "https://kauth.kakao.com/oauth/authorize",
                "https://kauth.kakao.com/oauth/token",
                "https://kapi.kakao.com/v2/user/me",
                vec!["account_email".into(), "profile_nickname".into()],
            ),
            naver: Self::load_provider(
                "NAVER",
                "https://nid.naver.com/oauth2.0/authorize",
                "https://nid.naver.com/oauth2.0/token",
                "https://openapi.naver.com/v1/nid/me",
                vec![],
            ),
            google: Self::load_provider(
                "GOOGLE",
                "https://accounts.google.com/o/oauth2/v2/auth",
                "https://oauth2.googleapis.com/token",
                "https://www.googleapis.com/oauth2/v3/userinfo",
                vec!["openid".into(), "email".into(), "profile".into()],
            ),
        }
    }

    fn load_provider(
        prefix: &str,
        auth_url: &str,
        token_url: &str,
        userinfo_url: &str,
        scopes: Vec<String>,
    ) -> Option<OAuthProviderConfig> {
        let client_id = std::env::var(format!("{prefix}_CLIENT_ID")).ok()?;
        let client_secret = std::env::var(format!("{prefix}_CLIENT_SECRET")).ok()?;
        if client_id.is_empty() || client_secret.is_empty() {
            return None;
        }
        Some(OAuthProviderConfig {
            client_id,
            client_secret,
            auth_url: auth_url.to_string(),
            token_url: token_url.to_string(),
            userinfo_url: userinfo_url.to_string(),
            scopes,
        })
    }

    pub fn get(&self, provider: &str) -> Option<&OAuthProviderConfig> {
        match provider {
            "kakao" => self.kakao.as_ref(),
            "naver" => self.naver.as_ref(),
            "google" => self.google.as_ref(),
            _ => None,
        }
    }
}

fn build_client(
    config: &OAuthProviderConfig,
    redirect_url: &str,
) -> Result<BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>, String> {
    let client = BasicClient::new(ClientId::new(config.client_id.clone()))
        .set_client_secret(ClientSecret::new(config.client_secret.clone()))
        .set_auth_uri(AuthUrl::new(config.auth_url.clone()).map_err(|e| e.to_string())?)
        .set_token_uri(TokenUrl::new(config.token_url.clone()).map_err(|e| e.to_string())?)
        .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).map_err(|e| e.to_string())?);
    Ok(client)
}

// ---------------------------------------------------------------------------
// OAuth Start — redirects to provider
// ---------------------------------------------------------------------------

pub async fn oauth_start(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    session: Session,
) -> impl IntoResponse {
    let config = match state.oauth.get(&provider) {
        Some(c) => c,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "success": false, "error": "Unknown provider" })),
            )
                .into_response();
        }
    };

    let base_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let redirect_url = format!("{base_url}/api/auth/oauth/{provider}/callback");

    let client = match build_client(config, &redirect_url) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": e })),
            )
                .into_response();
        }
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let mut auth_request = client.authorize_url(CsrfToken::new_random);
    for scope in &config.scopes {
        auth_request = auth_request.add_scope(Scope::new(scope.clone()));
    }
    let (auth_url, csrf_token) = auth_request.set_pkce_challenge(pkce_challenge).url();

    // Store state in session
    let _ = session.insert(OAUTH_CSRF_KEY, csrf_token.secret().clone()).await;
    let _ = session.insert(OAUTH_PKCE_KEY, pkce_verifier.secret().clone()).await;
    let _ = session.insert(OAUTH_PROVIDER_KEY, provider).await;

    Redirect::temporary(auth_url.as_str()).into_response()
}

// ---------------------------------------------------------------------------
// OAuth Callback — exchanges code, upserts user, creates session
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct OAuthCallbackParams {
    pub code: String,
    pub state: String,
}

pub async fn oauth_callback(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(params): Query<OAuthCallbackParams>,
    session: Session,
) -> impl IntoResponse {
    // Verify CSRF
    let stored_csrf: Option<String> = session.remove(OAUTH_CSRF_KEY).await.unwrap_or(None);
    if stored_csrf.as_deref() != Some(&params.state) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": "Invalid CSRF state" })),
        )
            .into_response();
    }

    let config = match state.oauth.get(&provider) {
        Some(c) => c,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "success": false, "error": "Unknown provider" })),
            )
                .into_response();
        }
    };

    let base_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let redirect_url = format!("{base_url}/api/auth/oauth/{provider}/callback");

    let client = match build_client(config, &redirect_url) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": e })),
            )
                .into_response();
        }
    };

    // Exchange code for token
    let pkce_secret: Option<String> = session.remove(OAUTH_PKCE_KEY).await.unwrap_or(None);
    let pkce_verifier = PkceCodeVerifier::new(pkce_secret.unwrap_or_default());

    let oauth_http_client = oauth2_reqwest::ClientBuilder::new()
        .redirect(oauth2_reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": format!("HTTP client error: {e}") })),
            )
                .into_response();
        });
    let oauth_http_client = match oauth_http_client {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    let token_result: Result<BasicTokenResponse, _> = client
        .exchange_code(AuthorizationCode::new(params.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&oauth_http_client)
        .await;

    let token = match token_result {
        Ok(t) => t,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "success": false, "error": format!("Token exchange failed: {e}") })),
            )
                .into_response();
        }
    };

    // Fetch user profile from provider
    let access_token = token.access_token().secret().clone();
    let http_client = reqwest::Client::new();
    let userinfo_resp = http_client
        .get(&config.userinfo_url)
        .bearer_auth(&access_token)
        .send()
        .await;

    let profile_json: serde_json::Value = match userinfo_resp {
        Ok(resp) => resp.json::<serde_json::Value>().await.unwrap_or_default(),
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "success": false, "error": format!("Userinfo failed: {e}") })),
            )
                .into_response();
        }
    };

    // Extract email/name from provider-specific profile shape
    let (email, name, image) = extract_profile(&provider, &profile_json);

    let email = match email {
        Some(e) => e,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "success": false, "error": "Email not provided by OAuth provider" })),
            )
                .into_response();
        }
    };

    // Upsert user
    let row: (Uuid, bominal_types::enums::UserRole, bominal_types::enums::KycLevel) = sqlx::query_as(
        "INSERT INTO users (id, email, name, image, kyc_level, is_active)
         VALUES ($1, $2, $3, $4, 'EMAIL_VERIFIED', true)
         ON CONFLICT (email) DO UPDATE SET
           name = COALESCE(EXCLUDED.name, users.name),
           image = COALESCE(EXCLUDED.image, users.image),
           updated_at = NOW()
         RETURNING id, role, kyc_level",
    )
    .bind(Uuid::new_v4())
    .bind(&email)
    .bind(&name)
    .bind(&image)
    .fetch_one(&state.pool)
    .await
    .unwrap_or_else(|_| (Uuid::new_v4(), bominal_types::enums::UserRole::Senior, bominal_types::enums::KycLevel::None));

    let auth_user = AuthUser {
        id: row.0,
        email,
        name: name.unwrap_or_default(),
        role: row.1,
        kyc_level: row.2,
        tenant_id: None,
    };

    let _ = session.insert(SESSION_USER_KEY, &auth_user).await;

    // Redirect to home
    Redirect::temporary("/").into_response()
}

/// Extract email, name, image from provider-specific profile JSON.
fn extract_profile(
    provider: &str,
    json: &serde_json::Value,
) -> (Option<String>, Option<String>, Option<String>) {
    match provider {
        "kakao" => {
            let account = json.get("kakao_account");
            let email = account.and_then(|a| a.get("email")).and_then(|v| v.as_str()).map(String::from);
            let profile = account.and_then(|a| a.get("profile"));
            let name = profile.and_then(|p| p.get("nickname")).and_then(|v| v.as_str()).map(String::from);
            let image = profile.and_then(|p| p.get("profile_image_url")).and_then(|v| v.as_str()).map(String::from);
            (email, name, image)
        }
        "naver" => {
            let resp = json.get("response");
            let email = resp.and_then(|r| r.get("email")).and_then(|v| v.as_str()).map(String::from);
            let name = resp
                .and_then(|r| r.get("name").or_else(|| r.get("nickname")))
                .and_then(|v| v.as_str())
                .map(String::from);
            let image = resp.and_then(|r| r.get("profile_image")).and_then(|v| v.as_str()).map(String::from);
            (email, name, image)
        }
        "google" | _ => {
            let email = json.get("email").and_then(|v| v.as_str()).map(String::from);
            let name = json.get("name").and_then(|v| v.as_str()).map(String::from);
            let image = json.get("picture").and_then(|v| v.as_str()).map(String::from);
            (email, name, image)
        }
    }
}
