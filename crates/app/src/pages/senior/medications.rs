use leptos::prelude::*;
use uuid::Uuid;

use bominal_types::{InstructionTiming, Medication, MedicationFrequency, MedicationSchedule};
use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use super::InfoRow;

/// Wrapper matching the server's medication response shape.
#[derive(Debug, Clone, serde::Deserialize)]
pub(super) struct MedicationWithSchedules {
    pub medication: Medication,
    #[allow(dead_code)]
    pub schedules: Vec<MedicationSchedule>,
}

/// Format instruction timing as Korean text.
fn format_instruction_timing(timing: &InstructionTiming, minutes: Option<i32>) -> String {
    let base = match timing {
        InstructionTiming::BeforeMeal => "식전",
        InstructionTiming::WithMeal => "식사 중",
        InstructionTiming::AfterMeal => "식후",
        InstructionTiming::EmptyStomach => "공복",
        InstructionTiming::Bedtime => "취침 전",
        InstructionTiming::Anytime => "시간 무관",
    };
    match minutes {
        Some(m) if m > 0 => format!("{} {}분", base, m),
        _ => base.to_string(),
    }
}

/// Estimate remaining days from total_quantity, frequency, and doses_per_intake.
fn estimate_remaining_days(med: &Medication) -> Option<i32> {
    let total = med.total_quantity?;
    if total <= 0 {
        return Some(0);
    }
    let doses_per_day = match med.frequency {
        MedicationFrequency::OnceDaily => 1,
        MedicationFrequency::TwiceDaily => 2,
        MedicationFrequency::ThreeTimesDaily => 3,
        MedicationFrequency::FourTimesDaily => 4,
        MedicationFrequency::EveryOtherDay => 1, // ~0.5/day, estimate conservatively
        MedicationFrequency::Weekly => 1,
        MedicationFrequency::AsNeeded | MedicationFrequency::Custom => return None,
    };
    let daily_consumption = doses_per_day * med.doses_per_intake.max(1);
    if daily_consumption <= 0 {
        return None;
    }
    let days = total / daily_consumption;
    // EveryOtherDay means roughly double
    let days = if med.frequency == MedicationFrequency::EveryOtherDay {
        days * 2
    } else if med.frequency == MedicationFrequency::Weekly {
        days * 7
    } else {
        days
    };
    Some(days)
}

/// Render a remaining-days badge with appropriate color.
fn remaining_days_badge(days: i32) -> impl IntoView {
    let (class, label) = if days <= 3 {
        ("bg-danger-light text-danger", "약 보충 필요".to_string())
    } else if days <= 7 {
        (
            "bg-warning-light text-warning",
            format!("{}일분 남음", days),
        )
    } else {
        (
            "bg-success-light text-success",
            format!("{}일분 남음", days),
        )
    };
    view! {
        <span class={format!("inline-block mt-1 text-xs px-2 py-1 rounded-full {}", class)}>
            {label}
        </span>
    }
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
                                            let timing_text = med.instruction_timing.as_ref().map(|t| {
                                                format_instruction_timing(t, med.instruction_minutes)
                                            });
                                            let remaining = estimate_remaining_days(&med);
                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                                    <div class="flex items-center justify-between">
                                                        <p class="text-lg font-medium text-txt-primary">{med.name}</p>
                                                        <span class={format!("text-xs px-2 py-1 rounded-full {active_class}")}>{active_label}</span>
                                                    </div>
                                                    <p class="text-base text-txt-secondary mt-1">{format!("{} · {}", med.dosage, med.form)}</p>
                                                    {timing_text.map(|txt| view! {
                                                        <p class="text-sm text-txt-tertiary mt-1">{txt}</p>
                                                    })}
                                                    <div class="flex items-center gap-2 mt-2">
                                                        <span class="inline-block text-xs px-2 py-1 rounded-full bg-primary-light text-primary">
                                                            {format!("{}", med.frequency)}
                                                        </span>
                                                        {remaining.map(|days| remaining_days_badge(days))}
                                                    </div>
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

/// Single medication detail with schedule and events -- fetches from API.
#[component]
pub fn MedicationDetailPage(
    #[prop(into)] person_id: Uuid,
    #[prop(into)] medication_id: Uuid,
) -> impl IntoView {
    let _ = person_id;
    let med_id = medication_id;
    let medication = LocalResource::new(move || {
        let id = med_id;
        async move {
            crate::api::get::<serde_json::Value>(&format!("/api/medications/{}", id)).await
        }
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/medications" class="text-primary text-lg">"< 약물 목록"</a>

            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match medication.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(data) => {
                                    // Response may have nested { medication: {...}, schedules: [...] }
                                    let med = data.get("medication").unwrap_or(&data);
                                    let name = med.get("name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("알 수 없음")
                                        .to_string();
                                    let dosage = med.get("dosage")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let form = med.get("form")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let frequency = med.get("frequency")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let prescribed_by = med.get("prescribed_by")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    let start_date_raw = med.get("start_date")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let start_date_short: String = start_date_raw.chars().take(10).collect();
                                    let end_date = med.get("end_date")
                                        .and_then(|v| v.as_str())
                                        .map(|s| {
                                            let short: String = s.chars().take(10).collect();
                                            short
                                        });
                                    let side_effects = med.get("side_effects")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    let notes = med.get("notes")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());

                                    // Instruction timing
                                    let instruction_timing_text = med.get("instruction_timing")
                                        .and_then(|v| v.as_str())
                                        .map(|timing_str| {
                                            let base = match timing_str {
                                                "BeforeMeal" => "식전",
                                                "WithMeal" => "식사 중",
                                                "AfterMeal" => "식후",
                                                "EmptyStomach" => "공복",
                                                "Bedtime" => "취침 전",
                                                "Anytime" => "시간 무관",
                                                other => other,
                                            };
                                            let minutes = med.get("instruction_minutes")
                                                .and_then(|v| v.as_i64());
                                            match minutes {
                                                Some(m) if m > 0 => format!("{} {}분", base, m),
                                                _ => base.to_string(),
                                            }
                                        });
                                    let instruction_text_val = med.get("instruction_text")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());

                                    // Remaining quantity
                                    let total_qty = med.get("total_quantity")
                                        .and_then(|v| v.as_i64());
                                    let doses_per_intake = med.get("doses_per_intake")
                                        .and_then(|v| v.as_i64())
                                        .unwrap_or(1)
                                        .max(1);
                                    let remaining_info = total_qty.map(|qty| {
                                        format!("총 {}정 (1회 {}정)", qty, doses_per_intake)
                                    });

                                    view! {
                                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                            <h1 class="text-xl font-bold text-txt-primary mb-3">{name}</h1>
                                            <InfoRow label="용량".to_string() value=dosage />
                                            <InfoRow label="제형".to_string() value=form />
                                            <InfoRow label="복용 빈도".to_string() value=frequency />
                                            {instruction_timing_text.map(|t| view! {
                                                <InfoRow label="복용 지시".to_string() value=t />
                                            })}
                                            {instruction_text_val.map(|t| view! {
                                                <InfoRow label="추가 지시".to_string() value=t />
                                            })}
                                            {remaining_info.map(|r| view! {
                                                <InfoRow label="잔여 수량".to_string() value=r />
                                            })}
                                            {prescribed_by.map(|p| view! {
                                                <InfoRow label="처방의".to_string() value=p />
                                            })}
                                            <InfoRow label="시작일".to_string() value=start_date_short />
                                            {end_date.map(|e| view! {
                                                <InfoRow label="종료일".to_string() value=e />
                                            })}
                                            {side_effects.map(|s| view! {
                                                <div class="mt-3 p-3 bg-warning-light rounded-xl">
                                                    <p class="text-sm text-warning font-medium">"부작용"</p>
                                                    <p class="text-base text-txt-primary mt-1">{s}</p>
                                                </div>
                                            })}
                                            {notes.map(|n| view! {
                                                <div class="mt-3 p-3 bg-surface-subtle rounded-xl">
                                                    <p class="text-sm text-txt-tertiary font-medium">"메모"</p>
                                                    <p class="text-base text-txt-primary mt-1">{n}</p>
                                                </div>
                                            })}
                                        </div>
                                    }.into_any()
                                }
                                None => view! { <EmptyState message="약물 정보를 찾을 수 없습니다." /> }.into_any(),
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

/// Group label for time-of-day medication slots.
fn time_slot_label(slot: &str) -> &'static str {
    match slot {
        "MORNING" => "아침",
        "AFTERNOON" => "점심",
        "EVENING" => "저녁",
        "BEDTIME" => "취침전",
        _ => "기타",
    }
}

/// Today's medication events with taken/missed status -- fetches from API.
#[component]
pub fn MedicationLogPage() -> impl IntoView {
    let today = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/medications/today")
    });

    let status_error = RwSignal::new(Option::<String>::None);

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="오늘의 복약 기록" subtitle="복약 상태를 확인하세요" />

            <Show when=move || status_error.get().is_some()>
                <div class="bg-danger-light rounded-2xl p-4 text-danger text-lg">
                    {move || status_error.get().unwrap_or_default()}
                </div>
            </Show>

            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match today.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! { <EmptyState message="오늘 예정된 복약이 없습니다." /> }.into_any()
                            } else {
                                // Compute summary
                                let total = items.len();
                                let taken_count = items.iter()
                                    .filter(|e| {
                                        e.get("status")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s == "TAKEN")
                                            .unwrap_or(false)
                                    })
                                    .count();

                                // Group by time_slot
                                let slots = ["MORNING", "AFTERNOON", "EVENING", "BEDTIME", "OTHER"];

                                view! {
                                    <div class="space-y-4">
                                        // Summary card
                                        <div class="bg-primary-light rounded-2xl p-4 text-center">
                                            <p class="text-lg font-semibold text-primary">
                                                {format!("{}/{} 복용 완료", taken_count, total)}
                                            </p>
                                        </div>

                                        // Group by time slot
                                        {slots.into_iter().filter_map(|slot| {
                                            let slot_items: Vec<&serde_json::Value> = items.iter()
                                                .filter(|e| {
                                                    let s = e.get("time_slot")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("OTHER");
                                                    s == slot
                                                })
                                                .collect();

                                            if slot_items.is_empty() {
                                                return None;
                                            }

                                            let label = time_slot_label(slot);
                                            let cards = slot_items.into_iter().map(|event| {
                                                let med_name = event.get("medication_name")
                                                    .or_else(|| event.get("name"))
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("약물")
                                                    .to_string();
                                                let dosage = event.get("dosage")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                let time = event.get("scheduled_time")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                let status = event.get("status")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("SCHEDULED")
                                                    .to_string();
                                                let event_id = event.get("id")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                let (status_label, status_class) = match status.as_str() {
                                                    "TAKEN" => ("복용 완료", "bg-success-light text-success"),
                                                    "MISSED" => ("미복용", "bg-danger-light text-danger"),
                                                    _ => ("예정", "bg-primary-light text-primary"),
                                                };
                                                let is_scheduled = status == "SCHEDULED";
                                                let take_id = event_id.clone();
                                                let miss_id = event_id.clone();

                                                view! {
                                                    <div class="bg-surface-card rounded-2xl p-4 shadow-sm">
                                                        <div class="flex items-center justify-between">
                                                            <div>
                                                                <p class="text-lg font-medium text-txt-primary">{med_name}</p>
                                                                {if !dosage.is_empty() {
                                                                    Some(view! { <p class="text-sm text-txt-secondary">{dosage}</p> })
                                                                } else { None }}
                                                                {if !time.is_empty() {
                                                                    Some(view! { <p class="text-sm text-txt-tertiary">{time}</p> })
                                                                } else { None }}
                                                            </div>
                                                            <span class={format!("text-xs px-2 py-1 rounded-full {status_class}")}>{status_label}</span>
                                                        </div>
                                                        {if is_scheduled {
                                                            let err_sig = status_error;
                                                            Some(view! {
                                                                <div class="flex gap-2 mt-3">
                                                                    <button
                                                                        class="flex-1 bg-success text-white text-sm font-medium rounded-xl \
                                                                               py-2 hover:opacity-90 active:scale-[0.98] transition-all"
                                                                        on:click=move |_| {
                                                                            let id = take_id.clone();
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
                                                                    <button
                                                                        class="flex-1 bg-danger text-white text-sm font-medium rounded-xl \
                                                                               py-2 hover:opacity-90 active:scale-[0.98] transition-all"
                                                                        on:click=move |_| {
                                                                            let id = miss_id.clone();
                                                                            let err = err_sig;
                                                                            leptos::task::spawn_local(async move {
                                                                                let body = serde_json::json!({"status": "MISSED"});
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
                                                                    >"미복용"</button>
                                                                </div>
                                                            })
                                                        } else {
                                                            None
                                                        }}
                                                    </div>
                                                }
                                            }).collect_view();

                                            Some(view! {
                                                <div class="space-y-2">
                                                    <h3 class="text-base font-semibold text-txt-secondary">{label}</h3>
                                                    {cards}
                                                </div>
                                            })
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
