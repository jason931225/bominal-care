// =============================================================================
// Consent Routes — GET /consent, POST /consent, DELETE /consent/:id
// =============================================================================

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use crate::middleware::validate::ValidatedJson;
use bominal_db::queries::{consent, profile};
use bominal_types::ApiResponse;
use bominal_types::inputs::ConsentInput;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_consents).post(grant_consent))
        .route("/{id}", delete(revoke_consent))
}

/// GET /api/consent
async fn list_consents(
    State(state): State<AppState>,
    user: AuthUser,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Consent, Action::List) {
        return e.into_response();
    }

    let person_id = match resolve_person_id(&state.pool, user.id).await {
        Ok(id) => id,
        Err(resp) => return resp.into_response(),
    };

    match consent::get_consents_for_person(&state.pool, person_id).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error listing consents: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/consent
async fn grant_consent(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<ConsentInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Consent, Action::Create) {
        return e.into_response();
    }

    let data = consent::GrantConsentData {
        subject_person_id: input.subject_person_id,
        purpose: input.purpose,
        granted_by: user.id,
        expires_at: input.expires_at,
    };

    match consent::grant_consent(&state.pool, &data).await {
        Ok(created) => (StatusCode::CREATED, Json(ApiResponse::success(created))).into_response(),
        Err(e) => {
            tracing::error!("DB error granting consent: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// DELETE /api/consent/:id
async fn revoke_consent(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Consent, Action::Delete) {
        return e.into_response();
    }

    match consent::revoke_consent(&state.pool, id, user.id).await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error revoking consent: {e}");
            let (status, msg) = match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "동의를 찾을 수 없습니다"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "서버 오류"),
            };
            (status, Json(ApiResponse::<()>::error(msg))).into_response()
        }
    }
}

async fn resolve_person_id(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Uuid, (StatusCode, Json<ApiResponse<()>>)> {
    match profile::get_person_profile_by_user_id(pool, user_id).await {
        Ok(Some(p)) => Ok(p.id),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("프로필을 찾을 수 없습니다")),
        )),
        Err(e) => {
            tracing::error!("DB error resolving person_id: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            ))
        }
    }
}
