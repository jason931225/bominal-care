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
    let error_msg = RwSignal::new(None::<String>);
    let success_msg = RwSignal::new(None::<String>);

    // Upcoming appointments from the API
    let upcoming = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/appointments?upcoming=true")
    });

    let on_submit = move |_| {
        let institution_val = institution.get();
        let date_val = date.get();
        let time_val = time.get();
        let purpose_val = purpose.get();
        let notes_val = notes.get();

        if institution_val.is_empty() || date_val.is_empty() || time_val.is_empty() {
            error_msg.set(Some(t("medical.appointments.required_fields").to_string()));
            return;
        }

        error_msg.set(None);
        success_msg.set(None);
        set_submitting.set(true);

        leptos::task::spawn_local(async move {
            let appointment_date = format!("{}T{}:00Z", date_val, time_val);
            let body = serde_json::json!({
                "institution_name": institution_val,
                "appointment_date": appointment_date,
                "purpose": if purpose_val.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(purpose_val) },
                "notes": if notes_val.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(notes_val) },
            });
            match crate::api::post::<serde_json::Value, _>("/api/appointments", &body).await {
                Ok(resp) if resp.success => {
                    success_msg.set(Some(t("medical.appointments.success").to_string()));
                    set_institution.set(String::new());
                    set_date.set(String::new());
                    set_time.set(String::new());
                    set_purpose.set(String::new());
                    set_notes.set(String::new());
                }
                Ok(resp) => error_msg.set(resp.error.or_else(|| Some(t("common.error_generic").to_string()))),
                Err(e) => error_msg.set(Some(e)),
            }
            set_submitting.set(false);
        });
    };

    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("medical.appointments.title").to_string()
                subtitle=t("medical.appointments.subtitle").to_string()
            />

            // Success / error feedback
            {move || error_msg.get().map(|msg| view! {
                <div class="bg-red-50 border border-red-200 rounded-xl p-4 text-sm text-red-700">{msg}</div>
            })}
            {move || success_msg.get().map(|msg| view! {
                <div class="bg-green-50 border border-green-200 rounded-xl p-4 text-sm text-green-700">{msg}</div>
            })}

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
                    <Suspense fallback=move || view! {
                        <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
                    }>
                        {move || Suspend::new(async move {
                            match upcoming.await {
                                Ok(resp) if resp.success => {
                                    let items = resp.data.unwrap_or_default();
                                    if items.is_empty() {
                                        view! {
                                            <EmptyState message=t("medical.appointments.no_upcoming").to_string() />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <ul class="space-y-3">
                                                {items.into_iter().map(|item| {
                                                    let institution_name = item.get("institution_name")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("-")
                                                        .to_string();
                                                    let appt_date = item.get("appointment_date")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("-")
                                                        .to_string();
                                                    let appt_purpose = item.get("purpose")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("")
                                                        .to_string();
                                                    view! {
                                                        <li class="border border-gray-100 rounded-xl p-3 space-y-1">
                                                            <p class="text-sm font-medium text-txt-primary">{institution_name}</p>
                                                            <p class="text-xs text-txt-tertiary">{appt_date}</p>
                                                            {(!appt_purpose.is_empty()).then(|| view! {
                                                                <p class="text-xs text-txt-secondary">{appt_purpose}</p>
                                                            })}
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        }.into_any()
                                    }
                                }
                                _ => view! {
                                    <p class="text-center text-gray-500 py-8">{t("medical.appointments.no_upcoming")}</p>
                                }.into_any(),
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
