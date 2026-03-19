// =============================================================================
// Profile Routes — GET /profile/me, PATCH /profile
// =============================================================================

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use bominal_db::queries::profile;
use bominal_types::ApiResponse;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_my_profile))
        .route("/", patch(update_my_profile))
}

/// GET /api/profile/me
async fn get_my_profile(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Profile, Action::Read) {
        return e.into_response();
    }

    match profile::get_person_profile_by_user_id(&state.pool, user.id).await {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("Profile not found")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error fetching profile: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response()
        }
    }
}

/// PATCH /api/profile
async fn update_my_profile(
    State(state): State<AppState>,
    user: AuthUser,
    Json(data): Json<profile::UpdatePersonProfileData>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Profile, Action::Update) {
        return e.into_response();
    }

    // First look up the user's person_profile to get the profile ID
    let existing = match profile::get_person_profile_by_user_id(&state.pool, user.id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::<()>::error("Profile not found")),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("DB error looking up profile: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response();
        }
    };

    match profile::update_person_profile(&state.pool, existing.id, &data).await {
        Ok(updated) => Json(ApiResponse::success(updated)).into_response(),
        Err(e) => {
            tracing::error!("DB error updating profile: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response()
        }
    }
}
