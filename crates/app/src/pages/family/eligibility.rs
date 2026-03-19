use leptos::prelude::*;

// =============================================================================
// Eligibility pages — LTCI grade info and application
// =============================================================================

/// Shows LTCI eligibility grade information with renewal options.
#[component]
pub fn EligibilityPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"장기요양 등급 안내"</h1>
                <p class="text-sm text-txt-secondary mt-1">"장기요양보험 등급 판정 정보입니다."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <p class="text-sm text-txt-tertiary">"현재 등급"</p>
                    <p class="text-lg font-bold text-[var(--portal-accent)]">"3등급"</p>
                </div>
                <div>
                    <p class="text-sm text-txt-tertiary">"판정일"</p>
                    <p class="text-sm text-txt-secondary">"2025-12-15"</p>
                </div>
                <div>
                    <p class="text-sm text-txt-tertiary">"유효기간"</p>
                    <p class="text-sm text-txt-secondary">"2025-12-15 ~ 2027-12-14"</p>
                </div>
                <div class="bg-[var(--portal-accent-light)] rounded-xl p-4">
                    <p class="text-sm text-[var(--portal-accent)]">"등급 갱신이 필요하시면 아래 버튼을 눌러 신청하세요."</p>
                </div>
                <a href="/family/eligibility/apply" class="block text-center bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">
                    "등급 신청 / 갱신"
                </a>
            </div>
        </div>
    }
}

/// LTCI eligibility application form for new or renewal assessments.
#[component]
pub fn EligibilityApplyPage() -> impl IntoView {
    let (applicant_name, set_applicant_name) = signal(String::new());
    let (reason, set_reason) = signal(String::new());

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
                <button class="w-full bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">
                    "신청서 제출"
                </button>
            </div>
        </div>
    }
}
