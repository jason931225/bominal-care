// =============================================================================
// Permission checking — replaces require_roles with RBAC-based authorization
// =============================================================================

use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::rbac::{Action, Resource, Scope, has_permission};

use super::extractor::{AuthError, AuthUser};

/// Check that the user has the required permission.
/// Returns the scope at which the permission was granted, so the caller
/// can enforce ownership / org boundaries.
pub fn require_permission(
    user: &AuthUser,
    resource: Resource,
    action: Action,
) -> Result<Scope, AuthError> {
    // Try scopes in order of most-restrictive first
    for scope in &[Scope::Own, Scope::Assigned, Scope::Linked, Scope::Org, Scope::All] {
        if has_permission(user.role, resource, action, Some(*scope)) {
            return Ok(*scope);
        }
    }
    tracing::warn!(
        user_id = %user.id,
        role = ?user.role,
        resource = %resource,
        action = %action,
        "Forbidden: insufficient permissions"
    );
    Err(AuthError::Forbidden)
}

/// Verify that the user is allowed to access the given entity based on scope.
/// - Own: user's person_id must match entity_person_id
/// - Assigned: user must be linked to the entity (via care plan, family, etc.)
/// - Linked: user must be linked via institution (handoff / provider)
/// - Org: user's provider_id must match entity's provider (checked via DB)
/// - All: no restriction
pub async fn require_ownership(
    user: &AuthUser,
    entity_person_id: Uuid,
    scope: Scope,
    pool: &PgPool,
) -> Result<(), AuthError> {
    match scope {
        Scope::All => Ok(()),
        Scope::Own => {
            if user.person_id == Some(entity_person_id) {
                Ok(())
            } else {
                tracing::warn!(
                    user_id = %user.id,
                    entity_person_id = %entity_person_id,
                    "Ownership check failed: not own resource"
                );
                Err(AuthError::Forbidden)
            }
        }
        Scope::Assigned => {
            // Check family_relationships or care_plan assignments
            let linked = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(
                    SELECT 1 FROM family_relationships
                    WHERE family_person_id = (SELECT id FROM person_profiles WHERE user_id = $1 LIMIT 1)
                      AND senior_person_id = $2
                    UNION ALL
                    SELECT 1 FROM visits v
                    JOIN care_plans cp ON cp.id = v.care_plan_id
                    WHERE v.caregiver_id = (SELECT id FROM caregiver_applications WHERE user_id = $1 LIMIT 1)
                      AND cp.senior_id = $2
                )"
            )
            .bind(user.id)
            .bind(entity_person_id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Ownership query failed");
                AuthError::Forbidden
            })?;

            if linked {
                Ok(())
            } else {
                Err(AuthError::Forbidden)
            }
        }
        Scope::Linked | Scope::Org => {
            // Check provider organization linkage
            let provider_id = user.provider_id.ok_or(AuthError::Forbidden)?;
            let linked = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(
                    SELECT 1 FROM care_plans
                    WHERE senior_id = $1 AND provider_id = $2
                )"
            )
            .bind(entity_person_id)
            .bind(provider_id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Org ownership query failed");
                AuthError::Forbidden
            })?;

            if linked {
                Ok(())
            } else {
                Err(AuthError::Forbidden)
            }
        }
    }
}

/// Helper to check permission and return a standard API error response.
pub fn check_permission(
    user: &AuthUser,
    resource: Resource,
    action: Action,
) -> Result<Scope, (StatusCode, String)> {
    require_permission(user, resource, action).map_err(|_| {
        (
            StatusCode::FORBIDDEN,
            "권한이 부족합니다.".to_string(),
        )
    })
}
