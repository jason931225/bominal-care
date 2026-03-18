export type DomainEventType =
  | 'visit.completed'
  | 'visit.missed'
  | 'medication.taken'
  | 'medication.missed'
  | 'meal.delivered'
  | 'meal.failed'
  | 'transport.completed'
  | 'transport.failed'
  | 'symptom.reported'
  | 'incident.created'
  | 'eligibility.status_changed'
  | 'referral.updated';

export interface DomainEvent {
  readonly id: string;
  readonly type: DomainEventType;
  readonly timestamp: Date;
  readonly subjectPersonId?: string;
  readonly actorUserId?: string;
  readonly entityType?: string;
  readonly entityId?: string;
  readonly payload: Record<string, unknown>;
  readonly metadata?: Record<string, unknown>;
}

export type EventHandler = (event: DomainEvent) => Promise<void>;

export interface EventSubscription {
  readonly eventType: DomainEventType;
  readonly handler: EventHandler;
  readonly id: string;
}
