import { z } from "zod";

// -----------------------------------------------------------------------------
// Enum Schemas
// -----------------------------------------------------------------------------

export const GenderSchema = z.enum([
  "MALE",
  "FEMALE",
  "OTHER",
  "PREFER_NOT_TO_SAY",
]);

export type Gender = z.infer<typeof GenderSchema>;

export const RelationshipTypeSchema = z.enum([
  "SPOUSE",
  "CHILD",
  "SIBLING",
  "PARENT",
  "GRANDCHILD",
  "OTHER_RELATIVE",
  "LEGAL_GUARDIAN",
  "FRIEND",
  "SOCIAL_WORKER",
]);

export type RelationshipType = z.infer<typeof RelationshipTypeSchema>;

// -----------------------------------------------------------------------------
// PersonProfileInput
// Excludes: id, createdAt, updatedAt
// -----------------------------------------------------------------------------

export const PersonProfileInputSchema = z.object({
  userId: z.string().min(1),
  koreanName: z.string().optional(),
  englishName: z.string().optional(),
  dateOfBirth: z.coerce.date().optional(),
  gender: GenderSchema.optional(),
  phone: z.string().optional(),
  address: z.string().optional(),
  city: z.string().optional(),
  district: z.string().optional(),
  postalCode: z.string().optional(),
  emergencyContactName: z.string().optional(),
  emergencyContactPhone: z.string().optional(),
  createdBy: z.string().optional(),
  updatedBy: z.string().optional(),
});

export type PersonProfileInput = z.infer<typeof PersonProfileInputSchema>;

// -----------------------------------------------------------------------------
// SeniorProfileInput
// Excludes: id, createdAt, updatedAt
// -----------------------------------------------------------------------------

export const SeniorProfileInputSchema = z.object({
  personId: z.string().min(1),
  careLevel: z.number().int().min(1).max(5).optional(),
  hasLtciCertification: z.boolean().default(false),
  ltciNumber: z.string().optional(),
  primaryDiagnosis: z.string().optional(),
  mobilityLevel: z.string().optional(),
  cognitiveLevel: z.string().optional(),
  livesAlone: z.boolean().default(false),
  preferredLanguage: z.string().default("ko"),
});

export type SeniorProfileInput = z.infer<typeof SeniorProfileInputSchema>;

// -----------------------------------------------------------------------------
// FamilyRelationshipInput
// -----------------------------------------------------------------------------

export const FamilyRelationshipInputSchema = z.object({
  seniorPersonId: z.string().min(1),
  familyPersonId: z.string().min(1),
  relationshipType: RelationshipTypeSchema,
  isPrimaryContact: z.boolean().default(false),
  canMakeDecisions: z.boolean().default(false),
});

export type FamilyRelationshipInput = z.infer<
  typeof FamilyRelationshipInputSchema
>;
