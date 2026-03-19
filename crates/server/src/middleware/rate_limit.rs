// =============================================================================
// Rate Limiting — per-endpoint-group limits using governor
// =============================================================================

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use once_cell::sync::Lazy;
use std::num::NonZeroU32;
use std::sync::Arc;

type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Global rate limiter: 200 requests per 60 seconds (IP-level fallback).
static GLOBAL_LIMITER: Lazy<Arc<Limiter>> = Lazy::new(|| {
    Arc::new(RateLimiter::direct(
        Quota::per_minute(NonZeroU32::new(200).unwrap()),
    ))
});

/// Rate limiter for auth endpoints: 5 per 60 seconds.
static AUTH_LIMITER: Lazy<Arc<Limiter>> = Lazy::new(|| {
    Arc::new(RateLimiter::direct(
        Quota::per_minute(NonZeroU32::new(5).unwrap()),
    ))
});

/// Rate limiter for write endpoints: 30 per 60 seconds.
static WRITE_LIMITER: Lazy<Arc<Limiter>> = Lazy::new(|| {
    Arc::new(RateLimiter::direct(
        Quota::per_minute(NonZeroU32::new(30).unwrap()),
    ))
});

/// Global rate limiting middleware.
pub async fn global_rate_limit(request: Request, next: Next) -> Response {
    if GLOBAL_LIMITER.check().is_err() {
        return rate_limit_response();
    }
    next.run(request).await
}

/// Auth-specific rate limiting middleware (applied to /api/auth/* routes).
pub async fn auth_rate_limit(request: Request, next: Next) -> Response {
    if AUTH_LIMITER.check().is_err() {
        return rate_limit_response();
    }
    next.run(request).await
}

/// Write-operation rate limiting middleware (applied to POST/PATCH/DELETE routes).
pub async fn write_rate_limit(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    if method == axum::http::Method::GET || method == axum::http::Method::HEAD {
        return next.run(request).await;
    }
    if WRITE_LIMITER.check().is_err() {
        return rate_limit_response();
    }
    next.run(request).await
}

fn rate_limit_response() -> Response {
    let body = serde_json::json!({
        "success": false,
        "data": null,
        "error": "요청이 너무 많습니다. 잠시 후 다시 시도해 주세요.",
    });
    (StatusCode::TOO_MANY_REQUESTS, axum::Json(body)).into_response()
}
