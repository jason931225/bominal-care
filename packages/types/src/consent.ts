import { z } from "zod";

// -----------------------------------------------------------------------------
// Enum Schemas
// -----------------------------------------------------------------------------

export const ConsentPurposeSchema = z.enum([
  "NO_SHARE",
  "MEDICAL_SHARE",
  "GOVERNMENT_SHARE",
  "BOTH_SHARE",
]);

export type ConsentPurpose = z.infer<typeof ConsentPurposeSchema>;

// -----------------------------------------------------------------------------
// ConsentInput
// -----------------------------------------------------------------------------

export const ConsentInputSchema = z.object({
  subjectPersonId: z.string().min(1),
  purpose: ConsentPurposeSchema,
  expiresAt: z.coerce.date().optional(),
});

export type ConsentInput = z.infer<typeof ConsentInputSchema>;

// -----------------------------------------------------------------------------
// ConsentResponse — full consent record shape
// -----------------------------------------------------------------------------

export const ConsentResponseSchema = z.object({
  id: z.string(),
  subjectPersonId: z.string(),
  purpose: ConsentPurposeSchema,
  grantedBy: z.string(),
  isActive: z.boolean(),
  grantedAt: z.coerce.date(),
  revokedAt: z.coerce.date().nullable(),
  expiresAt: z.coerce.date().nullable(),
  createdAt: z.coerce.date(),
  updatedAt: z.coerce.date(),
});

export type ConsentResponse = z.infer<typeof ConsentResponseSchema>;
