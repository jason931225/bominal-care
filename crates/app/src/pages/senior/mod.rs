// =============================================================================
// Senior Portal Pages
// =============================================================================
//
// Split into domain modules:
//   - medications: MedicationsListPage, MedicationDetailPage, MedicationLogPage
//   - appointments: AppointmentsListPage, AppointmentDetailPage, AppointmentNewPage
//   - profile: ProfilePage, EmergencyPage, ConsentPage, MedicalHistoryPage, CarePlanPage
//   - services: ServicesPage, meals, partners, rides, HousingPage, OpportunitiesPage
//   - settings: SettingsPage, MorePage, NotificationsPage
// =============================================================================

mod appointments;
mod medications;
mod profile;
mod services;
mod settings;

pub use appointments::*;
pub use medications::*;
pub use profile::*;
pub use services::*;
pub use settings::*;

use leptos::prelude::*;

use bominal_types::{Appointment, Medication};
use crate::components::senior::QuickAction;

use medications::MedicationWithSchedules;

// =============================================================================
// Dashboard
// =============================================================================

/// Estimate remaining days from a Medication.
fn estimate_remaining_days_for(med: &Medication) -> Option<i32> {
    let total = med.total_quantity?;
    if total <= 0 {
        return Some(0);
    }
    let doses_per_day = match med.frequency {
        bominal_types::MedicationFrequency::OnceDaily => 1,
        bominal_types::MedicationFrequency::TwiceDaily => 2,
        bominal_types::MedicationFrequency::ThreeTimesDaily => 3,
        bominal_types::MedicationFrequency::FourTimesDaily => 4,
        bominal_types::MedicationFrequency::EveryOtherDay => 1,
        bominal_types::MedicationFrequency::Weekly => 1,
        bominal_types::MedicationFrequency::AsNeeded
        | bominal_types::MedicationFrequency::Custom => return None,
    };
    let daily = doses_per_day * med.doses_per_intake.max(1);
    if daily <= 0 {
        return None;
    }
    let days = total / daily;
    let days = match med.frequency {
        bominal_types::MedicationFrequency::EveryOtherDay => days * 2,
        bominal_types::MedicationFrequency::Weekly => days * 7,
        _ => days,
    };
    Some(days)
}

/// Senior portal dashboard with greeting, medication reminders,
/// upcoming appointments, and quick-action cards.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let medications = LocalResource::new(|| {
        crate::api::get::<Vec<MedicationWithSchedules>>("/api/medications")
    });
    let appointments = LocalResource::new(|| {
        crate::api::get::<Vec<Appointment>>("/api/appointments?page=1&limit=10")
    });
    let today_events = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/medications/today")
    });

    let status_error = RwSignal::new(Option::<String>::None);

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-8">
            // Greeting
            <div>
                <h1 class="text-2xl font-bold text-txt-primary">"안녕하세요!"</h1>
                <p class="text-lg text-txt-secondary mt-1">"오늘도 건강한 하루 보내세요."</p>
            </div>

            // Low supply alert
            <Suspense fallback=move || view! { <div></div> }>
                {move || Suspend::new(async move {
                    match medications.await {
                        Ok(resp) if resp.success => {
                            let items: Vec<Medication> = resp.data.unwrap_or_default()
                                .into_iter().map(|mws| mws.medication).collect();
                            let low_supply: Vec<(String, i32)> = items.iter()
                                .filter_map(|med| {
                                    let days = estimate_remaining_days_for(med)?;
                                    if days <= 7 { Some((med.name.clone(), days)) } else { None }
                                })
                                .collect();
                            if low_supply.is_empty() {
                                view! { <div></div> }.into_any()
                            } else {
                                view! {
                                    <div class="bg-warning-light text-warning rounded-2xl p-4 space-y-1">
                                        <p class="text-base font-semibold">{"\u{26a0}\u{fe0f} 약 보충이 필요합니다"}</p>
                                        {low_supply.into_iter().map(|(name, days)| {
                                            let msg = if days <= 3 {
                                                format!("{}: 약 보충 필요", name)
                                            } else {
                                                format!("{}: {}일분 남음", name, days)
                                            };
                                            view! { <p class="text-sm">{msg}</p> }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! { <div></div> }.into_any(),
                    }
                })}
            </Suspense>

            // Status error display
            <Show when=move || status_error.get().is_some()>
                <div class="bg-danger-light rounded-2xl p-4 text-danger text-lg">
                    {move || status_error.get().unwrap_or_default()}
                </div>
            </Show>

            // Today's medication reminders with tick-off
            <section>
                <h2 class="text-xl font-semibold text-txt-primary mb-3">"오늘의 복약"</h2>

                // Progress summary
                <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                    {move || Suspend::new(async move {
                        match today_events.await {
                            Ok(resp) if resp.success => {
                                let events = resp.data.unwrap_or_default();
                                let total = events.len();
                                let taken = events.iter()
                                    .filter(|e| {
                                        e.get("status")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s == "TAKEN")
                                            .unwrap_or(false)
                                    })
                                    .count();
                                if total > 0 {
                                    view! {
                                        <div class="bg-primary-light rounded-2xl p-3 text-center mb-3">
                                            <p class="text-base font-semibold text-primary">
                                                {format!("오늘 {}/{} 복용 완료", taken, total)}
                                            </p>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }
                            _ => view! { <div></div> }.into_any(),
                        }
                    })}
                </Suspense>

                // Medication cards with tick-off buttons
                <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                    {move || Suspend::new(async move {
                        // Fetch both medications and today's events
                        let meds_result = medications.await;
                        let events_result = today_events.await;

                        let events_list: Vec<serde_json::Value> = events_result
                            .ok()
                            .and_then(|r| if r.success { r.data } else { None })
                            .unwrap_or_default();

                        match meds_result {
                            Ok(resp) if resp.success => {
                                let items: Vec<Medication> = resp.data.unwrap_or_default()
                                    .into_iter().map(|mws| mws.medication).collect();
                                if items.is_empty() {
                                    view! { <p class="text-lg text-txt-tertiary">"오늘 예정된 복약이 없습니다."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-3">
                                            {items.into_iter().map(|med| {
                                                let med_id_str = med.id.to_string();
                                                // Find today's event for this medication
                                                let event = events_list.iter().find(|e| {
                                                    e.get("medication_id")
                                                        .and_then(|v| v.as_str())
                                                        .map(|s| s == med_id_str)
                                                        .unwrap_or(false)
                                                });
                                                let event_status = event
                                                    .and_then(|e| e.get("status"))
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                let event_id = event
                                                    .and_then(|e| e.get("id"))
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                let is_taken = event_status == "TAKEN";
                                                let is_scheduled = event_status == "SCHEDULED";

                                                view! {
                                                    <div class="bg-surface-card rounded-2xl p-4 shadow-sm">
                                                        <p class="text-lg font-medium text-txt-primary">{med.name}</p>
                                                        <p class="text-base text-txt-secondary">{format!("{} · {}", med.dosage, med.form)}</p>
                                                        {if is_taken {
                                                            Some(view! {
                                                                <p class="mt-2 text-sm text-success font-medium">{"\u{2705} 복용 완료"}</p>
                                                            }.into_any())
                                                        } else if is_scheduled {
                                                            let eid = event_id.clone();
                                                            let err_sig = status_error;
                                                            Some(view! {
                                                                <button
                                                                    class="mt-2 w-full bg-teal-600 text-white text-sm font-medium \
                                                                           rounded-xl py-2 hover:opacity-90 active:scale-[0.98] transition-all"
                                                                    on:click=move |_| {
                                                                        let id = eid.clone();
                                                                        let err = err_sig;
                                                                        leptos::task::spawn_local(async move {
                                                                            let body = serde_json::json!({"status": "TAKEN"});
                                                                            match crate::api::post::<serde_json::Value, _>(
                                                                                &format!("/api/medications/events/{}/status", id), &body
                                                                            ).await {
                                                                                Ok(resp) if resp.success => {
                                                                                    if let Some(w) = leptos::web_sys::window() {
                                                                                        let _ = w.location().reload();
                                                                                    }
                                                                                }
                                                                                Ok(resp) => err.set(resp.error),
                                                                                Err(e) => err.set(Some(e)),
                                                                            }
                                                                        });
                                                                    }
                                                                >"복용 완료"</button>
                                                            }.into_any())
                                                        } else {
                                                            None
                                                        }}
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            Ok(resp) => view! { <p class="text-danger">{resp.error.unwrap_or_default()}</p> }.into_any(),
                            Err(e) => view! { <p class="text-danger">{e}</p> }.into_any(),
                        }
                    })}
                </Suspense>
            </section>

            // Upcoming appointments
            <section>
                <h2 class="text-xl font-semibold text-txt-primary mb-3">"예정된 진료"</h2>
                <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                    {move || Suspend::new(async move {
                        match appointments.await {
                            Ok(resp) if resp.success => {
                                let items = resp.data.unwrap_or_default();
                                if items.is_empty() {
                                    view! { <p class="text-lg text-txt-tertiary">"예정된 진료가 없습니다."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-3">
                                            {items.into_iter().map(|appt| {
                                                view! {
                                                    <div class="bg-surface-card rounded-2xl p-4 shadow-sm">
                                                        <p class="text-lg font-medium text-txt-primary">{appt.institution_name}</p>
                                                        <p class="text-base text-txt-secondary">{crate::api::format_date_kr(&appt.appointment_date)}</p>
                                                        {appt.purpose.map(|p| view! { <p class="text-base text-txt-tertiary">{p}</p> })}
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            Ok(resp) => view! { <p class="text-danger">{resp.error.unwrap_or_default()}</p> }.into_any(),
                            Err(e) => view! { <p class="text-danger">{e}</p> }.into_any(),
                        }
                    })}
                </Suspense>
            </section>

            // Quick action cards
            <section>
                <h2 class="text-xl font-semibold text-txt-primary mb-3">"빠른 메뉴"</h2>
                <div class="grid grid-cols-2 gap-4">
                    <QuickAction label="예약" href="/appointments" icon="\u{1f4c5}" />
                    <QuickAction label="약물" href="/medications" icon="\u{1f48a}" />
                    <QuickAction label="돌봄" href="/care" icon="\u{1f49c}" />
                    <QuickAction label="긴급" href="/emergency" icon="\u{1f6a8}" />
                </div>
            </section>
        </div>
    }
}

// =============================================================================
// Shared helper components (used by sub-modules via super::)
// =============================================================================

/// A label-value row for detail pages.
#[component]
pub(crate) fn InfoRow(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between py-2 border-b border-surface-subtle">
            <span class="text-base text-txt-tertiary">{label}</span>
            <span class="text-lg text-txt-primary font-medium">{value}</span>
        </div>
    }
}

/// A label-value row for service info pages.
#[component]
pub(crate) fn ServiceInfoItem(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <li class="flex items-start gap-3">
            <span class="text-base font-medium text-txt-secondary min-w-[5rem]">{label}</span>
            <span class="text-lg text-txt-primary">{value}</span>
        </li>
    }
}

/// A form field row with label for forms.
#[component]
pub(crate) fn FormRow(
    #[prop(into)] label: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="space-y-1">
            <label class="text-base font-medium text-txt-secondary">{label}</label>
            {children()}
        </div>
    }
}
