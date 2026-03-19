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

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-8">
            // Greeting
            <div>
                <h1 class="text-2xl font-bold text-txt-primary">"안녕하세요!"</h1>
                <p class="text-lg text-txt-secondary mt-1">"오늘도 건강한 하루 보내세요."</p>
            </div>

            // Today's medication reminders
            <section>
                <h2 class="text-xl font-semibold text-txt-primary mb-3">"오늘의 복약"</h2>
                <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                    {move || Suspend::new(async move {
                        match medications.await {
                            Ok(resp) if resp.success => {
                                let items: Vec<Medication> = resp.data.unwrap_or_default()
                                    .into_iter().map(|mws| mws.medication).collect();
                                if items.is_empty() {
                                    view! { <p class="text-lg text-txt-tertiary">"오늘 예정된 복약이 없습니다."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-3">
                                            {items.into_iter().map(|med| {
                                                view! {
                                                    <div class="bg-surface-card rounded-2xl p-4 shadow-sm">
                                                        <p class="text-lg font-medium text-txt-primary">{med.name}</p>
                                                        <p class="text-base text-txt-secondary">{format!("{} · {}", med.dosage, med.form)}</p>
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

/// Reusable pagination controls.
#[component]
pub(crate) fn PaginationControls(
    page: RwSignal<i64>,
    total: i64,
    limit: i64,
) -> impl IntoView {
    let has_next = total > page.get_untracked() * limit;

    view! {
        <div class="flex justify-between items-center pt-4">
            <button
                class="text-lg text-primary disabled:text-txt-disabled"
                disabled=move || page.get() <= 1
                on:click=move |_| page.set(page.get_untracked() - 1)
            >"< 이전"</button>
            <span class="text-base text-txt-tertiary">
                {move || format!("{}페이지", page.get())}
            </span>
            <button
                class="text-lg text-primary disabled:text-txt-disabled"
                disabled=move || !has_next
                on:click=move |_| page.set(page.get_untracked() + 1)
            >"다음 >"</button>
        </div>
    }
}
