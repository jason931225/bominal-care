use leptos::prelude::*;

// =============================================================================
// Care pages — timeline, medications, care plan, observability
// =============================================================================

/// Displays a 30-day care timeline with visits, medications, and appointments.
#[component]
pub fn TimelinePage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/visits")
    });

    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"케어 타임라인"</h1>
                <p class="text-sm text-txt-secondary mt-1">"최근 30일간의 돌봄 기록입니다."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-20" /> }>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            let visits: Vec<serde_json::Value> = items.into_iter().take(30).collect();
                            if visits.is_empty() {
                                view! {
                                    <p class="text-center text-txt-secondary py-8">"아직 기록이 없습니다"</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-0">
                                        {visits.into_iter().map(|visit| {
                                            let date = visit.get("scheduled_start")
                                                .or_else(|| visit.get("date"))
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.chars().take(10).collect::<String>())
                                                .unwrap_or_else(|| "-".to_string());
                                            let caregiver = visit.get("caregiver_name")
                                                .or_else(|| visit.get("caregiver_id"))
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string())
                                                .unwrap_or_else(|| "미지정".to_string());
                                            let status = visit.get("status")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("unknown")
                                                .to_string();
                                            let duration = {
                                                let mins = visit.get("duration_minutes")
                                                    .and_then(|v| v.as_i64());
                                                match mins {
                                                    Some(m) => format!("{}분", m),
                                                    None => "-".to_string(),
                                                }
                                            };
                                            let (badge_bg, badge_text) = match status.as_str() {
                                                "completed" | "Completed" => ("bg-success-light text-success", "완료"),
                                                "in_progress" | "InProgress" => ("bg-blue-100 text-blue-700", "진행 중"),
                                                "scheduled" | "Scheduled" => ("bg-yellow-100 text-yellow-700", "예정"),
                                                "missed" | "Missed" => ("bg-red-100 text-red-700", "미방문"),
                                                _ => ("bg-gray-100 text-gray-600", "기타"),
                                            };
                                            view! {
                                                <div class="relative pl-8 pb-6 border-l-2 border-gray-200">
                                                    <div class="absolute -left-2 top-0 w-4 h-4 rounded-full bg-[var(--portal-accent)]"></div>
                                                    <div class="bg-surface-card rounded-2xl p-4 shadow-sm">
                                                        <div class="flex justify-between items-start mb-2">
                                                            <p class="text-sm font-semibold text-txt-primary">{date}</p>
                                                            <span class={format!("text-xs px-2 py-0.5 rounded-full {}", badge_bg)}>{badge_text}</span>
                                                        </div>
                                                        <p class="text-sm text-txt-secondary">"담당: "{caregiver}</p>
                                                        <p class="text-xs text-txt-tertiary mt-1">"소요 시간: "{duration}</p>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-txt-secondary py-8">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Displays the senior's current medications and adherence status.
#[component]
pub fn MedicationsPage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/medications")
    });

    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"복약 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"어르신의 복약 현황입니다."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-20" /> }>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-txt-secondary py-8">"등록된 약물이 없습니다"</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|med| {
                                            let name = med.get("name")
                                                .or_else(|| med.get("medication_name"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("약물명 없음")
                                                .to_string();
                                            let dosage = med.get("dosage")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("-")
                                                .to_string();
                                            let form = med.get("form")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                            let frequency = med.get("frequency")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("-")
                                                .to_string();
                                            let is_active = med.get("is_active")
                                                .and_then(|v| v.as_bool())
                                                .unwrap_or(true);
                                            let (badge_cls, badge_label) = if is_active {
                                                ("bg-success-light text-success", "복용 중")
                                            } else {
                                                ("bg-gray-100 text-txt-disabled", "중단")
                                            };
                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                                    <div class="flex justify-between items-start mb-2">
                                                        <p class="font-semibold text-txt-primary">{name}</p>
                                                        <span class={format!("text-xs px-2 py-1 rounded-full {}", badge_cls)}>{badge_label}</span>
                                                    </div>
                                                    <p class="text-sm text-txt-secondary">{dosage}
                                                        {if !form.is_empty() {
                                                            format!(" · {}", form)
                                                        } else {
                                                            String::new()
                                                        }}
                                                    </p>
                                                    <p class="text-xs text-txt-tertiary mt-1">"복용 주기: "{frequency}</p>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-txt-secondary py-8">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Shows the senior's active care plan with goals and schedule.
#[component]
pub fn CarePlanPage() -> impl IntoView {
    let data =
        LocalResource::new(|| crate::api::get::<Vec<bominal_types::CarePlan>>("/api/care-plans"));

    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"케어 플랜"</h1>
                <p class="text-sm text-txt-secondary mt-1">"어르신의 돌봄 계획입니다."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-20" /> }>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-txt-secondary py-8">"데이터가 없습니다."</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|plan| {
                                            let title = plan.title.clone();
                                            let description = plan.description.clone().unwrap_or_default();
                                            let status = format!("{}", plan.status);
                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                                    <div class="flex justify-between items-center">
                                                        <p class="font-semibold text-txt-primary">{title}</p>
                                                        <span class="text-xs px-2 py-1 rounded-full bg-[var(--portal-accent-light)] text-[var(--portal-accent)]">{status}</span>
                                                    </div>
                                                    {if !description.is_empty() {
                                                        view! { <p class="text-sm text-txt-secondary">{description}</p> }.into_any()
                                                    } else {
                                                        view! { <></> }.into_any()
                                                    }}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-txt-secondary py-8">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Displays care quality observability signals and metrics.
#[component]
pub fn ObservabilityPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"케어 품질 신호"</h1>
                <p class="text-sm text-txt-secondary mt-1">"돌봄 품질 관련 신호를 확인하세요."</p>
            </div>
            <div class="bg-surface-card rounded-2xl p-6 shadow-sm text-center space-y-3">
                <div class="mx-auto w-14 h-14 bg-surface-subtle rounded-full flex items-center justify-center">
                    <svg class="w-7 h-7 text-txt-disabled" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
                    </svg>
                </div>
                <p class="text-sm font-medium text-txt-primary">"케어 품질 모니터링"</p>
                <p class="text-sm text-txt-tertiary">"케어 품질 모니터링 기능이 준비 중입니다."</p>
                <span class="inline-block text-xs px-3 py-1 rounded-full bg-surface-subtle text-txt-disabled">"준비 중"</span>
            </div>
        </div>
    }
}
