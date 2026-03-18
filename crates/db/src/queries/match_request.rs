// =============================================================================
// Match Request Queries — MatchRequest, scoring, and recommendations
// Ported from packages/db/src/services/match.service.ts
// =============================================================================

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::enums::{Gender, MatchRequestStatus, ServiceCategory};
use bominal_types::models::{CaregiverApplication, MatchRecommendation, MatchRequest};
use bominal_types::state_machines::match_request_machine;

// ---------------------------------------------------------------------------
// Input structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CreateMatchRequestData {
    pub senior_id: Uuid,
    pub requested_by: Uuid,
    pub service_category: ServiceCategory,
    pub region_city: String,
    pub region_district: String,
    pub start_date: Option<chrono::DateTime<Utc>>,
    pub end_date: Option<chrono::DateTime<Utc>>,
    pub schedule_notes: Option<String>,
    pub language_preference: Option<String>,
    pub gender_preference: Option<Gender>,
    pub requires_dementia_experience: bool,
    pub requires_overnight_care: bool,
    pub additional_notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Scoring types — pure, serializable data
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ScoringCriteria {
    pub region_city: String,
    pub region_district: String,
    pub service_category: ServiceCategory,
    pub language_preference: Option<String>,
    pub gender_preference: Option<Gender>,
    pub requires_dementia_experience: bool,
    pub requires_overnight_care: bool,
    pub requested_days: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CandidateSlot {
    pub day_of_week: String,
}

#[derive(Debug, Clone)]
pub struct CandidateServiceType {
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct CandidateRegion {
    pub city: String,
    pub district: String,
}

#[derive(Debug, Clone)]
pub struct CandidateData {
    pub application_id: Uuid,
    pub provider_id: Option<Uuid>,
    pub languages_spoken: String,
    pub preferred_gender: Option<String>,
    pub has_dementia_experience: bool,
    pub has_overnight_availability: bool,
    pub smoking_status: bool,
    pub pet_friendly: bool,
    pub availability_slots: Vec<CandidateSlot>,
    pub service_types: Vec<CandidateServiceType>,
    pub service_regions: Vec<CandidateRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub region_match: f64,
    pub schedule_overlap: f64,
    pub service_type_match: f64,
    pub language_match: f64,
    pub gender_preference_match: f64,
    pub dementia_experience: f64,
    pub mobility_skills: f64,
    pub smoking_pet_preferences: f64,
    pub total: f64,
}

// ---------------------------------------------------------------------------
// Result structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PaginatedMatchRequests {
    pub data: Vec<MatchRequest>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct RecommendationWithApplication {
    pub recommendation: MatchRecommendation,
    pub caregiver_application: CaregiverApplication,
}

#[derive(Debug, Clone)]
pub struct MatchRequestWithRecommendations {
    pub match_request: MatchRequest,
    pub recommendations: Vec<RecommendationWithApplication>,
}

// ---------------------------------------------------------------------------
// Internal row types for joined queries
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, sqlx::FromRow)]
struct SlotRow {
    application_id: Uuid,
    day_of_week: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct ServiceTypeRow {
    application_id: Uuid,
    category: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct RegionRow {
    provider_id: Uuid,
    city: String,
    district: String,
}

// ---------------------------------------------------------------------------
// Pure scoring engine
// ---------------------------------------------------------------------------

pub fn score_candidate(criteria: &ScoringCriteria, candidate: &CandidateData) -> ScoreBreakdown {
    // 1. Region match (required -- 0 if no match): 25 points
    let region_match = if candidate.service_regions.iter().any(|r| {
        r.city == criteria.region_city && r.district == criteria.region_district
    }) {
        25.0
    } else {
        0.0
    };

    // Short-circuit: no region match means candidate is ineligible
    if region_match == 0.0 {
        return ScoreBreakdown {
            region_match: 0.0,
            schedule_overlap: 0.0,
            service_type_match: 0.0,
            language_match: 0.0,
            gender_preference_match: 0.0,
            dementia_experience: 0.0,
            mobility_skills: 0.0,
            smoking_pet_preferences: 0.0,
            total: 0.0,
        };
    }

    // 2. Schedule overlap: 20 points
    let schedule_overlap = if !criteria.requested_days.is_empty() {
        let available_days: std::collections::HashSet<&str> = candidate
            .availability_slots
            .iter()
            .map(|s| s.day_of_week.as_str())
            .collect();
        let matched = criteria
            .requested_days
            .iter()
            .filter(|d| available_days.contains(d.as_str()))
            .count();
        (matched as f64 / criteria.requested_days.len() as f64 * 20.0).round()
    } else {
        20.0
    };

    // 3. Service type match: 15 points
    let service_type_match = if candidate
        .service_types
        .iter()
        .any(|st| st.category == criteria.service_category.to_string())
    {
        15.0
    } else {
        0.0
    };

    // 4. Language match: 10 points
    let language_match = match &criteria.language_preference {
        Some(pref) => {
            let spoken: Vec<String> = candidate
                .languages_spoken
                .split(',')
                .map(|l| l.trim().to_lowercase())
                .collect();
            if spoken.contains(&pref.to_lowercase()) {
                10.0
            } else {
                0.0
            }
        }
        None => 10.0,
    };

    // 5. Gender preference match: 10 points
    let gender_preference_match = match &criteria.gender_preference {
        Some(pref) => {
            let pref_str = pref.to_string();
            match &candidate.preferred_gender {
                None => 10.0,
                Some(cg) if cg == &pref_str => 10.0,
                _ => 0.0,
            }
        }
        None => 10.0,
    };

    // 6. Dementia experience: 10 points
    let dementia_experience = if !criteria.requires_dementia_experience
        || candidate.has_dementia_experience
    {
        10.0
    } else {
        0.0
    };

    // 7. Mobility skills (overnight availability as proxy): 5 points
    let mobility_skills =
        if !criteria.requires_overnight_care || candidate.has_overnight_availability {
            5.0
        } else {
            0.0
        };

    // 8. Smoking / pet preferences: 5 points
    let smoking_pet_preferences = (if !candidate.smoking_status { 2.5 } else { 0.0 })
        + (if candidate.pet_friendly { 2.5 } else { 0.0 });

    let total = region_match
        + schedule_overlap
        + service_type_match
        + language_match
        + gender_preference_match
        + dementia_experience
        + mobility_skills
        + smoking_pet_preferences;

    ScoreBreakdown {
        region_match,
        schedule_overlap,
        service_type_match,
        language_match,
        gender_preference_match,
        dementia_experience,
        mobility_skills,
        smoking_pet_preferences,
        total,
    }
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

pub async fn create_match_request(
    pool: &PgPool,
    data: &CreateMatchRequestData,
) -> Result<MatchRequest, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query_as::<_, MatchRequest>(
        r#"INSERT INTO match_requests (
             id, senior_id, requested_by, status, service_category,
             region_city, region_district, start_date, end_date,
             schedule_notes, language_preference, gender_preference,
             requires_dementia_experience, requires_overnight_care,
             additional_notes, created_at, updated_at
           ) VALUES ($1,$2,$3,'CREATED',$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$15)
           RETURNING *"#,
    )
    .bind(id)
    .bind(data.senior_id)
    .bind(data.requested_by)
    .bind(data.service_category)
    .bind(&data.region_city)
    .bind(&data.region_district)
    .bind(data.start_date)
    .bind(data.end_date)
    .bind(&data.schedule_notes)
    .bind(&data.language_preference)
    .bind(data.gender_preference)
    .bind(data.requires_dementia_experience)
    .bind(data.requires_overnight_care)
    .bind(&data.additional_notes)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn search_candidates(
    pool: &PgPool,
    match_request_id: Uuid,
) -> Result<Vec<MatchRecommendation>, sqlx::Error> {
    // Fetch the match request
    let match_request = sqlx::query_as::<_, MatchRequest>(
        "SELECT * FROM match_requests WHERE id = $1",
    )
    .bind(match_request_id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    // Validate state transition to SEARCHING
    let machine = match_request_machine();
    if !machine.can_transition(match_request.status, MatchRequestStatus::Searching) {
        return Err(sqlx::Error::Protocol(format!(
            "Cannot begin search for match request in status: {}",
            match_request.status
        )));
    }

    // Transition to SEARCHING
    sqlx::query(
        "UPDATE match_requests SET status = 'SEARCHING', updated_at = NOW() WHERE id = $1",
    )
    .bind(match_request_id)
    .execute(pool)
    .await?;

    // Fetch approved caregivers
    let applications = sqlx::query_as::<_, CaregiverApplication>(
        r#"SELECT ca.*
           FROM caregiver_applications ca
           WHERE ca.status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')"#,
    )
    .fetch_all(pool)
    .await?;

    // Fetch availability slots for approved caregivers
    let slots = sqlx::query_as::<_, SlotRow>(
        r#"SELECT application_id, day_of_week::text AS day_of_week
           FROM availability_slots
           WHERE application_id IN (
             SELECT id FROM caregiver_applications
             WHERE status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')
           ) AND is_active = true"#,
    )
    .fetch_all(pool)
    .await?;

    // Fetch service types for approved caregivers
    let services = sqlx::query_as::<_, ServiceTypeRow>(
        r#"SELECT application_id, category::text AS category
           FROM service_types
           WHERE application_id IN (
             SELECT id FROM caregiver_applications
             WHERE status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')
           ) AND is_active = true"#,
    )
    .fetch_all(pool)
    .await?;

    // Fetch service regions via provider organizations
    let regions = sqlx::query_as::<_, RegionRow>(
        r#"SELECT sr.provider_id, sr.city, sr.district
           FROM service_regions sr
           JOIN provider_organizations po ON po.id = sr.provider_id
           JOIN caregiver_applications ca ON ca.provider_id = po.id
           WHERE ca.status IN ('APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER')
             AND sr.is_active = true"#,
    )
    .fetch_all(pool)
    .await?;

    // Group slots by application_id
    let mut slots_by_app: std::collections::HashMap<Uuid, Vec<CandidateSlot>> =
        std::collections::HashMap::new();
    for slot in &slots {
        slots_by_app
            .entry(slot.application_id)
            .or_default()
            .push(CandidateSlot {
                day_of_week: slot.day_of_week.clone(),
            });
    }

    // Group service types by application_id
    let mut services_by_app: std::collections::HashMap<Uuid, Vec<CandidateServiceType>> =
        std::collections::HashMap::new();
    for svc in &services {
        services_by_app
            .entry(svc.application_id)
            .or_default()
            .push(CandidateServiceType {
                category: svc.category.clone(),
            });
    }

    // Group regions by provider_id
    let mut regions_by_provider: std::collections::HashMap<Uuid, Vec<CandidateRegion>> =
        std::collections::HashMap::new();
    for reg in &regions {
        regions_by_provider
            .entry(reg.provider_id)
            .or_default()
            .push(CandidateRegion {
                city: reg.city.clone(),
                district: reg.district.clone(),
            });
    }

    // Build scoring criteria from match request
    let criteria = ScoringCriteria {
        region_city: match_request.region_city.clone(),
        region_district: match_request.region_district.clone(),
        service_category: match_request.service_category,
        language_preference: match_request.language_preference.clone(),
        gender_preference: match_request.gender_preference,
        requires_dementia_experience: match_request.requires_dementia_experience,
        requires_overnight_care: match_request.requires_overnight_care,
        requested_days: Vec::new(),
    };

    // Score each candidate
    let mut scored: Vec<(Uuid, ScoreBreakdown)> = applications
        .iter()
        .map(|app| {
            let candidate = CandidateData {
                application_id: app.id,
                provider_id: app.provider_id,
                languages_spoken: app.languages_spoken.clone(),
                preferred_gender: app.preferred_gender.map(|g| g.to_string()),
                has_dementia_experience: app.has_dementia_experience,
                has_overnight_availability: app.has_overnight_availability,
                smoking_status: app.smoking_status,
                pet_friendly: app.pet_friendly,
                availability_slots: slots_by_app
                    .get(&app.id)
                    .cloned()
                    .unwrap_or_default(),
                service_types: services_by_app
                    .get(&app.id)
                    .cloned()
                    .unwrap_or_default(),
                service_regions: app
                    .provider_id
                    .and_then(|pid| regions_by_provider.get(&pid).cloned())
                    .unwrap_or_default(),
            };
            let breakdown = score_candidate(&criteria, &candidate);
            (app.id, breakdown)
        })
        .filter(|(_, b)| b.total > 0.0)
        .collect();

    // Sort descending by total score
    scored.sort_by(|a, b| b.1.total.partial_cmp(&a.1.total).unwrap_or(std::cmp::Ordering::Equal));

    // Delete previous recommendations
    sqlx::query("DELETE FROM match_recommendations WHERE match_request_id = $1")
        .bind(match_request_id)
        .execute(pool)
        .await?;

    // Persist recommendations in a transaction
    let mut tx = pool.begin().await?;
    let mut recommendations = Vec::new();

    for (rank, (app_id, breakdown)) in scored.iter().enumerate() {
        let rec_id = Uuid::new_v4();
        let score_json = serde_json::to_value(breakdown)
            .unwrap_or(serde_json::Value::Null);
        let rank_val = (rank + 1) as i32;

        let rec = sqlx::query_as::<_, MatchRecommendation>(
            r#"INSERT INTO match_recommendations (
                 id, match_request_id, caregiver_application_id, score,
                 score_breakdown, rank, is_selected, created_at
               ) VALUES ($1,$2,$3,$4,$5,$6,false,NOW())
               RETURNING *"#,
        )
        .bind(rec_id)
        .bind(match_request_id)
        .bind(app_id)
        .bind(breakdown.total)
        .bind(score_json)
        .bind(rank_val)
        .fetch_one(&mut *tx)
        .await?;

        recommendations.push(rec);
    }

    // Transition to RECOMMENDATIONS_READY
    sqlx::query(
        "UPDATE match_requests SET status = 'RECOMMENDATIONS_READY', updated_at = NOW() WHERE id = $1",
    )
    .bind(match_request_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(recommendations)
}

pub async fn select_recommendation(
    pool: &PgPool,
    recommendation_id: Uuid,
) -> Result<MatchRecommendation, sqlx::Error> {
    // Fetch the recommendation
    let recommendation = sqlx::query_as::<_, MatchRecommendation>(
        "SELECT * FROM match_recommendations WHERE id = $1",
    )
    .bind(recommendation_id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    // Fetch the parent match request
    let match_request = sqlx::query_as::<_, MatchRequest>(
        "SELECT * FROM match_requests WHERE id = $1",
    )
    .bind(recommendation.match_request_id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    // Validate state transition
    let machine = match_request_machine();
    if !machine.can_transition(match_request.status, MatchRequestStatus::Selected) {
        return Err(sqlx::Error::Protocol(format!(
            "Cannot select recommendation for match request in status: {}",
            match_request.status
        )));
    }

    // Update in a transaction
    let mut tx = pool.begin().await?;

    let updated = sqlx::query_as::<_, MatchRecommendation>(
        r#"UPDATE match_recommendations
           SET is_selected = true, selected_at = NOW()
           WHERE id = $1
           RETURNING *"#,
    )
    .bind(recommendation_id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE match_requests SET status = 'SELECTED', updated_at = NOW() WHERE id = $1",
    )
    .bind(recommendation.match_request_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(updated)
}

pub async fn get_match_request(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<MatchRequestWithRecommendations>, sqlx::Error> {
    let match_request = sqlx::query_as::<_, MatchRequest>(
        "SELECT * FROM match_requests WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let match_request = match match_request {
        Some(mr) => mr,
        None => return Ok(None),
    };

    // Fetch recommendations with their caregiver applications
    let recs = sqlx::query_as::<_, MatchRecommendation>(
        r#"SELECT * FROM match_recommendations
           WHERE match_request_id = $1
           ORDER BY rank ASC"#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let mut recommendations = Vec::with_capacity(recs.len());
    for rec in recs {
        let app = sqlx::query_as::<_, CaregiverApplication>(
            "SELECT * FROM caregiver_applications WHERE id = $1",
        )
        .bind(rec.caregiver_application_id)
        .fetch_one(pool)
        .await?;

        recommendations.push(RecommendationWithApplication {
            recommendation: rec,
            caregiver_application: app,
        });
    }

    Ok(Some(MatchRequestWithRecommendations {
        match_request,
        recommendations,
    }))
}

pub async fn list_match_requests(
    pool: &PgPool,
    senior_id: Option<Uuid>,
    limit: i64,
    offset: i64,
) -> Result<PaginatedMatchRequests, sqlx::Error> {
    let data = sqlx::query_as::<_, MatchRequest>(
        r#"SELECT * FROM match_requests
           WHERE ($1::uuid IS NULL OR senior_id = $1)
           ORDER BY created_at DESC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(senior_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let row: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM match_requests
           WHERE ($1::uuid IS NULL OR senior_id = $1)"#,
    )
    .bind(senior_id)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedMatchRequests {
        data,
        total: row.0,
    })
}
