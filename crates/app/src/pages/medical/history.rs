use leptos::prelude::*;

use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use crate::i18n::t;

// =============================================================================
// History — medical history for current session patient
// =============================================================================

/// Medical history page for the current handoff session patient.
///
/// Displays a list of medical conditions with dates, allowing clinicians
/// to review the patient's background before prescribing or scheduling.
#[component]
pub fn HistoryPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("medical.history.title").to_string()
                subtitle=t("medical.history.subtitle").to_string()
            />

            // Current patient indicator
            <div class="bg-[var(--portal-accent-light)] rounded-xl p-4 flex items-center gap-3">
                <svg class="w-5 h-5 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
                <p class="text-sm text-[var(--portal-accent)] font-medium">{t("medical.history.no_patient")}</p>
            </div>

            // Conditions list
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">{t("medical.history.conditions")}</h2>
                <EmptyState message=t("medical.history.no_conditions").to_string() />
            </div>

            // Past prescriptions
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">{t("medical.history.past_prescriptions")}</h2>
                <EmptyState message=t("medical.history.no_prescriptions").to_string() />
            </div>

            // Past appointments
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">{t("medical.history.past_appointments")}</h2>
                <EmptyState message=t("medical.history.no_appointments").to_string() />
            </div>
        </div>
    }
}
