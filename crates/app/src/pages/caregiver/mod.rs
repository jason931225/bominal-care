use leptos::prelude::*;

// =============================================================================
// 1. DashboardPage — today's stats, next visit, alerts, weekly summary
// =============================================================================

/// Generic placeholder page for routes not yet implemented.
#[component]
pub fn StubPage() -> impl IntoView {
    view! {
        <div class="p-6">
            <p class="text-sm text-gray-500">"준비 중입니다."</p>
        </div>
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"요양보호사 대시보드"</h1>
                <p class="text-sm text-gray-600 mt-1">"오늘의 스케줄과 업무를 확인하세요."</p>
            </div>

            // Today's stats
            <div class="grid grid-cols-2 gap-4">
                <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"오늘 방문"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"4"<span class="text-sm font-normal text-gray-500">" 건"</span></p>
                </div>
                <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"근무 시간"</p>
                    <p class="text-2xl font-bold text-teal-600 mt-1">"6.5"<span class="text-sm font-normal text-gray-500">" 시간"</span></p>
                </div>
            </div>

            // Next visit card
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="flex items-center justify-between mb-3">
                    <h2 class="font-semibold text-gray-900">"다음 방문"</h2>
                    <span class="text-xs font-medium text-teal-700 bg-teal-50 px-2 py-1 rounded-full">"30분 후"</span>
                </div>
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center">
                        <svg class="w-5 h-5 text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                        </svg>
                    </div>
                    <div>
                        <p class="font-medium text-gray-900">"김복순님"</p>
                        <p class="text-sm text-gray-500">"14:00 - 16:00 · 방문요양"</p>
                    </div>
                </div>
                <div class="mt-3 flex gap-2">
                    <a href="/caregiver/schedule/visit-1" class="flex-1 text-center py-2 bg-teal-600 text-white text-sm rounded-lg hover:bg-teal-700">"상세보기"</a>
                    <a href="/caregiver/check-in/visit-1" class="flex-1 text-center py-2 border border-teal-600 text-teal-600 text-sm rounded-lg hover:bg-teal-50">"체크인"</a>
                </div>
            </div>

            // Alerts
            <div class="bg-orange-50 border border-orange-200 rounded-xl p-4">
                <div class="flex items-center gap-2 mb-2">
                    <svg class="w-5 h-5 text-orange-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" />
                    </svg>
                    <h3 class="font-semibold text-orange-800">"알림"</h3>
                </div>
                <ul class="space-y-1 text-sm text-orange-700">
                    <li>"· 박영자님 투약 시간 변경 (12:00 → 13:00)"</li>
                    <li>"· 내일 교육 일정 확인 필요"</li>
                </ul>
            </div>

            // Weekly summary
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"이번 주 요약"</h2>
                <div class="grid grid-cols-3 gap-3 text-center">
                    <div>
                        <p class="text-lg font-bold text-gray-900">"18"</p>
                        <p class="text-xs text-gray-500">"총 방문"</p>
                    </div>
                    <div>
                        <p class="text-lg font-bold text-teal-600">"32"</p>
                        <p class="text-xs text-gray-500">"근무 시간"</p>
                    </div>
                    <div>
                        <p class="text-lg font-bold text-blue-600">"7"</p>
                        <p class="text-xs text-gray-500">"고객 수"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// 2. ScheduleListPage — week date strip + daily visit list
// =============================================================================

#[component]
pub fn ScheduleListPage() -> impl IntoView {
    let selected_day = RwSignal::new(2_usize); // index into week

    let days: Vec<(&str, &str)> = vec![
        ("월", "14"), ("화", "15"), ("수", "16"),
        ("목", "17"), ("금", "18"), ("토", "19"), ("일", "20"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <h1 class="text-xl font-bold text-gray-900">"스케줄"</h1>

            // Week date strip
            <div class="flex gap-2 overflow-x-auto pb-2">
                {days.into_iter().enumerate().map(|(i, (label, date))| {
                    let is_selected = move || selected_day.get() == i;
                    let cls = move || {
                        if is_selected() {
                            "flex flex-col items-center px-3 py-2 rounded-xl bg-teal-600 text-white min-w-[3rem]"
                        } else {
                            "flex flex-col items-center px-3 py-2 rounded-xl bg-white border border-gray-200 text-gray-700 min-w-[3rem]"
                        }
                    };
                    view! {
                        <button class=cls on:click=move |_| selected_day.set(i)>
                            <span class="text-xs font-medium">{label}</span>
                            <span class="text-lg font-bold">{date}</span>
                        </button>
                    }
                }).collect_view()}
            </div>

            // Visit list for selected day
            <div class="space-y-3">
                <ScheduleVisitCard time="09:00 - 11:00" client="이순자님" service="방문요양" status="완료" />
                <ScheduleVisitCard time="11:30 - 12:30" client="박영자님" service="방문목욕" status="완료" />
                <ScheduleVisitCard time="14:00 - 16:00" client="김복순님" service="방문요양" status="예정" />
                <ScheduleVisitCard time="16:30 - 18:00" client="최영희님" service="방문간호" status="예정" />
            </div>
        </div>
    }
}

#[component]
fn ScheduleVisitCard(
    #[prop(into)] time: String,
    #[prop(into)] client: String,
    #[prop(into)] service: String,
    #[prop(into)] status: String,
) -> impl IntoView {
    let badge_cls = match status.as_str() {
        "완료" => "text-xs font-medium px-2 py-0.5 rounded-full bg-green-100 text-green-700",
        "진행중" => "text-xs font-medium px-2 py-0.5 rounded-full bg-blue-100 text-blue-700",
        _ => "text-xs font-medium px-2 py-0.5 rounded-full bg-gray-100 text-gray-600",
    };

    view! {
        <a href="/caregiver/schedule/visit-1" class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium text-teal-700">{time}</span>
                <span class=badge_cls>{status}</span>
            </div>
            <p class="font-medium text-gray-900">{client}</p>
            <p class="text-sm text-gray-500">{service}</p>
        </a>
    }
}

// =============================================================================
// 3. ScheduleDetailPage — visit detail with client info, services, checklist
// =============================================================================

#[component]
pub fn ScheduleDetailPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/schedule" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"방문 상세"</h1>
            </div>

            // Client info
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="flex items-center gap-3 mb-3">
                    <div class="w-12 h-12 bg-teal-100 rounded-full flex items-center justify-center">
                        <span class="text-lg font-bold text-teal-700">"김"</span>
                    </div>
                    <div>
                        <p class="font-semibold text-gray-900">"김복순님"</p>
                        <p class="text-sm text-gray-500">"82세 · 장기요양 3등급"</p>
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-3 text-sm">
                    <div>
                        <p class="text-gray-500">"시간"</p>
                        <p class="font-medium text-gray-900">"14:00 - 16:00"</p>
                    </div>
                    <div>
                        <p class="text-gray-500">"서비스 유형"</p>
                        <p class="font-medium text-gray-900">"방문요양"</p>
                    </div>
                    <div>
                        <p class="text-gray-500">"주소"</p>
                        <p class="font-medium text-gray-900">"서울시 강남구 역삼동"</p>
                    </div>
                    <div>
                        <p class="text-gray-500">"연락처"</p>
                        <p class="font-medium text-gray-900">"010-1234-5678"</p>
                    </div>
                </div>
            </div>

            // Services
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"제공 서비스"</h2>
                <div class="space-y-2">
                    <ServiceTag label="신체활동지원" />
                    <ServiceTag label="일상생활지원" />
                    <ServiceTag label="정서지원" />
                </div>
            </div>

            // Care checklist
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"케어 체크리스트"</h2>
                <CareChecklist />
            </div>

            // Action buttons
            <div class="flex gap-3">
                <a href="/caregiver/check-in/visit-1" class="flex-1 text-center py-3 bg-teal-600 text-white font-medium rounded-xl hover:bg-teal-700">"체크인"</a>
                <a href="/caregiver/notes/new" class="flex-1 text-center py-3 border border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50">"기록 작성"</a>
            </div>
        </div>
    }
}

#[component]
fn ServiceTag(#[prop(into)] label: String) -> impl IntoView {
    view! {
        <span class="inline-block px-3 py-1 bg-teal-50 text-teal-700 text-sm rounded-full">{label}</span>
    }
}

#[component]
fn CareChecklist() -> impl IntoView {
    let items = vec![
        ("혈압 측정", false),
        ("투약 확인", false),
        ("식사 보조", false),
        ("개인위생 관리", false),
        ("운동 보조", false),
        ("정서적 대화", false),
    ];

    view! {
        <ul class="space-y-2">
            {items.into_iter().map(|(label, checked)| {
                let is_checked = RwSignal::new(checked);
                view! {
                    <li class="flex items-center gap-3">
                        <button
                            class="w-5 h-5 rounded border-2 flex items-center justify-center transition-colors"
                            class=("border-teal-600 bg-teal-600", move || is_checked.get())
                            class=("border-gray-300", move || !is_checked.get())
                            on:click=move |_| is_checked.update(|v| *v = !*v)
                        >
                            <Show when=move || is_checked.get()>
                                <svg class="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                </svg>
                            </Show>
                        </button>
                        <span class="text-sm text-gray-700">{label}</span>
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}

// =============================================================================
// 4. CheckInPage — location verification + time display
// =============================================================================

#[component]
pub fn CheckInPage() -> impl IntoView {
    let location_status = RwSignal::new("확인 중...");
    let is_verified = RwSignal::new(false);

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <div class="flex items-center gap-3">
                <a href="/caregiver/schedule" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"체크인"</h1>
            </div>

            // Time display
            <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100 text-center">
                <p class="text-sm text-gray-500">"현재 시각"</p>
                <p class="text-4xl font-bold text-gray-900 mt-2">"14:00"</p>
                <p class="text-sm text-gray-500 mt-1">"2026년 3월 17일 (화)"</p>
            </div>

            // Visit info
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-2">"방문 정보"</h2>
                <div class="space-y-2 text-sm">
                    <div class="flex justify-between">
                        <span class="text-gray-500">"고객"</span>
                        <span class="font-medium text-gray-900">"김복순님"</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-500">"예정 시간"</span>
                        <span class="font-medium text-gray-900">"14:00 - 16:00"</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-500">"서비스"</span>
                        <span class="font-medium text-gray-900">"방문요양"</span>
                    </div>
                </div>
            </div>

            // Location verification
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"위치 확인"</h2>
                <div class="flex items-center gap-3 mb-4">
                    <div
                        class="w-10 h-10 rounded-full flex items-center justify-center"
                        class=("bg-green-100", move || is_verified.get())
                        class=("bg-yellow-100", move || !is_verified.get())
                    >
                        <svg class="w-5 h-5" class=("text-green-600", move || is_verified.get()) class=("text-yellow-600", move || !is_verified.get()) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                    </div>
                    <div>
                        <p class="text-sm font-medium text-gray-900">{move || location_status.get()}</p>
                        <p class="text-xs text-gray-500">"서울시 강남구 역삼동 123-45"</p>
                    </div>
                </div>
                <button
                    class="w-full py-2 text-sm bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200"
                    on:click=move |_| {
                        location_status.set("위치 확인 완료");
                        is_verified.set(true);
                    }
                >"위치 다시 확인"</button>
            </div>

            // Check-in button
            <button
                class="w-full py-4 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700 disabled:opacity-50"
                disabled=move || !is_verified.get()
            >"체크인 완료"</button>
        </div>
    }
}

// =============================================================================
// 5. CheckOutPage — checkout form with location + notes
// =============================================================================

#[component]
pub fn CheckOutPage() -> impl IntoView {
    let notes = RwSignal::new(String::new());
    let condition = RwSignal::new("양호".to_string());

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <div class="flex items-center gap-3">
                <a href="/caregiver/schedule" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"체크아웃"</h1>
            </div>

            // Time summary
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="grid grid-cols-2 gap-4 text-center">
                    <div>
                        <p class="text-sm text-gray-500">"체크인"</p>
                        <p class="text-xl font-bold text-gray-900">"14:02"</p>
                    </div>
                    <div>
                        <p class="text-sm text-gray-500">"현재 시각"</p>
                        <p class="text-xl font-bold text-teal-600">"16:05"</p>
                    </div>
                </div>
                <div class="mt-3 pt-3 border-t border-gray-100 text-center">
                    <p class="text-sm text-gray-500">"총 근무 시간"</p>
                    <p class="text-lg font-bold text-gray-900">"2시간 3분"</p>
                </div>
            </div>

            // Client condition
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"고객 상태"</h2>
                <div class="flex gap-2">
                    {["양호", "보통", "주의"].into_iter().map(|opt| {
                        let opt_str = opt.to_string();
                        let is_selected = {
                            let opt_str = opt_str.clone();
                            move || condition.get() == opt_str
                        };
                        let is_not_selected = {
                            let opt_str2 = opt_str.clone();
                            move || condition.get() != opt_str2
                        };
                        let on_click = {
                            let opt_str = opt_str.clone();
                            move |_| condition.set(opt_str.clone())
                        };
                        view! {
                            <button
                                class="flex-1 py-2 text-sm rounded-lg border transition-colors"
                                class=("bg-teal-600 text-white border-teal-600", is_selected)
                                class=("bg-white text-gray-700 border-gray-300", is_not_selected)
                                on:click=on_click
                            >{opt}</button>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Location
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
                        <svg class="w-4 h-4 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                        </svg>
                    </div>
                    <div>
                        <p class="text-sm font-medium text-gray-900">"위치 확인 완료"</p>
                        <p class="text-xs text-gray-500">"서울시 강남구 역삼동 123-45"</p>
                    </div>
                </div>
            </div>

            // Notes
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"방문 메모"</h2>
                <textarea
                    class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 resize-none"
                    rows="4"
                    placeholder="특이사항이나 메모를 입력하세요..."
                    prop:value=move || notes.get()
                    on:input=move |ev| notes.set(event_target_value(&ev))
                />
            </div>

            <button class="w-full py-4 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700">"체크아웃 완료"</button>
        </div>
    }
}

// =============================================================================
// 6. ClientsListPage — client list with care level badges
// =============================================================================

#[component]
pub fn ClientsListPage() -> impl IntoView {
    let search = RwSignal::new(String::new());

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <h1 class="text-xl font-bold text-gray-900">"담당 고객"</h1>

            // Search
            <div class="relative">
                <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-4.35-4.35M11 19a8 8 0 100-16 8 8 0 000 16z" />
                </svg>
                <input
                    type="search"
                    class="w-full pl-10 pr-4 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-teal-500"
                    placeholder="고객 검색..."
                    prop:value=move || search.get()
                    on:input=move |ev| search.set(event_target_value(&ev))
                />
            </div>

            // Client list
            <div class="space-y-3">
                <ClientCard name="김복순" age=82 care_level="3등급" services="방문요양" next_visit="오늘 14:00" />
                <ClientCard name="이순자" age=78 care_level="2등급" services="방문요양, 방문목욕" next_visit="오늘 09:00" />
                <ClientCard name="박영자" age=85 care_level="4등급" services="방문목욕" next_visit="내일 11:00" />
                <ClientCard name="최영희" age=76 care_level="3등급" services="방문간호" next_visit="오늘 16:30" />
                <ClientCard name="정순옥" age=88 care_level="1등급" services="방문요양, 방문간호" next_visit="수요일 10:00" />
            </div>
        </div>
    }
}

#[component]
fn ClientCard(
    #[prop(into)] name: String,
    age: u32,
    #[prop(into)] care_level: String,
    #[prop(into)] services: String,
    #[prop(into)] next_visit: String,
) -> impl IntoView {
    let initial = name.chars().next().unwrap_or('?').to_string();

    let level_cls = match care_level.as_str() {
        "1등급" => "bg-red-100 text-red-700",
        "2등급" => "bg-orange-100 text-orange-700",
        "3등급" => "bg-yellow-100 text-yellow-700",
        "4등급" => "bg-green-100 text-green-700",
        _ => "bg-gray-100 text-gray-700",
    };

    view! {
        <a href="/caregiver/clients/client-1" class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
            <div class="flex items-center gap-3">
                <div class="w-11 h-11 bg-teal-100 rounded-full flex items-center justify-center shrink-0">
                    <span class="text-lg font-bold text-teal-700">{initial}</span>
                </div>
                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                        <p class="font-semibold text-gray-900">{name}"님"</p>
                        <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full {level_cls}")}>{care_level}</span>
                    </div>
                    <p class="text-sm text-gray-500">{age}"세 · "{services}</p>
                    <p class="text-xs text-teal-600 mt-0.5">"다음 방문: "{next_visit}</p>
                </div>
                <svg class="w-5 h-5 text-gray-400 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                </svg>
            </div>
        </a>
    }
}

// =============================================================================
// 7. ClientDetailPage — client profile with care plan link
// =============================================================================

#[component]
pub fn ClientDetailPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/clients" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"고객 상세"</h1>
            </div>

            // Profile
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                <div class="w-16 h-16 bg-teal-100 rounded-full flex items-center justify-center mx-auto mb-3">
                    <span class="text-2xl font-bold text-teal-700">"김"</span>
                </div>
                <h2 class="text-lg font-bold text-gray-900">"김복순님"</h2>
                <p class="text-sm text-gray-500">"82세 · 여성"</p>
                <span class="inline-block mt-2 px-3 py-1 bg-yellow-100 text-yellow-700 text-xs font-medium rounded-full">"장기요양 3등급"</span>
            </div>

            // Details
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"기본 정보"</h3>
                <dl class="space-y-2 text-sm">
                    <div class="flex justify-between"><dt class="text-gray-500">"주소"</dt><dd class="font-medium text-gray-900">"서울시 강남구 역삼동"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"연락처"</dt><dd class="font-medium text-gray-900">"010-1234-5678"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"보호자"</dt><dd class="font-medium text-gray-900">"김철수 (아들)"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"보호자 연락처"</dt><dd class="font-medium text-gray-900">"010-9876-5432"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"특이사항"</dt><dd class="font-medium text-gray-900">"치매 초기, 당뇨"</dd></div>
                </dl>
            </div>

            // Quick links
            <div class="grid grid-cols-2 gap-3">
                <a href="/caregiver/clients/client-1/care-plan" class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <svg class="w-6 h-6 text-teal-600 mx-auto mb-1" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <p class="text-sm font-medium text-gray-900">"케어플랜"</p>
                </a>
                <a href="/caregiver/clients/client-1/medications" class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <svg class="w-6 h-6 text-blue-600 mx-auto mb-1" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
                    </svg>
                    <p class="text-sm font-medium text-gray-900">"투약 정보"</p>
                </a>
            </div>
        </div>
    }
}

// =============================================================================
// 8. ClientCarePlanPage — client's care plan detail
// =============================================================================

#[component]
pub fn ClientCarePlanPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/clients/client-1" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"케어플랜"</h1>
            </div>

            // Plan header
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="flex items-center justify-between mb-2">
                    <h2 class="font-semibold text-gray-900">"김복순님 케어플랜"</h2>
                    <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-green-100 text-green-700">"활성"</span>
                </div>
                <p class="text-sm text-gray-500">"2026.01.15 ~ 2026.07.14"</p>
            </div>

            // Goals
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"케어 목표"</h3>
                <ul class="space-y-3">
                    <CarePlanGoal title="일상생활 자립 지원" desc="식사, 세면, 착탈의 보조를 통한 자립 유지" progress=70 />
                    <CarePlanGoal title="인지기능 유지" desc="인지 자극 활동 및 대화를 통한 기능 유지" progress=55 />
                    <CarePlanGoal title="안전한 환경 관리" desc="낙상 예방 및 가정 내 안전 확인" progress=85 />
                </ul>
            </div>

            // Schedule
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"서비스 일정"</h3>
                <div class="space-y-2 text-sm">
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"월·수·금"</span>
                        <span class="text-gray-500">"14:00 - 16:00 방문요양"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"화·목"</span>
                        <span class="text-gray-500">"10:00 - 11:00 방문간호"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CarePlanGoal(
    #[prop(into)] title: String,
    #[prop(into)] desc: String,
    progress: u32,
) -> impl IntoView {
    let width = format!("width: {}%", progress);
    let bar_color = if progress >= 70 { "bg-teal-500" } else { "bg-yellow-500" };

    view! {
        <li>
            <div class="flex items-center justify-between mb-1">
                <p class="text-sm font-medium text-gray-900">{title}</p>
                <span class="text-xs text-gray-500">{progress}"%"</span>
            </div>
            <p class="text-xs text-gray-500 mb-2">{desc}</p>
            <div class="w-full bg-gray-200 rounded-full h-1.5">
                <div class={format!("{bar_color} h-1.5 rounded-full")} style=width />
            </div>
        </li>
    }
}

// =============================================================================
// 9. ClientMedicationsPage — client's medications list
// =============================================================================

#[component]
pub fn ClientMedicationsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/clients/client-1" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"투약 정보"</h1>
                <span class="text-sm text-gray-500">"김복순님"</span>
            </div>

            <div class="space-y-3">
                <MedicationCard name="메트포르민 500mg" schedule="하루 2회 (아침, 저녁 식후)" purpose="당뇨" status="복용중" />
                <MedicationCard name="도네페질 5mg" schedule="하루 1회 (저녁 식후)" purpose="치매" status="복용중" />
                <MedicationCard name="아스피린 100mg" schedule="하루 1회 (아침 식후)" purpose="혈액순환" status="복용중" />
                <MedicationCard name="칼슘+비타민D" schedule="하루 1회 (점심 식후)" purpose="골다공증" status="복용중" />
            </div>

            // Notes
            <div class="bg-orange-50 border border-orange-200 rounded-xl p-4">
                <p class="text-sm font-medium text-orange-800 mb-1">"투약 주의사항"</p>
                <p class="text-sm text-orange-700">"메트포르민과 도네페질 복용 간격을 최소 2시간 유지해주세요."</p>
            </div>
        </div>
    }
}

#[component]
fn MedicationCard(
    #[prop(into)] name: String,
    #[prop(into)] schedule: String,
    #[prop(into)] purpose: String,
    #[prop(into)] status: String,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
            <div class="flex items-center justify-between mb-2">
                <h3 class="font-medium text-gray-900">{name}</h3>
                <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-green-100 text-green-700">{status}</span>
            </div>
            <p class="text-sm text-gray-500">{schedule}</p>
            <p class="text-xs text-gray-400 mt-1">"용도: "{purpose}</p>
        </div>
    }
}

// =============================================================================
// 10. MedicationsPage — all assigned client medications overview
// =============================================================================

#[component]
pub fn MedicationsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <h1 class="text-xl font-bold text-gray-900">"투약 관리"</h1>
            <p class="text-sm text-gray-600">"담당 고객의 투약 현황을 확인하세요."</p>

            // Summary
            <div class="grid grid-cols-3 gap-3">
                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                    <p class="text-lg font-bold text-green-600">"12"</p>
                    <p class="text-xs text-gray-500">"복용 완료"</p>
                </div>
                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                    <p class="text-lg font-bold text-yellow-600">"3"</p>
                    <p class="text-xs text-gray-500">"예정"</p>
                </div>
                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                    <p class="text-lg font-bold text-red-600">"1"</p>
                    <p class="text-xs text-gray-500">"미복용"</p>
                </div>
            </div>

            // Per-client medication list
            <div class="space-y-4">
                <ClientMedSummary client="김복순님" count=4 next_time="12:00 점심 식후" alert=false />
                <ClientMedSummary client="이순자님" count=3 next_time="14:00 오후" alert=false />
                <ClientMedSummary client="박영자님" count=5 next_time="아침 미복용" alert=true />
                <ClientMedSummary client="최영희님" count=2 next_time="18:00 저녁 식후" alert=false />
            </div>
        </div>
    }
}

#[component]
fn ClientMedSummary(
    #[prop(into)] client: String,
    count: u32,
    #[prop(into)] next_time: String,
    alert: bool,
) -> impl IntoView {
    let border = if alert { "border-red-200 bg-red-50" } else { "border-gray-100 bg-white" };

    view! {
        <a href="/caregiver/clients/client-1/medications" class={format!("block rounded-xl p-4 shadow-sm border {border} hover:shadow-md transition-shadow")}>
            <div class="flex items-center justify-between">
                <div>
                    <p class="font-medium text-gray-900">{client}</p>
                    <p class="text-sm text-gray-500">{count}"개 약물"</p>
                </div>
                <div class="text-right">
                    <p class="text-sm" class=("text-red-600 font-medium", alert) class=("text-gray-500", !alert)>{next_time}</p>
                    {if alert {
                        Some(view! { <span class="text-xs text-red-600">"확인 필요"</span> })
                    } else {
                        None
                    }}
                </div>
            </div>
        </a>
    }
}

// =============================================================================
// 11. NotesListPage — daily observation notes list
// =============================================================================

#[component]
pub fn NotesListPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-xl font-bold text-gray-900">"관찰 기록"</h1>
                <a href="/caregiver/notes/new" class="px-4 py-2 bg-teal-600 text-white text-sm font-medium rounded-lg hover:bg-teal-700">"새 기록"</a>
            </div>

            <div class="space-y-3">
                <NoteCard date="2026.03.17" client="김복순님" preview="오늘 식사를 잘 하셨고, 오후에 산책을 30분 하셨습니다." category="일상 관찰" />
                <NoteCard date="2026.03.17" client="이순자님" preview="혈압이 약간 높아 안정을 취하도록 하였습니다." category="건강 상태" />
                <NoteCard date="2026.03.16" client="김복순님" preview="인지 활동으로 퍼즐 맞추기를 하셨습니다. 집중력이 양호합니다." category="인지 활동" />
                <NoteCard date="2026.03.16" client="박영자님" preview="목욕 후 피부 상태 확인. 특이사항 없음." category="신체 상태" />
                <NoteCard date="2026.03.15" client="최영희님" preview="가족 면회 후 기분이 좋아지셨습니다." category="정서 상태" />
            </div>
        </div>
    }
}

#[component]
fn NoteCard(
    #[prop(into)] date: String,
    #[prop(into)] client: String,
    #[prop(into)] preview: String,
    #[prop(into)] category: String,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
            <div class="flex items-center justify-between mb-2">
                <span class="text-xs text-gray-500">{date}</span>
                <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-blue-50 text-blue-700">{category}</span>
            </div>
            <p class="text-sm font-medium text-gray-900 mb-1">{client}</p>
            <p class="text-sm text-gray-600 line-clamp-2">{preview}</p>
        </div>
    }
}

// =============================================================================
// 12. NoteNewPage — create new observation form
// =============================================================================

#[component]
pub fn NoteNewPage() -> impl IntoView {
    let client = RwSignal::new(String::new());
    let category = RwSignal::new("일상 관찰".to_string());
    let content = RwSignal::new(String::new());

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/notes" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"관찰 기록 작성"</h1>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                // Client select
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"고객 선택"</label>
                    <select
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500"
                        on:change=move |ev| client.set(event_target_value(&ev))
                    >
                        <option value="">"고객을 선택하세요"</option>
                        <option value="kim">"김복순님"</option>
                        <option value="lee">"이순자님"</option>
                        <option value="park">"박영자님"</option>
                        <option value="choi">"최영희님"</option>
                    </select>
                </div>

                // Category
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"분류"</label>
                    <div class="flex flex-wrap gap-2">
                        {["일상 관찰", "건강 상태", "신체 상태", "인지 활동", "정서 상태"].into_iter().map(|cat| {
                            let cat_str = cat.to_string();
                            let is_active = {
                                let cat_str = cat_str.clone();
                                move || category.get() == cat_str
                            };
                            let is_not_active = {
                                let cat_str2 = cat_str.clone();
                                move || category.get() != cat_str2
                            };
                            let on_click = {
                                let cat_str = cat_str.clone();
                                move |_| category.set(cat_str.clone())
                            };
                            view! {
                                <button
                                    type="button"
                                    class="px-3 py-1.5 text-xs rounded-full border transition-colors"
                                    class=("bg-teal-600 text-white border-teal-600", is_active)
                                    class=("bg-white text-gray-600 border-gray-300", is_not_active)
                                    on:click=on_click
                                >{cat}</button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Content
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"관찰 내용"</label>
                    <textarea
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 resize-none"
                        rows="6"
                        placeholder="관찰 내용을 상세히 기록해주세요..."
                        prop:value=move || content.get()
                        on:input=move |ev| content.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <button class="w-full py-3 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700">"기록 저장"</button>
        </div>
    }
}

// =============================================================================
// 13. IncidentPage — report incident form
// =============================================================================

#[component]
pub fn IncidentPage() -> impl IntoView {
    let severity = RwSignal::new("보통".to_string());
    let description = RwSignal::new(String::new());
    let actions_taken = RwSignal::new(String::new());

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/notes" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"사고/이상 보고"</h1>
            </div>

            <div class="bg-red-50 border border-red-200 rounded-xl p-4">
                <p class="text-sm text-red-800">"응급 상황 시 먼저 119에 연락하시고, 이후 보고서를 작성해주세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                // Client
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"고객"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500">
                        <option value="">"고객을 선택하세요"</option>
                        <option value="kim">"김복순님"</option>
                        <option value="lee">"이순자님"</option>
                        <option value="park">"박영자님"</option>
                    </select>
                </div>

                // Incident type
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"사고 유형"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500">
                        <option value="">"유형을 선택하세요"</option>
                        <option value="fall">"낙상"</option>
                        <option value="medication">"투약 사고"</option>
                        <option value="injury">"부상"</option>
                        <option value="wandering">"배회"</option>
                        <option value="behavioral">"행동 이상"</option>
                        <option value="other">"기타"</option>
                    </select>
                </div>

                // Severity
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"심각도"</label>
                    <div class="flex gap-2">
                        {["경미", "보통", "심각", "응급"].into_iter().map(|sev| {
                            let sev_str = sev.to_string();
                            let color = match sev {
                                "경미" => ("bg-green-600 text-white border-green-600", "bg-white text-gray-600 border-gray-300"),
                                "보통" => ("bg-yellow-500 text-white border-yellow-500", "bg-white text-gray-600 border-gray-300"),
                                "심각" => ("bg-orange-600 text-white border-orange-600", "bg-white text-gray-600 border-gray-300"),
                                _ => ("bg-red-600 text-white border-red-600", "bg-white text-gray-600 border-gray-300"),
                            };
                            let is_active = {
                                let sev_str = sev_str.clone();
                                move || severity.get() == sev_str
                            };
                            let is_not_active = {
                                let sev_str2 = sev_str.clone();
                                move || severity.get() != sev_str2
                            };
                            let on_click = {
                                let sev_str = sev_str.clone();
                                move |_| severity.set(sev_str.clone())
                            };
                            view! {
                                <button
                                    type="button"
                                    class="flex-1 py-2 text-xs rounded-lg border transition-colors"
                                    class=(color.0, is_active)
                                    class=(color.1, is_not_active)
                                    on:click=on_click
                                >{sev}</button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Description
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"상황 설명"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <textarea
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 resize-none"
                        rows="4"
                        placeholder="사고 상황을 상세히 기술해주세요..."
                        prop:value=move || description.get()
                        on:input=move |ev| description.set(event_target_value(&ev))
                    />
                </div>

                // Actions taken
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"조치 사항"</label>
                    <textarea
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 resize-none"
                        rows="3"
                        placeholder="취한 조치를 기록해주세요..."
                        prop:value=move || actions_taken.get()
                        on:input=move |ev| actions_taken.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <button class="w-full py-3 bg-red-600 text-white font-semibold rounded-xl hover:bg-red-700">"보고서 제출"</button>
        </div>
    }
}

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
// 17. ApplyOverviewPage — application landing / CTA page
// =============================================================================

#[component]
pub fn ApplyOverviewPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <div class="text-center pt-4">
                <div class="w-16 h-16 bg-teal-100 rounded-2xl flex items-center justify-center mx-auto mb-4">
                    <svg class="w-8 h-8 text-teal-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                    </svg>
                </div>
                <h1 class="text-2xl font-bold text-gray-900">"요양보호사 지원"</h1>
                <p class="text-sm text-gray-600 mt-2">"전문 요양보호사로 활동하세요. 간단한 지원 절차를 통해 시작할 수 있습니다."</p>
            </div>

            // Steps overview
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-4">"지원 절차"</h2>
                <div class="space-y-4">
                    <ApplyStepPreview step=1 title="본인 인증" desc="신분증 확인 및 본인 인증" />
                    <ApplyStepPreview step=2 title="자격 등록" desc="요양보호사 자격증 및 관련 서류 업로드" />
                    <ApplyStepPreview step=3 title="서비스 지역" desc="활동 가능한 지역 선택" />
                    <ApplyStepPreview step=4 title="근무 일정" desc="가능한 근무 시간 설정" />
                    <ApplyStepPreview step=5 title="서비스 유형" desc="제공 가능한 서비스 선택" />
                    <ApplyStepPreview step=6 title="추천인" desc="추천인 정보 입력" />
                    <ApplyStepPreview step=7 title="검토 및 제출" desc="지원서 확인 및 제출" />
                </div>
            </div>

            <a href="/caregiver/apply/identity" class="block w-full py-4 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700 text-center">"지원 시작하기"</a>
        </div>
    }
}

#[component]
fn ApplyStepPreview(
    step: u32,
    #[prop(into)] title: String,
    #[prop(into)] desc: String,
) -> impl IntoView {
    view! {
        <div class="flex items-start gap-3">
            <div class="w-7 h-7 bg-teal-100 rounded-full flex items-center justify-center shrink-0">
                <span class="text-xs font-bold text-teal-700">{step.to_string()}</span>
            </div>
            <div>
                <p class="text-sm font-medium text-gray-900">{title}</p>
                <p class="text-xs text-gray-500">{desc}</p>
            </div>
        </div>
    }
}

// =============================================================================
// 18. ApplyIdentityPage — identity verification step
// =============================================================================

#[component]
pub fn ApplyIdentityPage() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let national_id = RwSignal::new(String::new());
    let phone = RwSignal::new(String::new());

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=1 total=7 title="본인 인증" />

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"이름"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <input
                        type="text"
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500"
                        placeholder="실명을 입력하세요"
                        prop:value=move || name.get()
                        on:input=move |ev| name.set(event_target_value(&ev))
                    />
                </div>
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"주민등록번호"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <input
                        type="text"
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500"
                        placeholder="000000-0000000"
                        prop:value=move || national_id.get()
                        on:input=move |ev| national_id.set(event_target_value(&ev))
                    />
                </div>
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"휴대전화"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <input
                        type="tel"
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500"
                        placeholder="010-0000-0000"
                        prop:value=move || phone.get()
                        on:input=move |ev| phone.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <ApplyNavButtons prev_href="" next_href="/caregiver/apply/credentials" />
        </div>
    }
}

#[component]
fn ApplyStepHeader(
    step: u32,
    total: u32,
    #[prop(into)] title: String,
) -> impl IntoView {
    view! {
        <div>
            <p class="text-sm text-teal-600 font-medium">"단계 "{step.to_string()}" / "{total.to_string()}</p>
            <h1 class="text-xl font-bold text-gray-900 mt-1">{title}</h1>
            <div class="mt-3 w-full bg-gray-200 rounded-full h-1.5">
                <div
                    class="bg-teal-600 h-1.5 rounded-full transition-all"
                    style=format!("width: {}%", (step as f64 / total as f64 * 100.0) as u32)
                />
            </div>
        </div>
    }
}

#[component]
fn ApplyNavButtons(
    #[prop(into)] prev_href: String,
    #[prop(into)] next_href: String,
) -> impl IntoView {
    view! {
        <div class="flex gap-3">
            {if !prev_href.is_empty() {
                Some(view! {
                    <a href=prev_href class="flex-1 text-center py-3 border border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50">"이전"</a>
                })
            } else {
                None
            }}
            <a href=next_href class="flex-1 text-center py-3 bg-teal-600 text-white font-medium rounded-xl hover:bg-teal-700">"다음"</a>
        </div>
    }
}

// =============================================================================
// 19. ApplyCredentialsPage — credentials upload step
// =============================================================================

#[component]
pub fn ApplyCredentialsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=2 total=7 title="자격 등록" />

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                // Caregiver certificate
                <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700">"요양보호사 자격증"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <div class="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center cursor-pointer hover:border-teal-400">
                        <svg class="w-8 h-8 text-gray-400 mx-auto mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                        </svg>
                        <p class="text-sm text-gray-600">"클릭하여 파일 업로드"</p>
                        <p class="text-xs text-gray-400 mt-1">"JPG, PNG, PDF (최대 10MB)"</p>
                    </div>
                </div>

                // Additional certifications
                <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700">"추가 자격증 (선택)"</label>
                    <div class="border-2 border-dashed border-gray-300 rounded-lg p-4 text-center cursor-pointer hover:border-teal-400">
                        <p class="text-sm text-gray-500">"치매전문교육, 응급처치 등"</p>
                    </div>
                </div>

                // Experience
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"경력 기간"</label>
                    <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500">
                        <option value="">"경력을 선택하세요"</option>
                        <option value="0">"신입"</option>
                        <option value="1">"1년 미만"</option>
                        <option value="3">"1~3년"</option>
                        <option value="5">"3~5년"</option>
                        <option value="10">"5년 이상"</option>
                    </select>
                </div>
            </div>

            <ApplyNavButtons prev_href="/caregiver/apply/identity" next_href="/caregiver/apply/region" />
        </div>
    }
}

// =============================================================================
// 20. ApplyServiceRegionPage — service region selection
// =============================================================================

#[component]
pub fn ApplyServiceRegionPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=3 total=7 title="서비스 지역" />

            <p class="text-sm text-gray-600">"활동 가능한 지역을 선택해주세요. (복수 선택 가능)"</p>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                // City select
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"시/도"</label>
                    <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500">
                        <option value="">"시/도를 선택하세요"</option>
                        <option value="seoul">"서울특별시"</option>
                        <option value="gyeonggi">"경기도"</option>
                        <option value="incheon">"인천광역시"</option>
                        <option value="busan">"부산광역시"</option>
                    </select>
                </div>

                // District checkboxes
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"구/군"</label>
                    <div class="grid grid-cols-2 gap-2">
                        {["강남구", "서초구", "송파구", "마포구", "영등포구", "강서구", "용산구", "종로구"].into_iter().map(|district| {
                            let checked = RwSignal::new(false);
                            view! {
                                <label class="flex items-center gap-2 p-2 rounded-lg border border-gray-200 cursor-pointer hover:bg-gray-50">
                                    <input
                                        type="checkbox"
                                        class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500"
                                        prop:checked=move || checked.get()
                                        on:change=move |_| checked.update(|v| *v = !*v)
                                    />
                                    <span class="text-sm text-gray-700">{district}</span>
                                </label>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>

            <ApplyNavButtons prev_href="/caregiver/apply/credentials" next_href="/caregiver/apply/schedule" />
        </div>
    }
}

// =============================================================================
// 21. ApplySchedulePage — availability schedule step
// =============================================================================

#[component]
pub fn ApplySchedulePage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=4 total=7 title="근무 일정" />

            <p class="text-sm text-gray-600">"근무 가능한 요일과 시간대를 선택해주세요."</p>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                {["월요일", "화요일", "수요일", "목요일", "금요일", "토요일", "일요일"].into_iter().map(|day| {
                    let enabled = RwSignal::new(false);
                    view! {
                        <div class="flex items-center justify-between py-2 border-b border-gray-50 last:border-0">
                            <label class="flex items-center gap-3">
                                <input
                                    type="checkbox"
                                    class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500"
                                    prop:checked=move || enabled.get()
                                    on:change=move |_| enabled.update(|v| *v = !*v)
                                />
                                <span class="text-sm font-medium text-gray-700">{day}</span>
                            </label>
                            <Show when=move || enabled.get()>
                                <div class="flex items-center gap-1 text-sm text-gray-500">
                                    <select class="px-2 py-1 border border-gray-200 rounded text-xs">
                                        <option>"09:00"</option>
                                        <option>"10:00"</option>
                                        <option>"11:00"</option>
                                    </select>
                                    <span>"~"</span>
                                    <select class="px-2 py-1 border border-gray-200 rounded text-xs">
                                        <option>"17:00"</option>
                                        <option>"18:00"</option>
                                        <option>"19:00"</option>
                                    </select>
                                </div>
                            </Show>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Preferences
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                <h3 class="font-semibold text-gray-900">"추가 설정"</h3>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500" />
                    <span class="text-sm text-gray-700">"야간 근무 가능"</span>
                </label>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500" />
                    <span class="text-sm text-gray-700">"주말 근무 가능"</span>
                </label>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500" />
                    <span class="text-sm text-gray-700">"긴급 호출 가능"</span>
                </label>
            </div>

            <ApplyNavButtons prev_href="/caregiver/apply/region" next_href="/caregiver/apply/services" />
        </div>
    }
}

// =============================================================================
// 22. ApplyServicesPage — service types selection
// =============================================================================

#[component]
pub fn ApplyServicesPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=5 total=7 title="서비스 유형" />

            <p class="text-sm text-gray-600">"제공 가능한 서비스를 모두 선택해주세요."</p>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                <ServiceTypeOption title="방문요양" desc="식사, 세면, 배설 등 일상생활 지원" />
                <ServiceTypeOption title="방문목욕" desc="이동식 욕조를 이용한 목욕 서비스" />
                <ServiceTypeOption title="방문간호" desc="간호, 진료보조 등 의료 서비스" />
                <ServiceTypeOption title="주야간보호" desc="주간/야간 시설 보호 서비스" />
                <ServiceTypeOption title="인지활동" desc="치매 예방 및 인지 자극 프로그램" />
                <ServiceTypeOption title="정서지원" desc="말벗, 외출 동행 등 정서적 지원" />
            </div>

            // Specializations
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                <h3 class="font-semibold text-gray-900">"전문 분야"</h3>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500" />
                    <span class="text-sm text-gray-700">"치매 케어 경험"</span>
                </label>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500" />
                    <span class="text-sm text-gray-700">"와상 환자 케어"</span>
                </label>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500" />
                    <span class="text-sm text-gray-700">"재활 보조"</span>
                </label>
            </div>

            <ApplyNavButtons prev_href="/caregiver/apply/schedule" next_href="/caregiver/apply/references" />
        </div>
    }
}

#[component]
fn ServiceTypeOption(
    #[prop(into)] title: String,
    #[prop(into)] desc: String,
) -> impl IntoView {
    let selected = RwSignal::new(false);

    view! {
        <button
            type="button"
            class="w-full flex items-start gap-3 p-3 rounded-lg border text-left transition-colors"
            class=("border-teal-500 bg-teal-50", move || selected.get())
            class=("border-gray-200 hover:bg-gray-50", move || !selected.get())
            on:click=move |_| selected.update(|v| *v = !*v)
        >
            <div
                class="w-5 h-5 rounded border-2 flex items-center justify-center shrink-0 mt-0.5"
                class=("border-teal-600 bg-teal-600", move || selected.get())
                class=("border-gray-300", move || !selected.get())
            >
                <Show when=move || selected.get()>
                    <svg class="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                    </svg>
                </Show>
            </div>
            <div>
                <p class="text-sm font-medium text-gray-900">{title}</p>
                <p class="text-xs text-gray-500">{desc}</p>
            </div>
        </button>
    }
}

// =============================================================================
// 23. ApplyReferencesPage — references form
// =============================================================================

#[component]
pub fn ApplyReferencesPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=6 total=7 title="추천인" />

            <p class="text-sm text-gray-600">"이전 근무지의 추천인 정보를 입력해주세요. (선택사항)"</p>

            <div class="space-y-4">
                <ReferenceForm index=1 />
                <ReferenceForm index=2 />
            </div>

            <button type="button" class="w-full py-3 border-2 border-dashed border-gray-300 text-gray-500 text-sm rounded-xl hover:border-gray-400">"+ 추천인 추가"</button>

            <ApplyNavButtons prev_href="/caregiver/apply/services" next_href="/caregiver/apply/review" />
        </div>
    }
}

#[component]
fn ReferenceForm(index: u32) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
            <h3 class="font-semibold text-gray-900">"추천인 "{index.to_string()}</h3>
            <div class="space-y-1">
                <label class="text-sm font-medium text-gray-700">"이름"</label>
                <input type="text" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-teal-500" placeholder="추천인 이름" />
            </div>
            <div class="space-y-1">
                <label class="text-sm font-medium text-gray-700">"관계"</label>
                <input type="text" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-teal-500" placeholder="예: 이전 기관 관리자" />
            </div>
            <div class="space-y-1">
                <label class="text-sm font-medium text-gray-700">"연락처"</label>
                <input type="tel" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-teal-500" placeholder="010-0000-0000" />
            </div>
        </div>
    }
}

// =============================================================================
// 24. ApplyReviewPage — review & submit application
// =============================================================================

#[component]
pub fn ApplyReviewPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=7 total=7 title="검토 및 제출" />

            <p class="text-sm text-gray-600">"입력하신 정보를 확인해주세요."</p>

            // Review sections
            <ReviewSection title="본인 정보" href="/caregiver/apply/identity">
                <ReviewItem label="이름" value="홍길동" />
                <ReviewItem label="휴대전화" value="010-5555-1234" />
            </ReviewSection>

            <ReviewSection title="자격 정보" href="/caregiver/apply/credentials">
                <ReviewItem label="자격증" value="요양보호사 자격증" />
                <ReviewItem label="경력" value="5년" />
            </ReviewSection>

            <ReviewSection title="서비스 지역" href="/caregiver/apply/region">
                <ReviewItem label="지역" value="서울시 강남구, 서초구" />
            </ReviewSection>

            <ReviewSection title="근무 일정" href="/caregiver/apply/schedule">
                <ReviewItem label="가능 요일" value="월~금" />
                <ReviewItem label="시간" value="09:00 - 18:00" />
            </ReviewSection>

            <ReviewSection title="서비스 유형" href="/caregiver/apply/services">
                <ReviewItem label="서비스" value="방문요양, 방문목욕" />
                <ReviewItem label="전문 분야" value="치매 케어" />
            </ReviewSection>

            // Agreement
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                <label class="flex items-start gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500 mt-0.5" />
                    <span class="text-sm text-gray-700">"입력한 정보가 사실과 다름없음을 확인하며, 개인정보 처리방침에 동의합니다."</span>
                </label>
            </div>

            <div class="flex gap-3">
                <a href="/caregiver/apply/references" class="flex-1 text-center py-3 border border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50">"이전"</a>
                <button class="flex-1 py-3 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700">"지원서 제출"</button>
            </div>
        </div>
    }
}

#[component]
fn ReviewSection(
    #[prop(into)] title: String,
    #[prop(into)] href: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
            <div class="flex items-center justify-between mb-3">
                <h3 class="font-semibold text-gray-900">{title}</h3>
                <a href=href class="text-xs text-teal-600 font-medium hover:text-teal-700">"수정"</a>
            </div>
            <dl class="space-y-2 text-sm">
                {children()}
            </dl>
        </div>
    }
}

#[component]
fn ReviewItem(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex justify-between">
            <dt class="text-gray-500">{label}</dt>
            <dd class="font-medium text-gray-900">{value}</dd>
        </div>
    }
}

// =============================================================================
// 25. ApplyStatusPage — application status tracker
// =============================================================================

#[component]
pub fn ApplyStatusPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <h1 class="text-xl font-bold text-gray-900">"지원 현황"</h1>

            // Status card
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                <div class="w-16 h-16 bg-yellow-100 rounded-full flex items-center justify-center mx-auto mb-3">
                    <svg class="w-8 h-8 text-yellow-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                </div>
                <h2 class="text-lg font-bold text-gray-900">"심사 중"</h2>
                <p class="text-sm text-gray-500 mt-1">"지원서가 검토되고 있습니다."</p>
                <p class="text-xs text-gray-400 mt-2">"제출일: 2026.03.15"</p>
            </div>

            // Progress steps
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-4">"진행 상태"</h3>
                <div class="space-y-4">
                    <StatusStep label="지원서 제출" date="2026.03.15" done=true />
                    <StatusStep label="서류 검토" date="2026.03.16" done=true />
                    <StatusStep label="자격 확인" date="진행 중" done=false />
                    <StatusStep label="최종 승인" date="" done=false />
                </div>
            </div>

            // Contact
            <div class="bg-blue-50 border border-blue-200 rounded-xl p-4">
                <p class="text-sm text-blue-800">"문의사항은 고객센터(1588-0000)로 연락해주세요."</p>
            </div>
        </div>
    }
}

#[component]
fn StatusStep(
    #[prop(into)] label: String,
    #[prop(into)] date: String,
    done: bool,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3">
            <div
                class="w-8 h-8 rounded-full flex items-center justify-center shrink-0"
                class=("bg-teal-600", done)
                class=("bg-gray-200", !done)
            >
                {if done {
                    view! { <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3"><path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" /></svg> }.into_any()
                } else {
                    view! { <div class="w-2 h-2 bg-gray-400 rounded-full" /> }.into_any()
                }}
            </div>
            <div class="flex-1">
                <p class="text-sm font-medium" class=("text-gray-900", done) class=("text-gray-500", !done)>{label}</p>
                {if !date.is_empty() {
                    Some(view! { <p class="text-xs text-gray-400">{date}</p> })
                } else {
                    None
                }}
            </div>
        </div>
    }
}

// =============================================================================
// 26. TasksListPage — assigned tasks list
// =============================================================================

#[component]
pub fn TasksListPage() -> impl IntoView {
    let filter = RwSignal::new("전체".to_string());

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <h1 class="text-xl font-bold text-gray-900">"업무 목록"</h1>

            // Filter tabs
            <div class="flex gap-2 overflow-x-auto">
                {["전체", "오늘", "미완료", "완료"].into_iter().map(|tab| {
                    let tab_str = tab.to_string();
                    let is_active = {
                        let tab_str = tab_str.clone();
                        move || filter.get() == tab_str
                    };
                    let is_not_active = {
                        let tab_str2 = tab_str.clone();
                        move || filter.get() != tab_str2
                    };
                    let on_click = {
                        let tab_str = tab_str.clone();
                        move |_| filter.set(tab_str.clone())
                    };
                    view! {
                        <button
                            class="px-4 py-1.5 text-sm rounded-full whitespace-nowrap transition-colors"
                            class=("bg-teal-600 text-white", is_active)
                            class=("bg-gray-100 text-gray-600", is_not_active)
                            on:click=on_click
                        >{tab}</button>
                    }
                }).collect_view()}
            </div>

            // Task list
            <div class="space-y-3">
                <TaskCard title="김복순님 혈압 측정" client="김복순님" time="14:00" priority="높음" done=false />
                <TaskCard title="이순자님 투약 확인" client="이순자님" time="09:30" priority="높음" done=true />
                <TaskCard title="박영자님 목욕 준비" client="박영자님" time="11:00" priority="보통" done=true />
                <TaskCard title="최영희님 산책 동행" client="최영희님" time="15:00" priority="보통" done=false />
                <TaskCard title="김복순님 일지 작성" client="김복순님" time="16:00" priority="낮음" done=false />
            </div>
        </div>
    }
}

#[component]
fn TaskCard(
    #[prop(into)] title: String,
    #[prop(into)] client: String,
    #[prop(into)] time: String,
    #[prop(into)] priority: String,
    done: bool,
) -> impl IntoView {
    let is_done = RwSignal::new(done);

    let prio_cls = match priority.as_str() {
        "높음" => "bg-red-100 text-red-700",
        "보통" => "bg-yellow-100 text-yellow-700",
        _ => "bg-gray-100 text-gray-600",
    };

    view! {
        <a href="/caregiver/tasks/task-1" class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
            <div class="flex items-start gap-3">
                <button
                    class="w-5 h-5 rounded border-2 flex items-center justify-center shrink-0 mt-0.5"
                    class=("border-teal-600 bg-teal-600", move || is_done.get())
                    class=("border-gray-300", move || !is_done.get())
                    on:click=move |ev| {
                        ev.prevent_default();
                        ev.stop_propagation();
                        is_done.update(|v| *v = !*v);
                    }
                >
                    <Show when=move || is_done.get()>
                        <svg class="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                        </svg>
                    </Show>
                </button>
                <div class="flex-1 min-w-0">
                    <p class="font-medium text-gray-900" class=("line-through text-gray-400", move || is_done.get())>{title}</p>
                    <div class="flex items-center gap-2 mt-1">
                        <span class="text-xs text-gray-500">{client}</span>
                        <span class="text-xs text-gray-400">"·"</span>
                        <span class="text-xs text-gray-500">{time}</span>
                    </div>
                </div>
                <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full shrink-0 {prio_cls}")}>{priority}</span>
            </div>
        </a>
    }
}

// =============================================================================
// 27. TaskDetailPage — task detail
// =============================================================================

#[component]
pub fn TaskDetailPage() -> impl IntoView {
    let is_complete = RwSignal::new(false);

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/tasks" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"업무 상세"</h1>
            </div>

            // Task header
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="flex items-center justify-between mb-3">
                    <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-red-100 text-red-700">"높음"</span>
                    <span class="text-xs font-medium px-2 py-0.5 rounded-full" class=("bg-green-100 text-green-700", move || is_complete.get()) class=("bg-yellow-100 text-yellow-700", move || !is_complete.get())>
                        {move || if is_complete.get() { "완료" } else { "미완료" }}
                    </span>
                </div>
                <h2 class="text-lg font-bold text-gray-900">"김복순님 혈압 측정"</h2>
                <p class="text-sm text-gray-500 mt-1">"매일 방문 시 혈압을 측정하고 기록해주세요."</p>
            </div>

            // Details
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"상세 정보"</h3>
                <dl class="space-y-2 text-sm">
                    <div class="flex justify-between"><dt class="text-gray-500">"고객"</dt><dd class="font-medium text-gray-900">"김복순님"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"예정 시간"</dt><dd class="font-medium text-gray-900">"14:00"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"반복"</dt><dd class="font-medium text-gray-900">"매일"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"카테고리"</dt><dd class="font-medium text-gray-900">"건강 체크"</dd></div>
                </dl>
            </div>

            // Instructions
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"수행 지침"</h3>
                <ol class="space-y-2 text-sm text-gray-700 list-decimal list-inside">
                    <li>"안정을 취한 상태에서 5분 후 측정"</li>
                    <li>"좌측 팔에 커프를 감고 측정"</li>
                    <li>"수축기/이완기 혈압 기록"</li>
                    <li>"이상 수치 시 즉시 보고 (수축기 160 이상)"</li>
                </ol>
            </div>

            <button
                class="w-full py-4 font-semibold rounded-xl transition-colors"
                class=("bg-teal-600 text-white hover:bg-teal-700", move || !is_complete.get())
                class=("bg-gray-200 text-gray-500", move || is_complete.get())
                on:click=move |_| is_complete.set(true)
            >
                {move || if is_complete.get() { "완료됨" } else { "업무 완료" }}
            </button>
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
                        <a href="/auth/signin" class="flex items-center justify-between py-2">
                            <span class="text-sm text-red-600">"로그아웃"</span>
                        </a>
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
