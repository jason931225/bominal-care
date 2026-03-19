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

    /// Simplified constructor for Phase 1 (CSR API-only mode).
    /// Creates placeholder webauthn/oauth instances. Demo auth bypasses these.
    pub fn new_simple(pool: PgPool) -> Self {
        let rp_id = std::env::var("WEBAUTHN_RP_ID")
            .unwrap_or_else(|_| "localhost".to_string());
        let rp_origin_str = std::env::var("WEBAUTHN_RP_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        // webauthn_rs::prelude re-exports Url
        let rp_origin = webauthn_rs::prelude::Url::parse(&rp_origin_str)
            .expect("Invalid WEBAUTHN_RP_ORIGIN URL");

        let webauthn = webauthn_rs::WebauthnBuilder::new(&rp_id, &rp_origin)
            .expect("Failed to create WebauthnBuilder")
            .build()
            .expect("Failed to build Webauthn");

        let oauth = OAuthProviders::from_env();

        Self {
            pool,
            webauthn: Arc::new(webauthn),
            oauth: Arc::new(oauth),
        }
    }
}
