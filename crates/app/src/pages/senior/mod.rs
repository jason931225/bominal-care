// =============================================================================
// Senior Portal Pages -- 28 page components
// =============================================================================
//
// Organized by domain:
//   1. Dashboard
//   2. Medications (list, detail, log)
//   3. Appointments (list, detail, new)
//   4. Medical History (list, detail)
//   5. Care Plans (overview, detail)
//   6. Consent (list, detail)
//   7. Profile & Emergency
//   8. Housing (list, detail)
//   9. Services (menu, meals, partners, rides)
//  10. Opportunities (list, detail)
//  11. Notifications
//  12. Settings & More
// =============================================================================

use leptos::prelude::*;
use uuid::Uuid;

use crate::components::data_display::{EmptyState, StatusBadge};
use crate::components::feedback::LoadingSpinner;
use crate::components::layout::PageHeader;
use crate::components::senior::{ConsentToggle, MedicationCard, QuickAction};
use crate::server_fns::{
    appointments, care_plans, consent, medical_history, medications, notifications, profile,
};

// =============================================================================
// 1. Dashboard
// =============================================================================

/// Senior portal dashboard with greeting, medication reminders,
/// upcoming appointments, and quick-action cards.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let person_id = Signal::derive(|| Uuid::default());

    let meds = Resource::new(move || person_id.get(), |pid| async move {
        medications::get_today_events(pid).await.unwrap_or_default()
    });

    let appts = Resource::new(move || person_id.get(), |pid| async move {
        appointments::list_appointments(pid, 1, 3)
            .await
            .map(|r| r.data)
            .unwrap_or_default()
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            // Greeting
            <div>
                <h1 class="text-2xl font-bold text-gray-900">"안녕하세요!"</h1>
                <p class="text-lg text-gray-600 mt-1">"오늘도 건강한 하루 보내세요."</p>
            </div>

            // Today's medication reminders
            <section>
                <h2 class="text-xl font-semibold text-gray-800 mb-3">"오늘의 복약"</h2>
                <Suspense fallback=move || view! { <LoadingSpinner /> }>
                    {move || Suspend::new(async move {
                        let events = meds.await;
                        if events.is_empty() {
                            view! {
                                <p class="text-lg text-gray-500">"오늘 예정된 복약이 없습니다."</p>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-2">
                                    {events.into_iter().map(|ev| {
                                        let taken = ev.status.to_string() == "TAKEN";
                                        let time_str = ev.scheduled_for.format("%H:%M").to_string();
                                        view! {
                                            <MedicationCard
                                                name=ev.medication_id.to_string()
                                                time=time_str
                                                is_taken=taken
                                            />
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </section>

            // Upcoming appointments
            <section>
                <h2 class="text-xl font-semibold text-gray-800 mb-3">"예정된 진료"</h2>
                <Suspense fallback=move || view! { <LoadingSpinner /> }>
                    {move || Suspend::new(async move {
                        let items = appts.await;
                        if items.is_empty() {
                            view! {
                                <p class="text-lg text-gray-500">"예정된 진료가 없습니다."</p>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-2">
                                    {items.into_iter().map(|a| {
                                        view! {
                                            <a href=format!("/appointments/{}", a.id)
                                               class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                                <p class="text-lg font-medium text-gray-900">{a.institution_name.clone()}</p>
                                                <p class="text-base text-gray-500">{a.appointment_date.format("%m/%d %H:%M").to_string()}</p>
                                            </a>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </section>

            // Quick action cards
            <section>
                <h2 class="text-xl font-semibold text-gray-800 mb-3">"빠른 메뉴"</h2>
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
// 2. Medications
// =============================================================================

/// List of active medications with frequency badges.
#[component]
pub fn MedicationsListPage() -> impl IntoView {
    let person_id = Signal::derive(|| Uuid::default());

    let meds = Resource::new(move || person_id.get(), |pid| async move {
        medications::list_medications(pid).await.unwrap_or_default()
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="약물 관리" subtitle="현재 복용 중인 약물 목록" />
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    let items = meds.await;
                    if items.is_empty() {
                        view! { <EmptyState message="등록된 약물이 없습니다." /> }.into_any()
                    } else {
                        view! {
                            <div class="space-y-3">
                                {items.into_iter().map(|m| {
                                    let freq = m.medication.frequency.to_string();
                                    view! {
                                        <a href=format!("/medications/{}", m.medication.id)
                                           class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <p class="text-lg font-medium text-gray-900">{m.medication.name.clone()}</p>
                                                    <p class="text-base text-gray-500">{m.medication.dosage.clone()}</p>
                                                </div>
                                                <StatusBadge status=freq />
                                            </div>
                                        </a>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Single medication detail with schedule and events.
#[component]
pub fn MedicationDetailPage(
    #[prop(into)] person_id: Uuid,
    #[prop(into)] medication_id: Uuid,
) -> impl IntoView {
    let med = Resource::new(
        move || (person_id, medication_id),
        |(pid, mid)| async move {
            medications::get_medication(pid, mid).await.ok().flatten()
        },
    );

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/medications" class="text-blue-600 text-lg">"< 약물 목록"</a>
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match med.await {
                        Some(m) => view! {
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                                <h1 class="text-2xl font-bold text-gray-900">{m.medication.name.clone()}</h1>
                                <div class="space-y-2">
                                    <InfoRow label="용량" value=m.medication.dosage.clone() />
                                    <InfoRow label="형태" value=m.medication.form.clone() />
                                    <InfoRow label="빈도" value=m.medication.frequency.to_string() />
                                    {m.medication.prescribed_by.clone().map(|pb| view! {
                                        <InfoRow label="처방의" value=pb />
                                    })}
                                    {m.medication.side_effects.clone().map(|se| view! {
                                        <InfoRow label="부작용" value=se />
                                    })}
                                    {m.medication.notes.clone().map(|n| view! {
                                        <InfoRow label="메모" value=n />
                                    })}
                                </div>
                                <div>
                                    <h2 class="text-lg font-semibold text-gray-800 mt-4 mb-2">"복약 일정"</h2>
                                    {if m.schedules.is_empty() {
                                        view! { <p class="text-base text-gray-500">"등록된 일정이 없습니다."</p> }.into_any()
                                    } else {
                                        view! {
                                            <div class="space-y-2">
                                                {m.schedules.into_iter().map(|s| view! {
                                                    <div class="bg-gray-50 rounded-lg p-3">
                                                        <p class="text-base text-gray-700">{s.time_of_day.clone()}</p>
                                                    </div>
                                                }).collect_view()}
                                            </div>
                                        }.into_any()
                                    }}
                                </div>
                            </div>
                        }.into_any(),
                        None => view! {
                            <EmptyState message="약물 정보를 찾을 수 없습니다." />
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Today's medication events with taken/missed status.
#[component]
pub fn MedicationLogPage() -> impl IntoView {
    let person_id = Signal::derive(|| Uuid::default());

    let events = Resource::new(move || person_id.get(), |pid| async move {
        medications::get_today_events(pid).await.unwrap_or_default()
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="오늘의 복약 기록" subtitle="복약 상태를 확인하세요" />
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    let items = events.await;
                    if items.is_empty() {
                        view! { <EmptyState message="오늘 예정된 복약이 없습니다." /> }.into_any()
                    } else {
                        view! {
                            <div class="space-y-3">
                                {items.into_iter().map(|ev| {
                                    let taken = ev.status.to_string() == "TAKEN";
                                    let time_str = ev.scheduled_for.format("%H:%M").to_string();
                                    view! {
                                        <MedicationCard
                                            name=ev.medication_id.to_string()
                                            time=time_str
                                            is_taken=taken
                                        />
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}

// =============================================================================
// 3. Appointments
// =============================================================================

/// Upcoming and past appointments with pagination.
#[component]
pub fn AppointmentsListPage() -> impl IntoView {
    let person_id = Signal::derive(|| Uuid::default());
    let page = RwSignal::new(1i64);

    let appts = Resource::new(
        move || (person_id.get(), page.get()),
        |(pid, pg)| async move {
            appointments::list_appointments(pid, pg, 20).await.ok()
        },
    );

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="진료 예약" subtitle="예약 내역을 관리하세요" />
            <a href="/appointments/new"
               class="block w-full text-center bg-blue-600 text-white text-lg font-semibold \
                      rounded-xl py-4 hover:bg-blue-700 transition-colors">
                "새 예약 등록"
            </a>
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match appts.await {
                        Some(data) => {
                            let total = data.total;
                            if data.data.is_empty() {
                                view! { <EmptyState message="예약 내역이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {data.data.into_iter().map(|a| {
                                            let date_str = a.appointment_date.format("%Y-%m-%d %H:%M").to_string();
                                            view! {
                                                <a href=format!("/appointments/{}", a.id)
                                                   class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                                    <div class="flex items-center justify-between">
                                                        <div>
                                                            <p class="text-lg font-medium text-gray-900">{a.institution_name.clone()}</p>
                                                            <p class="text-base text-gray-500">{date_str}</p>
                                                            {a.purpose.clone().map(|p| view! {
                                                                <p class="text-base text-gray-400">{p}</p>
                                                            })}
                                                        </div>
                                                        <StatusBadge status=a.status.to_string() />
                                                    </div>
                                                </a>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <PaginationControls page=page total=total limit=20 />
                                }.into_any()
                            }
                        }
                        None => view! { <EmptyState message="데이터를 불러올 수 없습니다." /> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Single appointment detail view.
#[component]
pub fn AppointmentDetailPage(
    #[prop(into)] appointment_id: Uuid,
) -> impl IntoView {
    let appt = Resource::new(
        move || appointment_id,
        |id| async move { appointments::get_appointment(id).await.ok().flatten() },
    );

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/appointments" class="text-blue-600 text-lg">"< 예약 목록"</a>
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match appt.await {
                        Some(a) => view! {
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                                <h1 class="text-2xl font-bold text-gray-900">{a.institution_name.clone()}</h1>
                                <InfoRow label="날짜" value=a.appointment_date.format("%Y-%m-%d %H:%M").to_string() />
                                <InfoRow label="상태" value=a.status.to_string() />
                                {a.purpose.clone().map(|p| view! { <InfoRow label="목적" value=p /> })}
                                {a.address.clone().map(|addr| view! { <InfoRow label="주소" value=addr /> })}
                                {a.notes.clone().map(|n| view! { <InfoRow label="메모" value=n /> })}
                            </div>
                        }.into_any(),
                        None => view! { <EmptyState message="예약을 찾을 수 없습니다." /> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Form to create a new appointment.
#[component]
pub fn AppointmentNewPage() -> impl IntoView {
    let institution = RwSignal::new(String::new());
    let date = RwSignal::new(String::new());
    let purpose = RwSignal::new(String::new());
    let address = RwSignal::new(String::new());
    let notes = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);

    let create = Action::new(move |_: &()| {
        let inst = institution.get_untracked();
        let dt = date.get_untracked();
        let purp = purpose.get_untracked();
        let addr = address.get_untracked();
        let nts = notes.get_untracked();
        async move {
            let parsed_date = chrono::DateTime::parse_from_str(
                &format!("{dt}T09:00:00+09:00"),
                "%Y-%m-%dT%H:%M:%S%z",
            )
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());

            let result = appointments::create_appointment(
                Uuid::default(),
                inst,
                None,
                parsed_date,
                if purp.is_empty() { None } else { Some(purp) },
                if nts.is_empty() { None } else { Some(nts) },
                if addr.is_empty() { None } else { Some(addr) },
                None,
            )
            .await;

            if result.is_ok() {
                submitted.set(true);
            }
            result
        }
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/appointments" class="text-blue-600 text-lg">"< 예약 목록"</a>
            <PageHeader title="새 예약 등록" />
            <Show when=move || submitted.get()>
                <div class="bg-green-50 border border-green-200 rounded-xl p-4 text-green-800 text-lg">
                    "예약이 등록되었습니다!"
                </div>
            </Show>
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <FormRow label="의료기관명">
                    <input type="text"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-blue-500"
                        prop:value=move || institution.get()
                        on:input=move |ev| institution.set(event_target_value(&ev))
                        placeholder="병원/의원 이름"
                    />
                </FormRow>
                <FormRow label="예약 날짜">
                    <input type="date"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-blue-500"
                        prop:value=move || date.get()
                        on:input=move |ev| date.set(event_target_value(&ev))
                    />
                </FormRow>
                <FormRow label="방문 목적">
                    <input type="text"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-blue-500"
                        prop:value=move || purpose.get()
                        on:input=move |ev| purpose.set(event_target_value(&ev))
                        placeholder="진료, 검사 등"
                    />
                </FormRow>
                <FormRow label="주소">
                    <input type="text"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-blue-500"
                        prop:value=move || address.get()
                        on:input=move |ev| address.set(event_target_value(&ev))
                        placeholder="병원 주소"
                    />
                </FormRow>
                <FormRow label="메모">
                    <textarea
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-blue-500"
                        rows=3
                        prop:value=move || notes.get()
                        on:input=move |ev| notes.set(event_target_value(&ev))
                        placeholder="추가 메모"
                    />
                </FormRow>
                <button
                    class="w-full bg-blue-600 text-white text-lg font-semibold rounded-xl \
                           py-4 hover:bg-blue-700 transition-colors disabled:opacity-50"
                    disabled=move || create.pending().get()
                    on:click=move |_| { create.dispatch(()); }
                >
                    {move || if create.pending().get() { "등록 중..." } else { "예약 등록" }}
                </button>
            </div>
        </div>
    }
}

// =============================================================================
// 4. Medical History
// =============================================================================

/// Paginated list of medical history conditions.
#[component]
pub fn MedicalHistoryPage() -> impl IntoView {
    let person_id = Signal::derive(|| Uuid::default());
    let page = RwSignal::new(1i64);

    let history = Resource::new(
        move || (person_id.get(), page.get()),
        |(pid, pg)| async move {
            medical_history::list_medical_history(pid, pg, 20).await.ok()
        },
    );

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="병력 기록" subtitle="과거 및 현재 질환 기록" />
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match history.await {
                        Some(data) => {
                            let total = data.total;
                            if data.data.is_empty() {
                                view! { <EmptyState message="병력 기록이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {data.data.into_iter().map(|entry| {
                                            view! {
                                                <a href=format!("/medical-history/{}", entry.id)
                                                   class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                                    <p class="text-lg font-medium text-gray-900">{entry.condition.clone()}</p>
                                                    <StatusBadge status=entry.status.clone() />
                                                    {entry.diagnosed_at.map(|d| view! {
                                                        <p class="text-base text-gray-500 mt-1">{d.format("%Y-%m-%d").to_string()}</p>
                                                    })}
                                                </a>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <PaginationControls page=page total=total limit=20 />
                                }.into_any()
                            }
                        }
                        None => view! { <EmptyState message="데이터를 불러올 수 없습니다." /> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Single medical history condition detail.
#[component]
pub fn MedicalHistoryDetailPage(
    #[prop(into)] entry_id: Uuid,
) -> impl IntoView {
    let entry = Resource::new(
        move || entry_id,
        |id| async move { medical_history::get_medical_history_entry(id).await.ok().flatten() },
    );

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/medical-history" class="text-blue-600 text-lg">"< 병력 기록"</a>
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match entry.await {
                        Some(e) => view! {
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                                <h1 class="text-2xl font-bold text-gray-900">{e.condition.clone()}</h1>
                                <InfoRow label="상태" value=e.status.clone() />
                                {e.diagnosed_at.map(|d| view! {
                                    <InfoRow label="진단일" value=d.format("%Y-%m-%d").to_string() />
                                })}
                                {e.treated_by.clone().map(|t| view! { <InfoRow label="담당의" value=t /> })}
                                {e.notes.clone().map(|n| view! { <InfoRow label="메모" value=n /> })}
                            </div>
                        }.into_any(),
                        None => view! { <EmptyState message="기록을 찾을 수 없습니다." /> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

// =============================================================================
// 5. Care Plans
// =============================================================================

/// Care plan overview -- list of care plans.
#[component]
pub fn CarePlanPage() -> impl IntoView {
    let senior_id = Signal::derive(|| Uuid::default());

    let plans = Resource::new(move || senior_id.get(), |sid| async move {
        care_plans::list_care_plans(sid, 1, 20).await.ok()
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="케어 플랜" subtitle="돌봄 계획을 확인하세요" />
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match plans.await {
                        Some(resp) => {
                            if resp.data.is_empty() {
                                view! { <EmptyState message="등록된 케어 플랜이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {resp.data.into_iter().map(|cp| {
                                            view! {
                                                <a href=format!("/care/{}", cp.id)
                                                   class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                                    <div class="flex items-center justify-between">
                                                        <div>
                                                            <p class="text-lg font-medium text-gray-900">{cp.title.clone()}</p>
                                                            {cp.description.clone().map(|d| view! {
                                                                <p class="text-base text-gray-500 line-clamp-2">{d}</p>
                                                            })}
                                                        </div>
                                                        <StatusBadge status=cp.status.to_string() />
                                                    </div>
                                                </a>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        None => view! { <EmptyState message="데이터를 불러올 수 없습니다." /> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Care plan detail with visits.
#[component]
pub fn CarePlanDetailPage(
    #[prop(into)] plan_id: Uuid,
) -> impl IntoView {
    let plan = Resource::new(
        move || plan_id,
        |id| async move { care_plans::get_care_plan(id).await.ok().flatten() },
    );

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/care" class="text-blue-600 text-lg">"< 케어 플랜"</a>
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match plan.await {
                        Some(detail) => {
                            let cp = detail.care_plan;
                            view! {
                                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                                    <h1 class="text-2xl font-bold text-gray-900">{cp.title.clone()}</h1>
                                    <StatusBadge status=cp.status.to_string() />
                                    {cp.description.clone().map(|d| view! {
                                        <p class="text-lg text-gray-600">{d}</p>
                                    })}
                                    {cp.start_date.map(|s| view! {
                                        <InfoRow label="시작일" value=s.format("%Y-%m-%d").to_string() />
                                    })}
                                    {cp.end_date.map(|e| view! {
                                        <InfoRow label="종료일" value=e.format("%Y-%m-%d").to_string() />
                                    })}
                                </div>

                                // Visits
                                <h2 class="text-xl font-semibold text-gray-800 mt-6 mb-3">"방문 기록"</h2>
                                {if detail.visits.is_empty() {
                                    view! { <p class="text-lg text-gray-500">"방문 기록이 없습니다."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-2">
                                            {detail.visits.into_iter().map(|v| view! {
                                                <div class="bg-gray-50 rounded-lg p-3 flex items-center justify-between">
                                                    <p class="text-base text-gray-700">
                                                        {v.scheduled_start.format("%m/%d %H:%M").to_string()}
                                                    </p>
                                                    <StatusBadge status=v.status.to_string() />
                                                </div>
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }}
                            }.into_any()
                        }
                        None => view! { <EmptyState message="케어 플랜을 찾을 수 없습니다." /> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

// =============================================================================
// 6. Consent
// =============================================================================

/// Consent toggles for data sharing.
#[component]
pub fn ConsentPage() -> impl IntoView {
    let person_id = Signal::derive(|| Uuid::default());

    let consents = Resource::new(move || person_id.get(), |pid| async move {
        consent::list_consents(pid).await.unwrap_or_default()
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="동의 관리" subtitle="데이터 공유 및 개인정보 동의" />
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    let items = consents.await;
                    if items.is_empty() {
                        view! { <EmptyState message="등록된 동의 내역이 없습니다." /> }.into_any()
                    } else {
                        view! {
                            <div class="space-y-3">
                                {items.into_iter().map(|c| {
                                    let active = RwSignal::new(c.is_active);
                                    let label = c.purpose.to_string();
                                    let detail_href = format!("/consent/{}", c.id);
                                    view! {
                                        <a href=detail_href class="block">
                                            <ConsentToggle
                                                label=label
                                                purpose="데이터 공유 동의".to_string()
                                                is_active=active
                                            />
                                        </a>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Single consent record detail.
#[component]
pub fn ConsentDetailPage(
    #[prop(into)] consent_id: Uuid,
) -> impl IntoView {
    let consents_resource = Resource::new(
        move || Uuid::default(),
        |pid| async move { consent::list_consents(pid).await.unwrap_or_default() },
    );

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/consent" class="text-blue-600 text-lg">"< 동의 관리"</a>
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || {
                    let cid = consent_id;
                    Suspend::new(async move {
                        let all = consents_resource.await;
                        match all.into_iter().find(|c| c.id == cid) {
                            Some(c) => view! {
                                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                                    <h1 class="text-2xl font-bold text-gray-900">{c.purpose.to_string()}</h1>
                                    <InfoRow
                                        label="상태"
                                        value=if c.is_active { "활성".to_string() } else { "비활성".to_string() }
                                    />
                                    <InfoRow label="동의일" value=c.granted_at.format("%Y-%m-%d").to_string() />
                                    {c.expires_at.map(|e| view! {
                                        <InfoRow label="만료일" value=e.format("%Y-%m-%d").to_string() />
                                    })}
                                    {c.revoked_at.map(|r| view! {
                                        <InfoRow label="철회일" value=r.format("%Y-%m-%d").to_string() />
                                    })}
                                </div>
                            }.into_any(),
                            None => view! { <EmptyState message="동의 기록을 찾을 수 없습니다." /> }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

// =============================================================================
// 7. Profile & Emergency
// =============================================================================

/// Personal info, health baseline, and family contacts.
#[component]
pub fn ProfilePage() -> impl IntoView {
    let user_id = Signal::derive(|| Uuid::default());

    let profile_res = Resource::new(move || user_id.get(), |uid| async move {
        profile::get_profile(uid).await.ok().flatten()
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="내 프로필" subtitle="개인정보 및 건강 기본 정보" />
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match profile_res.await {
                        Some(p) => view! {
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                                <h2 class="text-xl font-semibold text-gray-800">"개인정보"</h2>
                                <InfoRow label="이름" value=format!("{} {}", p.last_name, p.first_name) />
                                {p.phone.clone().map(|ph| view! { <InfoRow label="전화번호" value=ph /> })}
                                {p.address.clone().map(|a| view! { <InfoRow label="주소" value=a /> })}
                                {p.city.clone().map(|c| view! { <InfoRow label="시/군/구" value=c /> })}
                                {p.district.clone().map(|d| view! { <InfoRow label="동/읍/면" value=d /> })}
                                {p.date_of_birth.map(|dob| view! {
                                    <InfoRow label="생년월일" value=dob.format("%Y-%m-%d").to_string() />
                                })}
                                {p.gender.clone().map(|g| view! { <InfoRow label="성별" value=g /> })}
                            </div>

                            // Emergency contacts section
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3 mt-4">
                                <h2 class="text-xl font-semibold text-gray-800">"비상 연락처"</h2>
                                {p.emergency_contact_name.clone().map(|name| view! {
                                    <InfoRow label="연락처 이름" value=name />
                                })}
                                {p.emergency_contact_phone.clone().map(|phone| {
                                    let tel = format!("tel:{phone}");
                                    view! {
                                        <div class="flex items-center justify-between py-2 border-b border-gray-50">
                                            <span class="text-base text-gray-500">"연락처 전화"</span>
                                            <a href=tel class="text-lg text-blue-600 font-medium">{phone}</a>
                                        </div>
                                    }
                                })}
                            </div>
                        }.into_any(),
                        None => view! { <EmptyState message="프로필 정보를 불러올 수 없습니다." /> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Emergency contacts with tel: links and health info.
#[component]
pub fn EmergencyPage() -> impl IntoView {
    let user_id = Signal::derive(|| Uuid::default());

    let profile_res = Resource::new(move || user_id.get(), |uid| async move {
        profile::get_profile(uid).await.ok().flatten()
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="긴급 연락" subtitle="긴급 상황 시 사용하세요" />

            // 119 Emergency call
            <a href="tel:119"
               class="block w-full bg-red-600 text-white text-center text-xl font-bold \
                      rounded-2xl py-6 shadow-lg hover:bg-red-700 transition-colors">
                "119 응급 전화"
            </a>

            // Emergency contacts from profile
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match profile_res.await {
                        Some(p) => {
                            let ec_name = p.emergency_contact_name.clone();
                            let ec_phone = p.emergency_contact_phone.clone().unwrap_or_default();
                            view! {
                                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                                    <h2 class="text-xl font-semibold text-gray-800">"비상 연락처"</h2>
                                    {ec_name.map(|name| {
                                        let phone = ec_phone.clone();
                                        view! {
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <p class="text-lg font-medium text-gray-900">{name}</p>
                                                    <p class="text-base text-gray-500">{phone.clone()}</p>
                                                </div>
                                                <a href=format!("tel:{phone}")
                                                   class="bg-green-600 text-white px-5 py-3 rounded-xl text-lg \
                                                          font-semibold hover:bg-green-700 transition-colors">
                                                    "전화"
                                                </a>
                                            </div>
                                        }
                                    })}
                                </div>

                                // Health info summary
                                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                                    <h2 class="text-xl font-semibold text-gray-800">"건강 정보"</h2>
                                    <InfoRow label="이름" value=format!("{} {}", p.last_name, p.first_name) />
                                    {p.date_of_birth.map(|dob| view! {
                                        <InfoRow label="생년월일" value=dob.format("%Y-%m-%d").to_string() />
                                    })}
                                    {p.phone.clone().map(|ph| view! {
                                        <InfoRow label="전화번호" value=ph />
                                    })}
                                    {p.address.clone().map(|a| view! {
                                        <InfoRow label="주소" value=a />
                                    })}
                                </div>
                            }.into_any()
                        }
                        None => view! {
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                <p class="text-lg text-gray-500">"프로필 정보를 불러올 수 없습니다."</p>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

// =============================================================================
// 8. Housing
// =============================================================================

/// Housing options list (static content).
#[component]
pub fn HousingPage() -> impl IntoView {
    let options = vec![
        ("senior-housing", "시니어 주택", "어르신 전용 주거 시설"),
        ("care-home", "요양원", "전문 간호 요양 시설"),
        ("group-home", "공동 생활 가정", "소규모 돌봄 시설"),
        ("day-care", "주간보호센터", "주간 돌봄 프로그램"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="주거 서비스" subtitle="다양한 주거 옵션을 살펴보세요" />
            <div class="space-y-3">
                {options.into_iter().map(|(slug, title, desc)| {
                    view! {
                        <a href=format!("/housing/{slug}")
                           class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 \
                                  hover:shadow-md transition-shadow">
                            <p class="text-lg font-medium text-gray-900">{title}</p>
                            <p class="text-base text-gray-500 mt-1">{desc}</p>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Housing option detail.
#[component]
pub fn HousingDetailPage(
    #[prop(into)] housing_type: String,
) -> impl IntoView {
    let (title, description, features) = match housing_type.as_str() {
        "senior-housing" => (
            "시니어 주택",
            "어르신이 독립적으로 생활할 수 있는 전용 주거 시설입니다.",
            vec!["개인 공간 보장", "공동 활동 프로그램", "응급 호출 시스템", "식사 서비스"],
        ),
        "care-home" => (
            "요양원",
            "전문 간호 인력이 24시간 돌봄을 제공하는 시설입니다.",
            vec!["24시간 간호", "의료 서비스", "재활 프로그램", "여가 활동"],
        ),
        "group-home" => (
            "공동 생활 가정",
            "소규모 가정 환경에서 함께 생활하는 돌봄 시설입니다.",
            vec!["가정적 환경", "소규모 운영", "개별 맞춤 돌봄", "지역사회 연계"],
        ),
        _ => (
            "주간보호센터",
            "낮 시간 동안 다양한 프로그램과 돌봄을 제공합니다.",
            vec!["주간 돌봄", "식사 제공", "건강 체크", "사회 활동"],
        ),
    };

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/housing" class="text-blue-600 text-lg">"< 주거 서비스"</a>
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h1 class="text-2xl font-bold text-gray-900">{title}</h1>
                <p class="text-lg text-gray-600">{description}</p>
                <h2 class="text-xl font-semibold text-gray-800">"주요 특징"</h2>
                <ul class="space-y-2">
                    {features.into_iter().map(|f| view! {
                        <li class="flex items-center gap-2 text-lg text-gray-700">
                            <span class="text-green-500 font-bold">"·"</span>
                            {f}
                        </li>
                    }).collect_view()}
                </ul>
            </div>
        </div>
    }
}

// =============================================================================
// 9. Services
// =============================================================================

/// Available services menu.
#[component]
pub fn ServicesPage() -> impl IntoView {
    let services = vec![
        ("/services/meals", "식사 배달", "정기적인 식사 배달 서비스", "\u{1f371}"),
        ("/services/partners", "협력 서비스", "파트너 기관 연계 서비스", "\u{1f91d}"),
        ("/services/rides", "이동 서비스", "병원 및 외출 교통 지원", "\u{1f697}"),
        ("/housing", "주거 서비스", "주거 관련 정보 및 지원", "\u{1f3e0}"),
        ("/opportunities", "사회 참여", "자원봉사 및 사회활동", "\u{1f31f}"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="서비스" subtitle="이용 가능한 서비스를 선택하세요" />
            <div class="space-y-3">
                {services.into_iter().map(|(href, title, desc, icon)| {
                    view! {
                        <a href=href
                           class="flex items-center gap-4 bg-white rounded-xl p-5 shadow-sm \
                                  border border-gray-100 hover:shadow-md transition-shadow">
                            <span class="text-3xl">{icon}</span>
                            <div>
                                <p class="text-lg font-medium text-gray-900">{title}</p>
                                <p class="text-base text-gray-500">{desc}</p>
                            </div>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Meal delivery service info.
#[component]
pub fn ServicesMealsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/services" class="text-blue-600 text-lg">"< 서비스"</a>
            <PageHeader title="식사 배달" subtitle="정기적인 식사 배달 서비스" />
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="text-xl font-semibold text-gray-800">"서비스 안내"</h2>
                <ul class="space-y-3">
                    <ServiceInfoItem label="배달 시간" value="오전 11:30 ~ 오후 12:30" />
                    <ServiceInfoItem label="식사 종류" value="한식 정식 (저염/저당 옵션)" />
                    <ServiceInfoItem label="배달 지역" value="서울 전 지역" />
                    <ServiceInfoItem label="이용 요금" value="1식 5,000원 (지원금 적용 가능)" />
                </ul>
            </div>
            <a href="tel:1588-0000"
               class="block w-full bg-blue-600 text-white text-center text-lg font-semibold \
                      rounded-xl py-4 hover:bg-blue-700 transition-colors">
                "신청 전화하기"
            </a>
        </div>
    }
}

/// Partner services list.
#[component]
pub fn ServicesPartnersPage() -> impl IntoView {
    let partners = vec![
        ("복지관 연계", "지역 복지관 프로그램 안내"),
        ("건강검진", "무료 건강검진 서비스"),
        ("법률 상담", "무료 법률 상담 지원"),
        ("심리 상담", "정신건강 상담 서비스"),
        ("일자리 지원", "시니어 일자리 매칭"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/services" class="text-blue-600 text-lg">"< 서비스"</a>
            <PageHeader title="협력 서비스" subtitle="파트너 기관 연계 서비스" />
            <div class="space-y-3">
                {partners.into_iter().map(|(title, desc)| {
                    view! {
                        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                            <p class="text-lg font-medium text-gray-900">{title}</p>
                            <p class="text-base text-gray-500 mt-1">{desc}</p>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Transport / ride services.
#[component]
pub fn ServicesRidesPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/services" class="text-blue-600 text-lg">"< 서비스"</a>
            <PageHeader title="이동 서비스" subtitle="병원 방문 및 외출 교통 지원" />
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="text-xl font-semibold text-gray-800">"이용 안내"</h2>
                <ul class="space-y-3">
                    <ServiceInfoItem label="이용 시간" value="오전 8:00 ~ 오후 6:00" />
                    <ServiceInfoItem label="예약 방법" value="최소 1일 전 전화 예약" />
                    <ServiceInfoItem label="이용 요금" value="편도 3,000원 (지원금 적용 가능)" />
                    <ServiceInfoItem label="이용 범위" value="병원, 관공서, 복지시설" />
                </ul>
            </div>
            <a href="tel:1588-0000"
               class="block w-full bg-blue-600 text-white text-center text-lg font-semibold \
                      rounded-xl py-4 hover:bg-blue-700 transition-colors">
                "예약 전화하기"
            </a>
        </div>
    }
}

// =============================================================================
// 10. Opportunities
// =============================================================================

/// Volunteer and social opportunities list.
#[component]
pub fn OpportunitiesPage() -> impl IntoView {
    let opportunities = vec![
        ("opp-1", "노인 대학", "평생학습 프로그램", "매주 화/목"),
        ("opp-2", "건강 체조", "어르신 건강 체조 모임", "매주 월/수/금"),
        ("opp-3", "봉사 활동", "지역사회 봉사 프로그램", "매월 둘째 토요일"),
        ("opp-4", "문화 교실", "서예, 그림, 음악 수업", "매주 수요일"),
        ("opp-5", "걷기 모임", "동네 산책 및 걷기 운동", "매일 오전 7시"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="사회 참여" subtitle="다양한 활동에 참여하세요" />
            <div class="space-y-3">
                {opportunities.into_iter().map(|(id, title, desc, schedule)| {
                    view! {
                        <a href=format!("/opportunities/{id}")
                           class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 \
                                  hover:shadow-md transition-shadow">
                            <p class="text-lg font-medium text-gray-900">{title}</p>
                            <p class="text-base text-gray-500 mt-1">{desc}</p>
                            <p class="text-base text-blue-600 mt-1">{schedule}</p>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Opportunity detail.
#[component]
pub fn OpportunityDetailPage(
    #[prop(into)] opportunity_id: String,
) -> impl IntoView {
    let (title, desc, schedule, location, contact) = match opportunity_id.as_str() {
        "opp-1" => (
            "노인 대학",
            "평생학습 프로그램으로 다양한 주제의 강좌를 수강할 수 있습니다.",
            "매주 화/목 10:00-12:00",
            "서울시 종로구 복지관",
            "02-1234-5678",
        ),
        "opp-2" => (
            "건강 체조",
            "전문 강사와 함께하는 어르신 맞춤 건강 체조 프로그램입니다.",
            "매주 월/수/금 09:00-10:00",
            "동네 공원 or 복지관",
            "02-2345-6789",
        ),
        "opp-3" => (
            "봉사 활동",
            "지역사회를 위한 봉사 프로그램에 참여할 수 있습니다.",
            "매월 둘째 토요일 09:00-12:00",
            "지역 복지관",
            "02-3456-7890",
        ),
        "opp-4" => (
            "문화 교실",
            "서예, 그림, 음악 등 다양한 문화 수업을 제공합니다.",
            "매주 수요일 14:00-16:00",
            "문화센터",
            "02-4567-8901",
        ),
        _ => (
            "걷기 모임",
            "동네 이웃과 함께 산책하며 건강을 챙기는 모임입니다.",
            "매일 오전 7:00-8:00",
            "동네 공원 입구",
            "02-5678-9012",
        ),
    };

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/opportunities" class="text-blue-600 text-lg">"< 사회 참여"</a>
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h1 class="text-2xl font-bold text-gray-900">{title}</h1>
                <p class="text-lg text-gray-600">{desc}</p>
                <div class="space-y-2">
                    <InfoRow label="일정" value=schedule.to_string() />
                    <InfoRow label="장소" value=location.to_string() />
                    <InfoRow label="문의" value=contact.to_string() />
                </div>
            </div>
            <a href=format!("tel:{contact}")
               class="block w-full bg-blue-600 text-white text-center text-lg font-semibold \
                      rounded-xl py-4 hover:bg-blue-700 transition-colors">
                "문의 전화하기"
            </a>
        </div>
    }
}

// =============================================================================
// 11. Notifications
// =============================================================================

/// Notification list with read/unread status.
#[component]
pub fn NotificationsPage() -> impl IntoView {
    let user_id = Signal::derive(|| Uuid::default());
    let page = RwSignal::new(1i64);

    let notifs = Resource::new(
        move || (user_id.get(), page.get()),
        |(uid, pg)| async move {
            notifications::list_notifications(uid, pg, 20).await.ok()
        },
    );

    let mark_all = Action::new(move |_: &()| {
        let uid = user_id.get_untracked();
        async move { notifications::mark_all_as_read(uid).await }
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <div class="flex items-center justify-between">
                <PageHeader title="알림" />
                <button
                    class="text-base text-blue-600 hover:text-blue-800"
                    on:click=move |_| { mark_all.dispatch(()); }
                >"모두 읽음"</button>
            </div>
            <Suspense fallback=move || view! { <LoadingSpinner /> }>
                {move || Suspend::new(async move {
                    match notifs.await {
                        Some(data) => {
                            let total = data.total;
                            if data.data.is_empty() {
                                view! { <EmptyState message="알림이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-2">
                                        {data.data.into_iter().map(|n| {
                                            let bg = if n.is_read { "bg-white" } else { "bg-blue-50" };
                                            let cls = format!(
                                                "{bg} rounded-xl p-4 shadow-sm border border-gray-100"
                                            );
                                            view! {
                                                <div class=cls>
                                                    <p class="text-lg font-medium text-gray-900">{n.title.clone()}</p>
                                                    <p class="text-base text-gray-600 mt-1">{n.message.clone()}</p>
                                                    <p class="text-sm text-gray-400 mt-2">
                                                        {n.created_at.format("%m/%d %H:%M").to_string()}
                                                    </p>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <PaginationControls page=page total=total limit=20 />
                                }.into_any()
                            }
                        }
                        None => view! { <EmptyState message="알림을 불러올 수 없습니다." /> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

// =============================================================================
// 12. Settings & More
// =============================================================================

/// Settings menu.
#[component]
pub fn SettingsPage() -> impl IntoView {
    let menu_items = vec![
        ("/profile", "내 프로필", "개인정보 수정"),
        ("/consent", "동의 관리", "데이터 공유 설정"),
        ("/notifications", "알림 설정", "알림 수신 관리"),
        ("/auth/signin", "로그아웃", "안전하게 로그아웃"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="설정" />
            <div class="space-y-2">
                {menu_items.into_iter().map(|(href, title, desc)| {
                    view! {
                        <a href=href
                           class="flex items-center justify-between bg-white rounded-xl p-5 \
                                  shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                            <div>
                                <p class="text-lg font-medium text-gray-900">{title}</p>
                                <p class="text-base text-gray-500">{desc}</p>
                            </div>
                            <span class="text-gray-400 text-xl">">"</span>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Additional menu items (More page).
#[component]
pub fn MorePage() -> impl IntoView {
    let sections = vec![
        ("건강", vec![
            ("/medications", "약물 관리", "\u{1f48a}"),
            ("/medication-log", "복약 기록", "\u{1f4cb}"),
            ("/medical-history", "병력 기록", "\u{1f3e5}"),
        ]),
        ("돌봄", vec![
            ("/care", "케어 플랜", "\u{1f49c}"),
            ("/appointments", "진료 예약", "\u{1f4c5}"),
        ]),
        ("생활", vec![
            ("/services", "서비스", "\u{1f527}"),
            ("/housing", "주거 서비스", "\u{1f3e0}"),
            ("/opportunities", "사회 참여", "\u{1f31f}"),
        ]),
        ("계정", vec![
            ("/profile", "내 프로필", "\u{1f464}"),
            ("/consent", "동의 관리", "\u{1f512}"),
            ("/emergency", "긴급 연락", "\u{1f6a8}"),
            ("/settings", "설정", "\u{2699}\u{fe0f}"),
        ]),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <PageHeader title="더보기" />
            {sections.into_iter().map(|(section_title, items)| {
                view! {
                    <section>
                        <h2 class="text-lg font-semibold text-gray-500 mb-2">{section_title}</h2>
                        <div class="space-y-1">
                            {items.into_iter().map(|(href, label, icon)| {
                                view! {
                                    <a href=href
                                       class="flex items-center gap-4 bg-white rounded-xl px-5 py-4 \
                                              shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                                        <span class="text-2xl">{icon}</span>
                                        <span class="text-lg font-medium text-gray-900">{label}</span>
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    </section>
                }
            }).collect_view()}
        </div>
    }
}

// =============================================================================
// Shared helper components (private to this module)
// =============================================================================

/// A label-value row for detail pages.
#[component]
fn InfoRow(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between py-2 border-b border-gray-50">
            <span class="text-base text-gray-500">{label}</span>
            <span class="text-lg text-gray-900 font-medium">{value}</span>
        </div>
    }
}

/// A label-value row for service info pages.
#[component]
fn ServiceInfoItem(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <li class="flex items-start gap-3">
            <span class="text-base font-medium text-gray-700 min-w-[5rem]">{label}</span>
            <span class="text-lg text-gray-900">{value}</span>
        </li>
    }
}

/// A form field row with label for the appointment form.
#[component]
fn FormRow(
    #[prop(into)] label: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="space-y-1">
            <label class="text-base font-medium text-gray-700">{label}</label>
            {children()}
        </div>
    }
}

/// Reusable pagination controls.
#[component]
fn PaginationControls(
    page: RwSignal<i64>,
    total: i64,
    limit: i64,
) -> impl IntoView {
    let has_next = total > page.get_untracked() * limit;

    view! {
        <div class="flex justify-between items-center pt-4">
            <button
                class="text-lg text-blue-600 disabled:text-gray-300"
                disabled=move || page.get() <= 1
                on:click=move |_| page.set(page.get_untracked() - 1)
            >"< 이전"</button>
            <span class="text-base text-gray-500">
                {move || format!("{}페이지", page.get())}
            </span>
            <button
                class="text-lg text-blue-600 disabled:text-gray-300"
                disabled=move || !has_next
                on:click=move |_| page.set(page.get_untracked() + 1)
            >"다음 >"</button>
        </div>
    }
}
