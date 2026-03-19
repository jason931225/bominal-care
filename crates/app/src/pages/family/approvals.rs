use leptos::prelude::*;
use uuid::Uuid;

// =============================================================================
// Approval pages — list and detail
// =============================================================================

/// Lists pending approval items requiring family member decisions.
#[component]
pub fn ApprovalsListPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"승인 대기 목록"</h1>
                <p class="text-sm text-txt-secondary mt-1">"결정이 필요한 항목들입니다."</p>
            </div>

            <div class="space-y-3">
                {
                    let id1 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"approval-care-plan-change").to_string();
                    let id2 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"approval-medication-change").to_string();
                    let href1 = format!("/family/approvals/{id1}");
                    let href2 = format!("/family/approvals/{id2}");
                    view! {
                        <a href=href1 class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="flex justify-between items-center">
                                <div>
                                    <p class="font-medium text-txt-primary">"케어 플랜 변경 승인"</p>
                                    <p class="text-sm text-txt-tertiary">"방문 시간 변경 요청"</p>
                                </div>
                                <span class="text-xs px-2 py-1 rounded-full bg-warning-light text-warning">"대기 중"</span>
                            </div>
                        </a>
                        <a href=href2 class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="flex justify-between items-center">
                                <div>
                                    <p class="font-medium text-txt-primary">"약물 변경 승인"</p>
                                    <p class="text-sm text-txt-tertiary">"처방 변경에 대한 동의 필요"</p>
                                </div>
                                <span class="text-xs px-2 py-1 rounded-full bg-warning-light text-warning">"대기 중"</span>
                            </div>
                        </a>
                    }
                }
            </div>
        </div>
    }
}

/// Shows individual approval detail with approve and reject action buttons.
#[component]
pub fn ApprovalDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"승인 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"승인 요청의 상세 내용입니다."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <p class="text-sm text-txt-tertiary">"요청 유형"</p>
                    <p class="font-medium text-txt-primary">"케어 플랜 변경"</p>
                </div>
                <div>
                    <p class="text-sm text-txt-tertiary">"요청 내용"</p>
                    <p class="text-sm text-txt-secondary">"방문 시간을 오전 10시에서 오후 2시로 변경 요청합니다."</p>
                </div>
                <div>
                    <p class="text-sm text-txt-tertiary">"요청일"</p>
                    <p class="text-sm text-txt-secondary">"2026-03-15"</p>
                </div>
                <div class="flex gap-3">
                    <button class="flex-1 bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">"승인"</button>
                    <button class="flex-1 border border-danger text-danger rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-danger-light active:scale-[0.98] transition-all">"거부"</button>
                </div>
            </div>
        </div>
    }
}
