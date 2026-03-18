// =============================================================================
// Lightweight State Machine — no external dependencies
// =============================================================================

import type {
  CaregiverApplicationStatus,
  MatchRequestStatus,
  VisitStatus,
  MedicationEventStatus,
  InstitutionReferralStatus,
  EligibilityCaseStatus,
  CarePlanStatus,
  AppointmentStatus,
} from '../types';

export type {
  CaregiverApplicationStatus,
  MatchRequestStatus,
  VisitStatus,
  MedicationEventStatus,
  InstitutionReferralStatus,
  EligibilityCaseStatus,
  CarePlanStatus,
  AppointmentStatus,
};

// -----------------------------------------------------------------------------
// Core types
// -----------------------------------------------------------------------------

export interface StateMachine<TState extends string> {
  readonly transitions: ReadonlyMap<TState, readonly TState[]>;
  canTransition(from: TState, to: TState): boolean;
  getValidTransitions(from: TState): readonly TState[];
}

export function createStateMachine<TState extends string>(
  transitionMap: Record<TState, readonly TState[]>,
): StateMachine<TState> {
  const transitions = new Map<TState, readonly TState[]>(
    Object.entries(transitionMap) as [TState, readonly TState[]][],
  ) as ReadonlyMap<TState, readonly TState[]>;

  return {
    transitions,

    canTransition(from: TState, to: TState): boolean {
      const allowed = transitions.get(from);
      return allowed !== undefined && allowed.includes(to);
    },

    getValidTransitions(from: TState): readonly TState[] {
      return transitions.get(from) ?? [];
    },
  };
}

// -----------------------------------------------------------------------------
// Helper — builds a transition map where every state can also go to a set of
// "escape" states (e.g. SUSPENDED, REJECTED, CANCELLED).
// -----------------------------------------------------------------------------

function addEscapesToAll<TState extends string>(
  base: Record<TState, readonly TState[]>,
  escapeStates: readonly TState[],
): Record<TState, readonly TState[]> {
  const result = {} as Record<TState, readonly TState[]>;
  for (const [state, nexts] of Object.entries(base) as [TState, readonly TState[]][]) {
    const additional = escapeStates.filter(
      (e) => e !== state && !(nexts as readonly TState[]).includes(e),
    );
    result[state] = [...nexts, ...additional] as readonly TState[];
  }
  return result;
}

// -----------------------------------------------------------------------------
// 1. Caregiver Application
//    DRAFT → SUBMITTED → IDENTITY_VERIFIED → CREDENTIAL_REVIEW
//      → APPROVED_PRIVATE_PAY | APPROVED_UNDER_PROVIDER
//    any → SUSPENDED | REJECTED
// -----------------------------------------------------------------------------

const caregiverApplicationBase: Record<CaregiverApplicationStatus, readonly CaregiverApplicationStatus[]> = {
  DRAFT: ['SUBMITTED'],
  SUBMITTED: ['IDENTITY_VERIFIED'],
  IDENTITY_VERIFIED: ['CREDENTIAL_REVIEW'],
  CREDENTIAL_REVIEW: ['APPROVED_PRIVATE_PAY', 'APPROVED_UNDER_PROVIDER'],
  APPROVED_PRIVATE_PAY: [],
  APPROVED_UNDER_PROVIDER: [],
  SUSPENDED: [],
  REJECTED: [],
};

export const caregiverApplicationMachine = createStateMachine<CaregiverApplicationStatus>(
  addEscapesToAll(caregiverApplicationBase, ['SUSPENDED', 'REJECTED']),
);

// -----------------------------------------------------------------------------
// 2. Match Request
//    CREATED → SEARCHING → RECOMMENDATIONS_READY → SELECTED → BOOKED → FULFILLED
//    any → CANCELLED
// -----------------------------------------------------------------------------

const matchRequestBase: Record<MatchRequestStatus, readonly MatchRequestStatus[]> = {
  CREATED: ['SEARCHING'],
  SEARCHING: ['RECOMMENDATIONS_READY'],
  RECOMMENDATIONS_READY: ['SELECTED'],
  SELECTED: ['BOOKED'],
  BOOKED: ['FULFILLED'],
  FULFILLED: [],
  CANCELLED: [],
};

export const matchRequestMachine = createStateMachine<MatchRequestStatus>(
  addEscapesToAll(matchRequestBase, ['CANCELLED']),
);

// -----------------------------------------------------------------------------
// 3. Visit
//    SCHEDULED → CAREGIVER_ACKNOWLEDGED → IN_PROGRESS → COMPLETED
//    SCHEDULED → MISSED
//    any → CANCELLED
// -----------------------------------------------------------------------------

const visitBase: Record<VisitStatus, readonly VisitStatus[]> = {
  SCHEDULED: ['CAREGIVER_ACKNOWLEDGED', 'MISSED'],
  CAREGIVER_ACKNOWLEDGED: ['IN_PROGRESS'],
  IN_PROGRESS: ['COMPLETED'],
  COMPLETED: [],
  MISSED: [],
  CANCELLED: [],
};

export const visitMachine = createStateMachine<VisitStatus>(
  addEscapesToAll(visitBase, ['CANCELLED']),
);

// -----------------------------------------------------------------------------
// 4. Medication Event
//    SCHEDULED → REMINDER_SENT → TAKEN | MISSED | HELD
//    MISSED → ESCALATED
//    HELD → ESCALATED
// -----------------------------------------------------------------------------

const medicationEventTransitions: Record<MedicationEventStatus, readonly MedicationEventStatus[]> = {
  SCHEDULED: ['REMINDER_SENT', 'TAKEN', 'MISSED', 'HELD'],
  REMINDER_SENT: ['TAKEN', 'MISSED', 'HELD'],
  TAKEN: [],
  MISSED: ['ESCALATED'],
  HELD: ['ESCALATED'],
  ESCALATED: [],
};

export const medicationEventMachine = createStateMachine<MedicationEventStatus>(
  medicationEventTransitions,
);

// -----------------------------------------------------------------------------
// 5. Institution Referral
//    CREATED → ACCEPTED → BOOKED → ATTENDED → DISCHARGED
//    any → CLOSED
// -----------------------------------------------------------------------------

const referralBase: Record<InstitutionReferralStatus, readonly InstitutionReferralStatus[]> = {
  CREATED: ['ACCEPTED'],
  ACCEPTED: ['BOOKED'],
  BOOKED: ['ATTENDED'],
  ATTENDED: ['DISCHARGED'],
  DISCHARGED: [],
  CLOSED: [],
};

export const referralMachine = createStateMachine<InstitutionReferralStatus>(
  addEscapesToAll(referralBase, ['CLOSED']),
);

// -----------------------------------------------------------------------------
// 6. Eligibility Case
//    NOT_STARTED → SCREENING → DOCS_MISSING | UNDER_REVIEW
//    DOCS_MISSING → UNDER_REVIEW
//    UNDER_REVIEW → APPROVED | DENIED
//    DENIED → APPEALED
//    APPEALED → FINAL
// -----------------------------------------------------------------------------

const eligibilityCaseTransitions: Record<EligibilityCaseStatus, readonly EligibilityCaseStatus[]> = {
  NOT_STARTED: ['SCREENING'],
  SCREENING: ['DOCS_MISSING', 'UNDER_REVIEW'],
  DOCS_MISSING: ['UNDER_REVIEW'],
  UNDER_REVIEW: ['APPROVED', 'DENIED'],
  APPROVED: [],
  DENIED: ['APPEALED'],
  APPEALED: ['FINAL'],
  FINAL: [],
};

export const eligibilityCaseMachine = createStateMachine<EligibilityCaseStatus>(
  eligibilityCaseTransitions,
);

// -----------------------------------------------------------------------------
// 7. Care Plan
//    DRAFT → ACTIVE → PAUSED | COMPLETED
//    PAUSED → ACTIVE
//    any → CANCELLED
// -----------------------------------------------------------------------------

const carePlanBase: Record<CarePlanStatus, readonly CarePlanStatus[]> = {
  DRAFT: ['ACTIVE'],
  ACTIVE: ['PAUSED', 'COMPLETED'],
  PAUSED: ['ACTIVE'],
  COMPLETED: [],
  CANCELLED: [],
};

export const carePlanMachine = createStateMachine<CarePlanStatus>(
  addEscapesToAll(carePlanBase, ['CANCELLED']),
);

// -----------------------------------------------------------------------------
// 8. Appointment
//    SCHEDULED → CONFIRMED → IN_PROGRESS → COMPLETED
//    SCHEDULED → CANCELLED
//    CONFIRMED → NO_SHOW
// -----------------------------------------------------------------------------

const appointmentTransitions: Record<AppointmentStatus, readonly AppointmentStatus[]> = {
  SCHEDULED: ['CONFIRMED', 'CANCELLED'],
  CONFIRMED: ['IN_PROGRESS', 'NO_SHOW', 'CANCELLED'],
  IN_PROGRESS: ['COMPLETED', 'CANCELLED'],
  COMPLETED: [],
  CANCELLED: [],
  NO_SHOW: [],
};

export const appointmentMachine = createStateMachine<AppointmentStatus>(
  appointmentTransitions,
);
