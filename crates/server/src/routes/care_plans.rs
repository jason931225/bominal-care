// =============================================================================
// Care Plan Routes — CRUD for /care-plans
// =============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use uuid::Uuid;

use crate::{auth::{extractor::AuthUser, permission::require_permission}, AppState};
use crate::middleware::validate::ValidatedJson;
use bominal_db::queries::{care_plan, platform_event, profile};
use bominal_types::inputs::{CarePlanInput, UpdateCarePlanInput};
use bominal_types::rbac::{Resource, Action};
use bominal_types::{ApiResponse, PaginationMeta, PaginationParams};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_care_plans).post(create_care_plan))
        .route("/{id}", get(get_care_plan).patch(update_care_plan))
}

/// GET /api/care-plans?page=1&limit=20
async fn list_care_plans(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CarePlan, Action::List) {
        return e.into_response();
    }

    let person_id = match resolve_person_id(&state.pool, user.id).await {
        Ok(id) => id,
        Err(resp) => return resp.into_response(),
    };

    let params = PaginationParams::new(params.page, params.limit);

    match care_plan::list_care_plans(&state.pool, person_id, params.limit, params.offset()).await {
        Ok(result) => {
            let meta = PaginationMeta::new(result.total, params.page, params.limit);
            Json(ApiResponse::success_with_meta(result.data, meta)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing care plans: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// POST /api/care-plans
async fn create_care_plan(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(input): ValidatedJson<CarePlanInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CarePlan, Action::Create) {
        return e.into_response();
    }

    let data = care_plan::CreateCarePlanData {
        senior_id: input.senior_id,
        provider_id: input.provider_id,
        title: input.title,
        description: input.description,
        start_date: input.start_date,
        end_date: input.end_date,
        goals: input.goals,
        created_by: Some(user.id),
    };

    match care_plan::create_care_plan(&state.pool, &data).await {
        Ok(created) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "care_plan",
                created.id,
                "created",
                "internal",
                "care_operations",
                None,
                None,
                None,
                None,
                None,
            )
            .await;
            (StatusCode::CREATED, Json(ApiResponse::success(created))).into_response()
        }
        Err(e) => {
            tracing::error!("DB error creating care plan: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// GET /api/care-plans/:id
async fn get_care_plan(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CarePlan, Action::Read) {
        return e.into_response();
    }

    match care_plan::get_care_plan(&state.pool, id).await {
        Ok(Some(data)) => Json(ApiResponse::success(data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error("케어플랜을 찾을 수 없습니다")),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("DB error fetching care plan: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error("서버 오류")),
            )
                .into_response()
        }
    }
}

/// PATCH /api/care-plans/:id
async fn update_care_plan(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(input): ValidatedJson<UpdateCarePlanInput>,
) -> impl IntoResponse {
    if let Err(e) = require_permission(&user, Resource::CarePlan, Action::Update) {
        return e.into_response();
    }

    let data = care_plan::UpdateCarePlanData {
        title: input.title,
        description: input.description,
        start_date: input.start_date,
        end_date: input.end_date,
        goals: input.goals,
        provider_id: input.provider_id,
        updated_by: None,
    };

    match care_plan::update_care_plan(&state.pool, id, &data).await {
        Ok(updated) => {
            let _ = platform_event::insert_event(
                &state.pool,
                Some(user.id),
                Some(&user.role.to_string()),
                None,
                "care_plan",
                id,
                "updated",
                "internal",
                "care_operations",
                None,
                None,
                None,
                None,
                None,
            )
            .await;
            Json(ApiResponse::success(updated)).into_response()
        }
        Err(e) => {
            tracing::error!("DB error updating care plan: {e}");
            let (status, msg) = match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "케어플랜을 찾을 수 없습니다"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "서버 오류"),
            };
            (
                status,
                Json(ApiResponse::<()>::error(msg)),
            )
                .into_response()
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
