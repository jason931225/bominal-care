export interface MessageRequest {
  readonly to: string; // phone number or kakao user id
  readonly channel: 'sms' | 'kakao' | 'push';
  readonly title?: string;
  readonly body: string;
  readonly templateId?: string;
  readonly templateVars?: Record<string, string>;
}

export interface MessageResult {
  readonly messageId: string;
  readonly sentAt: Date;
  readonly channel: string;
  readonly status: 'sent' | 'failed';
}

export interface MessagingProvider {
  send(request: MessageRequest): Promise<MessageResult>;
  sendBulk(requests: readonly MessageRequest[]): Promise<readonly MessageResult[]>;
}
