import { Pool } from 'pg';

import { DomainEvent, DomainEventType, EventHandler } from '../types';

// Maps every DomainEventType to the corresponding observability_event_type string
// value as stored in the database.
const DOMAIN_EVENT_TO_OBSERVABILITY_TYPE: Record<DomainEventType, string> = {
  'visit.completed': 'VISIT_COMPLETED',
  'visit.missed': 'VISIT_MISSED',
  'medication.taken': 'MEDICATION_TAKEN',
  'medication.missed': 'MEDICATION_MISSED',
  'meal.delivered': 'MEAL_DELIVERED',
  'meal.failed': 'MEAL_FAILED',
  'transport.completed': 'TRANSPORT_COMPLETED',
  'transport.failed': 'TRANSPORT_FAILED',
  'symptom.reported': 'SYMPTOM_REPORTED',
  'incident.created': 'INCIDENT_CREATED',
  'eligibility.status_changed': 'ELIGIBILITY_STATUS_CHANGED',
  'referral.updated': 'REFERRAL_UPDATED',
};

// Infer a signal severity from the event payload when present, otherwise
// derive a sensible default based on the event type.
function resolveSignalSeverity(
  event: DomainEvent,
): 'INFO' | 'WARNING' | 'ALERT' | 'CRITICAL' {
  const payloadSeverity = event.payload['severity'];
  if (typeof payloadSeverity === 'string') {
    const upper = payloadSeverity.toUpperCase();
    if (upper === 'CRITICAL') return 'CRITICAL';
    if (upper === 'ALERT' || upper === 'HIGH') return 'ALERT';
    if (upper === 'WARNING' || upper === 'MEDIUM') return 'WARNING';
  }

  // Default by event type — failures and misses are warnings; critical
  // incident/symptom events handled via payload severity above.
  const warningTypes: DomainEventType[] = [
    'visit.missed',
    'medication.missed',
    'meal.failed',
    'transport.failed',
  ];
  if (warningTypes.includes(event.type)) {
    return 'WARNING';
  }

  return 'INFO';
}

function buildSignalMessage(event: DomainEvent): string {
  const base = `Event "${event.type}" recorded`;
  if (event.subjectPersonId !== undefined) {
    return `${base} for personId=${event.subjectPersonId}`;
  }
  return base;
}

export function createSignalHandler(pool: Pool): EventHandler {
  return async (event: DomainEvent): Promise<void> => {
    const observabilityEventType = DOMAIN_EVENT_TO_OBSERVABILITY_TYPE[event.type];
    const severity = resolveSignalSeverity(event);
    const message = buildSignalMessage(event);

    await pool.query(
      `INSERT INTO observability_signals
         (id, event_type, severity, subject_person_id, actor_user_id, entity_type, entity_id, message, metadata)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
      [
        crypto.randomUUID(),
        observabilityEventType,
        severity,
        event.subjectPersonId ?? null,
        event.actorUserId ?? null,
        event.entityType ?? null,
        event.entityId ?? null,
        message,
        event.metadata !== undefined ? JSON.stringify(event.metadata) : null,
      ],
    );
  };
}
