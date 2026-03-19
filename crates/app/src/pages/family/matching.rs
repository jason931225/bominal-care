use leptos::prelude::*;

// =============================================================================
// Matching pages — search, results, detail
// =============================================================================

/// Search form for caregiver matching with region and service type filters.
#[component]
pub fn MatchingSearchPage() -> impl IntoView {
    let (region, set_region) = signal(String::new());
    let (service, set_service) = signal(String::new());

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"요양보호사 매칭"</h1>
                <p class="text-sm text-txt-secondary mt-1">"조건을 입력하고 매칭을 시작하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"지역"</label>
                    <input
                        type="text"
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        placeholder="예: 서울시 강남구"
                        prop:value=move || region.get()
                        on:input=move |ev| set_region.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"서비스 유형"</label>
                    <select
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        on:change=move |ev| set_service.set(event_target_value(&ev))
                    >
                        <option value="">"선택하세요"</option>
                        <option value="home_care">"방문요양"</option>
                        <option value="home_bath">"방문목욕"</option>
                        <option value="home_nursing">"방문간호"</option>
                        <option value="day_care">"주야간보호"</option>
                    </select>
                </div>
                <a
                    href="/family/matching/results"
                    class="block w-full text-center bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all"
                >
                    "매칭 검색"
                </a>
            </div>
        </div>
    }
}

/// Displays match recommendation results with compatibility scores.
#[component]
pub fn MatchingResultsPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"매칭 결과"</h1>
                <p class="text-sm text-txt-secondary mt-1">"추천 요양보호사 목록입니다."</p>
            </div>
            <div class="skeleton h-4 w-48"></div>
        </div>
    }
}

/// Shows a caregiver profile from a match recommendation with rating and score.
#[component]
pub fn MatchingDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"요양보호사 프로필"</h1>
                <p class="text-sm text-txt-secondary mt-1">"매칭된 요양보호사의 상세 정보입니다."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex items-center gap-4">
                    <div class="w-14 h-14 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center">
                        <span class="text-xl font-bold text-[var(--portal-accent)]">"김"</span>
                    </div>
                    <div>
                        <p class="font-semibold text-txt-primary">"김요양"</p>
                        <p class="text-sm text-txt-tertiary">"경력 8년 · 치매 전문"</p>
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-3">
                    <div class="bg-surface-page rounded-xl p-3">
                        <p class="text-xs text-txt-tertiary">"매칭 점수"</p>
                        <p class="text-lg font-bold text-[var(--portal-accent)]">"92"<span class="text-xs text-txt-disabled">"/100"</span></p>
                    </div>
                    <div class="bg-surface-page rounded-xl p-3">
                        <p class="text-xs text-txt-tertiary">"평점"</p>
                        <p class="text-lg font-bold text-yellow-500">"4.8"<span class="text-xs text-txt-disabled">"/5"</span></p>
                    </div>
                </div>
                <button class="w-full bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">
                    "매칭 요청"
                </button>
            </div>
        </div>
    }
}
