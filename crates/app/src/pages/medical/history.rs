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
    let history =
        LocalResource::new(|| {
            crate::api::get::<Vec<bominal_types::MedicalHistoryEntry>>("/api/medical-history")
        });

    let prescriptions = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/prescriptions")
    });

    let appointments = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/appointments")
    });

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
                <Suspense fallback=move || view! {
                    <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
                }>
                    {move || Suspend::new(async move {
                        match history.await {
                            Ok(resp) if resp.success => {
                                let entries = resp.data.unwrap_or_default();
                                if entries.is_empty() {
                                    view! {
                                        <EmptyState message=t("medical.history.no_conditions").to_string() />
                                    }.into_any()
                                } else {
                                    view! {
                                        <ul class="space-y-3">
                                            {entries.into_iter().map(|entry| {
                                                let condition = entry.condition.clone();
                                                let status = entry.status.clone();
                                                let diagnosed = entry.diagnosed_at
                                                    .map(|d| crate::api::format_date_kr(&d))
                                                    .unwrap_or_else(|| "-".to_string());
                                                let treated_by = entry.treated_by.clone().unwrap_or_else(|| "-".to_string());
                                                view! {
                                                    <li class="border border-gray-100 rounded-xl p-3 space-y-1">
                                                        <div class="flex items-center justify-between">
                                                            <p class="text-sm font-medium text-txt-primary">{condition}</p>
                                                            <span class="text-xs bg-[var(--portal-accent-light)] text-[var(--portal-accent)] px-2 py-0.5 rounded-full">{status}</span>
                                                        </div>
                                                        <p class="text-xs text-txt-tertiary">{t("medical.history.diagnosed")}{": "}{diagnosed}</p>
                                                        <p class="text-xs text-txt-tertiary">{t("medical.history.treated_by")}{": "}{treated_by}</p>
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    }.into_any()
                                }
                            }
                            _ => view! {
                                <p class="text-center text-gray-500 py-8">{t("medical.history.no_conditions")}</p>
                            }.into_any(),
                        }
                    })}
                </Suspense>
            </div>

            // Past prescriptions
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">{t("medical.history.past_prescriptions")}</h2>
                <Suspense fallback=move || view! {
                    <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
                }>
                    {move || Suspend::new(async move {
                        match prescriptions.await {
                            Ok(resp) if resp.success => {
                                let items = resp.data.unwrap_or_default();
                                if items.is_empty() {
                                    view! {
                                        <EmptyState message=t("medical.history.no_prescriptions").to_string() />
                                    }.into_any()
                                } else {
                                    view! {
                                        <ul class="space-y-3">
                                            {items.into_iter().map(|item| {
                                                let med = item.get("medication_name")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("-")
                                                    .to_string();
                                                let dosage = item.get("dosage")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("-")
                                                    .to_string();
                                                let freq = item.get("frequency")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("-")
                                                    .to_string();
                                                let created = item.get("created_at")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("-")
                                                    .to_string();
                                                view! {
                                                    <li class="border border-gray-100 rounded-xl p-3 space-y-1">
                                                        <p class="text-sm font-medium text-txt-primary">{med}</p>
                                                        <p class="text-xs text-txt-tertiary">{dosage}{" · "}{freq}</p>
                                                        <p class="text-xs text-txt-tertiary">{created}</p>
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    }.into_any()
                                }
                            }
                            _ => view! {
                                <p class="text-center text-gray-500 py-8">{t("medical.history.no_prescriptions")}</p>
                            }.into_any(),
                        }
                    })}
                </Suspense>
            </div>

            // Past appointments
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">{t("medical.history.past_appointments")}</h2>
                <Suspense fallback=move || view! {
                    <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
                }>
                    {move || Suspend::new(async move {
                        match appointments.await {
                            Ok(resp) if resp.success => {
                                let items = resp.data.unwrap_or_default();
                                if items.is_empty() {
                                    view! {
                                        <EmptyState message=t("medical.history.no_appointments").to_string() />
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
                                <p class="text-center text-gray-500 py-8">{t("medical.history.no_appointments")}</p>
                            }.into_any(),
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}
