use leptos::prelude::*;

// =============================================================================
// Help pages — hub, booking, emergency, report
// =============================================================================

/// Presents help options menu for the senior including booking, emergency, and reporting.
#[component]
pub fn HelpSeniorPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"도움이 필요하신가요?"</h1>
                <p class="text-sm text-txt-secondary mt-1">"어르신을 위한 도움 옵션입니다."</p>
            </div>

            <div class="space-y-3">
                <a href="/family/help/book" class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                    <p class="font-medium text-txt-primary">"서비스 예약"</p>
                    <p class="text-sm text-txt-tertiary mt-1">"방문요양, 방문목욕 등 서비스를 예약합니다."</p>
                </a>
                <a href="/family/help/emergency" class="block bg-danger-light rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                    <p class="font-medium text-danger">"긴급 연락처"</p>
                    <p class="text-sm text-txt-tertiary mt-1">"응급 상황 시 즉시 연락할 수 있습니다."</p>
                </a>
                <a href="/family/help/report" class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                    <p class="font-medium text-txt-primary">"우려사항 신고"</p>
                    <p class="text-sm text-txt-tertiary mt-1">"돌봄 관련 우려사항을 신고합니다."</p>
                </a>
            </div>
        </div>
    }
}

/// Service booking form for scheduling visits on behalf of the senior.
#[component]
pub fn HelpBookPage() -> impl IntoView {
    let (service_type, set_service_type) = signal(String::new());
    let (preferred_date, set_preferred_date) = signal(String::new());

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"서비스 예약"</h1>
                <p class="text-sm text-txt-secondary mt-1">"어르신을 위한 서비스를 예약하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"서비스 유형"</label>
                    <select
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        on:change=move |ev| set_service_type.set(event_target_value(&ev))
                    >
                        <option value="">"선택하세요"</option>
                        <option value="home_care">"방문요양"</option>
                        <option value="home_bath">"방문목욕"</option>
                        <option value="home_nursing">"방문간호"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"희망 날짜"</label>
                    <input
                        type="date"
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        prop:value=move || preferred_date.get()
                        on:input=move |ev| set_preferred_date.set(event_target_value(&ev))
                    />
                </div>
                <button class="w-full bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">
                    "예약 요청"
                </button>
            </div>
        </div>
    }
}

/// Displays emergency contact numbers for urgent situations.
#[component]
pub fn HelpEmergencyPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-danger">"긴급 연락처"</h1>
                <p class="text-sm text-txt-secondary mt-1">"응급 상황 시 아래 번호로 연락하세요."</p>
            </div>

            <div class="space-y-3">
                <div class="bg-danger-light rounded-2xl p-5">
                    <p class="font-semibold text-danger">"119 응급전화"</p>
                    <a href="tel:119" class="text-lg font-bold text-danger mt-1 block">"119"</a>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="font-medium text-txt-primary">"담당 요양보호사"</p>
                    <a href="tel:010-1234-5678" class="text-sm text-[var(--portal-accent)] hover:underline mt-1 block">"010-1234-5678"</a>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="font-medium text-txt-primary">"담당 기관"</p>
                    <a href="tel:02-1234-5678" class="text-sm text-[var(--portal-accent)] hover:underline mt-1 block">"02-1234-5678"</a>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="font-medium text-txt-primary">"국민건강보험공단"</p>
                    <a href="tel:1577-1000" class="text-sm text-[var(--portal-accent)] hover:underline mt-1 block">"1577-1000"</a>
                </div>
            </div>
        </div>
    }
}

/// Form for reporting care-related concerns or safety issues.
#[component]
pub fn HelpReportPage() -> impl IntoView {
    let (category, set_category) = signal(String::new());
    let (description, set_description) = signal(String::new());

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"우려사항 신고"</h1>
                <p class="text-sm text-txt-secondary mt-1">"돌봄 관련 문제를 신고해 주세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"신고 유형"</label>
                    <select
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        on:change=move |ev| set_category.set(event_target_value(&ev))
                    >
                        <option value="">"선택하세요"</option>
                        <option value="quality">"서비스 품질"</option>
                        <option value="safety">"안전 문제"</option>
                        <option value="abuse">"학대 의심"</option>
                        <option value="other">"기타"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"상세 설명"</label>
                    <textarea
                        rows=4
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        placeholder="구체적인 내용을 적어주세요."
                        prop:value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                    ></textarea>
                </div>
                <button class="w-full bg-danger text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">
                    "신고 제출"
                </button>
            </div>
        </div>
    }
}
