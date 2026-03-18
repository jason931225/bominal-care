// =============================================================================
// Application State — shared across all Axum handlers
// =============================================================================

use sqlx::PgPool;
use std::sync::Arc;
use webauthn_rs::Webauthn;

use crate::auth::oauth::OAuthProviders;

/// Shared application state injected into all Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub webauthn: Arc<Webauthn>,
    pub oauth: Arc<OAuthProviders>,
}

impl AppState {
    pub fn new(pool: PgPool, webauthn: Webauthn, oauth: OAuthProviders) -> Self {
        Self {
            pool,
            webauthn: Arc::new(webauthn),
            oauth: Arc::new(oauth),
        }
    }
}
