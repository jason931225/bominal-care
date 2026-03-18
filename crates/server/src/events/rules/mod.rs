// =============================================================================
// Rule Engine + Business Rules
// Ported from packages/events/src/rules/engine.ts
//            and packages/events/src/rules/definitions.ts
// =============================================================================

use std::collections::HashMap;

use bominal_types::events::{DomainEvent, DomainEventType, RuleAction, RuleResult};

// ---------------------------------------------------------------------------
// RuleContext — async trait for looking up recent events and consent
// ---------------------------------------------------------------------------

pub trait RuleContext: Send + Sync {
    fn get_recent_events(
        &self,
        subject_person_id: &str,
        event_type: DomainEventType,
        within_hours: u64,
    ) -> impl Future<Output = Vec<DomainEvent>> + Send;

    fn get_consent(
        &self,
        subject_person_id: &str,
    ) -> impl Future<Output = Option<ConsentInfo>> + Send;
}

#[derive(Debug, Clone)]
pub struct ConsentInfo {
    pub purpose: String,
}

// ---------------------------------------------------------------------------
// RuleContextDyn — object-safe version of RuleContext for dynamic dispatch
// ---------------------------------------------------------------------------

pub trait RuleContextDyn: Send + Sync {
    fn get_recent_events<'a>(
        &'a self,
        subject_person_id: &'a str,
        event_type: DomainEventType,
        within_hours: u64,
    ) -> std::pin::Pin<Box<dyn Future<Output = Vec<DomainEvent>> + Send + 'a>>;

    fn get_consent<'a>(
        &'a self,
        subject_person_id: &'a str,
    ) -> std::pin::Pin<Box<dyn Future<Output = Option<ConsentInfo>> + Send + 'a>>;
}

impl<T: RuleContext> RuleContextDyn for T {
    fn get_recent_events<'a>(
        &'a self,
        subject_person_id: &'a str,
        event_type: DomainEventType,
        within_hours: u64,
    ) -> std::pin::Pin<Box<dyn Future<Output = Vec<DomainEvent>> + Send + 'a>> {
        Box::pin(RuleContext::get_recent_events(
            self,
            subject_person_id,
            event_type,
            within_hours,
        ))
    }

    fn get_consent<'a>(
        &'a self,
        subject_person_id: &'a str,
    ) -> std::pin::Pin<Box<dyn Future<Output = Option<ConsentInfo>> + Send + 'a>> {
        Box::pin(RuleContext::get_consent(self, subject_person_id))
    }
}

// ---------------------------------------------------------------------------
// Rule — trait for business rules that evaluate domain events
// ---------------------------------------------------------------------------

pub trait Rule: Send + Sync {
    fn rule_id(&self) -> &str;
    fn rule_name(&self) -> &str;
    fn event_types(&self) -> &[DomainEventType];

    fn evaluate(
        &self,
        event: &DomainEvent,
        context: &dyn RuleContextDyn,
    ) -> impl Future<Output = RuleResult> + Send;
}

// ---------------------------------------------------------------------------
// RuleDyn — object-safe wrapper for Rule
// ---------------------------------------------------------------------------

/// Object-safe wrapper trait for Rule (needed because `Rule::evaluate` uses
/// `impl Future`, which prevents `dyn Rule`).
pub trait RuleDyn: Send + Sync {
    fn dyn_rule_id(&self) -> &str;
    fn dyn_rule_name(&self) -> &str;
    fn dyn_event_types(&self) -> &[DomainEventType];
    fn evaluate_dyn<'a>(
        &'a self,
        event: &'a DomainEvent,
        context: &'a dyn RuleContextDyn,
    ) -> std::pin::Pin<Box<dyn Future<Output = RuleResult> + Send + 'a>>;
}

impl<T: Rule> RuleDyn for T {
    fn dyn_rule_id(&self) -> &str {
        Rule::rule_id(self)
    }

    fn dyn_rule_name(&self) -> &str {
        Rule::rule_name(self)
    }

    fn dyn_event_types(&self) -> &[DomainEventType] {
        Rule::event_types(self)
    }

    fn evaluate_dyn<'a>(
        &'a self,
        event: &'a DomainEvent,
        context: &'a dyn RuleContextDyn,
    ) -> std::pin::Pin<Box<dyn Future<Output = RuleResult> + Send + 'a>> {
        Box::pin(Rule::evaluate(self, event, context))
    }
}

// ---------------------------------------------------------------------------
// RuleEngine — registers rules and evaluates them against events
// ---------------------------------------------------------------------------

pub struct RuleEngine {
    rules: HashMap<String, Box<dyn RuleDyn>>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn register<R: Rule + 'static>(&mut self, rule: R) {
        self.rules
            .insert(rule.rule_id().to_owned(), Box::new(rule));
    }

    pub async fn evaluate(
        &self,
        event: &DomainEvent,
        context: &dyn RuleContextDyn,
    ) -> Vec<RuleResult> {
        let matching: Vec<&Box<dyn RuleDyn>> = self
            .rules
            .values()
            .filter(|rule| rule.dyn_event_types().contains(&event.event_type))
            .collect();

        let mut results = Vec::with_capacity(matching.len());
        for rule in matching {
            let result = rule.evaluate_dyn(event, context).await;
            results.push(result);
        }

        results
    }

    /// Register the four built-in business rules.
    pub fn register_defaults(&mut self) {
        self.register(MissedMedicationAlert);
        self.register(MissedVisitAlert);
        self.register(HighSeveritySymptom);
        self.register(MissingDocsReminder);
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Helper: build a non-triggered result
// ---------------------------------------------------------------------------

fn not_triggered(name: &str) -> RuleResult {
    RuleResult {
        rule_name: name.to_owned(),
        triggered: false,
        actions: vec![],
    }
}

// ===========================================================================
// Rule 1: MissedMedicationAlert
// 2+ medication.missed in 48h -> alert family + supervisor
// ===========================================================================

const MISSED_MED_NAME: &str = "MissedMedicationAlert";

pub struct MissedMedicationAlert;

impl Rule for MissedMedicationAlert {
    fn rule_id(&self) -> &str {
        "missed-medication-alert"
    }

    fn rule_name(&self) -> &str {
        MISSED_MED_NAME
    }

    fn event_types(&self) -> &[DomainEventType] {
        &[DomainEventType::MedicationMissed]
    }

    async fn evaluate(
        &self,
        event: &DomainEvent,
        context: &dyn RuleContextDyn,
    ) -> RuleResult {
        let subject_id = match &event.subject_person_id {
            Some(id) if !id.is_empty() => id,
            _ => return not_triggered(MISSED_MED_NAME),
        };

        let recent_missed = context
            .get_recent_events(subject_id, DomainEventType::MedicationMissed, 48)
            .await;

        if recent_missed.len() < 2 {
            return not_triggered(MISSED_MED_NAME);
        }

        let message = format!(
            "Senior (personId: {}) has missed {} medications in the past 48 hours. \
             Immediate follow-up required.",
            subject_id,
            recent_missed.len()
        );

        RuleResult {
            rule_name: MISSED_MED_NAME.to_owned(),
            triggered: true,
            actions: vec![
                RuleAction::Notify {
                    user_id: "role:family".to_owned(),
                    title: "Missed Medication Alert".to_owned(),
                    message: message.clone(),
                    notification_type: "ALERT".to_owned(),
                },
                RuleAction::Notify {
                    user_id: "role:caregiver_supervisor".to_owned(),
                    title: "Missed Medication Alert".to_owned(),
                    message,
                    notification_type: "ALERT".to_owned(),
                },
            ],
        }
    }
}

// ===========================================================================
// Rule 2: MissedVisitAlert
// Any visit.missed -> escalate to supervisor (critical)
// ===========================================================================

const MISSED_VISIT_NAME: &str = "MissedVisitAlert";

pub struct MissedVisitAlert;

impl Rule for MissedVisitAlert {
    fn rule_id(&self) -> &str {
        "missed-visit-alert"
    }

    fn rule_name(&self) -> &str {
        MISSED_VISIT_NAME
    }

    fn event_types(&self) -> &[DomainEventType] {
        &[DomainEventType::VisitMissed]
    }

    async fn evaluate(
        &self,
        event: &DomainEvent,
        _context: &dyn RuleContextDyn,
    ) -> RuleResult {
        let subject_part = event
            .subject_person_id
            .as_deref()
            .map(|id| format!(" for senior (personId: {})", id))
            .unwrap_or_default();

        let visit_part = event
            .entity_id
            .as_deref()
            .map(|id| format!(" (visitId: {})", id))
            .unwrap_or_default();

        let reason = format!(
            "A scheduled visit was missed{}{}. Immediate action required.",
            subject_part, visit_part
        );

        RuleResult {
            rule_name: MISSED_VISIT_NAME.to_owned(),
            triggered: true,
            actions: vec![RuleAction::Escalate {
                to_role: "supervisor".to_owned(),
                reason,
            }],
        }
    }
}

// ===========================================================================
// Rule 3: HighSeveritySymptom
// symptom.reported with HIGH/CRITICAL severity + medical consent
// -> route to medical queue
// ===========================================================================

const HIGH_SEVERITY_NAME: &str = "HighSeveritySymptom";

pub struct HighSeveritySymptom;

impl HighSeveritySymptom {
    fn is_high_severity(value: &str) -> bool {
        matches!(value, "HIGH" | "CRITICAL")
    }
}

impl Rule for HighSeveritySymptom {
    fn rule_id(&self) -> &str {
        "high-severity-symptom"
    }

    fn rule_name(&self) -> &str {
        HIGH_SEVERITY_NAME
    }

    fn event_types(&self) -> &[DomainEventType] {
        &[DomainEventType::SymptomReported]
    }

    async fn evaluate(
        &self,
        event: &DomainEvent,
        context: &dyn RuleContextDyn,
    ) -> RuleResult {
        let subject_id = match &event.subject_person_id {
            Some(id) if !id.is_empty() => id,
            _ => return not_triggered(HIGH_SEVERITY_NAME),
        };

        let severity_str = event
            .metadata
            .as_ref()
            .and_then(|m| m.get("severity"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if !Self::is_high_severity(severity_str) {
            return not_triggered(HIGH_SEVERITY_NAME);
        }

        let consent = context.get_consent(subject_id).await;
        let has_medical_consent = consent
            .as_ref()
            .map(|c| c.purpose == "MEDICAL_SHARE" || c.purpose == "BOTH_SHARE")
            .unwrap_or(false);

        if !has_medical_consent {
            return not_triggered(HIGH_SEVERITY_NAME);
        }

        let message = format!(
            "A {} severity symptom has been reported for senior (personId: {}). \
             Medical review required.",
            severity_str, subject_id
        );

        let notification_type = if severity_str == "CRITICAL" {
            "ACTION_REQUIRED"
        } else {
            "ALERT"
        };

        RuleResult {
            rule_name: HIGH_SEVERITY_NAME.to_owned(),
            triggered: true,
            actions: vec![
                RuleAction::Escalate {
                    to_role: "medical_review_queue".to_owned(),
                    reason: message.clone(),
                },
                RuleAction::Notify {
                    user_id: "role:medical_review_queue".to_owned(),
                    title: "High Severity Symptom".to_owned(),
                    message,
                    notification_type: notification_type.to_owned(),
                },
            ],
        }
    }
}

// ===========================================================================
// Rule 4: MissingDocsReminder
// eligibility.status_changed with DOCS_MISSING for 7+ days
// -> remind family + create caseworker task
// ===========================================================================

const MISSING_DOCS_NAME: &str = "MissingDocsReminder";
const DOCS_MISSING_THRESHOLD_DAYS: i64 = 7;

pub struct MissingDocsReminder;

impl Rule for MissingDocsReminder {
    fn rule_id(&self) -> &str {
        "missing-docs-reminder"
    }

    fn rule_name(&self) -> &str {
        MISSING_DOCS_NAME
    }

    fn event_types(&self) -> &[DomainEventType] {
        &[DomainEventType::EligibilityStatusChanged]
    }

    async fn evaluate(
        &self,
        event: &DomainEvent,
        _context: &dyn RuleContextDyn,
    ) -> RuleResult {
        let subject_id = match &event.subject_person_id {
            Some(id) if !id.is_empty() => id,
            _ => return not_triggered(MISSING_DOCS_NAME),
        };

        let new_status = event
            .metadata
            .as_ref()
            .and_then(|m| m.get("newStatus"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if new_status != "DOCS_MISSING" {
            return not_triggered(MISSING_DOCS_NAME);
        }

        // Determine when the status changed. Fall back to event timestamp.
        let change_date = event
            .metadata
            .as_ref()
            .and_then(|m| m.get("statusChangedAt"))
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or(event.occurred_at);

        let days_since_change = (chrono::Utc::now() - change_date).num_days();

        if days_since_change < DOCS_MISSING_THRESHOLD_DAYS {
            return not_triggered(MISSING_DOCS_NAME);
        }

        let family_message = format!(
            "Required documents for the eligibility case of senior (personId: {}) \
             have been missing for {} days. Please submit the missing documents \
             as soon as possible.",
            subject_id, days_since_change
        );

        let task_description = format!(
            "Follow up with family of senior (personId: {}): eligibility documents \
             have been missing for {} days.",
            subject_id, days_since_change
        );

        RuleResult {
            rule_name: MISSING_DOCS_NAME.to_owned(),
            triggered: true,
            actions: vec![
                RuleAction::Notify {
                    user_id: "role:family".to_owned(),
                    title: "Missing Documents Reminder".to_owned(),
                    message: family_message,
                    notification_type: "WARNING".to_owned(),
                },
                RuleAction::CreateTask {
                    assigned_to: "role:case_worker".to_owned(),
                    title: "Follow up on missing eligibility documents".to_owned(),
                    description: task_description,
                },
            ],
        }
    }
}
