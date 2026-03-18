// Identity
export type {
  IdentityProvider,
  IdentityVerificationRequest,
  IdentityVerificationResult,
} from './identity/types.js';
export { MockIdentityProvider } from './identity/mock.js';

// Messaging
export type { MessageRequest, MessageResult, MessagingProvider } from './messaging/types.js';
export { MockMessagingProvider } from './messaging/mock.js';

// Maps
export type { DistanceResult, GeocodingResult, MapProvider } from './maps/types.js';
export { MockMapProvider } from './maps/mock.js';

// Payments
export type { PaymentProvider, PaymentRequest, PaymentResult } from './payments/types.js';
export { MockPaymentProvider } from './payments/mock.js';

// Documents
export type { DocumentProvider, SignatureRequest, SignatureResult } from './documents/types.js';
export { MockDocumentProvider } from './documents/mock.js';

// Medical
export type { HealthRecord, MedicalProvider } from './medical/types.js';
export { MockMedicalProvider } from './medical/mock.js';

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

import { MockIdentityProvider } from './identity/mock.js';
import { MockMessagingProvider } from './messaging/mock.js';
import { MockMapProvider } from './maps/mock.js';
import { MockPaymentProvider } from './payments/mock.js';
import { MockDocumentProvider } from './documents/mock.js';
import { MockMedicalProvider } from './medical/mock.js';

export interface MockProviders {
  readonly identity: MockIdentityProvider;
  readonly messaging: MockMessagingProvider;
  readonly maps: MockMapProvider;
  readonly payments: MockPaymentProvider;
  readonly documents: MockDocumentProvider;
  readonly medical: MockMedicalProvider;
}

/**
 * Returns a fresh set of all mock provider instances.
 * Use this in tests or local development to wire up all external service stubs
 * without touching real APIs.
 *
 * @example
 * const providers = createMockProviders();
 * const result = await providers.identity.verify({ name: '홍길동', ... });
 */
export function createMockProviders(): MockProviders {
  return {
    identity: new MockIdentityProvider(),
    messaging: new MockMessagingProvider(),
    maps: new MockMapProvider(),
    payments: new MockPaymentProvider(),
    documents: new MockDocumentProvider(),
    medical: new MockMedicalProvider(),
  };
}
