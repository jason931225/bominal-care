pub mod common;
pub mod enums;
pub mod events;
pub mod inputs;
pub mod ledger;
pub mod medical;
pub mod models;
pub mod platform_event;
pub mod rbac;
pub mod state_machines;

// Re-export key types for convenience
pub use common::{ApiResponse, DateRange, PaginationMeta, PaginationParams};
pub use enums::*;
pub use events::{DomainEvent, DomainEventType, RuleAction, RuleResult};
pub use inputs::{ObservabilitySignalInput, ReferralInput, SessionUser};
pub use medical::{
    ClinicalEncounter, CreateAllergyInput, CreateEncounterInput, CreateLabResultInput,
    CreatePrescriptionInput, DocumentTransferRequest, GenericSubstitutionRecord, LabResult,
    MedicalHandoffSession, MedicalProfessionalProfile, PatientAllergy, Prescription,
    StartHandoffInput,
};
pub use models::*;
pub use platform_event::{
    AccessPolicy, AlertFiring, AlertRule, ComplianceReport, EventBuilder, EventCategory,
    IntegrityVerification, NewPlatformEvent, PlatformEvent, PolicyChangeLog, PolicyEffect,
    Sensitivity, SystemMetric,
};
pub use ledger::{
    ActorType, AppointmentLedgerEntry, CarePlanLedgerEntry, LedgerAction, MedicationLedgerEntry,
    NewLedgerEntry,
};
pub use rbac::{has_permission, Action, Permission, Resource, Scope};
pub use state_machines::StateMachine;
