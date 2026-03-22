use leptos::prelude::*;

// =============================================================================
// Approval pages — list and detail
// =============================================================================

/// Lists pending approval items requiring family member decisions.
#[component]
pub fn ApprovalsListPage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<Vec<bominal_types::ApprovalStep>>(
            "/api/care-plans?pending_approval=true",
        )
    });

    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"승인 대기 목록"</h1>
                <p class="text-sm text-txt-secondary mt-1">"결정이 필요한 항목들입니다."</p>
            </div>

            <div class="space-y-3">
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
                                            {items.into_iter().map(|item| {
                                                let href = format!("/family/approvals/{}", item.id);
                                                let step_name = item.step_name.clone();
                                                let status = item.status.clone();
                                                view! {
                                                    <a href=href class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                                                        <div class="flex justify-between items-center">
                                                            <div>
                                                                <p class="font-medium text-txt-primary">{step_name}</p>
                                                                <p class="text-sm text-txt-tertiary">{status}</p>
                                                            </div>
                                                            <span class="text-xs px-2 py-1 rounded-full bg-warning-light text-warning">"대기 중"</span>
                                                        </div>
                                                    </a>
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
