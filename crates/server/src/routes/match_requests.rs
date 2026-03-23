// =============================================================================
// Match Request Routes — GET /match-requests, POST /match-requests
// =============================================================================

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{auth::{extractor::AuthUser, permission::require_permission}, middleware::validate::ValidatedJson, AppState};
use bominal_db::queries::match_request;
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};
use bominal_types::inputs::MatchRequestInput;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_match_requests).post(create_match_request))
}

/// GET /api/match-requests?page=1&limit=20
async fn list_match_requests(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::MatchRequest, Action::List) {
        return e.into_response();
    }

    let params = PaginationParams::new(params.page, params.limit);

    match match_request::list_match_requests(&state.pool, None, params.limit, params.offset())
        .await
    {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing match requests: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/match-requests
async fn create_match_request(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<MatchRequestInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::MatchRequest, Action::Create) {
        return e.into_response();
    }

    let data = match_request::CreateMatchRequestData {
        senior_id: input.senior_id,
        requested_by: user.id,
        service_category: input.service_category,
        region_city: input.region_city,
        region_district: input.region_district,
        start_date: Some(input.start_date),
        end_date: input.end_date,
        schedule_notes: input.schedule_notes,
        language_preference: input.language_preference,
        gender_preference: input.gender_preference,
        requires_dementia_experience: input.requires_dementia_experience.unwrap_or(false),
        requires_overnight_care: input.requires_overnight_care.unwrap_or(false),
        additional_notes: input.additional_notes,
        schedule: input.requested_schedule.unwrap_or_default(),
    };

    let created = match match_request::create_match_request(&state.pool, &data).await {
        Ok(mr) => mr,
        Err(e) => {
            tracing::error!("DB error creating match request: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response();
        }
    };

    // Auto-run matching after creation
    let match_request_id = created.id;
    match match_request::search_candidates(&state.pool, match_request_id).await {
        Ok(recommendations) => {
            // Re-fetch the match request with its final status
            match match_request::get_match_request(&state.pool, match_request_id).await {
                Ok(Some(result)) => {
                    (StatusCode::CREATED, Json(ApiResponse::success(result))).into_response()
                }
                Ok(None) => {
                    // Shouldn't happen — we just created it
                    (StatusCode::CREATED, Json(ApiResponse::success(serde_json::json!({
                        "match_request": created,
                        "recommendations": recommendations,
                    })))).into_response()
                }
                Err(e) => {
                    tracing::error!("DB error fetching match request after search: {e}");
                    (StatusCode::CREATED, Json(ApiResponse::success(serde_json::json!({
                        "match_request": created,
                        "recommendations": recommendations,
                    })))).into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Auto-matching failed for match request {match_request_id}: {e}");
            // Still return the created match request — matching can be retried
            (StatusCode::CREATED, Json(ApiResponse::success(created))).into_response()
        }
    }
}
