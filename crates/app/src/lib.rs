pub mod api;
pub mod components;
pub mod i18n;
pub mod layouts;
pub mod pages;

// =============================================================================
// Auth context — shared across all pages
// =============================================================================

use serde::Deserialize;
use uuid::Uuid;

/// Client-side representation of the authenticated user.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: bominal_types::UserRole,
    pub kyc_level: bominal_types::KycLevel,
    pub tenant_id: Option<Uuid>,
}

/// Convenience accessor for the auth context signal.
pub fn use_auth() -> leptos::prelude::RwSignal<Option<AuthUser>> {
    leptos::prelude::use_context::<leptos::prelude::RwSignal<Option<AuthUser>>>()
        .expect("AuthContext not provided")
}

/// WASM entry point — replaces the #app loading div with the real app.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    use leptos::prelude::*;
    use leptos::web_sys;
    // Remove the loading placeholder
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(loading) = document.get_element_by_id("app") {
                loading.set_inner_html("");
            }
        }
    }
    mount_to_body(App);
}

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::i18n::t;

use crate::layouts::{
    CaregiverLayout, FamilyLayout, GovernmentLayout, InternalLayout, MedicalLayout, SeniorLayout,
};
use crate::pages::auth;
use crate::pages::caregiver as caregiver_pages;
use crate::pages::family as family_pages;
use crate::pages::government as government_pages;
use crate::pages::internal as internal_pages;
use crate::pages::medical as medical_pages;
use crate::pages::senior as senior_pages;

/// Root application component with router.
#[component]
pub fn App() -> impl IntoView {
    // Provide auth context to the entire app
    let auth_user = RwSignal::new(None::<AuthUser>);
    provide_context(auth_user);

    // Fetch current user on mount
    let _auth_check = LocalResource::new(move || async move {
        match crate::api::get_silent::<AuthUser>("/api/auth/me").await {
            Ok(resp) if resp.success => {
                auth_user.set(resp.data);
            }
            _ => {
                auth_user.set(None);
            }
        }
    });

    view! {
        <Router>
            <FlatRoutes fallback=|| view! { <NotFoundPage /> }>
                // --- Auth & legal (shared) ---
                <Route path=path!("/auth/signin") view=auth::SignInPage />
                <Route path=path!("/auth/error") view=auth::ErrorPage />
                <Route path=path!("/terms") view=auth::TermsPage />
                <Route path=path!("/privacy") view=auth::PrivacyPage />

                // --- Senior portal (default) ---
                <Route path=path!("/") view=|| view! { <SeniorLayout><senior_pages::DashboardPage /></SeniorLayout> } />
                <Route path=path!("/appointments") view=|| view! { <SeniorLayout><senior_pages::AppointmentsListPage /></SeniorLayout> } />
                <Route path=path!("/appointments/new") view=|| view! { <SeniorLayout><senior_pages::AppointmentNewPage /></SeniorLayout> } />
                <Route path=path!("/medications") view=|| view! { <SeniorLayout><senior_pages::MedicationsListPage /></SeniorLayout> } />
                <Route path=path!("/medication-log") view=|| view! { <SeniorLayout><senior_pages::MedicationLogPage /></SeniorLayout> } />
                <Route path=path!("/care") view=|| view! { <SeniorLayout><senior_pages::CarePlanPage /></SeniorLayout> } />
                <Route path=path!("/emergency") view=|| view! { <SeniorLayout><senior_pages::EmergencyPage /></SeniorLayout> } />
                <Route path=path!("/services") view=|| view! { <SeniorLayout><senior_pages::ServicesPage /></SeniorLayout> } />
                <Route path=path!("/services/meals") view=|| view! { <SeniorLayout><senior_pages::ServicesMealsPage /></SeniorLayout> } />
                <Route path=path!("/services/partners") view=|| view! { <SeniorLayout><senior_pages::ServicesPartnersPage /></SeniorLayout> } />
                <Route path=path!("/services/rides") view=|| view! { <SeniorLayout><senior_pages::ServicesRidesPage /></SeniorLayout> } />
                <Route path=path!("/profile") view=|| view! { <SeniorLayout><senior_pages::ProfilePage /></SeniorLayout> } />
                <Route path=path!("/consent") view=|| view! { <SeniorLayout><senior_pages::ConsentPage /></SeniorLayout> } />
                <Route path=path!("/notifications") view=|| view! { <SeniorLayout><senior_pages::NotificationsPage /></SeniorLayout> } />
                <Route path=path!("/settings") view=|| view! { <SeniorLayout><senior_pages::SettingsPage /></SeniorLayout> } />
                <Route path=path!("/medical-history") view=|| view! { <SeniorLayout><senior_pages::MedicalHistoryPage /></SeniorLayout> } />
                <Route path=path!("/housing") view=|| view! { <SeniorLayout><senior_pages::HousingPage /></SeniorLayout> } />
                <Route path=path!("/opportunities") view=|| view! { <SeniorLayout><senior_pages::OpportunitiesPage /></SeniorLayout> } />
                <Route path=path!("/more") view=|| view! { <SeniorLayout><senior_pages::MorePage /></SeniorLayout> } />

                // --- Family portal ---
                <Route path=path!("/family") view=|| view! { <FamilyLayout><family_pages::DashboardPage /></FamilyLayout> } />
                <Route path=path!("/family/timeline") view=|| view! { <FamilyLayout><family_pages::TimelinePage /></FamilyLayout> } />
                <Route path=path!("/family/matching") view=|| view! { <FamilyLayout><family_pages::MatchingSearchPage /></FamilyLayout> } />
                <Route path=path!("/family/matching/results") view=|| view! { <FamilyLayout><family_pages::MatchingResultsPage /></FamilyLayout> } />
                <Route path=path!("/family/matching/detail") view=|| view! { <FamilyLayout><family_pages::MatchingDetailPage /></FamilyLayout> } />
                <Route path=path!("/family/approvals") view=|| view! { <FamilyLayout><family_pages::ApprovalsListPage /></FamilyLayout> } />
                <Route path=path!("/family/approvals/:id") view=|| view! { <FamilyLayout><family_pages::ApprovalDetailPage /></FamilyLayout> } />
                <Route path=path!("/family/payments") view=|| view! { <FamilyLayout><family_pages::PaymentsListPage /></FamilyLayout> } />
                <Route path=path!("/family/payments/:id") view=|| view! { <FamilyLayout><family_pages::PaymentDetailPage /></FamilyLayout> } />
                <Route path=path!("/family/medications") view=|| view! { <FamilyLayout><family_pages::MedicationsPage /></FamilyLayout> } />
                <Route path=path!("/family/care") view=|| view! { <FamilyLayout><family_pages::CarePlanPage /></FamilyLayout> } />
                <Route path=path!("/family/help") view=|| view! { <FamilyLayout><family_pages::HelpSeniorPage /></FamilyLayout> } />
                <Route path=path!("/family/help/book") view=|| view! { <FamilyLayout><family_pages::HelpBookPage /></FamilyLayout> } />
                <Route path=path!("/family/help/emergency") view=|| view! { <FamilyLayout><family_pages::HelpEmergencyPage /></FamilyLayout> } />
                <Route path=path!("/family/help/report") view=|| view! { <FamilyLayout><family_pages::HelpReportPage /></FamilyLayout> } />
                <Route path=path!("/family/observability") view=|| view! { <FamilyLayout><family_pages::ObservabilityPage /></FamilyLayout> } />
                <Route path=path!("/family/eligibility") view=|| view! { <FamilyLayout><family_pages::EligibilityPage /></FamilyLayout> } />
                <Route path=path!("/family/eligibility/apply") view=|| view! { <FamilyLayout><family_pages::EligibilityApplyPage /></FamilyLayout> } />
                <Route path=path!("/family/documents") view=|| view! { <FamilyLayout><family_pages::DocumentsPage /></FamilyLayout> } />
                <Route path=path!("/family/documents/:id") view=|| view! { <FamilyLayout><family_pages::DocumentDetailPage /></FamilyLayout> } />
                <Route path=path!("/family/notifications") view=|| view! { <FamilyLayout><family_pages::NotificationsPage /></FamilyLayout> } />
                <Route path=path!("/family/profile") view=|| view! { <FamilyLayout><family_pages::ProfilePage /></FamilyLayout> } />
                <Route path=path!("/family/settings") view=|| view! { <FamilyLayout><family_pages::SettingsPage /></FamilyLayout> } />

                // --- Internal (provider) portal ---
                <Route path=path!("/internal") view=|| view! { <InternalLayout><internal_pages::DashboardPage /></InternalLayout> } />
                <Route path=path!("/internal/clients") view=|| view! { <InternalLayout><internal_pages::ClientsListPage /></InternalLayout> } />
                <Route path=path!("/internal/clients/:id") view=|| view! { <InternalLayout><internal_pages::ClientDetailPage /></InternalLayout> } />
                <Route path=path!("/internal/clients/:id/care-plan") view=|| view! { <InternalLayout><internal_pages::ClientCarePlanPage /></InternalLayout> } />
                <Route path=path!("/internal/clients/:id/care-plan/edit") view=|| view! { <InternalLayout><internal_pages::ClientCarePlanEditPage /></InternalLayout> } />
                <Route path=path!("/internal/caregivers") view=|| view! { <InternalLayout><internal_pages::CaregiversListPage /></InternalLayout> } />
                <Route path=path!("/internal/caregivers/:id") view=|| view! { <InternalLayout><internal_pages::CaregiverDetailPage /></InternalLayout> } />
                <Route path=path!("/internal/caregivers/applications") view=|| view! { <InternalLayout><internal_pages::ApplicationsListPage /></InternalLayout> } />
                <Route path=path!("/internal/caregivers/applications/:id") view=|| view! { <InternalLayout><internal_pages::ApplicationDetailPage /></InternalLayout> } />
                <Route path=path!("/internal/schedules") view=|| view! { <InternalLayout><internal_pages::SchedulesPage /></InternalLayout> } />
                <Route path=path!("/internal/schedules/conflicts") view=|| view! { <InternalLayout><internal_pages::ScheduleConflictsPage /></InternalLayout> } />
                <Route path=path!("/internal/quality") view=|| view! { <InternalLayout><internal_pages::QualityPage /></InternalLayout> } />
                <Route path=path!("/internal/incidents") view=|| view! { <InternalLayout><internal_pages::IncidentsListPage /></InternalLayout> } />
                <Route path=path!("/internal/incidents/:id") view=|| view! { <InternalLayout><internal_pages::IncidentDetailPage /></InternalLayout> } />
                <Route path=path!("/internal/referrals") view=|| view! { <InternalLayout><internal_pages::ReferralsListPage /></InternalLayout> } />
                <Route path=path!("/internal/referrals/new") view=|| view! { <InternalLayout><internal_pages::ReferralNewPage /></InternalLayout> } />
                <Route path=path!("/internal/compliance") view=|| view! { <InternalLayout><internal_pages::CompliancePage /></InternalLayout> } />
                <Route path=path!("/internal/reports") view=|| view! { <InternalLayout><internal_pages::ReportsPage /></InternalLayout> } />
                <Route path=path!("/internal/settings") view=|| view! { <InternalLayout><internal_pages::SettingsPage /></InternalLayout> } />

                // --- Government portal ---
                <Route path=path!("/gov") view=|| view! { <GovernmentLayout><government_pages::DashboardPage /></GovernmentLayout> } />
                <Route path=path!("/gov/eligibility") view=|| view! { <GovernmentLayout><government_pages::EligibilityListPage /></GovernmentLayout> } />
                <Route path=path!("/gov/eligibility/:id") view=|| view! { <GovernmentLayout><government_pages::EligibilityDetailPage /></GovernmentLayout> } />
                <Route path=path!("/gov/providers") view=|| view! { <GovernmentLayout><government_pages::ProvidersPage /></GovernmentLayout> } />
                <Route path=path!("/gov/programs") view=|| view! { <GovernmentLayout><government_pages::ProgramsPage /></GovernmentLayout> } />
                <Route path=path!("/gov/audit") view=|| view! { <GovernmentLayout><government_pages::AuditPage /></GovernmentLayout> } />
                <Route path=path!("/gov/observability") view=|| view! { <GovernmentLayout><government_pages::ObservabilityPage /></GovernmentLayout> } />
                <Route path=path!("/gov/settings") view=|| view! { <GovernmentLayout><government_pages::SettingsPage /></GovernmentLayout> } />

                // --- Caregiver portal ---
                <Route path=path!("/caregiver") view=|| view! { <CaregiverLayout><caregiver_pages::DashboardPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/schedule") view=|| view! { <CaregiverLayout><caregiver_pages::ScheduleListPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/schedule/:id") view=|| view! { <CaregiverLayout><caregiver_pages::ScheduleDetailPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/check-in/:id") view=|| view! { <CaregiverLayout><caregiver_pages::CheckInPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/check-out/:id") view=|| view! { <CaregiverLayout><caregiver_pages::CheckOutPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/clients") view=|| view! { <CaregiverLayout><caregiver_pages::ClientsListPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/clients/:id") view=|| view! { <CaregiverLayout><caregiver_pages::ClientDetailPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/clients/:id/care-plan") view=|| view! { <CaregiverLayout><caregiver_pages::ClientCarePlanPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/clients/:id/medications") view=|| view! { <CaregiverLayout><caregiver_pages::ClientMedicationsPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/medications") view=|| view! { <CaregiverLayout><caregiver_pages::MedicationsPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/notifications") view=|| view! { <CaregiverLayout><caregiver_pages::NotificationsPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/profile") view=|| view! { <CaregiverLayout><caregiver_pages::ProfilePage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/profile/availability") view=|| view! { <CaregiverLayout><caregiver_pages::ProfileAvailabilityPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply") view=|| view! { <CaregiverLayout><caregiver_pages::ApplyOverviewPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply/identity") view=|| view! { <CaregiverLayout><caregiver_pages::ApplyIdentityPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply/credentials") view=|| view! { <CaregiverLayout><caregiver_pages::ApplyCredentialsPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply/service-region") view=|| view! { <CaregiverLayout><caregiver_pages::ApplyServiceRegionPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply/schedule") view=|| view! { <CaregiverLayout><caregiver_pages::ApplySchedulePage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply/services") view=|| view! { <CaregiverLayout><caregiver_pages::ApplyServicesPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply/references") view=|| view! { <CaregiverLayout><caregiver_pages::ApplyReferencesPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply/review") view=|| view! { <CaregiverLayout><caregiver_pages::ApplyReviewPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply/status") view=|| view! { <CaregiverLayout><caregiver_pages::ApplyStatusPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/tasks") view=|| view! { <CaregiverLayout><caregiver_pages::TasksListPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/tasks/:id") view=|| view! { <CaregiverLayout><caregiver_pages::TaskDetailPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/notes") view=|| view! { <CaregiverLayout><caregiver_pages::NotesListPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/notes/new") view=|| view! { <CaregiverLayout><caregiver_pages::NoteNewPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/incident") view=|| view! { <CaregiverLayout><caregiver_pages::IncidentPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/settings") view=|| view! { <CaregiverLayout><caregiver_pages::SettingsPage /></CaregiverLayout> } />

                // --- Medical portal ---
                <Route path=path!("/medical") view=|| view! { <MedicalLayout><medical_pages::DashboardPage /></MedicalLayout> } />
                <Route path=path!("/medical/patients") view=|| view! { <MedicalLayout><medical_pages::PatientsPage /></MedicalLayout> } />
                <Route path=path!("/medical/patients/session") view=|| view! { <MedicalLayout><medical_pages::PatientSessionPage /></MedicalLayout> } />
                <Route path=path!("/medical/prescriptions") view=|| view! { <MedicalLayout><medical_pages::PrescriptionsPage /></MedicalLayout> } />
                <Route path=path!("/medical/appointments") view=|| view! { <MedicalLayout><medical_pages::AppointmentsPage /></MedicalLayout> } />
                <Route path=path!("/medical/history") view=|| view! { <MedicalLayout><medical_pages::HistoryPage /></MedicalLayout> } />
            </FlatRoutes>
        </Router>
    }
}

/// 404 page component.
#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-surface-page">
            <div class="text-center animate-fade-in">
                <h1 class="text-6xl font-bold text-txt-disabled">"404"</h1>
                <p class="mt-4 text-lg text-txt-secondary">{t("error.not_found")}</p>
                <a href="/" class="mt-6 inline-block px-6 py-3 bg-primary text-white rounded-xl hover:bg-primary-hover active:scale-[0.98] transition-all">
                    {t("error.go_home")}
                </a>
            </div>
        </div>
    }
}
