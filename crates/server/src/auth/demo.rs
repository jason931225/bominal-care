// =============================================================================
// Demo Login — development only (#[cfg(debug_assertions)])
// Route: POST /api/auth/demo
// =============================================================================

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use tower_sessions::Session;
use uuid::Uuid;

use crate::AppState;
use super::extractor::{AuthUser, SESSION_USER_KEY};

#[derive(Deserialize)]
pub struct DemoLoginRequest {
    pub email: String,
}

/// Dev-only demo login that looks up a user by email and creates a session.
#[cfg(debug_assertions)]
pub async fn demo_login(
    State(state): State<AppState>,
    session: Session,
    Json(req): Json<DemoLoginRequest>,
) -> impl IntoResponse {
    let user: Option<(
        Uuid,
        Option<String>,
        Option<String>,
        bominal_types::enums::UserRole,
        bominal_types::enums::KycLevel,
    )> = sqlx::query_as(
        "SELECT id, email, name, role, kyc_level FROM users WHERE email = $1 AND is_active = true",
    )
    .bind(&req.email)
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
                    "error": "User not found or inactive",
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

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "data": auth_user,
        })),
    )
}

/// Logout — clears the session.
pub async fn logout(session: Session) -> impl IntoResponse {
    let _ = session.flush().await;
    (
        StatusCode::OK,
        Json(serde_json::json!({ "success": true })),
    )
}
