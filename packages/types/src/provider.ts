import { z } from "zod";

// -----------------------------------------------------------------------------
// Enum Schemas
// -----------------------------------------------------------------------------

export const ProviderTypeSchema = z.enum([
  "HOME_CARE_AGENCY",
  "NURSING_HOSPITAL",
  "CLINIC",
  "PHARMACY",
  "REHABILITATION_CENTER",
  "DEMENTIA_CENTER",
  "SILVER_TOWN",
  "TRANSPORT_SERVICE",
  "MEAL_SERVICE",
  "COMMUNITY_CENTER",
]);

export type ProviderType = z.infer<typeof ProviderTypeSchema>;

export const CaregiverApplicationStatusSchema = z.enum([
  "DRAFT",
  "SUBMITTED",
  "IDENTITY_VERIFIED",
  "CREDENTIAL_REVIEW",
  "APPROVED_PRIVATE_PAY",
  "APPROVED_UNDER_PROVIDER",
  "SUSPENDED",
  "REJECTED",
]);

export type CaregiverApplicationStatus = z.infer<
  typeof CaregiverApplicationStatusSchema
>;

export const CredentialTypeSchema = z.enum([
  "CAREGIVER_CERTIFICATE",
  "NURSING_LICENSE",
  "SOCIAL_WORKER_LICENSE",
  "CPR_CERTIFICATION",
  "DEMENTIA_TRAINING",
  "FIRST_AID",
  "OTHER",
]);

export type CredentialType = z.infer<typeof CredentialTypeSchema>;

export const CredentialStatusSchema = z.enum([
  "PENDING",
  "VERIFIED",
  "EXPIRED",
  "REJECTED",
]);

export type CredentialStatus = z.infer<typeof CredentialStatusSchema>;

// -----------------------------------------------------------------------------
// ProviderOrgInput
// Excludes: id, createdAt, updatedAt
// -----------------------------------------------------------------------------

export const ProviderOrgInputSchema = z.object({
  name: z.string().min(1),
  type: ProviderTypeSchema,
  registrationNumber: z.string().min(1),
  address: z.string().optional(),
  city: z.string().optional(),
  district: z.string().optional(),
  postalCode: z.string().optional(),
  phone: z.string().optional(),
  email: z.string().email().optional(),
  website: z.string().url().optional(),
  licenseNumber: z.string().optional(),
  licenseExpiresAt: z.coerce.date().optional(),
  isActive: z.boolean().default(true),
  description: z.string().optional(),
  latitude: z.number().min(-90).max(90).optional(),
  longitude: z.number().min(-180).max(180).optional(),
  createdBy: z.string().optional(),
  updatedBy: z.string().optional(),
});

export type ProviderOrgInput = z.infer<typeof ProviderOrgInputSchema>;

// -----------------------------------------------------------------------------
// CaregiverApplicationInput
// -----------------------------------------------------------------------------

export const CaregiverApplicationInputSchema = z.object({
  userId: z.string().min(1),
  providerId: z.string().optional(),
  experienceYears: z.number().int().nonnegative().optional(),
  bio: z.string().optional(),
  specializations: z.string().optional(),
  hasDementiaExperience: z.boolean().default(false),
  hasOvernightAvailability: z.boolean().default(false),
  smokingStatus: z.boolean().default(false),
  petFriendly: z.boolean().default(true),
  preferredGender: z
    .enum(["MALE", "FEMALE", "OTHER", "PREFER_NOT_TO_SAY"])
    .optional(),
  languagesSpoken: z.string().default("ko"),
});

export type CaregiverApplicationInput = z.infer<
  typeof CaregiverApplicationInputSchema
>;

// -----------------------------------------------------------------------------
// CaregiverApplicationUpdate — status transition + reason
// -----------------------------------------------------------------------------

export const CaregiverApplicationUpdateSchema = z.object({
  status: CaregiverApplicationStatusSchema,
  rejectionReason: z.string().optional(),
  reviewedBy: z.string().optional(),
});

export type CaregiverApplicationUpdate = z.infer<
  typeof CaregiverApplicationUpdateSchema
>;

// -----------------------------------------------------------------------------
// CaregiverCredentialInput
// -----------------------------------------------------------------------------

export const CaregiverCredentialInputSchema = z.object({
  type: CredentialTypeSchema,
  issuer: z.string().optional(),
  issuedAt: z.coerce.date().optional(),
  expiresAt: z.coerce.date().optional(),
  documentUrl: z.string().url().optional(),
});

export type CaregiverCredentialInput = z.infer<
  typeof CaregiverCredentialInputSchema
>;
