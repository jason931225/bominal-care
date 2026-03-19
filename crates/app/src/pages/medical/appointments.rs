use leptos::prelude::*;

use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use crate::i18n::t;
use crate::pages::senior::FormRow;

// =============================================================================
// Appointments — book on behalf of senior, view upcoming
// =============================================================================

/// Appointments page for medical staff.
///
/// Allows booking appointments on behalf of a senior patient in the current
/// handoff session. Also shows upcoming appointments for the session patient.
#[component]
pub fn AppointmentsPage() -> impl IntoView {
    let (institution, set_institution) = signal(String::new());
    let (date, set_date) = signal(String::new());
    let (time, set_time) = signal(String::new());
    let (purpose, set_purpose) = signal(String::new());
    let (notes, set_notes) = signal(String::new());
    let (submitting, set_submitting) = signal(false);

    let on_submit = move |_| {
        set_submitting.set(true);
        leptos::task::spawn_local(async move {
            // Placeholder: submit appointment booking to API
            set_submitting.set(false);
        });
    };

    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("medical.appointments.title").to_string()
                subtitle=t("medical.appointments.subtitle").to_string()
            />

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                // Booking form
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                    <h2 class="font-semibold text-txt-primary">{t("medical.appointments.new_booking")}</h2>

                    // Current patient indicator
                    <div class="bg-[var(--portal-accent-light)] rounded-xl p-3 flex items-center gap-2">
                        <div class="w-2 h-2 bg-[var(--portal-accent)] rounded-full"></div>
                        <p class="text-xs text-[var(--portal-accent)] font-medium">{t("medical.appointments.no_patient")}</p>
                    </div>

                    <FormRow label=t("medical.appointments.institution").to_string()>
                        <input
                            type="text"
                            class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                            placeholder=t("medical.appointments.institution_placeholder")
                            prop:value=move || institution.get()
                            on:input=move |ev| set_institution.set(event_target_value(&ev))
                        />
                    </FormRow>
                    <FormRow label=t("common.date").to_string()>
                        <input
                            type="date"
                            class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                            prop:value=move || date.get()
                            on:input=move |ev| set_date.set(event_target_value(&ev))
                        />
                    </FormRow>
                    <FormRow label=t("common.time").to_string()>
                        <input
                            type="time"
                            class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                            prop:value=move || time.get()
                            on:input=move |ev| set_time.set(event_target_value(&ev))
                        />
                    </FormRow>
                    <FormRow label=t("medical.appointments.purpose").to_string()>
                        <input
                            type="text"
                            class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                            placeholder=t("medical.appointments.purpose_placeholder")
                            prop:value=move || purpose.get()
                            on:input=move |ev| set_purpose.set(event_target_value(&ev))
                        />
                    </FormRow>
                    <FormRow label=t("common.notes").to_string()>
                        <textarea
                            class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                            rows=3
                            placeholder=t("medical.appointments.notes_placeholder")
                            prop:value=move || notes.get()
                            on:input=move |ev| set_notes.set(event_target_value(&ev))
                        />
                    </FormRow>
                    <button
                        class="w-full bg-[var(--portal-accent)] text-white text-sm font-semibold rounded-xl py-3 hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50 min-h-[44px]"
                        disabled=move || submitting.get()
                        on:click=on_submit
                    >
                        {move || if submitting.get() { t("medical.appointments.submitting") } else { t("medical.appointments.submit") }}
                    </button>
                </div>

                // Upcoming appointments
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                    <h2 class="font-semibold text-txt-primary">{t("medical.appointments.upcoming")}</h2>
                    <EmptyState message=t("medical.appointments.no_upcoming").to_string() />
                </div>
            </div>
        </div>
    }
}
