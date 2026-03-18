import { DomainEvent, DomainEventType } from '../types';

export interface Rule {
  readonly id: string;
  readonly name: string;
  readonly description: string;
  readonly eventTypes: readonly DomainEventType[];
  readonly evaluate: (event: DomainEvent, context: RuleContext) => Promise<RuleResult>;
}

export interface RuleContext {
  readonly getRecentEvents: (params: {
    subjectPersonId: string;
    eventType: DomainEventType;
    withinHours: number;
  }) => Promise<readonly DomainEvent[]>;
  readonly getConsent: (subjectPersonId: string) => Promise<{ purpose: string } | null>;
}

export interface RuleResult {
  readonly triggered: boolean;
  readonly actions: readonly RuleAction[];
}

export interface RuleAction {
  readonly type: 'notify' | 'escalate' | 'create_task';
  readonly targets: readonly string[]; // userIds or roles
  readonly message: string;
  readonly severity: 'info' | 'warning' | 'alert' | 'critical';
  readonly metadata?: Record<string, unknown>;
}

export class RuleEngine {
  private readonly rules: Map<string, Rule>;

  constructor() {
    this.rules = new Map();
  }

  registerRule(rule: Rule): void {
    this.rules.set(rule.id, rule);
  }

  async evaluate(event: DomainEvent, context: RuleContext): Promise<readonly RuleResult[]> {
    const matchingRules = Array.from(this.rules.values()).filter((rule) =>
      rule.eventTypes.includes(event.type),
    );

    const results = await Promise.all(
      matchingRules.map((rule) =>
        rule.evaluate(event, context).catch((error: unknown) => {
          console.error(
            `[RuleEngine] Error evaluating rule id="${rule.id}" for event type="${event.type}":`,
            error,
          );
          const failedResult: RuleResult = { triggered: false, actions: [] };
          return failedResult;
        }),
      ),
    );

    return results;
  }

  getRegisteredRules(): readonly Rule[] {
    return Array.from(this.rules.values());
  }
}

export const ruleEngine = new RuleEngine();
