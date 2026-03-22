// =============================================================================
// WebAuthn (Passkey) Authentication
// Routes: /api/auth/webauthn/{register,login}/{start,finish}
// =============================================================================

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use tower_sessions::Session;
use uuid::Uuid;
use webauthn_rs::prelude::*;

use crate::AppState;
use super::extractor::{AuthUser, SESSION_USER_KEY};

const WEBAUTHN_REG_STATE_KEY: &str = "webauthn_reg_state";
const WEBAUTHN_AUTH_STATE_KEY: &str = "webauthn_auth_state";

// ---------------------------------------------------------------------------
// Registration: Start
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct RegisterStartRequest {
    pub user_id: Uuid,
    pub user_name: String,
    pub display_name: String,
}

pub async fn register_start(
    State(state): State<AppState>,
    session: Session,
    Json(req): Json<RegisterStartRequest>,
) -> impl IntoResponse {
    // Fetch existing credentials for this user
    let existing: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT credential FROM webauthn_credentials WHERE user_id = $1",
    )
    .bind(req.user_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let exclude_credentials: Vec<CredentialID> = existing
        .iter()
        .filter_map(|v| serde_json::from_value::<Passkey>(v.clone()).ok())
        .map(|pk| pk.cred_id().clone())
        .collect();

    let exclude_opt = if exclude_credentials.is_empty() {
        None
    } else {
        Some(exclude_credentials)
    };

    let (ccr, reg_state) = match state.webauthn.start_passkey_registration(
        req.user_id,
        &req.user_name,
        &req.display_name,
        exclude_opt,
    ) {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "error": format!("WebAuthn error: {e}"),
                })),
            );
        }
    };

    // Store registration state and user_id in session (60s TTL handled by session config)
    let _ = session.insert("webauthn_reg_user_id", req.user_id).await;
    if let Err(e) = session.insert(WEBAUTHN_REG_STATE_KEY, reg_state).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "error": format!("Session error: {e}"),
            })),
        );
    }

    (StatusCode::OK, Json(serde_json::json!({ "success": true, "data": ccr })))
}

// ---------------------------------------------------------------------------
// Registration: Finish
// ---------------------------------------------------------------------------

pub async fn register_finish(
    State(state): State<AppState>,
    session: Session,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> impl IntoResponse {
    let reg_state: PasskeyRegistration = match session.remove(WEBAUTHN_REG_STATE_KEY).await {
        Ok(Some(s)) => s,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "error": "진행 중인 등록이 없습니다",
                })),
            );
        }
    };

    let passkey = match state.webauthn.finish_passkey_registration(&reg, &reg_state) {
        Ok(pk) => pk,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "error": format!("Registration failed: {e}"),
                })),
            );
        }
    };

    let credential_json = serde_json::to_value(&passkey).unwrap_or_default();
    let user_id: Uuid = match session.remove("webauthn_reg_user_id").await {
        Ok(Some(id)) => id,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "error": "등록 세션이 만료되었습니다",
                })),
            );
        }
    };

    if let Err(e) = sqlx::query(
        "INSERT INTO webauthn_credentials (id, user_id, credential) VALUES ($1, $2, $3)",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&credential_json)
    .execute(&state.pool)
    .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to store credential: {e}"),
            })),
        );
    }

    (StatusCode::OK, Json(serde_json::json!({ "success": true })))
}

// ---------------------------------------------------------------------------
// Login: Start
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct LoginStartRequest {
    pub user_id: Uuid,
}

pub async fn login_start(
    State(state): State<AppState>,
    session: Session,
    Json(req): Json<LoginStartRequest>,
) -> impl IntoResponse {
    let credentials: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT credential FROM webauthn_credentials WHERE user_id = $1",
    )
    .bind(req.user_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    if credentials.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": "등록된 패스키가 없습니다",
            })),
        );
    }

    let passkeys: Vec<Passkey> = credentials
        .into_iter()
        .filter_map(|v| serde_json::from_value(v).ok())
        .collect();

    let (rcr, auth_state) = match state.webauthn.start_passkey_authentication(&passkeys) {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "error": format!("WebAuthn error: {e}"),
                })),
            );
        }
    };

    if let Err(e) = session.insert(WEBAUTHN_AUTH_STATE_KEY, auth_state).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "error": format!("Session error: {e}"),
            })),
        );
    }

    (StatusCode::OK, Json(serde_json::json!({ "success": true, "data": rcr })))
}

// ---------------------------------------------------------------------------
// Login: Finish
// ---------------------------------------------------------------------------

pub async fn login_finish(
    State(state): State<AppState>,
    session: Session,
    Json(auth): Json<PublicKeyCredential>,
) -> impl IntoResponse {
    let auth_state: PasskeyAuthentication = match session.remove(WEBAUTHN_AUTH_STATE_KEY).await {
        Ok(Some(s)) => s,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "error": "진행 중인 인증이 없습니다",
                })),
            );
        }
    };

    let auth_result = match state.webauthn.finish_passkey_authentication(&auth, &auth_state) {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "error": format!("Authentication failed: {e}"),
                })),
            );
        }
    };

    // Find the user by credential. The auth_result gives us the credential ID used.
    let cred_id_bytes = auth_result.cred_id().to_vec();
    // Look up which user owns this credential by checking all stored credentials
    let rows: Vec<(Uuid, serde_json::Value)> = sqlx::query_as(
        "SELECT user_id, credential FROM webauthn_credentials",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let user_id = rows.iter().find_map(|(uid, cred_json)| {
        let pk: Passkey = serde_json::from_value(cred_json.clone()).ok()?;
        if pk.cred_id().to_vec() == cred_id_bytes {
            Some(*uid)
        } else {
            None
        }
    });

    let user_id = match user_id {
        Some(id) => id,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "error": "인증 정보를 찾을 수 없습니다",
                })),
            );
        }
    };

    // Fetch user and create session
    let user: Option<(Uuid, Option<String>, Option<String>, bominal_types::enums::UserRole, bominal_types::enums::KycLevel)> =
        sqlx::query_as(
            "SELECT id, email, name, role, kyc_level FROM users WHERE id = $1 AND is_active = true",
        )
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(None);

    let (id, email, name, role, kyc_level) = match user {
        Some(u) => u,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "error": "사용자를 찾을 수 없거나 비활성 상태입니다",
                })),
            );
        }
    };

    let auth_user = AuthUser {
        id,
        email: email.unwrap_or_default(),
        name: name.unwrap_or_default(),
        role,
        kyc_level,
        tenant_id: None,
        person_id: None,
        provider_id: None,
    };

    if let Err(e) = session.insert(SESSION_USER_KEY, &auth_user).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "error": format!("Session error: {e}"),
            })),
        );
    }

    (StatusCode::OK, Json(serde_json::json!({ "success": true, "data": auth_user })))
}
