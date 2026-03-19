// =============================================================================
// AuthUser extractor — reads session to get authenticated user
// =============================================================================

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use uuid::Uuid;

use bominal_types::enums::{KycLevel, UserRole};

/// Allowed-role lists for route-level authorization.
/// Each constant corresponds to one route group.
pub const ROLES_PROFILE: &[UserRole] = &[
    UserRole::Senior,
    UserRole::Family,
    UserRole::CaregiverApplicant,
    UserRole::CaregiverApproved,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::MedicalStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_MEDICATIONS: &[UserRole] = &[
    UserRole::Senior,
    UserRole::Family,
    UserRole::CaregiverApproved,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::MedicalStaff,
    UserRole::PharmacyStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_APPOINTMENTS: &[UserRole] = &[
    UserRole::Senior,
    UserRole::Family,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::MedicalStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_CONSENT: &[UserRole] = &[
    UserRole::Senior,
    UserRole::Family,
    UserRole::MedicalStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_CARE_PLANS: &[UserRole] = &[
    UserRole::Senior,
    UserRole::Family,
    UserRole::CaregiverApproved,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_VISITS: &[UserRole] = &[
    UserRole::CaregiverApproved,
    UserRole::Family,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_MEDICAL_HISTORY: &[UserRole] = &[
    UserRole::Senior,
    UserRole::Family,
    UserRole::MedicalStaff,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_MATCH_REQUESTS: &[UserRole] = &[
    UserRole::Family,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_CAREGIVER_APPLICATIONS: &[UserRole] = &[
    UserRole::CaregiverApplicant,
    UserRole::CaregiverApproved,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_REFERRALS: &[UserRole] = &[
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::MedicalStaff,
    UserRole::PlatformAdmin,
];

pub const ROLES_AUDIT: &[UserRole] = &[
    UserRole::GovernmentReviewer,
    UserRole::ProviderAdmin,
    UserRole::PlatformAdmin,
];

pub const ROLES_NOTIFICATIONS: &[UserRole] = &[
    UserRole::Senior,
    UserRole::Family,
    UserRole::CaregiverApplicant,
    UserRole::CaregiverApproved,
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::MedicalStaff,
    UserRole::GovernmentReviewer,
    UserRole::PlatformAdmin,
];

pub const ROLES_OBSERVABILITY: &[UserRole] = &[
    UserRole::ProviderAdmin,
    UserRole::ProviderStaff,
    UserRole::GovernmentReviewer,
    UserRole::PlatformAdmin,
];

/// The session key where we store the authenticated user.
pub const SESSION_USER_KEY: &str = "auth_user";

/// Authenticated user extracted from the session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub kyc_level: KycLevel,
    pub tenant_id: Option<Uuid>,
    /// The person_profile.id linked to this user (for ownership checks).
    #[serde(default)]
    pub person_id: Option<Uuid>,
    /// The provider_organization.id this user belongs to (for org-scope checks).
    #[serde(default)]
    pub provider_id: Option<Uuid>,
}

/// Error returned when authentication or authorization fails.
#[derive(Debug)]
pub enum AuthError {
    /// No session or session expired
    Unauthenticated,
    /// Session exists but user data is corrupted
    InvalidSession,
    /// User authenticated but lacks required role
    Forbidden,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::Unauthenticated => (StatusCode::UNAUTHORIZED, "Unauthenticated"),
            AuthError::InvalidSession => (StatusCode::UNAUTHORIZED, "Invalid session"),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "Insufficient permissions"),
        };
        let body = serde_json::json!({
            "success": false,
            "data": null,
            "error": message,
        });
        (status, axum::Json(body)).into_response()
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError::Unauthenticated)?;

        let user: AuthUser = session
            .get(SESSION_USER_KEY)
            .await
            .map_err(|_| AuthError::InvalidSession)?
            .ok_or(AuthError::Unauthenticated)?;

        Ok(user)
    }
}

/// Optional auth extractor — returns None instead of 401.
#[derive(Debug, Clone)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for MaybeAuthUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(MaybeAuthUser(Some(user))),
            Err(_) => Ok(MaybeAuthUser(None)),
        }
    }
}

/// Check if the authenticated user has one of the required roles.
/// Returns `Ok(())` when the role is in the allowed list, or
/// `Err(AuthError::Forbidden)` otherwise.
pub fn require_roles(user: &AuthUser, allowed: &[UserRole]) -> Result<(), AuthError> {
    if allowed.contains(&user.role) {
        Ok(())
    } else {
        tracing::warn!(
            user_id = %user.id,
            role = ?user.role,
            "Forbidden: role not in allowed list"
        );
        Err(AuthError::Forbidden)
    }
}
