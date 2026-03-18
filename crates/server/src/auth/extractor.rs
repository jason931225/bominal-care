// =============================================================================
// AuthUser extractor — reads session to get authenticated user
// =============================================================================

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use uuid::Uuid;

use bominal_types::enums::{KycLevel, UserRole};

/// The session key where we store the authenticated user.
pub const SESSION_USER_KEY: &str = "auth_user";

/// Authenticated user extracted from the session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub kyc_level: KycLevel,
    pub tenant_id: Option<Uuid>,
}

/// Error returned when authentication fails.
#[derive(Debug)]
pub enum AuthError {
    /// No session or session expired
    Unauthenticated,
    /// Session exists but user data is corrupted
    InvalidSession,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::Unauthenticated => (StatusCode::UNAUTHORIZED, "Unauthenticated"),
            AuthError::InvalidSession => (StatusCode::UNAUTHORIZED, "Invalid session"),
        };
        let body = serde_json::json!({
            "success": false,
            "data": null,
            "error": message,
        });
        (status, axum::Json(body)).into_response()
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError::Unauthenticated)?;

        let user: AuthUser = session
            .get(SESSION_USER_KEY)
            .await
            .map_err(|_| AuthError::InvalidSession)?
            .ok_or(AuthError::Unauthenticated)?;

        Ok(user)
    }
}

/// Optional auth extractor — returns None instead of 401.
#[derive(Debug, Clone)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for MaybeAuthUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(MaybeAuthUser(Some(user))),
            Err(_) => Ok(MaybeAuthUser(None)),
        }
    }
}
