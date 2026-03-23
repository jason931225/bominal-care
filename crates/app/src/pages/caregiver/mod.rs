use std::collections::HashSet;

use leptos::prelude::*;
use bominal_types::Visit;

pub mod schedule;
pub mod clients;
pub mod apply;
pub mod tasks;
pub mod profile;

pub use schedule::*;
pub use clients::*;
pub use apply::*;
pub use tasks::*;
pub use profile::*;

// =============================================================================
// 1. DashboardPage — today's stats, next visit, alerts, weekly summary
// =============================================================================

/// Compute total scheduled work hours from a list of visits.
fn compute_work_hours(visits: &[Visit]) -> f64 {
    visits.iter().map(|v| {
        let duration = v.scheduled_end - v.scheduled_start;
        duration.num_minutes() as f64 / 60.0
    }).sum()
}

/// Count distinct clients (by care_plan_id as a proxy).
fn count_unique_clients(visits: &[Visit]) -> usize {
    let set: HashSet<_> = visits.iter().map(|v| v.care_plan_id).collect();
    set.len()
}

/// Format hours as a display string: "8.5" or "0".
fn format_hours(h: f64) -> String {
    if h == 0.0 {
        "0".to_string()
    } else if (h - h.round()).abs() < 0.01 {
        format!("{}", h as u32)
    } else {
        format!("{:.1}", h)
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let visits = LocalResource::new(|| {
        crate::api::get::<Vec<Visit>>("/api/visits")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"요양보호사 대시보드"</h1>
                <p class="text-sm text-gray-600 mt-1">"오늘의 스케줄과 업무를 확인하세요."</p>
            </div>

            <Suspense fallback=move || view! {
                <div class="grid grid-cols-2 gap-4">
                    <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                        <p class="text-sm text-gray-500">"오늘 방문"</p>
                        <p class="text-2xl font-bold text-gray-300 mt-1">"..."</p>
                    </div>
                    <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                        <p class="text-sm text-gray-500">"근무 시간"</p>
                        <p class="text-2xl font-bold text-gray-300 mt-1">"..."</p>
                    </div>
                </div>
            }>
                {move || Suspend::new(async move {
                    match visits.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            let visit_count = items.len();
                            let work_hours = compute_work_hours(&items);
                            let work_hours_str = format_hours(work_hours);
                            let unique_clients = count_unique_clients(&items);
                            let next_visit = items.first().cloned();

                            view! {
                                <div>
                                    // Today's stats
                                    <div class="grid grid-cols-2 gap-4">
                                        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                            <p class="text-sm text-gray-500">"오늘 방문"</p>
                                            <p class="text-2xl font-bold text-gray-900 mt-1">
                                                {visit_count.to_string()}<span class="text-sm font-normal text-gray-500">" 건"</span>
                                            </p>
                                        </div>
                                        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                            <p class="text-sm text-gray-500">"근무 시간"</p>
                                            <p class="text-2xl font-bold text-teal-600 mt-1">{work_hours_str.clone()}<span class="text-sm font-normal text-gray-500">" 시간"</span></p>
                                        </div>
                                    </div>

                                    // Next visit card
                                    {match next_visit {
                                        Some(v) => {
                                            let start = format!("{}", v.scheduled_start.format("%H:%M"));
                                            let end = format!("{}", v.scheduled_end.format("%H:%M"));
                                            let status = format!("{}", v.status);
                                            let visit_id = v.id.to_string();
                                            let visit_id2 = v.id.to_string();
                                            view! {
                                                <div class="mt-6 bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                    <div class="flex items-center justify-between mb-3">
                                                        <h2 class="font-semibold text-gray-900">"다음 방문"</h2>
                                                        <span class="text-xs font-medium text-teal-700 bg-teal-50 px-2 py-1 rounded-full">{status}</span>
                                                    </div>
                                                    <div class="flex items-center gap-3">
                                                        <div class="w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center">
                                                            <svg class="w-5 h-5 text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                                <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                                            </svg>
                                                        </div>
                                                        <div>
                                                            <p class="text-sm text-gray-500">{format!("{start} - {end}")}</p>
                                                        </div>
                                                    </div>
                                                    <div class="mt-3 flex gap-2">
                                                        <a href={format!("/caregiver/schedule/{visit_id}")} class="flex-1 text-center py-2 bg-teal-600 text-white text-sm rounded-lg hover:bg-teal-700">"상세보기"</a>
                                                        <a href={format!("/caregiver/check-in/{visit_id2}")} class="flex-1 text-center py-2 border border-teal-600 text-teal-600 text-sm rounded-lg hover:bg-teal-50">"체크인"</a>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        }
                                        None => view! {
                                            <div class="mt-6 bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                <p class="text-sm text-gray-500">"예정된 방문이 없습니다."</p>
                                            </div>
                                        }.into_any(),
                                    }}

                                    // Remaining visits list
                                    {if items.len() > 1 {
                                        let remaining = items[1..].to_vec();
                                        view! {
                                            <div class="mt-4 space-y-2">
                                                {remaining.into_iter().map(|v| {
                                                    let start = format!("{}", v.scheduled_start.format("%H:%M"));
                                                    let end = format!("{}", v.scheduled_end.format("%H:%M"));
                                                    let status = format!("{}", v.status);
                                                    view! {
                                                        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 flex items-center justify-between">
                                                            <p class="text-sm text-gray-700">{format!("{start} - {end}")}</p>
                                                            <span class="text-xs px-2 py-1 rounded-full bg-gray-100 text-gray-600">{status}</span>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}

                                    // Weekly summary — computed from visit data
                                    <div class="mt-6 bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                        <h2 class="font-semibold text-gray-900 mb-3">"이번 주 요약"</h2>
                                        <div class="grid grid-cols-3 gap-3 text-center">
                                            <div>
                                                <p class="text-lg font-bold text-gray-900">{visit_count.to_string()}</p>
                                                <p class="text-xs text-gray-500">"총 방문"</p>
                                            </div>
                                            <div>
                                                <p class="text-lg font-bold text-teal-600">{work_hours_str}</p>
                                                <p class="text-xs text-gray-500">"근무 시간"</p>
                                            </div>
                                            <div>
                                                <p class="text-lg font-bold text-blue-600">{unique_clients.to_string()}</p>
                                                <p class="text-xs text-gray-500">"고객 수"</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        Ok(resp) => view! { <p class="text-red-500">{resp.error.unwrap_or_default()}</p> }.into_any(),
                        Err(e) => view! { <p class="text-red-500">{e}</p> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
