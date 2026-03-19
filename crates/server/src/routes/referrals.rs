// =============================================================================
// Referral Routes — GET /referrals, POST /referrals
// =============================================================================

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, middleware::validate::ValidatedJson, AppState};
use bominal_db::queries::referral;
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};
use bominal_types::inputs::ReferralInput;
use bominal_types::rbac::{Resource, Action};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_referrals).post(create_referral))
}

#[derive(Debug, Deserialize)]
struct ReferralQueryParams {
    #[serde(flatten)]
    pagination: PaginationParams,
    from_provider_id: Option<Uuid>,
    to_provider_id: Option<Uuid>,
    senior_person_id: Option<Uuid>,
}

/// GET /api/referrals?page=1&limit=20
async fn list_referrals(
    State(state): State<AppState>,
    user: AuthUser,
    Query(query): Query<ReferralQueryParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Referral, Action::List) {
        return e.into_response();
    }

    let params = PaginationParams::new(query.pagination.page, query.pagination.limit);

    let filters = referral::ReferralFilters {
        from_provider_id: query.from_provider_id,
        to_provider_id: query.to_provider_id,
        senior_person_id: query.senior_person_id,
    };

    match referral::list_referrals(&state.pool, &filters, params.limit, params.offset()).await {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing referrals: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response()
        }
    }
}

/// POST /api/referrals
async fn create_referral(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<ReferralInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::Referral, Action::Create) {
        return e.into_response();
    }

    let data = referral::CreateReferralData {
        from_provider_id: input.from_provider_id,
        to_provider_id: input.to_provider_id,
        senior_person_id: input.senior_person_id,
        reason: Some(input.reason),
        notes: input.notes,
    };

    match referral::create_referral(&state.pool, &data).await {
        Ok(created) => (StatusCode::CREATED, Json(ApiResponse::success(created))).into_response(),
        Err(e) => {
            tracing::error!("DB error creating referral: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("Internal server error")),
            )
                .into_response()
        }
    }
}
