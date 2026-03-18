import { Pool } from 'pg';

import { DomainEvent, EventHandler } from '../types';
import { RuleAction, RuleContext, RuleEngine } from '../rules/engine';

// Map RuleAction severity to the NotificationType enum values.
// Schema defines: INFO | WARNING | ALERT | ACTION_REQUIRED | REMINDER
const SEVERITY_TO_NOTIFICATION_TYPE: Record<
  RuleAction['severity'],
  'INFO' | 'WARNING' | 'ALERT' | 'ACTION_REQUIRED'
> = {
  info: 'INFO',
  warning: 'WARNING',
  alert: 'ALERT',
  critical: 'ACTION_REQUIRED',
};

function buildTitle(action: RuleAction): string {
  const typeLabel: Record<RuleAction['type'], string> = {
    notify: 'Notification',
    escalate: 'Escalation',
    create_task: 'Task',
  };
  return typeLabel[action.type];
}

export function createNotificationHandler(
  pool: Pool,
  ruleEngine: RuleEngine,
  ruleContext: RuleContext,
): EventHandler {
  return async (event: DomainEvent): Promise<void> => {
    const results = await ruleEngine.evaluate(event, ruleContext);

    const triggeredActions = results
      .filter((result) => result.triggered)
      .flatMap((result) => Array.from(result.actions));

    if (triggeredActions.length === 0) {
      return;
    }

    const notificationCreations = triggeredActions.flatMap((action) =>
      action.targets.map((target) => ({
        action,
        target,
      })),
    );

    await Promise.all(
      notificationCreations.map(async ({ action, target }) => {
        // Targets that start with "role:" are role-based and cannot be mapped
        // to a specific userId without a role-lookup service. Skip role targets
        // here — a separate role-fan-out service should handle those.
        // Targets that do not start with "role:" are treated as direct userIds.
        if (target.startsWith('role:')) {
          console.info(
            `[NotificationHandler] Skipping role-based target="${target}" for event type="${event.type}". A role-fan-out service should handle this.`,
          );
          return;
        }

        const notificationType = SEVERITY_TO_NOTIFICATION_TYPE[action.severity];

        await pool.query(
          `INSERT INTO notifications (id, user_id, type, title, message, is_read)
           VALUES ($1, $2, $3, $4, $5, $6)`,
          [
            crypto.randomUUID(),
            target,
            notificationType,
            buildTitle(action),
            action.message,
            false,
          ],
        );
      }),
    );
  };
}
