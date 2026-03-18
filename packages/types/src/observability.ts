import { z } from "zod";

// -----------------------------------------------------------------------------
// Enum Schemas
// -----------------------------------------------------------------------------

export const ObservabilityEventTypeSchema = z.enum([
  "VISIT_COMPLETED",
  "VISIT_MISSED",
  "MEDICATION_TAKEN",
  "MEDICATION_MISSED",
  "MEAL_DELIVERED",
  "MEAL_FAILED",
  "TRANSPORT_COMPLETED",
  "TRANSPORT_FAILED",
  "SYMPTOM_REPORTED",
  "INCIDENT_CREATED",
  "ELIGIBILITY_STATUS_CHANGED",
  "REFERRAL_UPDATED",
]);

export type ObservabilityEventType = z.infer<
  typeof ObservabilityEventTypeSchema
>;

export const SignalSeveritySchema = z.enum([
  "INFO",
  "WARNING",
  "ALERT",
  "CRITICAL",
]);

export type SignalSeverity = z.infer<typeof SignalSeveritySchema>;

export const NotificationTypeSchema = z.enum([
  "INFO",
  "WARNING",
  "ALERT",
  "ACTION_REQUIRED",
  "REMINDER",
]);

export type NotificationType = z.infer<typeof NotificationTypeSchema>;

export const EligibilityCaseStatusSchema = z.enum([
  "NOT_STARTED",
  "SCREENING",
  "DOCS_MISSING",
  "UNDER_REVIEW",
  "APPROVED",
  "DENIED",
  "APPEALED",
  "FINAL",
]);

export type EligibilityCaseStatus = z.infer<typeof EligibilityCaseStatusSchema>;

export const ClaimStatusSchema = z.enum([
  "DRAFT",
  "SUBMITTED",
  "UNDER_REVIEW",
  "APPROVED",
  "DENIED",
  "PAID",
]);

export type ClaimStatus = z.infer<typeof ClaimStatusSchema>;

export const AuditActionSchema = z.enum([
  "CREATE",
  "READ",
  "UPDATE",
  "DELETE",
  "LOGIN",
  "LOGOUT",
  "CONSENT_GRANT",
  "CONSENT_REVOKE",
  "STATUS_CHANGE",
]);

export type AuditAction = z.infer<typeof AuditActionSchema>;

// -----------------------------------------------------------------------------
// ObservabilitySignalInput
// -----------------------------------------------------------------------------

export const ObservabilitySignalInputSchema = z.object({
  eventType: ObservabilityEventTypeSchema,
  severity: SignalSeveritySchema.optional(),
  subjectPersonId: z.string().optional(),
  entityType: z.string().optional(),
  entityId: z.string().optional(),
  message: z.string().min(1),
  metadata: z.record(z.unknown()).optional(),
});

export type ObservabilitySignalInput = z.infer<
  typeof ObservabilitySignalInputSchema
>;

// -----------------------------------------------------------------------------
// NotificationInput
// -----------------------------------------------------------------------------

export const NotificationInputSchema = z.object({
  userId: z.string().min(1),
  type: NotificationTypeSchema,
  title: z.string().min(1),
  message: z.string().min(1),
  link: z.string().url().optional(),
});

export type NotificationInput = z.infer<typeof NotificationInputSchema>;

// -----------------------------------------------------------------------------
// EligibilityCaseInput
// -----------------------------------------------------------------------------

export const EligibilityCaseInputSchema = z.object({
  seniorId: z.string().min(1),
  programName: z.string().min(1),
  applicationDate: z.coerce.date().optional(),
});

export type EligibilityCaseInput = z.infer<typeof EligibilityCaseInputSchema>;

// -----------------------------------------------------------------------------
// ClaimInput
// -----------------------------------------------------------------------------

export const ClaimInputSchema = z.object({
  caseId: z.string().min(1),
  amount: z.number().positive(),
  currency: z.string().default("KRW"),
  serviceDate: z.coerce.date(),
  notes: z.string().optional(),
});

export type ClaimInput = z.infer<typeof ClaimInputSchema>;
