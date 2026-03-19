use leptos::prelude::*;
use uuid::Uuid;

use bominal_types::{Medication, MedicationSchedule};
use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;

/// Wrapper matching the server's medication response shape.
#[derive(Debug, Clone, serde::Deserialize)]
pub(super) struct MedicationWithSchedules {
    pub medication: Medication,
    #[allow(dead_code)]
    pub schedules: Vec<MedicationSchedule>,
}

/// List of active medications with frequency badges.
#[component]
pub fn MedicationsListPage() -> impl IntoView {
    let medications = LocalResource::new(|| {
        crate::api::get::<Vec<MedicationWithSchedules>>("/api/medications")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="약물 관리" subtitle="현재 복용 중인 약물 목록" />
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match medications.await {
                        Ok(resp) if resp.success => {
                            let items: Vec<Medication> = resp.data.unwrap_or_default()
                                .into_iter().map(|mws| mws.medication).collect();
                            if items.is_empty() {
                                view! { <EmptyState message="등록된 약물이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|med| {
                                            let active_class = if med.is_active {
                                                "bg-success-light text-success"
                                            } else {
                                                "bg-surface-subtle text-txt-tertiary"
                                            };
                                            let active_label = if med.is_active { "복용 중" } else { "중단" };
                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                                    <div class="flex items-center justify-between">
                                                        <p class="text-lg font-medium text-txt-primary">{med.name}</p>
                                                        <span class={format!("text-xs px-2 py-1 rounded-full {active_class}")}>{active_label}</span>
                                                    </div>
                                                    <p class="text-base text-txt-secondary mt-1">{format!("{} · {}", med.dosage, med.form)}</p>
                                                    <span class="inline-block mt-2 text-xs px-2 py-1 rounded-full bg-primary-light text-primary">
                                                        {format!("{}", med.frequency)}
                                                    </span>
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
        </div>
    }
}

/// Single medication detail with schedule and events.
#[component]
pub fn MedicationDetailPage(
    #[prop(into)] person_id: Uuid,
    #[prop(into)] medication_id: Uuid,
) -> impl IntoView {
    let _ = (person_id, medication_id);
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/medications" class="text-primary text-lg">"< 약물 목록"</a>
            <EmptyState message="약물 정보를 찾을 수 없습니다." />
        </div>
    }
}

/// Today's medication events with taken/missed status.
#[component]
pub fn MedicationLogPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="오늘의 복약 기록" subtitle="복약 상태를 확인하세요" />
            <EmptyState message="오늘 예정된 복약이 없습니다." />
        </div>
    }
}
