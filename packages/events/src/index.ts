// Types
export type { DomainEvent, DomainEventType, EventHandler, EventSubscription } from './types';

// Event Bus
export { EventBus, eventBus } from './emitter';

// Rule Engine
export type { Rule, RuleAction, RuleContext, RuleResult } from './rules/engine';
export { RuleEngine, ruleEngine } from './rules/engine';

// Rule Definitions
export {
  highSeveritySymptomRule,
  missedMedicationAlertRule,
  missedVisitAlertRule,
  missingDocsReminderRule,
} from './rules/definitions';

// Handler Factories
export { createNotificationHandler } from './handlers/notification';
export { createSignalHandler } from './handlers/signal';
