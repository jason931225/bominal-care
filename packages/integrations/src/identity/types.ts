export interface IdentityVerificationRequest {
  readonly name: string;
  readonly dateOfBirth: string;
  readonly phone: string;
  readonly identityDocumentUrl?: string;
}

export interface IdentityVerificationResult {
  readonly verified: boolean;
  readonly kycLevel: 'PHONE_VERIFIED' | 'IDENTITY_VERIFIED' | 'FULL_VERIFIED';
  readonly verifiedAt: Date;
  readonly provider: string;
}

export interface IdentityProvider {
  verify(request: IdentityVerificationRequest): Promise<IdentityVerificationResult>;
}
