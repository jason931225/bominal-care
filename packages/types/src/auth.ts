import { z } from "zod";

// -----------------------------------------------------------------------------
// Enum Schemas
// -----------------------------------------------------------------------------

export const UserRoleSchema = z.enum([
  "SENIOR",
  "FAMILY",
  "CAREGIVER_APPLICANT",
  "CAREGIVER_APPROVED",
  "PROVIDER_ADMIN",
  "PROVIDER_STAFF",
  "MEDICAL_STAFF",
  "GOVERNMENT_REVIEWER",
  "PARTNER_OPERATOR",
  "PLATFORM_ADMIN",
]);

export type UserRole = z.infer<typeof UserRoleSchema>;

export const KycLevelSchema = z.enum([
  "NONE",
  "EMAIL_VERIFIED",
  "PHONE_VERIFIED",
  "IDENTITY_VERIFIED",
  "FULL_VERIFIED",
]);

export type KycLevel = z.infer<typeof KycLevelSchema>;

// -----------------------------------------------------------------------------
// RegisterInput
// -----------------------------------------------------------------------------

export const RegisterInputSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
  name: z.string().min(1),
  phone: z.string().optional(),
  role: UserRoleSchema,
});

export type RegisterInput = z.infer<typeof RegisterInputSchema>;

// -----------------------------------------------------------------------------
// LoginInput
// -----------------------------------------------------------------------------

export const LoginInputSchema = z.object({
  email: z.string().email(),
  password: z.string().min(1),
});

export type LoginInput = z.infer<typeof LoginInputSchema>;

// -----------------------------------------------------------------------------
// SessionUser
// -----------------------------------------------------------------------------

export const SessionUserSchema = z.object({
  id: z.string(),
  email: z.string().email(),
  name: z.string(),
  role: UserRoleSchema,
  kycLevel: KycLevelSchema,
  tenantId: z.string().optional(),
});

export type SessionUser = z.infer<typeof SessionUserSchema>;
