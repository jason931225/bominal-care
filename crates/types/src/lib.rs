pub mod common;
pub mod enums;
pub mod events;
pub mod inputs;
pub mod models;
pub mod rbac;
pub mod state_machines;

// Re-export key types for convenience
pub use common::{ApiResponse, DateRange, PaginationMeta, PaginationParams};
pub use enums::*;
pub use events::{DomainEvent, DomainEventType, RuleAction, RuleResult};
pub use inputs::{ObservabilitySignalInput, ReferralInput, SessionUser};
pub use models::*;
pub use rbac::{has_permission, Action, Permission, Resource, Scope};
pub use state_machines::StateMachine;
