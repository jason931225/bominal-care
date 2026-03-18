import type { MessageRequest, MessageResult, MessagingProvider } from './types.js';

let messageCounter = 0;

function generateMessageId(): string {
  messageCounter += 1;
  return `mock-msg-${Date.now()}-${messageCounter}`;
}

/**
 * Mock messaging provider for development and testing.
 * Logs every outbound message to the console and returns a successful result.
 */
export class MockMessagingProvider implements MessagingProvider {
  async send(request: MessageRequest): Promise<MessageResult> {
    const messageId = generateMessageId();
    const sentAt = new Date();

    console.log(`[MockMessaging] Sending ${request.channel.toUpperCase()} message`, {
      messageId,
      to: request.to,
      channel: request.channel,
      title: request.title,
      body: request.body,
      templateId: request.templateId,
      templateVars: request.templateVars,
      sentAt: sentAt.toISOString(),
    });

    return {
      messageId,
      sentAt,
      channel: request.channel,
      status: 'sent',
    };
  }

  async sendBulk(requests: readonly MessageRequest[]): Promise<readonly MessageResult[]> {
    console.log(`[MockMessaging] Sending bulk: ${requests.length} message(s)`);
    const results = await Promise.all(requests.map((req) => this.send(req)));
    return results;
  }
}
