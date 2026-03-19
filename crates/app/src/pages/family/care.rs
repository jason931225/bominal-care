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
    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"케어 플랜"</h1>
                <p class="text-sm text-txt-secondary mt-1">"어르신의 돌봄 계획입니다."</p>
            </div>
            <div class="skeleton h-4 w-48"></div>
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
