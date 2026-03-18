import type { DefaultSession } from 'next-auth';

type UserRole =
  | 'SENIOR'
  | 'FAMILY'
  | 'CAREGIVER_APPLICANT'
  | 'CAREGIVER_APPROVED'
  | 'PROVIDER_ADMIN'
  | 'PROVIDER_STAFF'
  | 'MEDICAL_STAFF'
  | 'GOVERNMENT_REVIEWER'
  | 'PARTNER_OPERATOR'
  | 'PLATFORM_ADMIN';

type KycLevel =
  | 'NONE'
  | 'EMAIL_VERIFIED'
  | 'PHONE_VERIFIED'
  | 'IDENTITY_VERIFIED'
  | 'FULL_VERIFIED';

declare module 'next-auth' {
  interface Session {
    user: {
      id: string;
      role: UserRole;
      kycLevel: KycLevel;
      tenantId?: string;
    } & DefaultSession['user'];
  }
}
