import { z } from "zod";

// -----------------------------------------------------------------------------
// Enum Schemas
// -----------------------------------------------------------------------------

export const CarePlanStatusSchema = z.enum([
  "DRAFT",
  "ACTIVE",
  "PAUSED",
  "COMPLETED",
  "CANCELLED",
]);

export type CarePlanStatus = z.infer<typeof CarePlanStatusSchema>;

export const VisitStatusSchema = z.enum([
  "SCHEDULED",
  "CAREGIVER_ACKNOWLEDGED",
  "IN_PROGRESS",
  "COMPLETED",
  "MISSED",
  "CANCELLED",
]);

export type VisitStatus = z.infer<typeof VisitStatusSchema>;

export const IncidentSeveritySchema = z.enum([
  "LOW",
  "MEDIUM",
  "HIGH",
  "CRITICAL",
]);

export type IncidentSeverity = z.infer<typeof IncidentSeveritySchema>;

export const ObservationCategorySchema = z.enum([
  "VITAL_SIGNS",
  "MOOD",
  "APPETITE",
  "MOBILITY",
  "SLEEP",
  "PAIN",
  "COGNITIVE",
  "SKIN",
  "OTHER",
]);

export type ObservationCategory = z.infer<typeof ObservationCategorySchema>;

// -----------------------------------------------------------------------------
// CarePlanInput
// -----------------------------------------------------------------------------

export const CarePlanInputSchema = z.object({
  seniorId: z.string().min(1),
  providerId: z.string().optional(),
  title: z.string().min(1),
  description: z.string().optional(),
  startDate: z.coerce.date().optional(),
  endDate: z.coerce.date().optional(),
  goals: z.record(z.unknown()).optional(),
});

export type CarePlanInput = z.infer<typeof CarePlanInputSchema>;

// -----------------------------------------------------------------------------
// VisitInput
// -----------------------------------------------------------------------------

export const VisitInputSchema = z.object({
  carePlanId: z.string().min(1),
  caregiverId: z.string().min(1),
  scheduledStart: z.coerce.date(),
  scheduledEnd: z.coerce.date(),
  tasks: z.array(z.record(z.unknown())).optional(),
});

export type VisitInput = z.infer<typeof VisitInputSchema>;

// -----------------------------------------------------------------------------
// VisitCheckIn
// -----------------------------------------------------------------------------

export const VisitCheckInSchema = z.object({
  latitude: z.number().min(-90).max(90).optional(),
  longitude: z.number().min(-180).max(180).optional(),
});

export type VisitCheckIn = z.infer<typeof VisitCheckInSchema>;

// -----------------------------------------------------------------------------
// VisitCheckOut
// -----------------------------------------------------------------------------

export const VisitCheckOutSchema = z.object({
  latitude: z.number().min(-90).max(90).optional(),
  longitude: z.number().min(-180).max(180).optional(),
  notes: z.string().optional(),
});

export type VisitCheckOut = z.infer<typeof VisitCheckOutSchema>;

// -----------------------------------------------------------------------------
// DailyObservationInput
// -----------------------------------------------------------------------------

export const DailyObservationInputSchema = z.object({
  carePlanId: z.string().min(1),
  category: ObservationCategorySchema,
  date: z.coerce.date(),
  value: z.string().min(1),
  notes: z.string().optional(),
});

export type DailyObservationInput = z.infer<typeof DailyObservationInputSchema>;

// -----------------------------------------------------------------------------
// IncidentInput
// -----------------------------------------------------------------------------

export const IncidentInputSchema = z.object({
  visitId: z.string().optional(),
  severity: IncidentSeveritySchema,
  title: z.string().min(1),
  description: z.string().min(1),
  occurredAt: z.coerce.date(),
});

export type IncidentInput = z.infer<typeof IncidentInputSchema>;
