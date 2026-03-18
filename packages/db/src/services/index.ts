// =============================================================================
// Services barrel export
// =============================================================================

// State machines
export {
  createStateMachine,
  caregiverApplicationMachine,
  matchRequestMachine,
  visitMachine,
  medicationEventMachine,
  referralMachine,
  eligibilityCaseMachine,
  carePlanMachine,
  appointmentMachine,
} from './state-machine';
export type { StateMachine } from './state-machine';

// Profile
export {
  ProfileService,
  createPersonProfile,
  updatePersonProfile,
  getPersonProfile,
  getPersonProfileByUserId,
  createSeniorProfile,
  updateSeniorProfile,
  getSeniorProfile,
} from './profile.service';
export type {
  CreatePersonProfileData,
  UpdatePersonProfileData,
  CreateSeniorProfileData,
  UpdateSeniorProfileData,
  PersonProfileWithUser,
  SeniorProfileWithPerson,
} from './profile.service';

// Consent
export {
  ConsentService,
  grantConsent,
  revokeConsent,
  getActiveConsent,
  hasConsent,
  getConsentsForPerson,
} from './consent.service';
export type { GrantConsentData, ConsentPurpose } from './consent.service';

// Caregiver Application
export {
  CaregiverApplicationService,
  createApplication,
  updateApplication,
  submitApplication,
  transitionStatus as transitionApplicationStatus,
  getApplication,
  listApplications,
  addCredential,
  verifyCredential,
} from './caregiver-application.service';
export type {
  CreateApplicationData,
  UpdateApplicationData,
  ApplicationFilters,
  AddCredentialData,
  ApplicationWithRelations,
} from './caregiver-application.service';

// Match
export {
  MatchService,
  createMatchRequest,
  searchCandidates,
  scoreCandidateAtomic,
  selectRecommendation,
  getMatchRequest,
  listMatchRequests,
} from './match.service';
export type {
  CreateMatchRequestData,
  ScoringCriteria,
  CandidateData,
  ScoreBreakdown,
  MatchRequestWithRecommendations,
} from './match.service';

// Care Plan
export {
  CarePlanService,
  createCarePlan,
  activateCarePlan,
  updateCarePlan,
  getCarePlan,
  listCarePlans,
} from './care-plan.service';
export type {
  CreateCarePlanData,
  UpdateCarePlanData,
  CarePlanWithDetails,
} from './care-plan.service';

// Visit
export {
  VisitService,
  scheduleVisit,
  acknowledgeVisit,
  checkIn,
  checkOut,
  markMissed,
  getVisit,
  listVisits,
  getUpcomingVisits,
  setVisitEventEmitter,
} from './visit.service';
export type {
  ScheduleVisitData,
  CheckInData,
  CheckOutData,
  VisitFilters,
  VisitWithRelations,
  VisitEvent,
} from './visit.service';

// Medication
export {
  MedicationService,
  createMedication,
  updateMedication,
  addSchedule,
  generateEvents,
  updateEventStatus,
  getTodayEvents,
  getOverdueEvents,
  listMedications,
  setMedicationEventEmitter,
} from './medication.service';
export type {
  CreateMedicationData,
  UpdateMedicationData,
  AddScheduleData,
  DateRange,
  MedicationWithSchedules,
} from './medication.service';

// Medical History
export {
  MedicalHistoryService,
  createEntry,
  updateEntry,
  getEntry,
  listEntries,
  getActiveConditions,
} from './medical-history.service';
export type {
  CreateMedicalHistoryEntryData,
  UpdateMedicalHistoryEntryData,
} from './medical-history.service';

// Appointment
export {
  AppointmentService,
  createAppointment,
  updateAppointment,
  updateStatus as updateAppointmentStatus,
  getAppointment,
  listAppointments,
  getUpcomingAppointments,
} from './appointment.service';
export type {
  CreateAppointmentData,
  UpdateAppointmentData,
} from './appointment.service';

// Referral
export {
  ReferralService,
  createReferral,
  updateStatus as updateReferralStatus,
  getReferral,
  listReferrals,
  setReferralEventEmitter,
} from './referral.service';
export type {
  CreateReferralData,
  ReferralFilters,
  ReferralWithProviders,
  ReferralEvent,
} from './referral.service';

// Observability
export {
  ObservabilityService,
  createSignal,
  acknowledgeSignal,
  listSignals,
  getDashboardStats,
} from './observability.service';
export type {
  CreateSignalData,
  SignalFilters,
  DashboardStats,
} from './observability.service';

// Notification
export {
  NotificationService,
  createNotification,
  markAsRead,
  markAllAsRead,
  getUnreadCount,
  listNotifications,
} from './notification.service';
export type { CreateNotificationData } from './notification.service';

// Audit
export {
  AuditService,
  log,
  listLogs,
} from './audit.service';
export type { LogAuditData, AuditLogFilters } from './audit.service';

// Shared pagination types (re-exported for convenience)
export type { Pagination, PaginatedResult } from './audit.service';
