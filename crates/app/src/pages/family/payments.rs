use leptos::prelude::*;
use uuid::Uuid;

// =============================================================================
// Payment pages — list and detail
// =============================================================================

/// Displays payment history for care services with status badges.
#[component]
pub fn PaymentsListPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"결제 내역"</h1>
                <p class="text-sm text-txt-secondary mt-1">"돌봄 서비스 결제 기록입니다."</p>
            </div>

            <div class="space-y-3">
                {
                    let id1 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"payment-march-home-care").to_string();
                    let id2 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"payment-feb-home-care").to_string();
                    let href1 = format!("/family/payments/{id1}");
                    let href2 = format!("/family/payments/{id2}");
                    view! {
                        <a href=href1 class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="flex justify-between items-center">
                                <div>
                                    <p class="font-medium text-txt-primary">"3월 방문요양 서비스"</p>
                                    <p class="text-sm text-txt-tertiary">"2026-03-01 ~ 2026-03-15"</p>
                                </div>
                                <div class="text-right">
                                    <p class="font-bold text-txt-primary">"₩320,000"</p>
                                    <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"결제완료"</span>
                                </div>
                            </div>
                        </a>
                        <a href=href2 class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="flex justify-between items-center">
                                <div>
                                    <p class="font-medium text-txt-primary">"2월 방문요양 서비스"</p>
                                    <p class="text-sm text-txt-tertiary">"2026-02-01 ~ 2026-02-28"</p>
                                </div>
                                <div class="text-right">
                                    <p class="font-bold text-txt-primary">"₩640,000"</p>
                                    <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"결제완료"</span>
                                </div>
                            </div>
                        </a>
                    }
                }
            </div>
        </div>
    }
}

/// Shows a single payment detail view with breakdown and status.
#[component]
pub fn PaymentDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"결제 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"결제 건의 상세 정보입니다."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"서비스"</p>
                    <p class="font-medium text-txt-primary">"방문요양"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"기간"</p>
                    <p class="text-sm text-txt-secondary">"2026-03-01 ~ 2026-03-15"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"총 금액"</p>
                    <p class="font-bold text-txt-primary">"₩320,000"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"본인부담금 (15%)"</p>
                    <p class="font-bold text-[var(--portal-accent)]">"₩48,000"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"결제 상태"</p>
                    <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"결제완료"</span>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"결제일"</p>
                    <p class="text-sm text-txt-secondary">"2026-03-16"</p>
                </div>
            </div>
        </div>
    }
}
