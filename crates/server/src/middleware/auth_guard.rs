// =============================================================================
// Auth Guard Middleware — role-based route protection
// Ported from packages/auth/src/middleware.ts
// =============================================================================

use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

use bominal_types::enums::UserRole;
use bominal_types::rbac::{Action, Resource, Scope, has_permission};

use crate::auth::extractor::{AuthUser, SESSION_USER_KEY};

/// Middleware that requires authentication.
/// Returns 401 if no valid session exists.
pub async fn require_auth(request: Request, next: Next) -> Response {
    let session = request.extensions().get::<Session>();
    let user: Option<AuthUser> = match session {
        Some(s) => s.get(SESSION_USER_KEY).await.unwrap_or(None),
        None => None,
    };

    if user.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "success": false,
                "data": null,
                "error": "Unauthenticated",
            })),
        )
            .into_response();
    }

    next.run(request).await
}

/// Creates a middleware layer that checks if the user has one of the specified roles.
pub fn require_role(
    allowed_roles: &'static [UserRole],
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone
       + Send {
    move |request: Request, next: Next| {
        Box::pin(async move {
            let session = request.extensions().get::<Session>();
            let user: Option<AuthUser> = match session {
                Some(s) => s.get(SESSION_USER_KEY).await.unwrap_or(None),
                None => None,
            };

            match user {
                None => (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({
                        "success": false,
                        "data": null,
                        "error": "Unauthenticated",
                    })),
                )
                    .into_response(),
                Some(u) if !allowed_roles.contains(&u.role) => (
                    StatusCode::FORBIDDEN,
                    Json(serde_json::json!({
                        "success": false,
                        "data": null,
                        "error": format!("Forbidden: role {} is not allowed", u.role),
                    })),
                )
                    .into_response(),
                _ => next.run(request).await,
            }
        })
    }
}

/// Checks whether the user has a specific permission.
/// Used in handlers rather than middleware for fine-grained control.
pub fn check_permission(
    user: &AuthUser,
    resource: Resource,
    action: Action,
    scope: Option<Scope>,
) -> Result<(), Response> {
    if has_permission(user.role, resource, action, scope) {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "success": false,
                "data": null,
                "error": format!("Forbidden: missing {} on {}", action, resource),
            })),
        )
            .into_response())
    }
}
