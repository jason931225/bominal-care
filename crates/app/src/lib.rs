pub mod components;
pub mod layouts;
pub mod pages;
pub mod server_fns;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::layouts::{
    CaregiverLayout, FamilyLayout, GovernmentLayout, InternalLayout, SeniorLayout,
};
use crate::pages::auth;
use crate::pages::caregiver as caregiver_pages;
use crate::pages::family as family_pages;
use crate::pages::government as government_pages;
use crate::pages::internal as internal_pages;
use crate::pages::senior as senior_pages;

/// Root application component with router.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <FlatRoutes fallback=|| view! { <NotFoundPage /> }>
                // --- Auth (shared) ---
                <Route path=path!("/auth/signin") view=auth::SignInPage />
                <Route path=path!("/auth/error") view=auth::ErrorPage />

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
                <Route path=path!("/family/timeline") view=|| view! { <FamilyLayout><family_pages::StubPage /></FamilyLayout> } />
                <Route path=path!("/family/matching") view=|| view! { <FamilyLayout><family_pages::StubPage /></FamilyLayout> } />
                <Route path=path!("/family/approvals") view=|| view! { <FamilyLayout><family_pages::StubPage /></FamilyLayout> } />
                <Route path=path!("/family/payments") view=|| view! { <FamilyLayout><family_pages::StubPage /></FamilyLayout> } />
                <Route path=path!("/family/medications") view=|| view! { <FamilyLayout><family_pages::StubPage /></FamilyLayout> } />
                <Route path=path!("/family/care") view=|| view! { <FamilyLayout><family_pages::StubPage /></FamilyLayout> } />
                <Route path=path!("/family/notifications") view=|| view! { <FamilyLayout><family_pages::StubPage /></FamilyLayout> } />
                <Route path=path!("/family/settings") view=|| view! { <FamilyLayout><family_pages::StubPage /></FamilyLayout> } />

                // --- Internal (provider) portal ---
                <Route path=path!("/internal") view=|| view! { <InternalLayout><internal_pages::DashboardPage /></InternalLayout> } />
                <Route path=path!("/internal/clients") view=|| view! { <InternalLayout><internal_pages::StubPage /></InternalLayout> } />
                <Route path=path!("/internal/caregivers") view=|| view! { <InternalLayout><internal_pages::StubPage /></InternalLayout> } />
                <Route path=path!("/internal/schedules") view=|| view! { <InternalLayout><internal_pages::StubPage /></InternalLayout> } />
                <Route path=path!("/internal/quality") view=|| view! { <InternalLayout><internal_pages::StubPage /></InternalLayout> } />
                <Route path=path!("/internal/referrals") view=|| view! { <InternalLayout><internal_pages::StubPage /></InternalLayout> } />
                <Route path=path!("/internal/compliance") view=|| view! { <InternalLayout><internal_pages::StubPage /></InternalLayout> } />
                <Route path=path!("/internal/reports") view=|| view! { <InternalLayout><internal_pages::StubPage /></InternalLayout> } />
                <Route path=path!("/internal/settings") view=|| view! { <InternalLayout><internal_pages::StubPage /></InternalLayout> } />

                // --- Government portal ---
                <Route path=path!("/gov") view=|| view! { <GovernmentLayout><government_pages::DashboardPage /></GovernmentLayout> } />
                <Route path=path!("/gov/eligibility") view=|| view! { <GovernmentLayout><government_pages::StubPage /></GovernmentLayout> } />
                <Route path=path!("/gov/providers") view=|| view! { <GovernmentLayout><government_pages::StubPage /></GovernmentLayout> } />
                <Route path=path!("/gov/programs") view=|| view! { <GovernmentLayout><government_pages::StubPage /></GovernmentLayout> } />
                <Route path=path!("/gov/audit") view=|| view! { <GovernmentLayout><government_pages::StubPage /></GovernmentLayout> } />
                <Route path=path!("/gov/observability") view=|| view! { <GovernmentLayout><government_pages::StubPage /></GovernmentLayout> } />
                <Route path=path!("/gov/settings") view=|| view! { <GovernmentLayout><government_pages::StubPage /></GovernmentLayout> } />

                // --- Caregiver portal ---
                <Route path=path!("/caregiver") view=|| view! { <CaregiverLayout><caregiver_pages::DashboardPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/schedule") view=|| view! { <CaregiverLayout><caregiver_pages::StubPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/clients") view=|| view! { <CaregiverLayout><caregiver_pages::StubPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/medications") view=|| view! { <CaregiverLayout><caregiver_pages::StubPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/notifications") view=|| view! { <CaregiverLayout><caregiver_pages::StubPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/profile") view=|| view! { <CaregiverLayout><caregiver_pages::StubPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/apply") view=|| view! { <CaregiverLayout><caregiver_pages::StubPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/tasks") view=|| view! { <CaregiverLayout><caregiver_pages::StubPage /></CaregiverLayout> } />
                <Route path=path!("/caregiver/notes") view=|| view! { <CaregiverLayout><caregiver_pages::StubPage /></CaregiverLayout> } />
            </FlatRoutes>
        </Router>
    }
}

/// 404 page component.
#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-gray-300">"404"</h1>
                <p class="mt-4 text-lg text-gray-600">"페이지를 찾을 수 없습니다"</p>
                <a href="/" class="mt-6 inline-block px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                    "홈으로 돌아가기"
                </a>
            </div>
        </div>
    }
}
