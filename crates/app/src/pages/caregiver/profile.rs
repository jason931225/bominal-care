use leptos::prelude::*;

// =============================================================================
// 14. NotificationsPage — notification list
// =============================================================================

#[component]
pub fn NotificationsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-xl font-bold text-gray-900">"알림"</h1>
                <button class="text-sm text-teal-600 font-medium hover:text-teal-700">"모두 읽음"</button>
            </div>

            <div class="space-y-2">
                <NotificationItem title="스케줄 변경" body="내일 김복순님 방문 시간이 14:00에서 15:00으로 변경되었습니다." time="10분 전" unread=true icon_type="schedule" />
                <NotificationItem title="투약 알림" body="박영자님 오전 투약이 확인되지 않았습니다." time="1시간 전" unread=true icon_type="medication" />
                <NotificationItem title="교육 일정" body="요양보호사 보수교육이 3월 20일로 예정되어 있습니다." time="3시간 전" unread=false icon_type="info" />
                <NotificationItem title="케어플랜 업데이트" body="이순자님의 케어플랜이 갱신되었습니다. 확인해주세요." time="어제" unread=false icon_type="care" />
                <NotificationItem title="급여 명세서" body="2월 급여 명세서가 등록되었습니다." time="3일 전" unread=false icon_type="info" />
            </div>
        </div>
    }
}

#[component]
fn NotificationItem(
    #[prop(into)] title: String,
    #[prop(into)] body: String,
    #[prop(into)] time: String,
    unread: bool,
    #[prop(into)] icon_type: String,
) -> impl IntoView {
    let bg = if unread { "bg-teal-50 border-teal-100" } else { "bg-white border-gray-100" };
    let icon_bg = match icon_type.as_str() {
        "schedule" => "bg-blue-100 text-blue-600",
        "medication" => "bg-red-100 text-red-600",
        "care" => "bg-green-100 text-green-600",
        _ => "bg-gray-100 text-gray-600",
    };
    let icon_path = match icon_type.as_str() {
        "schedule" => "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
        "medication" => "M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z",
        "care" => "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z",
        _ => "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
    };

    view! {
        <div class={format!("rounded-xl p-4 border {bg}")}>
            <div class="flex gap-3">
                <div class={format!("w-9 h-9 rounded-full flex items-center justify-center shrink-0 {icon_bg}")}>
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                    </svg>
                </div>
                <div class="flex-1 min-w-0">
                    <div class="flex items-center justify-between mb-0.5">
                        <p class="text-sm font-semibold text-gray-900">{title}</p>
                        <span class="text-xs text-gray-400 shrink-0">{time}</span>
                    </div>
                    <p class="text-sm text-gray-600">{body}</p>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// 15. ProfilePage — caregiver profile with credentials
// =============================================================================

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <h1 class="text-xl font-bold text-gray-900">"내 프로필"</h1>

            // Profile card
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                <div class="w-20 h-20 bg-teal-100 rounded-full flex items-center justify-center mx-auto mb-3">
                    <span class="text-3xl font-bold text-teal-700">"홍"</span>
                </div>
                <h2 class="text-lg font-bold text-gray-900">"홍길동"</h2>
                <p class="text-sm text-gray-500">"요양보호사 · 경력 5년"</p>
                <span class="inline-block mt-2 px-3 py-1 bg-green-100 text-green-700 text-xs font-medium rounded-full">"활동 중"</span>
            </div>

            // Contact info
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"연락처 정보"</h3>
                <dl class="space-y-2 text-sm">
                    <div class="flex justify-between"><dt class="text-gray-500">"전화번호"</dt><dd class="font-medium text-gray-900">"010-5555-1234"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"이메일"</dt><dd class="font-medium text-gray-900">"hong@example.com"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"주소"</dt><dd class="font-medium text-gray-900">"서울시 마포구 합정동"</dd></div>
                </dl>
            </div>

            // Credentials
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"자격 및 인증"</h3>
                <div class="space-y-3">
                    <CredentialItem name="요양보호사 자격증" issuer="보건복지부" expires="2028.12.31" valid=true />
                    <CredentialItem name="치매전문교육 이수" issuer="중앙치매센터" expires="2027.06.30" valid=true />
                    <CredentialItem name="응급처치 자격증" issuer="대한적십자사" expires="2026.09.15" valid=true />
                </div>
            </div>

            // Links
            <div class="grid grid-cols-2 gap-3">
                <a href="/caregiver/profile/availability" class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <p class="text-sm font-medium text-gray-900">"근무 가능 시간"</p>
                </a>
                <a href="/caregiver/settings" class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <p class="text-sm font-medium text-gray-900">"설정"</p>
                </a>
            </div>
        </div>
    }
}

#[component]
fn CredentialItem(
    #[prop(into)] name: String,
    #[prop(into)] issuer: String,
    #[prop(into)] expires: String,
    valid: bool,
) -> impl IntoView {
    let badge = if valid {
        ("bg-green-100 text-green-700", "유효")
    } else {
        ("bg-red-100 text-red-700", "만료")
    };

    view! {
        <div class="flex items-center justify-between py-2 border-b border-gray-50 last:border-0">
            <div>
                <p class="text-sm font-medium text-gray-900">{name}</p>
                <p class="text-xs text-gray-500">{issuer}" · 만료: "{expires}</p>
            </div>
            <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full {}", badge.0)}>{badge.1}</span>
        </div>
    }
}

// =============================================================================
// 16. ProfileAvailabilityPage — edit availability slots
// =============================================================================

#[component]
pub fn ProfileAvailabilityPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/profile" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"근무 가능 시간"</h1>
            </div>

            <p class="text-sm text-gray-600">"근무 가능한 요일과 시간을 설정해주세요."</p>

            <div class="space-y-3">
                <AvailabilityDayRow day="월요일" start="09:00" end="18:00" enabled=true />
                <AvailabilityDayRow day="화요일" start="09:00" end="18:00" enabled=true />
                <AvailabilityDayRow day="수요일" start="09:00" end="18:00" enabled=true />
                <AvailabilityDayRow day="목요일" start="09:00" end="18:00" enabled=true />
                <AvailabilityDayRow day="금요일" start="09:00" end="18:00" enabled=true />
                <AvailabilityDayRow day="토요일" start="09:00" end="13:00" enabled=false />
                <AvailabilityDayRow day="일요일" start="" end="" enabled=false />
            </div>

            <button class="w-full py-3 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700">"저장"</button>
        </div>
    }
}

#[component]
fn AvailabilityDayRow(
    #[prop(into)] day: String,
    #[prop(into)] start: String,
    #[prop(into)] end: String,
    enabled: bool,
) -> impl IntoView {
    let is_on = RwSignal::new(enabled);

    view! {
        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <button
                        class="w-10 h-6 rounded-full transition-colors relative"
                        class=("bg-teal-600", move || is_on.get())
                        class=("bg-gray-300", move || !is_on.get())
                        on:click=move |_| is_on.update(|v| *v = !*v)
                    >
                        <span
                            class="absolute top-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform"
                            class=("left-[1.125rem]", move || is_on.get())
                            class=("left-0.5", move || !is_on.get())
                        />
                    </button>
                    <span class="font-medium text-gray-900">{day}</span>
                </div>
                <Show when=move || is_on.get()>
                    <div class="flex items-center gap-2 text-sm text-gray-600">
                        <span>{start.clone()}</span>
                        <span>"~"</span>
                        <span>{end.clone()}</span>
                    </div>
                </Show>
            </div>
        </div>
    }
}

// =============================================================================
// 28. SettingsPage — caregiver settings
// =============================================================================

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <h1 class="text-xl font-bold text-gray-900">"설정"</h1>

            // Notification settings
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-4">"알림 설정"</h3>
                <div class="space-y-3">
                    <SettingsToggle label="스케줄 알림" desc="방문 시작 30분 전 알림" default_on=true />
                    <SettingsToggle label="투약 알림" desc="고객 투약 시간 알림" default_on=true />
                    <SettingsToggle label="긴급 알림" desc="긴급 상황 알림" default_on=true />
                    <SettingsToggle label="교육/공지 알림" desc="교육 일정 및 공지사항" default_on=false />
                </div>
            </div>

            // Display settings
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-4">"화면 설정"</h3>
                <div class="space-y-3">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm font-medium text-gray-900">"글자 크기"</p>
                            <p class="text-xs text-gray-500">"앱 전체 글자 크기"</p>
                        </div>
                        <select class="px-3 py-1.5 border border-gray-300 rounded-lg text-sm">
                            <option>"보통"</option>
                            <option>"크게"</option>
                            <option>"매우 크게"</option>
                        </select>
                    </div>
                    <SettingsToggle label="다크 모드" desc="어두운 화면 모드" default_on=false />
                </div>
            </div>

            // Account
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-4">"계정"</h3>
                <div class="space-y-2">
                    <a href="/caregiver/profile" class="flex items-center justify-between py-2">
                        <span class="text-sm text-gray-700">"프로필 수정"</span>
                        <svg class="w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                        </svg>
                    </a>
                    <a href="/caregiver/profile/availability" class="flex items-center justify-between py-2">
                        <span class="text-sm text-gray-700">"근무 가능 시간 변경"</span>
                        <svg class="w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                        </svg>
                    </a>
                    <div class="pt-2 border-t border-gray-100">
                        <button
                            class="flex items-center justify-between py-2 w-full text-left"
                            on:click=move |_| {
                                leptos::task::spawn_local(async move {
                                    let _ = crate::api::post_no_body("/api/auth/logout").await;
                                    if let Some(window) = leptos::web_sys::window() {
                                        let _ = window.location().set_href("/auth/signin");
                                    }
                                });
                            }
                        >
                            <span class="text-sm text-red-600">"로그아웃"</span>
                        </button>
                    </div>
                </div>
            </div>

            // App info
            <div class="text-center text-xs text-gray-400 pt-4">
                <p>"요양보호사 포털 v1.0.0"</p>
            </div>
        </div>
    }
}

#[component]
fn SettingsToggle(
    #[prop(into)] label: String,
    #[prop(into)] desc: String,
    default_on: bool,
) -> impl IntoView {
    let is_on = RwSignal::new(default_on);

    view! {
        <div class="flex items-center justify-between">
            <div>
                <p class="text-sm font-medium text-gray-900">{label}</p>
                <p class="text-xs text-gray-500">{desc}</p>
            </div>
            <button
                class="w-10 h-6 rounded-full transition-colors relative"
                class=("bg-teal-600", move || is_on.get())
                class=("bg-gray-300", move || !is_on.get())
                on:click=move |_| is_on.update(|v| *v = !*v)
            >
                <span
                    class="absolute top-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform"
                    class=("left-[1.125rem]", move || is_on.get())
                    class=("left-0.5", move || !is_on.get())
                />
            </button>
        </div>
    }
}
