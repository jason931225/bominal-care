use leptos::prelude::*;

use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use crate::i18n::t;

// =============================================================================
// Patients — lookup and session views
// =============================================================================

/// Patient lookup page with search field.
///
/// Allows medical staff to search for patients by name or ID to initiate
/// or resume a handoff session from the senior portal.
#[component]
pub fn PatientsPage() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());

    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("medical.patients.title").to_string()
                subtitle=t("medical.patients.subtitle").to_string()
            />

            // Search bar
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <div class="relative">
                    <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-txt-tertiary" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                    </svg>
                    <input
                        type="text"
                        class="w-full pl-10 pr-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                        placeholder=t("medical.patients.search_placeholder")
                        prop:value=move || search_query.get()
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    />
                </div>
            </div>

            // Results area
            <EmptyState message=t("medical.patients.search_hint").to_string() />
        </div>
    }
}

/// Patient handoff session page.
///
/// Shows the current session with patient details handed off from the
/// senior portal, including demographics, care grade, and active care plan.
#[component]
pub fn PatientSessionPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("medical.session.title").to_string()
                subtitle=t("medical.session.subtitle").to_string()
            />

            // Session info card
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex items-center gap-4">
                    <div class="w-14 h-14 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center">
                        <svg class="w-7 h-7 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                        </svg>
                    </div>
                    <div>
                        <p class="font-semibold text-txt-primary">{t("medical.session.no_active")}</p>
                        <p class="text-sm text-txt-tertiary">{t("medical.session.no_active_sub")}</p>
                    </div>
                </div>
            </div>

            // Patient details — shown when session is active
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                    <h2 class="font-semibold text-txt-primary">{t("medical.session.demographics")}</h2>
                    <div class="space-y-2 text-sm">
                        <div class="flex justify-between">
                            <span class="text-txt-tertiary">{t("common.name")}</span>
                            <span class="text-txt-secondary">"-"</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-txt-tertiary">{t("medical.session.care_grade")}</span>
                            <span class="text-txt-secondary">"-"</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-txt-tertiary">{t("common.phone")}</span>
                            <span class="text-txt-secondary">"-"</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-txt-tertiary">{t("common.address")}</span>
                            <span class="text-txt-secondary">"-"</span>
                        </div>
                    </div>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                    <h2 class="font-semibold text-txt-primary">{t("medical.session.care_plan")}</h2>
                    <EmptyState message=t("medical.session.no_care_plan").to_string() />
                </div>
            </div>

            // Quick actions for session
            <div class="flex gap-3">
                <a href="/medical/prescriptions" class="flex-1 text-center bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all min-h-[44px] flex items-center justify-center">
                    {t("medical.session.write_prescription")}
                </a>
                <a href="/medical/appointments" class="flex-1 text-center border border-gray-200 text-txt-secondary rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-surface-subtle active:scale-[0.98] transition-all min-h-[44px] flex items-center justify-center">
                    {t("medical.session.book_appointment")}
                </a>
            </div>
        </div>
    }
}
