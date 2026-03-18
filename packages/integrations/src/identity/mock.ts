import type { IdentityProvider, IdentityVerificationRequest, IdentityVerificationResult } from './types.js';

const MOCK_PROVIDER_NAME = 'MockIdentityProvider';

/**
 * Mock identity provider for development and testing.
 * Simulates a 1-second network delay and always returns a verified result.
 * Documents with a URL are elevated to FULL_VERIFIED; phone-only to PHONE_VERIFIED.
 */
export class MockIdentityProvider implements IdentityProvider {
  async verify(request: IdentityVerificationRequest): Promise<IdentityVerificationResult> {
    await simulateDelay(1000);

    const kycLevel = resolveKycLevel(request);

    return {
      verified: true,
      kycLevel,
      verifiedAt: new Date(),
      provider: MOCK_PROVIDER_NAME,
    };
  }
}

function resolveKycLevel(
  request: IdentityVerificationRequest,
): IdentityVerificationResult['kycLevel'] {
  if (request.identityDocumentUrl) {
    return 'FULL_VERIFIED';
  }
  if (request.dateOfBirth) {
    return 'IDENTITY_VERIFIED';
  }
  return 'PHONE_VERIFIED';
}

function simulateDelay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
