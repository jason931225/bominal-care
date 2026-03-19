// =============================================================================
// Family Portal Pages
// =============================================================================
//
// Split into domain modules:
//   - approvals: ApprovalsListPage, ApprovalDetailPage
//   - care: TimelinePage, MedicationsPage, CarePlanPage, ObservabilityPage
//   - documents: DocumentsPage, DocumentDetailPage
//   - eligibility: EligibilityPage, EligibilityApplyPage
//   - help: HelpSeniorPage, HelpBookPage, HelpEmergencyPage, HelpReportPage
//   - matching: MatchingSearchPage, MatchingResultsPage, MatchingDetailPage
//   - payments: PaymentsListPage, PaymentDetailPage
//   - settings: SettingsPage, ProfilePage, NotificationsPage
// =============================================================================

mod approvals;
mod care;
mod documents;
mod eligibility;
mod help;
mod matching;
mod payments;
mod settings;

pub use approvals::*;
pub use care::*;
pub use documents::*;
pub use eligibility::*;
pub use help::*;
pub use matching::*;
pub use payments::*;
pub use settings::*;

use leptos::prelude::*;
use bominal_types::Visit;

// =============================================================================
// Dashboard
// =============================================================================

/// Unread notification count response.
#[derive(Debug, Clone, serde::Deserialize)]
struct UnreadCount {
    pub count: i64,
}

/// Family dashboard showing linked senior overview, timeline preview, and alerts.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let visits = LocalResource::new(|| {
        crate::api::get::<Vec<Visit>>("/api/visits")
    });
    let unread = LocalResource::new(|| {
        crate::api::get::<UnreadCount>("/api/notifications/unread-count")
    });

    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"가족 케어 대시보드"</h1>
                <p class="text-sm text-txt-secondary mt-1">"돌봄 대상자의 현황을 한눈에 확인하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                // Today's schedule card
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <h3 class="text-sm text-txt-tertiary">"오늘의 일정"</h3>
                    <Suspense fallback=move || view! { <div class="skeleton h-8 w-20 mt-1"></div> }>
                        {move || Suspend::new(async move {
                            match visits.await {
                                Ok(resp) if resp.success => {
                                    let items = resp.data.unwrap_or_default();
                                    let count = items.len();
                                    view! {
                                        <p class="text-2xl font-bold text-txt-primary mt-1">
                                            {count.to_string()}<span class="text-sm font-normal text-txt-tertiary">" 건"</span>
                                        </p>
                                    }.into_any()
                                }
                                _ => view! { <p class="text-2xl font-bold text-txt-primary mt-1">"—"</p> }.into_any(),
                            }
                        })}
                    </Suspense>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <h3 class="text-sm text-txt-tertiary">"복약 상태"</h3>
                    <p class="text-2xl font-bold text-success mt-1">"정상"</p>
                    <p class="text-sm text-txt-tertiary mt-1">"오전 약 복용 완료"</p>
                </div>
                // Notifications card
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <h3 class="text-sm text-txt-tertiary">"알림"</h3>
                    <Suspense fallback=move || view! { <div class="skeleton h-8 w-20 mt-1"></div> }>
                        {move || Suspend::new(async move {
                            match unread.await {
                                Ok(resp) if resp.success => {
                                    let cnt = resp.data.map(|d| d.count).unwrap_or(0);
                                    if cnt > 0 {
                                        view! {
                                            <p class="text-2xl font-bold text-warning mt-1">
                                                {cnt.to_string()}<span class="text-sm font-normal text-txt-tertiary">" 건"</span>
                                            </p>
                                        }.into_any()
                                    } else {
                                        view! { <p class="text-2xl font-bold text-txt-disabled mt-1">"없음"</p> }.into_any()
                                    }
                                }
                                _ => view! { <p class="text-2xl font-bold text-txt-primary mt-1">"—"</p> }.into_any(),
                            }
                        })}
                    </Suspense>
                </div>
            </div>

            // Timeline preview
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-3">"케어 타임라인 미리보기"</h2>
                <Suspense fallback=move || view! { <div class="skeleton h-4 w-48"></div> }>
                    {move || Suspend::new(async move {
                        match visits.await {
                            Ok(resp) if resp.success => {
                                let items = resp.data.unwrap_or_default();
                                if items.is_empty() {
                                    view! { <p class="text-sm text-txt-tertiary">"예정된 방문이 없습니다."</p> }.into_any()
                                } else {
                                    view! {
                                        <ul class="space-y-2 text-sm text-txt-secondary">
                                            {items.into_iter().map(|v| {
                                                let time_str = format!("{}", v.scheduled_start.format("%H:%M"));
                                                let status_str = format!("{}", v.status);
                                                view! {
                                                    <li class="flex items-center gap-2">
                                                        <span class="w-2 h-2 rounded-full bg-[var(--portal-accent)]"></span>
                                                        {format!("{time_str} — {status_str}")}
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    }.into_any()
                                }
                            }
                            _ => view! { <p class="text-sm text-txt-tertiary">"데이터를 불러올 수 없습니다."</p> }.into_any(),
                        }
                    })}
                </Suspense>
                <a href="/family/timeline" class="text-sm text-[var(--portal-accent)] hover:underline mt-3 inline-block">"전체 타임라인 보기 →"</a>
            </div>
        </div>
    }
}

// =============================================================================
// Stub
// =============================================================================

/// Generic placeholder page for routes not yet implemented.
#[component]
pub fn StubPage() -> impl IntoView {
    view! {
        <div class="p-6">
            <div class="skeleton h-4 w-32"></div>
        </div>
    }
}
