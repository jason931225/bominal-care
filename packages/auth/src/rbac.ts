type UserRole =
  | 'SENIOR'
  | 'FAMILY'
  | 'CAREGIVER_APPLICANT'
  | 'CAREGIVER_APPROVED'
  | 'PROVIDER_ADMIN'
  | 'PROVIDER_STAFF'
  | 'MEDICAL_STAFF'
  | 'GOVERNMENT_REVIEWER'
  | 'PARTNER_OPERATOR'
  | 'PLATFORM_ADMIN';

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

export enum Resource {
  PROFILE = 'PROFILE',
  SENIOR_PROFILE = 'SENIOR_PROFILE',
  CARE_PLAN = 'CARE_PLAN',
  VISIT = 'VISIT',
  MEDICATION = 'MEDICATION',
  MEDICAL_HISTORY = 'MEDICAL_HISTORY',
  MATCH_REQUEST = 'MATCH_REQUEST',
  PROVIDER = 'PROVIDER',
  CAREGIVER_APPLICATION = 'CAREGIVER_APPLICATION',
  ELIGIBILITY_CASE = 'ELIGIBILITY_CASE',
  OBSERVABILITY = 'OBSERVABILITY',
  NOTIFICATION = 'NOTIFICATION',
  CONSENT = 'CONSENT',
  AUDIT_LOG = 'AUDIT_LOG',
  REFERRAL = 'REFERRAL',
  APPOINTMENT = 'APPOINTMENT',
}

export enum Action {
  CREATE = 'CREATE',
  READ = 'READ',
  UPDATE = 'UPDATE',
  DELETE = 'DELETE',
  LIST = 'LIST',
  APPROVE = 'APPROVE',
  REJECT = 'REJECT',
}

export enum Scope {
  OWN = 'OWN',
  ASSIGNED = 'ASSIGNED',
  ORG = 'ORG',
  ALL = 'ALL',
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface Permission {
  resource: Resource;
  action: Action;
  scope: Scope;
}

// ---------------------------------------------------------------------------
// Role → Permission mappings
// ---------------------------------------------------------------------------

const p = (resource: Resource, action: Action, scope: Scope): Permission => ({
  resource,
  action,
  scope,
});

export const ROLE_PERMISSIONS: Record<UserRole, Permission[]> = {
  SENIOR: [
    // Own profile
    p(Resource.PROFILE, Action.READ, Scope.OWN),
    p(Resource.PROFILE, Action.UPDATE, Scope.OWN),
    p(Resource.SENIOR_PROFILE, Action.READ, Scope.OWN),
    p(Resource.SENIOR_PROFILE, Action.UPDATE, Scope.OWN),
    // Own medications
    p(Resource.MEDICATION, Action.READ, Scope.OWN),
    p(Resource.MEDICATION, Action.LIST, Scope.OWN),
    // Own medical history (read-only)
    p(Resource.MEDICAL_HISTORY, Action.READ, Scope.OWN),
    p(Resource.MEDICAL_HISTORY, Action.LIST, Scope.OWN),
    // Care plan (read-only)
    p(Resource.CARE_PLAN, Action.READ, Scope.OWN),
    p(Resource.CARE_PLAN, Action.LIST, Scope.OWN),
    // Consent management
    p(Resource.CONSENT, Action.CREATE, Scope.OWN),
    p(Resource.CONSENT, Action.READ, Scope.OWN),
    p(Resource.CONSENT, Action.UPDATE, Scope.OWN),
    p(Resource.CONSENT, Action.DELETE, Scope.OWN),
    p(Resource.CONSENT, Action.LIST, Scope.OWN),
    // Appointments
    p(Resource.APPOINTMENT, Action.CREATE, Scope.OWN),
    p(Resource.APPOINTMENT, Action.READ, Scope.OWN),
    p(Resource.APPOINTMENT, Action.UPDATE, Scope.OWN),
    p(Resource.APPOINTMENT, Action.LIST, Scope.OWN),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
  ],

  FAMILY: [
    // Own profile
    p(Resource.PROFILE, Action.READ, Scope.OWN),
    p(Resource.PROFILE, Action.UPDATE, Scope.OWN),
    // Delegated senior data (assigned = linked senior)
    p(Resource.SENIOR_PROFILE, Action.READ, Scope.ASSIGNED),
    p(Resource.MEDICATION, Action.READ, Scope.ASSIGNED),
    p(Resource.MEDICATION, Action.LIST, Scope.ASSIGNED),
    p(Resource.MEDICAL_HISTORY, Action.READ, Scope.ASSIGNED),
    p(Resource.MEDICAL_HISTORY, Action.LIST, Scope.ASSIGNED),
    p(Resource.CARE_PLAN, Action.READ, Scope.ASSIGNED),
    p(Resource.CARE_PLAN, Action.LIST, Scope.ASSIGNED),
    p(Resource.VISIT, Action.READ, Scope.ASSIGNED),
    p(Resource.VISIT, Action.LIST, Scope.ASSIGNED),
    // Match requests on behalf of senior
    p(Resource.MATCH_REQUEST, Action.CREATE, Scope.ASSIGNED),
    p(Resource.MATCH_REQUEST, Action.READ, Scope.ASSIGNED),
    p(Resource.MATCH_REQUEST, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.MATCH_REQUEST, Action.LIST, Scope.ASSIGNED),
    // Consent approval on behalf of senior
    p(Resource.CONSENT, Action.APPROVE, Scope.ASSIGNED),
    p(Resource.CONSENT, Action.READ, Scope.ASSIGNED),
    p(Resource.CONSENT, Action.LIST, Scope.ASSIGNED),
    // Appointments
    p(Resource.APPOINTMENT, Action.CREATE, Scope.ASSIGNED),
    p(Resource.APPOINTMENT, Action.READ, Scope.ASSIGNED),
    p(Resource.APPOINTMENT, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.APPOINTMENT, Action.LIST, Scope.ASSIGNED),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
  ],

  CAREGIVER_APPLICANT: [
    // Own profile
    p(Resource.PROFILE, Action.READ, Scope.OWN),
    p(Resource.PROFILE, Action.UPDATE, Scope.OWN),
    // Own caregiver application and credentials
    p(Resource.CAREGIVER_APPLICATION, Action.CREATE, Scope.OWN),
    p(Resource.CAREGIVER_APPLICATION, Action.READ, Scope.OWN),
    p(Resource.CAREGIVER_APPLICATION, Action.UPDATE, Scope.OWN),
    p(Resource.CAREGIVER_APPLICATION, Action.LIST, Scope.OWN),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
  ],

  CAREGIVER_APPROVED: [
    // Own profile
    p(Resource.PROFILE, Action.READ, Scope.OWN),
    p(Resource.PROFILE, Action.UPDATE, Scope.OWN),
    // Own application
    p(Resource.CAREGIVER_APPLICATION, Action.READ, Scope.OWN),
    p(Resource.CAREGIVER_APPLICATION, Action.UPDATE, Scope.OWN),
    // Assigned clients
    p(Resource.SENIOR_PROFILE, Action.READ, Scope.ASSIGNED),
    p(Resource.CARE_PLAN, Action.READ, Scope.ASSIGNED),
    p(Resource.CARE_PLAN, Action.LIST, Scope.ASSIGNED),
    // Visits
    p(Resource.VISIT, Action.READ, Scope.ASSIGNED),
    p(Resource.VISIT, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.VISIT, Action.LIST, Scope.ASSIGNED),
    // Medications (assigned clients, read-only + event recording)
    p(Resource.MEDICATION, Action.READ, Scope.ASSIGNED),
    p(Resource.MEDICATION, Action.LIST, Scope.ASSIGNED),
    p(Resource.MEDICATION, Action.UPDATE, Scope.ASSIGNED),
    // Observability
    p(Resource.OBSERVABILITY, Action.CREATE, Scope.ASSIGNED),
    p(Resource.OBSERVABILITY, Action.READ, Scope.ASSIGNED),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
  ],

  PROVIDER_ADMIN: [
    // Own org profile management
    p(Resource.PROVIDER, Action.READ, Scope.OWN),
    p(Resource.PROVIDER, Action.UPDATE, Scope.OWN),
    // Org caregivers
    p(Resource.CAREGIVER_APPLICATION, Action.READ, Scope.ORG),
    p(Resource.CAREGIVER_APPLICATION, Action.LIST, Scope.ORG),
    p(Resource.CAREGIVER_APPLICATION, Action.APPROVE, Scope.ORG),
    p(Resource.CAREGIVER_APPLICATION, Action.REJECT, Scope.ORG),
    p(Resource.CAREGIVER_APPLICATION, Action.UPDATE, Scope.ORG),
    // Org clients and senior profiles
    p(Resource.SENIOR_PROFILE, Action.READ, Scope.ORG),
    p(Resource.SENIOR_PROFILE, Action.LIST, Scope.ORG),
    p(Resource.PROFILE, Action.READ, Scope.ORG),
    p(Resource.PROFILE, Action.LIST, Scope.ORG),
    // Care plans
    p(Resource.CARE_PLAN, Action.CREATE, Scope.ORG),
    p(Resource.CARE_PLAN, Action.READ, Scope.ORG),
    p(Resource.CARE_PLAN, Action.UPDATE, Scope.ORG),
    p(Resource.CARE_PLAN, Action.DELETE, Scope.ORG),
    p(Resource.CARE_PLAN, Action.LIST, Scope.ORG),
    // Visits
    p(Resource.VISIT, Action.CREATE, Scope.ORG),
    p(Resource.VISIT, Action.READ, Scope.ORG),
    p(Resource.VISIT, Action.UPDATE, Scope.ORG),
    p(Resource.VISIT, Action.LIST, Scope.ORG),
    // Referrals
    p(Resource.REFERRAL, Action.CREATE, Scope.ORG),
    p(Resource.REFERRAL, Action.READ, Scope.ORG),
    p(Resource.REFERRAL, Action.UPDATE, Scope.ORG),
    p(Resource.REFERRAL, Action.LIST, Scope.ORG),
    p(Resource.REFERRAL, Action.APPROVE, Scope.ORG),
    // Observability
    p(Resource.OBSERVABILITY, Action.READ, Scope.ORG),
    p(Resource.OBSERVABILITY, Action.LIST, Scope.ORG),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
    p(Resource.NOTIFICATION, Action.CREATE, Scope.ORG),
  ],

  PROVIDER_STAFF: [
    // Own profile
    p(Resource.PROFILE, Action.READ, Scope.OWN),
    p(Resource.PROFILE, Action.UPDATE, Scope.OWN),
    // Assigned clients
    p(Resource.SENIOR_PROFILE, Action.READ, Scope.ASSIGNED),
    p(Resource.PROFILE, Action.READ, Scope.ASSIGNED),
    // Care plans (assigned)
    p(Resource.CARE_PLAN, Action.READ, Scope.ASSIGNED),
    p(Resource.CARE_PLAN, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.CARE_PLAN, Action.LIST, Scope.ASSIGNED),
    // Visits (assigned)
    p(Resource.VISIT, Action.CREATE, Scope.ASSIGNED),
    p(Resource.VISIT, Action.READ, Scope.ASSIGNED),
    p(Resource.VISIT, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.VISIT, Action.LIST, Scope.ASSIGNED),
    // Referrals (read assigned)
    p(Resource.REFERRAL, Action.READ, Scope.ASSIGNED),
    p(Resource.REFERRAL, Action.LIST, Scope.ASSIGNED),
    // Observability
    p(Resource.OBSERVABILITY, Action.CREATE, Scope.ASSIGNED),
    p(Resource.OBSERVABILITY, Action.READ, Scope.ASSIGNED),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
  ],

  MEDICAL_STAFF: [
    // Own profile
    p(Resource.PROFILE, Action.READ, Scope.OWN),
    p(Resource.PROFILE, Action.UPDATE, Scope.OWN),
    // Medical history (with consent)
    p(Resource.MEDICAL_HISTORY, Action.CREATE, Scope.ASSIGNED),
    p(Resource.MEDICAL_HISTORY, Action.READ, Scope.ASSIGNED),
    p(Resource.MEDICAL_HISTORY, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.MEDICAL_HISTORY, Action.LIST, Scope.ASSIGNED),
    // Medications (assigned)
    p(Resource.MEDICATION, Action.CREATE, Scope.ASSIGNED),
    p(Resource.MEDICATION, Action.READ, Scope.ASSIGNED),
    p(Resource.MEDICATION, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.MEDICATION, Action.LIST, Scope.ASSIGNED),
    // Referrals
    p(Resource.REFERRAL, Action.CREATE, Scope.ASSIGNED),
    p(Resource.REFERRAL, Action.READ, Scope.ASSIGNED),
    p(Resource.REFERRAL, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.REFERRAL, Action.LIST, Scope.ASSIGNED),
    // Appointments
    p(Resource.APPOINTMENT, Action.CREATE, Scope.ASSIGNED),
    p(Resource.APPOINTMENT, Action.READ, Scope.ASSIGNED),
    p(Resource.APPOINTMENT, Action.UPDATE, Scope.ASSIGNED),
    p(Resource.APPOINTMENT, Action.LIST, Scope.ASSIGNED),
    // Consent (read to verify access)
    p(Resource.CONSENT, Action.READ, Scope.ASSIGNED),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
  ],

  GOVERNMENT_REVIEWER: [
    // Read-only on eligibility cases
    p(Resource.ELIGIBILITY_CASE, Action.READ, Scope.ALL),
    p(Resource.ELIGIBILITY_CASE, Action.LIST, Scope.ALL),
    p(Resource.ELIGIBILITY_CASE, Action.UPDATE, Scope.ALL),
    p(Resource.ELIGIBILITY_CASE, Action.APPROVE, Scope.ALL),
    p(Resource.ELIGIBILITY_CASE, Action.REJECT, Scope.ALL),
    // Provider registry
    p(Resource.PROVIDER, Action.READ, Scope.ALL),
    p(Resource.PROVIDER, Action.LIST, Scope.ALL),
    // Observability (read-only)
    p(Resource.OBSERVABILITY, Action.READ, Scope.ALL),
    p(Resource.OBSERVABILITY, Action.LIST, Scope.ALL),
    // Audit logs (read-only)
    p(Resource.AUDIT_LOG, Action.READ, Scope.ALL),
    p(Resource.AUDIT_LOG, Action.LIST, Scope.ALL),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
  ],

  PARTNER_OPERATOR: [
    // Own provider profile
    p(Resource.PROVIDER, Action.READ, Scope.OWN),
    p(Resource.PROVIDER, Action.UPDATE, Scope.OWN),
    // Own services
    p(Resource.CARE_PLAN, Action.READ, Scope.OWN),
    p(Resource.CARE_PLAN, Action.LIST, Scope.OWN),
    p(Resource.REFERRAL, Action.READ, Scope.OWN),
    p(Resource.REFERRAL, Action.LIST, Scope.OWN),
    // Notifications
    p(Resource.NOTIFICATION, Action.READ, Scope.OWN),
    p(Resource.NOTIFICATION, Action.LIST, Scope.OWN),
    p(Resource.NOTIFICATION, Action.UPDATE, Scope.OWN),
  ],

  PLATFORM_ADMIN: [
    // Full access to all resources
    ...Object.values(Resource).flatMap((resource) =>
      Object.values(Action).map((action) => p(resource, action, Scope.ALL)),
    ),
  ],
};

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/**
 * Returns true if the given role has a permission matching the resource,
 * action, and optional scope. When scope is omitted, any scope matches.
 * PLATFORM_ADMIN always returns true.
 */
export function hasPermission(
  role: UserRole,
  resource: Resource,
  action: Action,
  scope?: Scope,
): boolean {
  const permissions = ROLE_PERMISSIONS[role] ?? [];
  return permissions.some(
    (perm) =>
      perm.resource === resource &&
      perm.action === action &&
      (scope === undefined || perm.scope === scope || perm.scope === Scope.ALL),
  );
}

/**
 * Returns all permissions for the given role.
 */
export function getAllPermissions(role: UserRole): Permission[] {
  return ROLE_PERMISSIONS[role] ?? [];
}
