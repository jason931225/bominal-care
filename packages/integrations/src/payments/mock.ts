import type { PaymentProvider, PaymentRequest, PaymentResult } from './types.js';

let paymentCounter = 0;

function generatePaymentId(): string {
  paymentCounter += 1;
  return `mock-pay-${Date.now()}-${paymentCounter}`;
}

// In-memory store of created payments keyed by paymentId
const paymentStore = new Map<string, PaymentResult>();

/**
 * Mock payment provider for development and testing.
 * All payments succeed immediately. Refunds return a new completed result
 * with the refunded amount (defaulting to the full original amount).
 */
export class MockPaymentProvider implements PaymentProvider {
  async createPayment(request: PaymentRequest): Promise<PaymentResult> {
    const paymentId = generatePaymentId();

    const result: PaymentResult = {
      paymentId,
      status: 'completed',
      amount: request.amount,
      currency: request.currency,
      paidAt: new Date(),
    };

    paymentStore.set(paymentId, result);

    console.log(`[MockPayment] Payment created`, {
      paymentId,
      orderId: request.orderId,
      customerName: request.customerName,
      amount: request.amount,
      currency: request.currency,
    });

    return result;
  }

  async getPayment(paymentId: string): Promise<PaymentResult> {
    const stored = paymentStore.get(paymentId);

    if (stored) {
      return stored;
    }

    // Return a plausible "not found" completed result for unknown IDs
    return {
      paymentId,
      status: 'completed',
      amount: 0,
      currency: 'KRW',
      paidAt: new Date(),
    };
  }

  async refund(paymentId: string, amount?: number): Promise<PaymentResult> {
    const original = paymentStore.get(paymentId);
    const refundAmount = amount ?? original?.amount ?? 0;

    const result: PaymentResult = {
      paymentId,
      status: 'completed',
      amount: refundAmount,
      currency: original?.currency ?? 'KRW',
      paidAt: new Date(),
    };

    // Update store to reflect refunded state
    paymentStore.set(paymentId, result);

    console.log(`[MockPayment] Refund processed`, {
      paymentId,
      refundAmount,
    });

    return result;
  }
}
