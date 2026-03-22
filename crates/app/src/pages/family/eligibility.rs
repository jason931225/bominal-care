use leptos::prelude::*;

// =============================================================================
// Eligibility pages — LTCI grade info and application
// =============================================================================

/// Shows LTCI eligibility grade information with renewal options.
#[component]
pub fn EligibilityPage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<Vec<bominal_types::EligibilityCase>>("/api/gov/eligibility-cases")
    });

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"장기요양 등급 안내"</h1>
                <p class="text-sm text-txt-secondary mt-1">"장기요양보험 등급 판정 정보입니다."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-20" /> }>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                        <p class="text-center text-txt-secondary py-4">"등급 정보가 없습니다."</p>
                                        <a href="/family/eligibility/apply" class="block text-center bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">
                                            "등급 신청"
                                        </a>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-4">
                                        {items.into_iter().map(|case| {
                                            let program_name = case.program_name.clone();
                                            let status = format!("{}", case.status);
                                            let application_date = case.application_date
                                                .map(|d| d.format("%Y-%m-%d").to_string())
                                                .unwrap_or_else(|| "-".to_string());
                                            let determination_date = case.determination_date
                                                .map(|d| d.format("%Y-%m-%d").to_string())
                                                .unwrap_or_else(|| "-".to_string());
                                            let notes = case.notes.clone().unwrap_or_default();
                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                                    <div>
                                                        <p class="text-sm text-txt-tertiary">"프로그램"</p>
                                                        <p class="text-lg font-bold text-[var(--portal-accent)]">{program_name}</p>
                                                    </div>
                                                    <div>
                                                        <p class="text-sm text-txt-tertiary">"상태"</p>
                                                        <p class="text-sm text-txt-secondary">{status}</p>
                                                    </div>
                                                    <div>
                                                        <p class="text-sm text-txt-tertiary">"신청일"</p>
                                                        <p class="text-sm text-txt-secondary">{application_date}</p>
                                                    </div>
                                                    <div>
                                                        <p class="text-sm text-txt-tertiary">"판정일"</p>
                                                        <p class="text-sm text-txt-secondary">{determination_date}</p>
                                                    </div>
                                                    {if !notes.is_empty() {
                                                        view! {
                                                            <div class="bg-[var(--portal-accent-light)] rounded-xl p-4">
                                                                <p class="text-sm text-[var(--portal-accent)]">{notes}</p>
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="bg-[var(--portal-accent-light)] rounded-xl p-4">
                                                                <p class="text-sm text-[var(--portal-accent)]">"등급 갱신이 필요하시면 아래 버튼을 눌러 신청하세요."</p>
                                                            </div>
                                                        }.into_any()
                                                    }}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                        <a href="/family/eligibility/apply" class="block text-center bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">
                                            "등급 신청 / 갱신"
                                        </a>
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                <p class="text-center text-txt-secondary py-4">"데이터를 불러올 수 없습니다."</p>
                                <a href="/family/eligibility/apply" class="block text-center bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">
                                    "등급 신청 / 갱신"
                                </a>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// LTCI eligibility application form for new or renewal assessments.
#[component]
pub fn EligibilityApplyPage() -> impl IntoView {
    let (applicant_name, set_applicant_name) = signal(String::new());
    let (reason, set_reason) = signal(String::new());
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"장기요양 등급 신청"</h1>
                <p class="text-sm text-txt-secondary mt-1">"등급 판정을 위한 신청서를 작성하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"신청인 이름"</label>
                    <input
                        type="text"
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        placeholder="이름을 입력하세요"
                        prop:value=move || applicant_name.get()
                        on:input=move |ev| set_applicant_name.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"신청 사유"</label>
                    <textarea
                        rows=3
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        placeholder="신청 사유를 적어주세요"
                        prop:value=move || reason.get()
                        on:input=move |ev| set_reason.set(event_target_value(&ev))
                    ></textarea>
                </div>
                <div class="bg-warning-light rounded-xl p-4">
                    <p class="text-sm text-warning">"신청 후 국민건강보험공단에서 방문 조사가 진행됩니다."</p>
                </div>
                {move || error_msg.get().map(|msg| view! {
                    <p class="text-sm text-danger">{msg}</p>
                })}
                <button
                    class="w-full bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                    prop:disabled=move || submitting.get()
                    on:click=move |_| {
                        let name = applicant_name.get();
                        let notes = reason.get();
                        leptos::task::spawn_local(async move {
                            submitting.set(true);
                            error_msg.set(None);
                            let body = serde_json::json!({
                                "applicant_name": name,
                                "notes": notes,
                            });
                            match crate::api::post::<serde_json::Value, _>("/api/gov/eligibility-cases", &body).await {
                                Ok(resp) if resp.success => {
                                    if let Some(window) = leptos::web_sys::window() {
                                        let _ = window.location().set_href("/family/eligibility");
                                    }
                                }
                                Ok(resp) => error_msg.set(resp.error),
                                Err(e) => error_msg.set(Some(e)),
                            }
                            submitting.set(false);
                        });
                    }
                >
                    {move || if submitting.get() { "처리 중..." } else { "신청서 제출" }}
                </button>
            </div>
        </div>
    }
}
