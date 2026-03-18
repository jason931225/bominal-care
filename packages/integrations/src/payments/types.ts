export interface PaymentRequest {
  readonly amount: number;
  readonly currency: string;
  readonly description: string;
  readonly customerName: string;
  readonly customerEmail?: string;
  readonly orderId: string;
}

export interface PaymentResult {
  readonly paymentId: string;
  readonly status: 'completed' | 'failed' | 'pending';
  readonly amount: number;
  readonly currency: string;
  readonly paidAt?: Date;
}

export interface PaymentProvider {
  createPayment(request: PaymentRequest): Promise<PaymentResult>;
  getPayment(paymentId: string): Promise<PaymentResult>;
  refund(paymentId: string, amount?: number): Promise<PaymentResult>;
}
