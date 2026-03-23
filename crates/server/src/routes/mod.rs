// =============================================================================
// API Routes — combines all sub-routers into a single Router<AppState>
// =============================================================================

mod allergies;
mod appointments;
mod audit;
mod availability;
mod benefits;
mod care_plans;
mod care_summary;
mod caregivers;
mod clinical;
mod community;
mod consent;
mod credentials;
mod dashboard;
mod dispensing;
mod emergency;
mod gov;
mod handoff;
mod incidents;
mod lab_results;
mod match_requests;
mod medical_history;
mod medications;
mod notifications;
mod observability;
mod prescriptions;
mod profile;
mod providers;
mod referrals;
mod system_metrics;
mod visits;
mod wellness;

use axum::Router;
use crate::AppState;

/// Returns the combined API router for all `/api/*` endpoints.
pub fn api_router() -> Router<AppState> {
    Router::new()
        .nest("/profile", profile::router())
        .nest("/medications", medications::router())
        .nest("/appointments", appointments::router())
        .nest("/consent", consent::router())
        .nest("/notifications", notifications::router())
        .nest("/care-plans", care_plans::router())
        .nest("/visits", visits::router())
        .nest("/medical-history", medical_history::router())
        .nest("/match-requests", match_requests::router())
        .nest("/caregiver-applications", caregivers::router())
        .nest("/referrals", referrals::router())
        .nest("/audit-logs", audit::router())
        .nest("/observability", observability::router())
        .nest("/providers", providers::router())
        .nest("/incidents", incidents::router())
        .nest("/gov", gov::router())
        .nest("/dashboard", dashboard::router())
        .nest("/credentials", credentials::router())
        .nest("/wellness", wellness::router())
        .nest("/emergency", emergency::router())
        .nest("/care-summaries", care_summary::router())
        .nest("/handoff", handoff::router())
        .nest("/prescriptions", prescriptions::router())
        .nest("/clinical", clinical::router())
        .nest("/lab-results", lab_results::router())
        .nest("/allergies", allergies::router())
        .nest("/availability", availability::router())
        .nest("/dispensing", dispensing::router())
        .nest("/benefits", benefits::router())
        .nest("/community", community::router())
        .nest("/system", system_metrics::router())
}
