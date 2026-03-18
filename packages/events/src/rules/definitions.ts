import { DomainEvent } from '../types';
import { Rule, RuleAction, RuleContext, RuleResult } from './engine';

// ---------------------------------------------------------------------------
// Rule 1: MissedMedicationAlert
// Triggers when 2+ medication.missed events occur for the same subjectPersonId
// within a 48-hour window. Notifies family members and caregiver supervisors.
// ---------------------------------------------------------------------------
export const missedMedicationAlertRule: Rule = {
  id: 'missed-medication-alert',
  name: 'MissedMedicationAlert',
  description:
    'Alerts family and caregiver supervisor when a senior misses 2 or more medications within 48 hours.',
  eventTypes: ['medication.missed'],

  async evaluate(event: DomainEvent, context: RuleContext): Promise<RuleResult> {
    const { subjectPersonId } = event;
    if (subjectPersonId === undefined || subjectPersonId === '') {
      return { triggered: false, actions: [] };
    }

    const recentMissed = await context.getRecentEvents({
      subjectPersonId,
      eventType: 'medication.missed',
      withinHours: 48,
    });

    if (recentMissed.length < 2) {
      return { triggered: false, actions: [] };
    }

    const action: RuleAction = {
      type: 'notify',
      targets: ['role:family', 'role:caregiver_supervisor'],
      message: `Senior (personId: ${subjectPersonId}) has missed ${recentMissed.length} medications in the past 48 hours. Immediate follow-up required.`,
      severity: 'alert',
      metadata: {
        subjectPersonId,
        missedCount: recentMissed.length,
        windowHours: 48,
      },
    };

    return { triggered: true, actions: [action] };
  },
};

// ---------------------------------------------------------------------------
// Rule 2: MissedVisitAlert
// Triggers immediately on any visit.missed event.
// Notifies the supervisor at critical severity.
// ---------------------------------------------------------------------------
export const missedVisitAlertRule: Rule = {
  id: 'missed-visit-alert',
  name: 'MissedVisitAlert',
  description: 'Immediately alerts the supervisor when a scheduled visit is missed.',
  eventTypes: ['visit.missed'],

  async evaluate(event: DomainEvent, _context: RuleContext): Promise<RuleResult> {
    const { subjectPersonId, entityId } = event;

    const action: RuleAction = {
      type: 'escalate',
      targets: ['role:supervisor'],
      message: `A scheduled visit was missed${subjectPersonId !== undefined ? ` for senior (personId: ${subjectPersonId})` : ''}${entityId !== undefined ? ` (visitId: ${entityId})` : ''}. Immediate action required.`,
      severity: 'critical',
      metadata: {
        subjectPersonId,
        visitId: entityId,
        timestamp: event.timestamp.toISOString(),
      },
    };

    return { triggered: true, actions: [action] };
  },
};

// ---------------------------------------------------------------------------
// Rule 3: HighSeveritySymptom
// Triggers on symptom.reported events where severity is HIGH or CRITICAL,
// but only when the subject has medical-share consent active.
// ---------------------------------------------------------------------------
const HIGH_SEVERITY_VALUES = new Set(['HIGH', 'CRITICAL']);

export const highSeveritySymptomRule: Rule = {
  id: 'high-severity-symptom',
  name: 'HighSeveritySymptom',
  description:
    'Routes high or critical symptom reports to the medical review queue when medical-share consent is active.',
  eventTypes: ['symptom.reported'],

  async evaluate(event: DomainEvent, context: RuleContext): Promise<RuleResult> {
    const { subjectPersonId } = event;
    if (subjectPersonId === undefined || subjectPersonId === '') {
      return { triggered: false, actions: [] };
    }

    const symptomSeverity = event.payload['severity'];
    if (typeof symptomSeverity !== 'string' || !HIGH_SEVERITY_VALUES.has(symptomSeverity)) {
      return { triggered: false, actions: [] };
    }

    const consent = await context.getConsent(subjectPersonId);
    const hasMedicalShareConsent =
      consent !== null &&
      (consent.purpose === 'MEDICAL_SHARE' || consent.purpose === 'BOTH_SHARE');

    if (!hasMedicalShareConsent) {
      return { triggered: false, actions: [] };
    }

    const action: RuleAction = {
      type: 'escalate',
      targets: ['role:medical_review_queue'],
      message: `A ${symptomSeverity} severity symptom has been reported for senior (personId: ${subjectPersonId}). Medical review required.`,
      severity: symptomSeverity === 'CRITICAL' ? 'critical' : 'alert',
      metadata: {
        subjectPersonId,
        symptomSeverity,
        symptomDescription: event.payload['description'],
        consentPurpose: consent?.purpose,
        entityId: event.entityId,
      },
    };

    return { triggered: true, actions: [action] };
  },
};

// ---------------------------------------------------------------------------
// Rule 4: MissingDocsReminder
// Triggers on eligibility.status_changed events where the new status is
// DOCS_MISSING and at least 7 days have passed since the status change.
// ---------------------------------------------------------------------------
const DOCS_MISSING_THRESHOLD_DAYS = 7;
const MS_PER_DAY = 24 * 60 * 60 * 1000;

export const missingDocsReminderRule: Rule = {
  id: 'missing-docs-reminder',
  name: 'MissingDocsReminder',
  description:
    'Sends a family reminder and creates a case-worker task when eligibility docs remain missing for 7+ days.',
  eventTypes: ['eligibility.status_changed'],

  async evaluate(event: DomainEvent, _context: RuleContext): Promise<RuleResult> {
    const { subjectPersonId } = event;
    if (subjectPersonId === undefined || subjectPersonId === '') {
      return { triggered: false, actions: [] };
    }

    const newStatus = event.payload['newStatus'];
    if (newStatus !== 'DOCS_MISSING') {
      return { triggered: false, actions: [] };
    }

    const statusChangedAt = event.payload['statusChangedAt'];
    const changeDate =
      statusChangedAt instanceof Date
        ? statusChangedAt
        : typeof statusChangedAt === 'string'
          ? new Date(statusChangedAt)
          : event.timestamp;

    const daysSinceChange = (Date.now() - changeDate.getTime()) / MS_PER_DAY;

    if (daysSinceChange < DOCS_MISSING_THRESHOLD_DAYS) {
      return { triggered: false, actions: [] };
    }

    const familyAction: RuleAction = {
      type: 'notify',
      targets: ['role:family'],
      message: `Required documents for the eligibility case of senior (personId: ${subjectPersonId}) have been missing for ${Math.floor(daysSinceChange)} days. Please submit the missing documents as soon as possible.`,
      severity: 'warning',
      metadata: {
        subjectPersonId,
        entityId: event.entityId,
        daysMissing: Math.floor(daysSinceChange),
        threshold: DOCS_MISSING_THRESHOLD_DAYS,
      },
    };

    const taskAction: RuleAction = {
      type: 'create_task',
      targets: ['role:case_worker'],
      message: `Follow up with family of senior (personId: ${subjectPersonId}): eligibility documents have been missing for ${Math.floor(daysSinceChange)} days.`,
      severity: 'warning',
      metadata: {
        subjectPersonId,
        entityId: event.entityId,
        daysMissing: Math.floor(daysSinceChange),
        threshold: DOCS_MISSING_THRESHOLD_DAYS,
      },
    };

    return { triggered: true, actions: [familyAction, taskAction] };
  },
};
