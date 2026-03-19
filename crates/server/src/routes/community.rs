// =============================================================================
// Community Routes — /api/community
// =============================================================================

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{auth::extractor::AuthUser, AppState};
use bominal_types::ApiResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/activities", get(activities))
        .route("/holidays", get(holidays))
        .route("/alerts", get(alerts))
}

async fn activities(
    State(state): State<AppState>,
    _user: AuthUser,
) -> impl IntoResponse {
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(a)), '[]'::json)
         FROM (SELECT * FROM community_activities WHERE is_active = TRUE AND start_time >= NOW() ORDER BY start_time LIMIT 20) a"
    )
    .fetch_one(&state.pool)
    .await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn holidays(
    State(state): State<AppState>,
    _user: AuthUser,
) -> impl IntoResponse {
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(h)), '[]'::json)
         FROM (SELECT * FROM korean_holidays WHERE holiday_date >= CURRENT_DATE ORDER BY holiday_date LIMIT 15) h"
    )
    .fetch_one(&state.pool)
    .await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}

async fn alerts(
    State(state): State<AppState>,
    _user: AuthUser,
) -> impl IntoResponse {
    match sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT COALESCE(json_agg(row_to_json(a)), '[]'::json)
         FROM (SELECT * FROM seasonal_alerts WHERE is_active = TRUE AND active_from <= CURRENT_DATE AND active_until >= CURRENT_DATE ORDER BY severity DESC) a"
    )
    .fetch_one(&state.pool)
    .await {
        Ok(data) => Json(ApiResponse::success(data)).into_response(),
        Err(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error("서버 오류"))).into_response()
        }
    }
}
