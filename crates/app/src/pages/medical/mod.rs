// =============================================================================
// Medical (Clinician) Portal Pages
// =============================================================================
//
// Bominal Medical portal for medical staff. Handles patient handoff sessions,
// prescriptions, appointments, and medical history viewing.
//
// Submodules:
//   - patients: PatientsPage, PatientSessionPage
//   - prescriptions: PrescriptionsPage
//   - appointments: AppointmentsPage
//   - history: HistoryPage
// =============================================================================

mod appointments;
mod history;
mod patients;
mod prescriptions;

pub use appointments::*;
pub use history::*;
pub use patients::*;
pub use prescriptions::*;

use leptos::prelude::*;

use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use crate::i18n::t;

// =============================================================================
// Dashboard
// =============================================================================

/// Medical portal dashboard showing active sessions, today's prescriptions,
/// quick action cards, and a recent activity feed.
#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("medical.dashboard.title").to_string()
                subtitle=t("medical.dashboard.subtitle").to_string()
            />

            // KPI cards
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">{t("medical.dashboard.active_sessions")}</p>
                    <p class="text-2xl font-bold text-txt-primary mt-1">"0"</p>
                    <p class="text-xs text-[var(--portal-accent)] mt-1">{t("medical.dashboard.active_sessions_sub")}</p>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">{t("medical.dashboard.today_prescriptions")}</p>
                    <p class="text-2xl font-bold text-txt-primary mt-1">"0"</p>
                    <p class="text-xs text-txt-tertiary mt-1">{t("medical.dashboard.today_prescriptions_sub")}</p>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">{t("medical.dashboard.today_appointments")}</p>
                    <p class="text-2xl font-bold text-txt-primary mt-1">"0"</p>
                    <p class="text-xs text-txt-tertiary mt-1">{t("medical.dashboard.today_appointments_sub")}</p>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">{t("medical.dashboard.patients_seen")}</p>
                    <p class="text-2xl font-bold text-txt-primary mt-1">"0"</p>
                    <p class="text-xs text-txt-tertiary mt-1">{t("medical.dashboard.patients_seen_sub")}</p>
                </div>
            </div>

            // Quick action cards
            <div>
                <h2 class="font-semibold text-txt-primary mb-3">{t("medical.dashboard.quick_actions")}</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                    <a href="/medical/patients" class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer min-h-[44px]">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 bg-[var(--portal-accent-light)] rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                </svg>
                            </div>
                            <div>
                                <p class="text-sm font-medium text-txt-primary">{t("medical.dashboard.action_patient_lookup")}</p>
                                <p class="text-xs text-txt-tertiary">{t("medical.dashboard.action_patient_lookup_sub")}</p>
                            </div>
                        </div>
                    </a>
                    <a href="/medical/prescriptions" class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer min-h-[44px]">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 bg-[var(--portal-accent-light)] rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                                </svg>
                            </div>
                            <div>
                                <p class="text-sm font-medium text-txt-primary">{t("medical.dashboard.action_new_prescription")}</p>
                                <p class="text-xs text-txt-tertiary">{t("medical.dashboard.action_new_prescription_sub")}</p>
                            </div>
                        </div>
                    </a>
                    <a href="/medical/appointments" class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer min-h-[44px]">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 bg-[var(--portal-accent-light)] rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                                </svg>
                            </div>
                            <div>
                                <p class="text-sm font-medium text-txt-primary">{t("medical.dashboard.action_book_appointment")}</p>
                                <p class="text-xs text-txt-tertiary">{t("medical.dashboard.action_book_appointment_sub")}</p>
                            </div>
                        </div>
                    </a>
                </div>
            </div>

            // Recent activity feed
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-3">{t("medical.dashboard.recent_activity")}</h2>
                <EmptyState message=t("medical.dashboard.no_activity").to_string() />
            </div>
        </div>
    }
}
