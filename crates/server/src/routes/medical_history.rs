// =============================================================================
// Medical History Routes — GET /medical-history, POST /medical-history
// =============================================================================

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use crate::middleware::validate::ValidatedJson;
use bominal_db::queries::{medical_history, profile};
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};
use bominal_types::inputs::MedicalHistoryInput;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_entries).post(create_entry))
}

/// GET /api/medical-history?page=1&limit=20
async fn list_entries(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::MedicalHistory, Action::List) {
        return e.into_response();
    }

    let person_id = match resolve_person_id(&state.pool, user.id).await {
        Ok(id) => id,
        Err(resp) => return resp.into_response(),
    };

    let params = PaginationParams::new(params.page, params.limit);

    match medical_history::list_entries(&state.pool, person_id, params.limit, params.offset()).await
    {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing medical history: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/medical-history
async fn create_entry(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<MedicalHistoryInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::MedicalHistory, Action::Create) {
        return e.into_response();
    }

    let data = medical_history::CreateMedicalHistoryEntryData {
        person_id: input.person_id,
        condition: input.condition,
        diagnosed_at: input.diagnosed_at,
        treated_by: input.treated_by,
        status: input.status,
        notes: input.notes,
        created_by: Some(user.id),
    };

    match medical_history::create_entry(&state.pool, &data).await {
        Ok(created) => (StatusCode::CREATED, Json(ApiResponse::success(created))).into_response(),
        Err(e) => {
            tracing::error!("DB error creating medical history entry: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

async fn resolve_person_id(
    pool: &sqlx::PgPool,
    user_id: uuid::Uuid,
) -> Result<uuid::Uuid, (StatusCode, Json<ApiResponse<()>>)> {
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
