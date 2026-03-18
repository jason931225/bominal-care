import { z } from "zod";

// -----------------------------------------------------------------------------
// Enum Schemas
// -----------------------------------------------------------------------------

export const MedicationFrequencySchema = z.enum([
  "ONCE_DAILY",
  "TWICE_DAILY",
  "THREE_TIMES_DAILY",
  "FOUR_TIMES_DAILY",
  "EVERY_OTHER_DAY",
  "WEEKLY",
  "AS_NEEDED",
  "CUSTOM",
]);

export type MedicationFrequency = z.infer<typeof MedicationFrequencySchema>;

export const MedicationEventStatusSchema = z.enum([
  "SCHEDULED",
  "REMINDER_SENT",
  "TAKEN",
  "MISSED",
  "HELD",
  "ESCALATED",
]);

export type MedicationEventStatus = z.infer<typeof MedicationEventStatusSchema>;

export const AppointmentStatusSchema = z.enum([
  "SCHEDULED",
  "CONFIRMED",
  "IN_PROGRESS",
  "COMPLETED",
  "CANCELLED",
  "NO_SHOW",
]);

export type AppointmentStatus = z.infer<typeof AppointmentStatusSchema>;

export const InstitutionReferralStatusSchema = z.enum([
  "CREATED",
  "ACCEPTED",
  "BOOKED",
  "ATTENDED",
  "DISCHARGED",
  "CLOSED",
]);

export type InstitutionReferralStatus = z.infer<
  typeof InstitutionReferralStatusSchema
>;

// -----------------------------------------------------------------------------
// MedicalHistoryInput
// -----------------------------------------------------------------------------

export const MedicalHistoryInputSchema = z.object({
  personId: z.string().min(1),
  condition: z.string().min(1),
  diagnosedAt: z.coerce.date().optional(),
  treatedBy: z.string().optional(),
  status: z.string().optional(),
  notes: z.string().optional(),
});

export type MedicalHistoryInput = z.infer<typeof MedicalHistoryInputSchema>;

// -----------------------------------------------------------------------------
// MedicationInput
// -----------------------------------------------------------------------------

export const MedicationInputSchema = z.object({
  personId: z.string().min(1),
  name: z.string().min(1),
  dosage: z.string().min(1),
  form: z.string().min(1),
  frequency: MedicationFrequencySchema,
  prescribedBy: z.string().optional(),
  startDate: z.coerce.date().optional(),
  endDate: z.coerce.date().optional(),
  sideEffects: z.string().optional(),
  notes: z.string().optional(),
});

export type MedicationInput = z.infer<typeof MedicationInputSchema>;

// -----------------------------------------------------------------------------
// MedicationScheduleInput
// -----------------------------------------------------------------------------

export const MedicationScheduleInputSchema = z.object({
  medicationId: z.string().min(1),
  timeOfDay: z.string().min(1),
  dayOfWeek: z
    .enum([
      "MONDAY",
      "TUESDAY",
      "WEDNESDAY",
      "THURSDAY",
      "FRIDAY",
      "SATURDAY",
      "SUNDAY",
    ])
    .optional(),
});

export type MedicationScheduleInput = z.infer<
  typeof MedicationScheduleInputSchema
>;

// -----------------------------------------------------------------------------
// MedicationEventUpdate
// -----------------------------------------------------------------------------

export const MedicationEventUpdateSchema = z.object({
  status: MedicationEventStatusSchema,
  takenAt: z.coerce.date().optional(),
  notes: z.string().optional(),
});

export type MedicationEventUpdate = z.infer<typeof MedicationEventUpdateSchema>;

// -----------------------------------------------------------------------------
// AppointmentInput
// -----------------------------------------------------------------------------

export const AppointmentInputSchema = z.object({
  personId: z.string().min(1),
  institutionName: z.string().min(1),
  institutionType: z
    .enum([
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
    ])
    .optional(),
  appointmentDate: z.coerce.date(),
  purpose: z.string().optional(),
  notes: z.string().optional(),
  address: z.string().optional(),
});

export type AppointmentInput = z.infer<typeof AppointmentInputSchema>;

// -----------------------------------------------------------------------------
// ReferralInput
// -----------------------------------------------------------------------------

export const ReferralInputSchema = z.object({
  fromProviderId: z.string().min(1),
  toProviderId: z.string().min(1),
  seniorPersonId: z.string().min(1),
  reason: z.string().min(1),
  notes: z.string().optional(),
});

export type ReferralInput = z.infer<typeof ReferralInputSchema>;
