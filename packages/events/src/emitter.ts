import { DomainEvent, DomainEventType, EventHandler } from './types';

export class EventBus {
  private readonly subscriptions: Map<DomainEventType, Set<EventHandler>>;

  constructor() {
    this.subscriptions = new Map();
  }

  subscribe(eventType: DomainEventType, handler: EventHandler): () => void {
    const existing = this.subscriptions.get(eventType);
    if (existing !== undefined) {
      existing.add(handler);
    } else {
      this.subscriptions.set(eventType, new Set([handler]));
    }

    return () => {
      const handlers = this.subscriptions.get(eventType);
      if (handlers !== undefined) {
        handlers.delete(handler);
        if (handlers.size === 0) {
          this.subscriptions.delete(eventType);
        }
      }
    };
  }

  async publish(event: DomainEvent): Promise<void> {
    const handlers = this.subscriptions.get(event.type);
    if (handlers === undefined || handlers.size === 0) {
      return;
    }

    const invocations = Array.from(handlers).map((handler) => handler(event));
    await Promise.all(invocations);
  }

  publishAndForget(event: DomainEvent): void {
    this.publish(event).catch((error: unknown) => {
      console.error(
        `[EventBus] Unhandled error publishing event type="${event.type}" id="${event.id}":`,
        error,
      );
    });
  }

  clear(): void {
    this.subscriptions.clear();
  }
}

export const eventBus = new EventBus();
