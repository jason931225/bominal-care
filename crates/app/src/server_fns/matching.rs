// =============================================================================
// Match Request Server Functions
// Wraps bominal_db::queries::match_request for Leptos SSR
// =============================================================================

use leptos::prelude::*;
use leptos::server;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bominal_types::enums::{Gender, ServiceCategory};
use bominal_types::inputs::MatchRequestInput;
use bominal_types::models::{MatchRecommendation, MatchRequest};

// ---------------------------------------------------------------------------
// Response types (serializable across the wire)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRequestListResponse {
    pub data: Vec<MatchRequest>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationWithApplicationResponse {
    pub recommendation: MatchRecommendation,
    pub caregiver_application: bominal_types::models::CaregiverApplication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRequestDetailResponse {
    pub match_request: MatchRequest,
    pub recommendations: Vec<RecommendationWithApplicationResponse>,
}

// ---------------------------------------------------------------------------
// Server functions
// ---------------------------------------------------------------------------

/// Paginated list of match requests with optional senior filter.
#[server]
pub async fn list_match_requests(
    senior_id: Option<Uuid>,
    page: i64,
    limit: i64,
) -> Result<MatchRequestListResponse, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let params = bominal_types::common::PaginationParams::new(page, limit);

    let result = bominal_db::queries::match_request::list_match_requests(
        &pool,
        senior_id,
        params.limit,
        params.offset(),
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(MatchRequestListResponse {
        data: result.data,
        total: result.total,
    })
}

/// Create a new match request.
#[server]
pub async fn create_match_request(
    input: MatchRequestInput,
) -> Result<MatchRequest, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let session = use_context::<bominal_types::inputs::SessionUser>()
        .ok_or_else(|| ServerFnError::new("Not authenticated"))?;

    let data = bominal_db::queries::match_request::CreateMatchRequestData {
        senior_id: input.senior_id,
        requested_by: session.id,
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
    };

    bominal_db::queries::match_request::create_match_request(&pool, &data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get a single match request with its recommendations.
#[server]
pub async fn get_match_request(
    id: Uuid,
) -> Result<Option<MatchRequestDetailResponse>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let result = bominal_db::queries::match_request::get_match_request(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(result.map(|r| MatchRequestDetailResponse {
        match_request: r.match_request,
        recommendations: r
            .recommendations
            .into_iter()
            .map(|rec| RecommendationWithApplicationResponse {
                recommendation: rec.recommendation,
                caregiver_application: rec.caregiver_application,
            })
            .collect(),
    }))
}

/// Trigger candidate search for a match request.
/// Transitions the request to SEARCHING, scores candidates, and returns recommendations.
#[server]
pub async fn search_candidates(
    match_request_id: Uuid,
) -> Result<Vec<MatchRecommendation>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    bominal_db::queries::match_request::search_candidates(&pool, match_request_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Select a recommendation for a match request.
/// Transitions the request to SELECTED.
#[server]
pub async fn select_recommendation(
    match_request_id: Uuid,
    recommendation_id: Uuid,
) -> Result<MatchRecommendation, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    // Verify the recommendation belongs to the specified match request
    let detail = bominal_db::queries::match_request::get_match_request(&pool, match_request_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Match request not found"))?;

    let belongs = detail
        .recommendations
        .iter()
        .any(|r| r.recommendation.id == recommendation_id);

    if !belongs {
        return Err(ServerFnError::new(
            "Recommendation does not belong to this match request",
        ));
    }

    bominal_db::queries::match_request::select_recommendation(&pool, recommendation_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
