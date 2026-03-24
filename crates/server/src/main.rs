// =============================================================================
// bominal-server -- Axum REST API + static file serving for CSR app
// =============================================================================

use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    http::{header, HeaderValue, Method, StatusCode},
    Router,
};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, services::ServeDir};
use tower_sessions::SessionManagerLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use bominal_server::{
    auth::demo::{demo_login, logout},
    auth::extractor::AuthUser,
    auth::oauth::{oauth_callback, oauth_start},
    auth::webauthn::{login_finish, login_start, register_finish, register_start},
    routes::api_router,
    AppState, PgSessionStore,
};

#[tokio::main]
async fn main() {
    // Load .env if present
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bominal_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Validate required env vars in production
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let is_prod = env == "production";
    if is_prod {
        for var in ["DATABASE_URL", "WEBAUTHN_RP_ID", "WEBAUTHN_RP_ORIGIN"] {
            if std::env::var(var).is_err() {
                tracing::error!("Required env var {var} is not set");
                std::process::exit(1);
            }
        }
    }

    // Database pool
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("../db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database connected and migrations applied");

    // Session store (PostgreSQL-backed, survives restarts)
    let session_store = PgSessionStore::new(pool.clone());

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(is_prod)
        .with_same_site(tower_sessions::cookie::SameSite::Lax);

    // Application state
    let state = AppState::new_simple(pool);

    // CORS — env-based origins in production, localhost in dev
    let cors = build_cors(is_prod);

    // Auth rate limit: 5 requests per 60 seconds (per router group).
    // NOTE: tower's RateLimitLayer is a global token bucket (per-service),
    // not per-IP. For per-IP rate limiting, consider the `governor` crate
    // with `tower-governor` middleware.
    // HandleErrorLayer converts Buffer/RateLimit errors into HTTP 429 responses.
    // BufferLayer is required because RateLimit<S> does not implement Clone,
    // but Axum routers require Clone on service layers.
    let auth_rate_limit = || {
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: tower::BoxError| async {
                StatusCode::TOO_MANY_REQUESTS
            }))
            .layer(BufferLayer::new(32))
            .layer(RateLimitLayer::new(5, Duration::from_secs(60)))
    };

    // Auth routes
    let auth_routes = Router::new()
        .route("/demo", axum::routing::post(demo_login))
        .route("/logout", axum::routing::post(logout))
        .route("/me", axum::routing::get(get_me))
        .layer(auth_rate_limit());

    // WebAuthn routes
    let webauthn_routes = Router::new()
        .route("/register/start", axum::routing::post(register_start))
        .route("/register/finish", axum::routing::post(register_finish))
        .route("/login/start", axum::routing::post(login_start))
        .route("/login/finish", axum::routing::post(login_finish))
        .layer(auth_rate_limit());

    // OAuth routes
    let oauth_routes = Router::new()
        .route("/{provider}/start", axum::routing::get(oauth_start))
        .route("/{provider}/callback", axum::routing::get(oauth_callback))
        .layer(auth_rate_limit());

    // Health endpoints (no auth required)
    let health_routes = Router::new()
        .route("/health", axum::routing::get(health))
        .route("/readiness", axum::routing::get(readiness));

    // Serve static files with SPA fallback (DIST_DIR env or ./dist)
    let dist_dir = std::env::var("DIST_DIR").unwrap_or_else(|_| "dist".to_string());
    let index_path = format!("{}/index.html", &dist_dir);
    let serve_dir = ServeDir::new(&dist_dir).fallback(
        tower_http::services::ServeFile::new(index_path),
    );

    // Global rate limit: 100 requests per second.
    // NOTE: This is a global token bucket (per-service), not per-IP.
    // For per-IP rate limiting, consider the `governor` crate with
    // `tower-governor` middleware.
    let global_rate_limit = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: tower::BoxError| async {
            StatusCode::TOO_MANY_REQUESTS
        }))
        .layer(BufferLayer::new(256))
        .layer(RateLimitLayer::new(100, Duration::from_secs(1)));

    // Assemble the full router
    let app = Router::new()
        .merge(health_routes)
        .nest("/api", api_router())
        .nest("/api/auth", auth_routes)
        .nest("/api/auth/webauthn", webauthn_routes)
        .nest("/api/auth/oauth", oauth_routes)
        .fallback_service(serve_dir)
        .layer(global_rate_limit)
        .layer(cors)
        .layer(session_layer)
        .layer(CompressionLayer::new())
        .with_state(state);

    // Start server with graceful shutdown
    let bind_addr = std::env::var("PORT")
        .map(|p| format!("0.0.0.0:{p}"))
        .or_else(|_| std::env::var("BIND_ADDR"))
        .unwrap_or_else(|_| "0.0.0.0:4001".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind listener");

    tracing::info!("Listening on http://{bind_addr}");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// GET /api/auth/me -- returns the current session user
async fn get_me(user: AuthUser) -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "success": true,
        "data": user,
    }))
}

/// GET /health -- simple liveness check
async fn health() -> &'static str {
    "OK"
}

/// GET /readiness -- checks DB connectivity
async fn readiness(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => (axum::http::StatusCode::OK, "READY"),
        Err(_) => (axum::http::StatusCode::SERVICE_UNAVAILABLE, "NOT READY"),
    }
}

fn build_cors(is_prod: bool) -> CorsLayer {
    let origins: Vec<HeaderValue> = if is_prod {
        std::env::var("CORS_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.trim().parse::<HeaderValue>().ok())
            .collect()
    } else {
        vec![
            "http://localhost:4000".parse().unwrap(),
            "http://127.0.0.1:4000".parse().unwrap(),
        ]
    };

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::COOKIE])
        .allow_credentials(true)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { tracing::info!("Received Ctrl+C, shutting down"); }
        _ = terminate => { tracing::info!("Received SIGTERM, shutting down"); }
    }
}
