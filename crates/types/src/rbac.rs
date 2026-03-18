// =============================================================================
// RBAC — 10 roles × 16 resources × 7 actions × 4 scopes
// Ported from packages/auth/src/rbac.ts (360 lines)
// =============================================================================

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::enums::UserRole;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Resource {
    Profile,
    SeniorProfile,
    CarePlan,
    Visit,
    Medication,
    MedicalHistory,
    MatchRequest,
    Provider,
    CaregiverApplication,
    EligibilityCase,
    Observability,
    Notification,
    Consent,
    AuditLog,
    Referral,
    Appointment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    List,
    Approve,
    Reject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Scope {
    Own,
    Assigned,
    Org,
    All,
}

// ---------------------------------------------------------------------------
// Permission
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Permission {
    pub resource: Resource,
    pub action: Action,
    pub scope: Scope,
}

const fn p(resource: Resource, action: Action, scope: Scope) -> Permission {
    Permission {
        resource,
        action,
        scope,
    }
}

// ---------------------------------------------------------------------------
// Role → Permission mappings
// ---------------------------------------------------------------------------

static ROLE_PERMISSIONS: Lazy<HashMap<UserRole, Vec<Permission>>> = Lazy::new(|| {
    use Action::*;
    use Resource::*;
    use Scope::*;

    let mut map = HashMap::new();

    // --- SENIOR ---
    map.insert(
        UserRole::Senior,
        vec![
            p(Profile, Read, Own),
            p(Profile, Update, Own),
            p(SeniorProfile, Read, Own),
            p(SeniorProfile, Update, Own),
            p(Medication, Read, Own),
            p(Medication, List, Own),
            p(MedicalHistory, Read, Own),
            p(MedicalHistory, List, Own),
            p(CarePlan, Read, Own),
            p(CarePlan, List, Own),
            p(Consent, Create, Own),
            p(Consent, Read, Own),
            p(Consent, Update, Own),
            p(Consent, Delete, Own),
            p(Consent, List, Own),
            p(Appointment, Create, Own),
            p(Appointment, Read, Own),
            p(Appointment, Update, Own),
            p(Appointment, List, Own),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
        ],
    );

    // --- FAMILY ---
    map.insert(
        UserRole::Family,
        vec![
            p(Profile, Read, Own),
            p(Profile, Update, Own),
            p(SeniorProfile, Read, Assigned),
            p(Medication, Read, Assigned),
            p(Medication, List, Assigned),
            p(MedicalHistory, Read, Assigned),
            p(MedicalHistory, List, Assigned),
            p(CarePlan, Read, Assigned),
            p(CarePlan, List, Assigned),
            p(Visit, Read, Assigned),
            p(Visit, List, Assigned),
            p(MatchRequest, Create, Assigned),
            p(MatchRequest, Read, Assigned),
            p(MatchRequest, Update, Assigned),
            p(MatchRequest, List, Assigned),
            p(Consent, Approve, Assigned),
            p(Consent, Read, Assigned),
            p(Consent, List, Assigned),
            p(Appointment, Create, Assigned),
            p(Appointment, Read, Assigned),
            p(Appointment, Update, Assigned),
            p(Appointment, List, Assigned),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
        ],
    );

    // --- CAREGIVER_APPLICANT ---
    map.insert(
        UserRole::CaregiverApplicant,
        vec![
            p(Profile, Read, Own),
            p(Profile, Update, Own),
            p(CaregiverApplication, Create, Own),
            p(CaregiverApplication, Read, Own),
            p(CaregiverApplication, Update, Own),
            p(CaregiverApplication, List, Own),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
        ],
    );

    // --- CAREGIVER_APPROVED ---
    map.insert(
        UserRole::CaregiverApproved,
        vec![
            p(Profile, Read, Own),
            p(Profile, Update, Own),
            p(CaregiverApplication, Read, Own),
            p(CaregiverApplication, Update, Own),
            p(SeniorProfile, Read, Assigned),
            p(CarePlan, Read, Assigned),
            p(CarePlan, List, Assigned),
            p(Visit, Read, Assigned),
            p(Visit, Update, Assigned),
            p(Visit, List, Assigned),
            p(Medication, Read, Assigned),
            p(Medication, List, Assigned),
            p(Medication, Update, Assigned),
            p(Observability, Create, Assigned),
            p(Observability, Read, Assigned),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
        ],
    );

    // --- PROVIDER_ADMIN ---
    map.insert(
        UserRole::ProviderAdmin,
        vec![
            p(Provider, Read, Own),
            p(Provider, Update, Own),
            p(CaregiverApplication, Read, Org),
            p(CaregiverApplication, List, Org),
            p(CaregiverApplication, Approve, Org),
            p(CaregiverApplication, Reject, Org),
            p(CaregiverApplication, Update, Org),
            p(SeniorProfile, Read, Org),
            p(SeniorProfile, List, Org),
            p(Profile, Read, Org),
            p(Profile, List, Org),
            p(CarePlan, Create, Org),
            p(CarePlan, Read, Org),
            p(CarePlan, Update, Org),
            p(CarePlan, Delete, Org),
            p(CarePlan, List, Org),
            p(Visit, Create, Org),
            p(Visit, Read, Org),
            p(Visit, Update, Org),
            p(Visit, List, Org),
            p(Referral, Create, Org),
            p(Referral, Read, Org),
            p(Referral, Update, Org),
            p(Referral, List, Org),
            p(Referral, Approve, Org),
            p(Observability, Read, Org),
            p(Observability, List, Org),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
            p(Notification, Create, Org),
        ],
    );

    // --- PROVIDER_STAFF ---
    map.insert(
        UserRole::ProviderStaff,
        vec![
            p(Profile, Read, Own),
            p(Profile, Update, Own),
            p(SeniorProfile, Read, Assigned),
            p(Profile, Read, Assigned),
            p(CarePlan, Read, Assigned),
            p(CarePlan, Update, Assigned),
            p(CarePlan, List, Assigned),
            p(Visit, Create, Assigned),
            p(Visit, Read, Assigned),
            p(Visit, Update, Assigned),
            p(Visit, List, Assigned),
            p(Referral, Read, Assigned),
            p(Referral, List, Assigned),
            p(Observability, Create, Assigned),
            p(Observability, Read, Assigned),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
        ],
    );

    // --- MEDICAL_STAFF ---
    map.insert(
        UserRole::MedicalStaff,
        vec![
            p(Profile, Read, Own),
            p(Profile, Update, Own),
            p(MedicalHistory, Create, Assigned),
            p(MedicalHistory, Read, Assigned),
            p(MedicalHistory, Update, Assigned),
            p(MedicalHistory, List, Assigned),
            p(Medication, Create, Assigned),
            p(Medication, Read, Assigned),
            p(Medication, Update, Assigned),
            p(Medication, List, Assigned),
            p(Referral, Create, Assigned),
            p(Referral, Read, Assigned),
            p(Referral, Update, Assigned),
            p(Referral, List, Assigned),
            p(Appointment, Create, Assigned),
            p(Appointment, Read, Assigned),
            p(Appointment, Update, Assigned),
            p(Appointment, List, Assigned),
            p(Consent, Read, Assigned),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
        ],
    );

    // --- GOVERNMENT_REVIEWER ---
    map.insert(
        UserRole::GovernmentReviewer,
        vec![
            p(EligibilityCase, Read, All),
            p(EligibilityCase, List, All),
            p(EligibilityCase, Update, All),
            p(EligibilityCase, Approve, All),
            p(EligibilityCase, Reject, All),
            p(Provider, Read, All),
            p(Provider, List, All),
            p(Observability, Read, All),
            p(Observability, List, All),
            p(AuditLog, Read, All),
            p(AuditLog, List, All),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
        ],
    );

    // --- PARTNER_OPERATOR ---
    map.insert(
        UserRole::PartnerOperator,
        vec![
            p(Provider, Read, Own),
            p(Provider, Update, Own),
            p(CarePlan, Read, Own),
            p(CarePlan, List, Own),
            p(Referral, Read, Own),
            p(Referral, List, Own),
            p(Notification, Read, Own),
            p(Notification, List, Own),
            p(Notification, Update, Own),
        ],
    );

    // --- PLATFORM_ADMIN — full access ---
    let admin_perms: Vec<Permission> = Resource::iter()
        .flat_map(|resource| Action::iter().map(move |action| p(resource, action, All)))
        .collect();
    map.insert(UserRole::PlatformAdmin, admin_perms);

    map
});

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Returns true if the given role has a permission matching the resource,
/// action, and optional scope. When scope is None, any scope matches.
/// PLATFORM_ADMIN always returns true.
pub fn has_permission(
    role: UserRole,
    resource: Resource,
    action: Action,
    scope: Option<Scope>,
) -> bool {
    let permissions = match ROLE_PERMISSIONS.get(&role) {
        Some(p) => p,
        None => return false,
    };
    permissions.iter().any(|perm| {
        perm.resource == resource
            && perm.action == action
            && match scope {
                None => true,
                Some(s) => perm.scope == s || perm.scope == Scope::All,
            }
    })
}

/// Returns all permissions for the given role.
pub fn all_permissions(role: UserRole) -> &'static [Permission] {
    ROLE_PERMISSIONS
        .get(&role)
        .map_or(&[], |v| v.as_slice())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn senior_can_read_own_profile() {
        assert!(has_permission(
            UserRole::Senior,
            Resource::Profile,
            Action::Read,
            Some(Scope::Own),
        ));
    }

    #[test]
    fn senior_cannot_delete_care_plan() {
        assert!(!has_permission(
            UserRole::Senior,
            Resource::CarePlan,
            Action::Delete,
            None,
        ));
    }

    #[test]
    fn family_can_create_match_request_assigned() {
        assert!(has_permission(
            UserRole::Family,
            Resource::MatchRequest,
            Action::Create,
            Some(Scope::Assigned),
        ));
    }

    #[test]
    fn family_cannot_create_match_request_all() {
        assert!(!has_permission(
            UserRole::Family,
            Resource::MatchRequest,
            Action::Create,
            Some(Scope::All),
        ));
    }

    #[test]
    fn provider_admin_can_approve_applications() {
        assert!(has_permission(
            UserRole::ProviderAdmin,
            Resource::CaregiverApplication,
            Action::Approve,
            Some(Scope::Org),
        ));
    }

    #[test]
    fn government_reviewer_can_read_all_eligibility() {
        assert!(has_permission(
            UserRole::GovernmentReviewer,
            Resource::EligibilityCase,
            Action::Read,
            Some(Scope::All),
        ));
    }

    #[test]
    fn government_reviewer_cannot_create_care_plan() {
        assert!(!has_permission(
            UserRole::GovernmentReviewer,
            Resource::CarePlan,
            Action::Create,
            None,
        ));
    }

    #[test]
    fn platform_admin_full_access() {
        // Admin can do anything on any resource
        assert!(has_permission(
            UserRole::PlatformAdmin,
            Resource::AuditLog,
            Action::Delete,
            Some(Scope::All),
        ));
        assert!(has_permission(
            UserRole::PlatformAdmin,
            Resource::CarePlan,
            Action::Create,
            Some(Scope::All),
        ));
    }

    #[test]
    fn caregiver_applicant_limited_access() {
        assert!(has_permission(
            UserRole::CaregiverApplicant,
            Resource::CaregiverApplication,
            Action::Create,
            Some(Scope::Own),
        ));
        assert!(!has_permission(
            UserRole::CaregiverApplicant,
            Resource::CarePlan,
            Action::Read,
            None,
        ));
    }

    #[test]
    fn scope_none_matches_any() {
        // When scope is None, any scope should match
        assert!(has_permission(
            UserRole::Senior,
            Resource::Profile,
            Action::Read,
            None,
        ));
    }
}
