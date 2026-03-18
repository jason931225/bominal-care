import { z } from "zod";

// -----------------------------------------------------------------------------
// Enum Schemas
// -----------------------------------------------------------------------------

export const MatchRequestStatusSchema = z.enum([
  "CREATED",
  "SEARCHING",
  "RECOMMENDATIONS_READY",
  "SELECTED",
  "BOOKED",
  "FULFILLED",
  "CANCELLED",
]);

export type MatchRequestStatus = z.infer<typeof MatchRequestStatusSchema>;

export const ServiceCategorySchema = z.enum([
  "PERSONAL_CARE",
  "COMPANION",
  "NURSING",
  "REHABILITATION",
  "DEMENTIA_CARE",
  "RESPITE",
  "TRANSPORT",
  "MEAL_DELIVERY",
  "HOME_MODIFICATION",
  "CLEANING",
]);

export type ServiceCategory = z.infer<typeof ServiceCategorySchema>;

export const DayOfWeekSchema = z.enum([
  "MONDAY",
  "TUESDAY",
  "WEDNESDAY",
  "THURSDAY",
  "FRIDAY",
  "SATURDAY",
  "SUNDAY",
]);

export type DayOfWeek = z.infer<typeof DayOfWeekSchema>;

// -----------------------------------------------------------------------------
// MatchRequestInput
// -----------------------------------------------------------------------------

export const MatchRequestInputSchema = z.object({
  seniorId: z.string().min(1),
  serviceCategory: ServiceCategorySchema,
  regionCity: z.string().min(1),
  regionDistrict: z.string().min(1),
  startDate: z.coerce.date(),
  endDate: z.coerce.date().optional(),
  scheduleNotes: z.string().optional(),
  languagePreference: z.string().optional(),
  genderPreference: z
    .enum(["MALE", "FEMALE", "OTHER", "PREFER_NOT_TO_SAY"])
    .optional(),
  requiresDementiaExperience: z.boolean().optional(),
  requiresOvernightCare: z.boolean().optional(),
  additionalNotes: z.string().optional(),
});

export type MatchRequestInput = z.infer<typeof MatchRequestInputSchema>;

// -----------------------------------------------------------------------------
// MatchCriteria
// -----------------------------------------------------------------------------

export const MatchCriteriaSchema = z.object({
  regionCity: z.string().min(1),
  regionDistrict: z.string().min(1),
  serviceCategory: ServiceCategorySchema,
  scheduleOverlap: z.boolean().optional(),
  language: z.string().optional(),
  genderPreference: z
    .enum(["MALE", "FEMALE", "OTHER", "PREFER_NOT_TO_SAY"])
    .optional(),
  dementiaExperience: z.boolean().optional(),
  overnightAvailable: z.boolean().optional(),
});

export type MatchCriteria = z.infer<typeof MatchCriteriaSchema>;

// -----------------------------------------------------------------------------
// MatchRecommendationResponse
// -----------------------------------------------------------------------------

export const MatchRecommendationResponseSchema = z.object({
  id: z.string(),
  caregiverApplicationId: z.string(),
  score: z.number(),
  scoreBreakdown: z.record(z.unknown()).nullable(),
  rank: z.number().int(),
  isSelected: z.boolean(),
});

export type MatchRecommendationResponse = z.infer<
  typeof MatchRecommendationResponseSchema
>;
