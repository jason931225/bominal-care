// =============================================================================
// CSRF Protection — double-submit cookie pattern
// =============================================================================

use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

const CSRF_HEADER: &str = "x-csrf-token";
const CSRF_COOKIE: &str = "csrf_token";

/// Middleware that validates CSRF token on state-changing methods.
/// Uses the double-submit cookie pattern: the client reads a CSRF token
/// from a cookie and sends it back in the X-CSRF-Token header.
/// GET/HEAD/OPTIONS are exempt.
pub async fn csrf_protection(request: Request, next: Next) -> Response {
    let method = request.method().clone();

    // Safe methods don't need CSRF validation
    if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS {
        return next.run(request).await;
    }

    // Extract the CSRF token from the header
    let header_token = request
        .headers()
        .get(CSRF_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    // Extract the CSRF token from the cookie
    let cookie_token = request
        .headers()
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .map(str::trim)
                .find(|c| c.starts_with(CSRF_COOKIE))
                .and_then(|c| c.split_once('='))
                .map(|(_, v)| v.to_string())
        });

    match (header_token, cookie_token) {
        (Some(header), Some(cookie)) if constant_time_eq(header.as_bytes(), cookie.as_bytes()) => {
            next.run(request).await
        }
        (None, None) => {
            // No CSRF tokens present — first request or no cookies yet, allow
            // (the auth extractor will catch unauthenticated users)
            next.run(request).await
        }
        _ => {
            let body = serde_json::json!({
                "success": false,
                "data": null,
                "error": "CSRF 토큰이 유효하지 않습니다.",
            });
            (StatusCode::FORBIDDEN, axum::Json(body)).into_response()
        }
    }
}

/// Constant-time comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}
