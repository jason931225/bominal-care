use leptos::prelude::*;

use crate::components::layout::PageHeader;
use crate::i18n::t;
use crate::pages::senior::FormRow;

// =============================================================================
// Prescriptions — entry form for new prescriptions
// =============================================================================

/// Prescription entry form for medical staff.
///
/// Provides fields for medication name, dosage, frequency, duration, and
/// notes. Submitted prescriptions are associated with the current handoff
/// session patient.
#[component]
pub fn PrescriptionsPage() -> impl IntoView {
    let (med_name, set_med_name) = signal(String::new());
    let (dosage, set_dosage) = signal(String::new());
    let (frequency, set_frequency) = signal(String::new());
    let (duration, set_duration) = signal(String::new());
    let (notes, set_notes) = signal(String::new());
    let (submitting, set_submitting) = signal(false);

    let on_submit = move |_| {
        set_submitting.set(true);
        leptos::task::spawn_local(async move {
            // Placeholder: submit prescription to API
            set_submitting.set(false);
        });
    };

    view! {
        <div class="space-y-8 max-w-2xl">
            <PageHeader
                title=t("medical.prescriptions.title").to_string()
                subtitle=t("medical.prescriptions.subtitle").to_string()
            />

            // Current patient indicator
            <div class="bg-[var(--portal-accent-light)] rounded-xl p-4 flex items-center gap-3">
                <svg class="w-5 h-5 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
                <p class="text-sm text-[var(--portal-accent)] font-medium">{t("medical.prescriptions.no_patient")}</p>
            </div>

            // Prescription form
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <FormRow label=t("medical.prescriptions.med_name").to_string()>
                    <input
                        type="text"
                        class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                        placeholder=t("medical.prescriptions.med_name_placeholder")
                        prop:value=move || med_name.get()
                        on:input=move |ev| set_med_name.set(event_target_value(&ev))
                    />
                </FormRow>
                <FormRow label=t("medical.prescriptions.dosage").to_string()>
                    <input
                        type="text"
                        class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                        placeholder=t("medical.prescriptions.dosage_placeholder")
                        prop:value=move || dosage.get()
                        on:input=move |ev| set_dosage.set(event_target_value(&ev))
                    />
                </FormRow>
                <FormRow label=t("medical.prescriptions.frequency").to_string()>
                    <select
                        class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                        prop:value=move || frequency.get()
                        on:change=move |ev| set_frequency.set(event_target_value(&ev))
                    >
                        <option value="">{t("form.select_placeholder")}</option>
                        <option value="once_daily">{t("medical.prescriptions.freq_once_daily")}</option>
                        <option value="twice_daily">{t("medical.prescriptions.freq_twice_daily")}</option>
                        <option value="three_daily">{t("medical.prescriptions.freq_three_daily")}</option>
                        <option value="as_needed">{t("medical.prescriptions.freq_as_needed")}</option>
                    </select>
                </FormRow>
                <FormRow label=t("medical.prescriptions.duration").to_string()>
                    <input
                        type="text"
                        class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                        placeholder=t("medical.prescriptions.duration_placeholder")
                        prop:value=move || duration.get()
                        on:input=move |ev| set_duration.set(event_target_value(&ev))
                    />
                </FormRow>
                <FormRow label=t("common.notes").to_string()>
                    <textarea
                        class="w-full px-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                        rows=3
                        placeholder=t("medical.prescriptions.notes_placeholder")
                        prop:value=move || notes.get()
                        on:input=move |ev| set_notes.set(event_target_value(&ev))
                    />
                </FormRow>
                <button
                    class="w-full bg-[var(--portal-accent)] text-white text-sm font-semibold rounded-xl py-3 hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50 min-h-[44px]"
                    disabled=move || submitting.get()
                    on:click=on_submit
                >
                    {move || if submitting.get() { t("medical.prescriptions.submitting") } else { t("medical.prescriptions.submit") }}
                </button>
            </div>
        </div>
    }
}
