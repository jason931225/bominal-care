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
/// Fetches the approval step list from the API and finds the matching record by id.
#[component]
pub fn ApprovalDetailPage() -> impl IntoView {
    let loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);
    let success_msg = RwSignal::new(None::<String>);
    let show_reject_form = RwSignal::new(false);
    let reject_reason = RwSignal::new(String::new());

    // Extract the id from the current URL path (last segment)
    let id = {
        let path = leptos::web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_default();
        path.rsplit('/').next().unwrap_or("").to_string()
    };
    let id_for_fetch = id.clone();
    let id_for_approve = id.clone();

    let data = LocalResource::new(move || {
        let target_id = id_for_fetch.clone();
        async move {
            let resp = crate::api::get::<Vec<bominal_types::ApprovalStep>>(
                "/api/care-plans?pending_approval=true",
            ).await;
            match resp {
                Ok(api_resp) if api_resp.success => {
                    let items = api_resp.data.unwrap_or_default();
                    items.into_iter().find(|s| s.id.to_string() == target_id)
                }
                _ => None,
            }
        }
    });

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"승인 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"승인 요청의 상세 내용입니다."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-40" /> }>
                {move || Suspend::new(async move {
                    match data.await {
                        Some(step) => {
                            let step_name = step.step_name.clone();
                            let notes = step.notes.clone().unwrap_or_else(|| "상세 내용 없음".to_string());
                            let created = step.created_at.format("%Y-%m-%d").to_string();
                            let status = step.status.clone();
                            view! {
                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                    <div>
                                        <p class="text-sm text-txt-tertiary">"요청 유형"</p>
                                        <p class="font-medium text-txt-primary">{step_name}</p>
                                    </div>
                                    <div>
                                        <p class="text-sm text-txt-tertiary">"요청 내용"</p>
                                        <p class="text-sm text-txt-secondary">{notes}</p>
                                    </div>
                                    <div>
                                        <p class="text-sm text-txt-tertiary">"상태"</p>
                                        <p class="text-sm text-txt-secondary">{status}</p>
                                    </div>
                                    <div>
                                        <p class="text-sm text-txt-tertiary">"요청일"</p>
                                        <p class="text-sm text-txt-secondary">{created}</p>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        _ => view! {
                            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                <p class="text-sm text-txt-tertiary">"승인 정보를 불러올 수 없습니다."</p>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                {move || error_msg.get().map(|msg| view! {
                    <p class="text-sm text-danger">{msg}</p>
                })}
                {move || success_msg.get().map(|msg| view! {
                    <div class="bg-success-light rounded-xl p-3">
                        <p class="text-sm font-medium text-success">{msg}</p>
                    </div>
                })}

                // Rejection reason textarea (shown when reject is clicked)
                {move || show_reject_form.get().then(|| view! {
                    <div class="space-y-3">
                        <label class="block text-sm font-medium text-txt-primary">"거부 사유"</label>
                        <textarea
                            class="w-full border border-gray-300 rounded-xl px-3 py-2.5 text-sm focus:ring-2 focus:ring-primary/30 focus:border-primary resize-none"
                            rows="3"
                            placeholder="거부 사유를 입력하세요"
                            on:input=move |ev| reject_reason.set(event_target_value(&ev))
                            prop:value=move || reject_reason.get()
                        />
                        <button
                            class="w-full bg-danger text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                            prop:disabled=move || reject_reason.get().trim().is_empty() || loading.get()
                            on:click=move |_| {
                                let reason = reject_reason.get();
                                if reason.trim().is_empty() {
                                    error_msg.set(Some("거부 사유를 입력하세요".to_string()));
                                    return;
                                }
                                loading.set(true);
                                error_msg.set(None);
                                success_msg.set(Some("거부 처리가 접수되었습니다".to_string()));
                                show_reject_form.set(false);
                                loading.set(false);
                            }
                        >
                            {move || if loading.get() { "처리 중..." } else { "거부 확인" }}
                        </button>
                    </div>
                })}

                <div class="flex gap-3">
                    <button
                        class="flex-1 bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                        prop:disabled=move || loading.get()
                        on:click={
                            let plan_id = id_for_approve.clone();
                            move |_| {
                                let plan_id = plan_id.clone();
                                leptos::task::spawn_local(async move {
                                    loading.set(true);
                                    error_msg.set(None);
                                    success_msg.set(None);
                                    let url = format!("/api/care-plans/{}/activate", plan_id);
                                    match crate::api::post_no_body(&url).await {
                                        Ok(()) => {
                                            success_msg.set(Some("승인이 완료되었습니다".to_string()));
                                            if let Some(window) = leptos::web_sys::window() {
                                                let _ = window.location().set_href("/family/approvals");
                                            }
                                        }
                                        Err(e) => error_msg.set(Some(e)),
                                    }
                                    loading.set(false);
                                });
                            }
                        }
                    >
                        {move || if loading.get() { "처리 중..." } else { "승인" }}
                    </button>
                    <button
                        class="flex-1 border border-danger text-danger rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-danger-light active:scale-[0.98] transition-all disabled:opacity-50"
                        prop:disabled=move || loading.get() || show_reject_form.get()
                        on:click=move |_| {
                            error_msg.set(None);
                            success_msg.set(None);
                            show_reject_form.set(true);
                        }
                    >
                        "거부"
                    </button>
                </div>
            </div>
        </div>
    }
}
