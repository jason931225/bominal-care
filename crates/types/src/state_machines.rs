// =============================================================================
// Lightweight State Machine — 8 domain workflows
// Ported from packages/db/src/services/state-machine.ts
// =============================================================================

use std::collections::HashMap;

use crate::enums::{
    AppointmentStatus, CarePlanStatus, CaregiverApplicationStatus, EligibilityCaseStatus,
    InstitutionReferralStatus, MatchRequestStatus, MedicationEventStatus, VisitStatus,
};

// ---------------------------------------------------------------------------
// Core state machine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct StateMachine<S: Copy + Eq + std::hash::Hash> {
    transitions: HashMap<S, Vec<S>>,
}

impl<S: Copy + Eq + std::hash::Hash> StateMachine<S> {
    pub fn new(transitions: HashMap<S, Vec<S>>) -> Self {
        Self { transitions }
    }

    pub fn can_transition(&self, from: S, to: S) -> bool {
        self.transitions
            .get(&from)
            .is_some_and(|allowed| allowed.contains(&to))
    }

    pub fn valid_transitions(&self, from: S) -> &[S] {
        self.transitions.get(&from).map_or(&[], |v| v.as_slice())
    }
}

/// Adds escape states to every non-escape state in the base map.
fn add_escapes<S: Copy + Eq + std::hash::Hash>(
    base: &mut HashMap<S, Vec<S>>,
    escapes: &[S],
) {
    let keys: Vec<S> = base.keys().copied().collect();
    for state in keys {
        let nexts = base.get_mut(&state).unwrap();
        for &esc in escapes {
            if esc != state && !nexts.contains(&esc) {
                nexts.push(esc);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 1. Caregiver Application
//    DRAFT → SUBMITTED → IDENTITY_VERIFIED → CREDENTIAL_REVIEW
//      → APPROVED_PRIVATE_PAY | APPROVED_UNDER_PROVIDER
//    any → SUSPENDED | REJECTED
// ---------------------------------------------------------------------------

pub fn caregiver_application_machine() -> StateMachine<CaregiverApplicationStatus> {
    use CaregiverApplicationStatus::*;
    let mut m = HashMap::from([
        (Draft, vec![Submitted]),
        (Submitted, vec![IdentityVerified]),
        (IdentityVerified, vec![CredentialReview]),
        (CredentialReview, vec![ApprovedPrivatePay, ApprovedUnderProvider]),
        (ApprovedPrivatePay, vec![]),
        (ApprovedUnderProvider, vec![]),
        (Suspended, vec![]),
        (Rejected, vec![]),
    ]);
    add_escapes(&mut m, &[Suspended, Rejected]);
    StateMachine::new(m)
}

// ---------------------------------------------------------------------------
// 2. Match Request
//    CREATED → SEARCHING → RECOMMENDATIONS_READY → SELECTED → BOOKED → FULFILLED
//    any → CANCELLED
// ---------------------------------------------------------------------------

pub fn match_request_machine() -> StateMachine<MatchRequestStatus> {
    use MatchRequestStatus::*;
    let mut m = HashMap::from([
        (Created, vec![Searching]),
        (Searching, vec![RecommendationsReady]),
        (RecommendationsReady, vec![Selected]),
        (Selected, vec![Booked]),
        (Booked, vec![Fulfilled]),
        (Fulfilled, vec![]),
        (Cancelled, vec![]),
    ]);
    add_escapes(&mut m, &[Cancelled]);
    StateMachine::new(m)
}

// ---------------------------------------------------------------------------
// 3. Visit
//    SCHEDULED → CAREGIVER_ACKNOWLEDGED → IN_PROGRESS → COMPLETED
//    SCHEDULED → MISSED
//    any → CANCELLED
// ---------------------------------------------------------------------------

pub fn visit_machine() -> StateMachine<VisitStatus> {
    use VisitStatus::*;
    let mut m = HashMap::from([
        (Scheduled, vec![CaregiverAcknowledged, Missed]),
        (CaregiverAcknowledged, vec![InProgress]),
        (InProgress, vec![Completed]),
        (Completed, vec![]),
        (Missed, vec![]),
        (Cancelled, vec![]),
    ]);
    add_escapes(&mut m, &[Cancelled]);
    StateMachine::new(m)
}

// ---------------------------------------------------------------------------
// 4. Medication Event
//    SCHEDULED → REMINDER_SENT → TAKEN | MISSED | HELD
//    MISSED → ESCALATED
//    HELD → ESCALATED
// ---------------------------------------------------------------------------

pub fn medication_event_machine() -> StateMachine<MedicationEventStatus> {
    use MedicationEventStatus::*;
    let m = HashMap::from([
        (Scheduled, vec![ReminderSent, Taken, Missed, Held]),
        (ReminderSent, vec![Taken, Missed, Held]),
        (Taken, vec![]),
        (Missed, vec![Escalated]),
        (Held, vec![Escalated]),
        (Escalated, vec![]),
    ]);
    StateMachine::new(m)
}

// ---------------------------------------------------------------------------
// 5. Institution Referral
//    CREATED → ACCEPTED → BOOKED → ATTENDED → DISCHARGED
//    any → CLOSED
// ---------------------------------------------------------------------------

pub fn referral_machine() -> StateMachine<InstitutionReferralStatus> {
    use InstitutionReferralStatus::*;
    let mut m = HashMap::from([
        (Created, vec![Accepted]),
        (Accepted, vec![Booked]),
        (Booked, vec![Attended]),
        (Attended, vec![Discharged]),
        (Discharged, vec![]),
        (Closed, vec![]),
    ]);
    add_escapes(&mut m, &[Closed]);
    StateMachine::new(m)
}

// ---------------------------------------------------------------------------
// 6. Eligibility Case
//    NOT_STARTED → SCREENING → DOCS_MISSING | UNDER_REVIEW
//    DOCS_MISSING → UNDER_REVIEW
//    UNDER_REVIEW → APPROVED | DENIED
//    DENIED → APPEALED
//    APPEALED → FINAL
// ---------------------------------------------------------------------------

pub fn eligibility_case_machine() -> StateMachine<EligibilityCaseStatus> {
    use EligibilityCaseStatus::*;
    let m = HashMap::from([
        (NotStarted, vec![Screening]),
        (Screening, vec![DocsMissing, UnderReview]),
        (DocsMissing, vec![UnderReview]),
        (UnderReview, vec![Approved, Denied]),
        (Approved, vec![]),
        (Denied, vec![Appealed]),
        (Appealed, vec![Final]),
        (Final, vec![]),
    ]);
    StateMachine::new(m)
}

// ---------------------------------------------------------------------------
// 7. Care Plan
//    DRAFT → ACTIVE → PAUSED | COMPLETED
//    PAUSED → ACTIVE
//    any → CANCELLED
// ---------------------------------------------------------------------------

pub fn care_plan_machine() -> StateMachine<CarePlanStatus> {
    use CarePlanStatus::*;
    let mut m = HashMap::from([
        (Draft, vec![Active]),
        (Active, vec![Paused, Completed]),
        (Paused, vec![Active]),
        (Completed, vec![]),
        (Cancelled, vec![]),
    ]);
    add_escapes(&mut m, &[Cancelled]);
    StateMachine::new(m)
}

// ---------------------------------------------------------------------------
// 8. Appointment
//    SCHEDULED → CONFIRMED → IN_PROGRESS → COMPLETED
//    SCHEDULED → CANCELLED
//    CONFIRMED → NO_SHOW | CANCELLED
//    IN_PROGRESS → CANCELLED
// ---------------------------------------------------------------------------

pub fn appointment_machine() -> StateMachine<AppointmentStatus> {
    use AppointmentStatus::*;
    let m = HashMap::from([
        (Scheduled, vec![Confirmed, Cancelled]),
        (Confirmed, vec![InProgress, NoShow, Cancelled]),
        (InProgress, vec![Completed, Cancelled]),
        (Completed, vec![]),
        (Cancelled, vec![]),
        (NoShow, vec![]),
    ]);
    StateMachine::new(m)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caregiver_application_happy_path() {
        use CaregiverApplicationStatus::*;
        let sm = caregiver_application_machine();
        assert!(sm.can_transition(Draft, Submitted));
        assert!(sm.can_transition(Submitted, IdentityVerified));
        assert!(sm.can_transition(IdentityVerified, CredentialReview));
        assert!(sm.can_transition(CredentialReview, ApprovedPrivatePay));
        assert!(sm.can_transition(CredentialReview, ApprovedUnderProvider));
    }

    #[test]
    fn caregiver_application_escape_states() {
        use CaregiverApplicationStatus::*;
        let sm = caregiver_application_machine();
        assert!(sm.can_transition(Draft, Suspended));
        assert!(sm.can_transition(Draft, Rejected));
        assert!(sm.can_transition(Submitted, Suspended));
        assert!(sm.can_transition(CredentialReview, Rejected));
        // Escape states also get cross-escapes (matches TS addEscapesToAll behavior)
        assert!(sm.can_transition(Suspended, Rejected));
        assert!(sm.can_transition(Rejected, Suspended));
    }

    #[test]
    fn caregiver_application_invalid() {
        use CaregiverApplicationStatus::*;
        let sm = caregiver_application_machine();
        assert!(!sm.can_transition(Draft, ApprovedPrivatePay));
        assert!(!sm.can_transition(Submitted, CredentialReview));
    }

    #[test]
    fn match_request_happy_path() {
        use MatchRequestStatus::*;
        let sm = match_request_machine();
        assert!(sm.can_transition(Created, Searching));
        assert!(sm.can_transition(Searching, RecommendationsReady));
        assert!(sm.can_transition(RecommendationsReady, Selected));
        assert!(sm.can_transition(Selected, Booked));
        assert!(sm.can_transition(Booked, Fulfilled));
    }

    #[test]
    fn match_request_cancel_from_any() {
        use MatchRequestStatus::*;
        let sm = match_request_machine();
        assert!(sm.can_transition(Created, Cancelled));
        assert!(sm.can_transition(Searching, Cancelled));
        assert!(sm.can_transition(Selected, Cancelled));
        // Fulfilled and Cancelled also get cross-escapes
        assert!(sm.can_transition(Fulfilled, Cancelled));
        assert!(!sm.can_transition(Cancelled, Created));
    }

    #[test]
    fn visit_happy_path() {
        use VisitStatus::*;
        let sm = visit_machine();
        assert!(sm.can_transition(Scheduled, CaregiverAcknowledged));
        assert!(sm.can_transition(CaregiverAcknowledged, InProgress));
        assert!(sm.can_transition(InProgress, Completed));
        assert!(sm.can_transition(Scheduled, Missed));
    }

    #[test]
    fn medication_event_flows() {
        use MedicationEventStatus::*;
        let sm = medication_event_machine();
        assert!(sm.can_transition(Scheduled, ReminderSent));
        assert!(sm.can_transition(ReminderSent, Taken));
        assert!(sm.can_transition(ReminderSent, Missed));
        assert!(sm.can_transition(Missed, Escalated));
        assert!(sm.can_transition(Held, Escalated));
        assert!(!sm.can_transition(Taken, Escalated));
    }

    #[test]
    fn referral_happy_path() {
        use InstitutionReferralStatus::*;
        let sm = referral_machine();
        assert!(sm.can_transition(Created, Accepted));
        assert!(sm.can_transition(Accepted, Booked));
        assert!(sm.can_transition(Booked, Attended));
        assert!(sm.can_transition(Attended, Discharged));
        assert!(sm.can_transition(Created, Closed));
    }

    #[test]
    fn eligibility_case_flows() {
        use EligibilityCaseStatus::*;
        let sm = eligibility_case_machine();
        assert!(sm.can_transition(NotStarted, Screening));
        assert!(sm.can_transition(Screening, DocsMissing));
        assert!(sm.can_transition(Screening, UnderReview));
        assert!(sm.can_transition(DocsMissing, UnderReview));
        assert!(sm.can_transition(UnderReview, Approved));
        assert!(sm.can_transition(UnderReview, Denied));
        assert!(sm.can_transition(Denied, Appealed));
        assert!(sm.can_transition(Appealed, Final));
        assert!(!sm.can_transition(Approved, Denied));
    }

    #[test]
    fn care_plan_flows() {
        use CarePlanStatus::*;
        let sm = care_plan_machine();
        assert!(sm.can_transition(Draft, Active));
        assert!(sm.can_transition(Active, Paused));
        assert!(sm.can_transition(Active, Completed));
        assert!(sm.can_transition(Paused, Active));
        assert!(sm.can_transition(Draft, Cancelled));
        assert!(sm.can_transition(Active, Cancelled));
        assert!(!sm.can_transition(Completed, Active));
    }

    #[test]
    fn appointment_flows() {
        use AppointmentStatus::*;
        let sm = appointment_machine();
        assert!(sm.can_transition(Scheduled, Confirmed));
        assert!(sm.can_transition(Scheduled, Cancelled));
        assert!(sm.can_transition(Confirmed, InProgress));
        assert!(sm.can_transition(Confirmed, NoShow));
        assert!(sm.can_transition(InProgress, Completed));
        assert!(!sm.can_transition(Completed, Cancelled));
        assert!(!sm.can_transition(NoShow, Confirmed));
    }
}
