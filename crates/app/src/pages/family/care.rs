use leptos::prelude::*;

// =============================================================================
// Care pages — timeline, medications, care plan, observability
// =============================================================================

/// Displays a 30-day care timeline with visits, medications, and appointments.
#[component]
pub fn TimelinePage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"케어 타임라인"</h1>
                <p class="text-sm text-txt-secondary mt-1">"최근 30일간의 돌봄 기록입니다."</p>
            </div>
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <div class="skeleton h-4 w-48"></div>
            </div>
        </div>
    }
}

/// Displays the senior's current medications and adherence status.
#[component]
pub fn MedicationsPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"복약 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"어르신의 복약 현황입니다."</p>
            </div>
            <div class="skeleton h-4 w-48"></div>
        </div>
    }
}

/// Shows the senior's active care plan with goals and schedule.
#[component]
pub fn CarePlanPage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<Vec<bominal_types::CarePlan>>("/api/care-plans")
    });

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
            <div class="skeleton h-4 w-48"></div>
        </div>
    }
}
