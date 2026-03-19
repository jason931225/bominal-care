use leptos::prelude::*;
use uuid::Uuid;

// =============================================================================
// TaskCard — task list item with checkbox toggle
// =============================================================================

#[component]
fn TaskCard(
    #[prop(into)] id: String,
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
    let href = format!("/caregiver/tasks/{id}");

    view! {
        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
            <div class="flex items-start gap-3">
                <button
                    class="mt-0.5 w-5 h-5 rounded border-2 flex items-center justify-center shrink-0 transition-colors"
                    class=("border-teal-600", move || is_done.get())
                    class=("bg-teal-600", move || is_done.get())
                    class=("border-gray-300", move || !is_done.get())
                    on:click=move |_| is_done.update(|v| *v = !*v)
                >
                    <Show when=move || is_done.get()>
                        <svg class="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                        </svg>
                    </Show>
                </button>
                <a href=href class="flex-1 min-w-0">
                    <div class="flex items-center justify-between mb-1">
                        <p
                            class="font-medium text-gray-900"
                            class=("line-through", move || is_done.get())
                            class=("text-gray-400", move || is_done.get())
                        >{title}</p>
                        <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full shrink-0 ml-2 {prio_cls}")}>{priority}</span>
                    </div>
                    <div class="flex items-center gap-2 mt-1">
                        <span class="text-xs text-gray-500">{client}</span>
                        <span class="text-xs text-gray-400">"·"</span>
                        <span class="text-xs text-teal-600">{time}</span>
                    </div>
                </a>
            </div>
        </div>
    }
}

// =============================================================================
// NoteCard — observation note list item with color-coded category
// =============================================================================

#[component]
fn NoteCard(
    #[prop(into)] date: String,
    #[prop(into)] client: String,
    #[prop(into)] preview: String,
    #[prop(into)] category: String,
) -> impl IntoView {
    let cat_cls = match category.as_str() {
        "건강" => "bg-green-100 text-green-700",
        "식사" => "bg-orange-100 text-orange-700",
        "정서" => "bg-purple-100 text-purple-700",
        "활동" => "bg-blue-100 text-blue-700",
        _ => "bg-gray-100 text-gray-600",
    };

    view! {
        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
            <div class="flex items-center justify-between mb-2">
                <span class="text-xs text-gray-500">{date}</span>
                <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full {cat_cls}")}>{category}</span>
            </div>
            <p class="text-sm font-medium text-gray-900 mb-1">{client}</p>
            <p class="text-sm text-gray-500 line-clamp-2">{preview}</p>
        </div>
    }
}

// =============================================================================
// TasksListPage — task list with filter tabs (전체/오늘/미완료/완료)
// =============================================================================

#[component]
pub fn TasksListPage() -> impl IntoView {
    let active_tab = RwSignal::new(0_usize);

    let id1 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"task-bp-check-kim").to_string();
    let id2 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"task-med-check-lee").to_string();
    let id3 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"task-bath-prep-park").to_string();
    let id4 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"task-walk-choi").to_string();
    let id5 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"task-journal-kim").to_string();

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <h1 class="text-xl font-bold text-gray-900">"업무 목록"</h1>

            // Filter tabs
            <div class="flex gap-2 overflow-x-auto pb-1">
                {["전체", "오늘", "미완료", "완료"].into_iter().enumerate().map(|(i, label)| {
                    let is_active = move || active_tab.get() == i;
                    let sel_cls = move || {
                        if is_active() {
                            "px-4 py-2 text-sm font-medium rounded-full bg-teal-600 text-white whitespace-nowrap"
                        } else {
                            "px-4 py-2 text-sm font-medium rounded-full bg-white border border-gray-200 text-gray-600 whitespace-nowrap"
                        }
                    };
                    view! {
                        <button class=sel_cls on:click=move |_| active_tab.set(i)>
                            {label}
                        </button>
                    }
                }).collect_view()}
            </div>

            // Task list
            <div class="space-y-3">
                <TaskCard
                    id=id1
                    title="김복순님 혈압 측정"
                    client="김복순님"
                    time="14:00"
                    priority="높음"
                    done=false
                />
                <TaskCard
                    id=id2
                    title="이순자님 투약 확인"
                    client="이순자님"
                    time="09:00"
                    priority="높음"
                    done=true
                />
                <TaskCard
                    id=id3
                    title="박영자님 목욕 준비"
                    client="박영자님"
                    time="11:30"
                    priority="보통"
                    done=true
                />
                <TaskCard
                    id=id4
                    title="최영희님 산책 동행"
                    client="최영희님"
                    time="16:30"
                    priority="보통"
                    done=false
                />
                <TaskCard
                    id=id5
                    title="김복순님 일지 작성"
                    client="김복순님"
                    time="17:00"
                    priority="낮음"
                    done=false
                />
            </div>
        </div>
    }
}

// =============================================================================
// TaskDetailPage — single task detail (김복순님 혈압 측정) with complete button
// =============================================================================

#[component]
pub fn TaskDetailPage() -> impl IntoView {
    let is_done = RwSignal::new(false);

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

            // Task info card
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="flex items-center justify-between mb-3">
                    <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-red-100 text-red-700">"높음"</span>
                    <span
                        class="text-xs font-medium px-2 py-0.5 rounded-full"
                        class=("bg-green-100", move || is_done.get())
                        class=("text-green-700", move || is_done.get())
                        class=("bg-yellow-100", move || !is_done.get())
                        class=("text-yellow-700", move || !is_done.get())
                    >
                        {move || if is_done.get() { "완료" } else { "미완료" }}
                    </span>
                </div>
                <h2 class="text-lg font-bold text-gray-900 mb-1">"김복순님 혈압 측정"</h2>
                <p class="text-sm text-gray-500">"매일 방문 시 혈압을 측정하고 기록해주세요."</p>
            </div>

            // Client info
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"고객 정보"</h3>
                <div class="flex items-center gap-3 mb-3">
                    <div class="w-11 h-11 bg-teal-100 rounded-full flex items-center justify-center">
                        <span class="text-lg font-bold text-teal-700">"김"</span>
                    </div>
                    <div>
                        <p class="font-medium text-gray-900">"김복순님"</p>
                        <p class="text-sm text-gray-500">"82세 · 장기요양 3등급"</p>
                    </div>
                </div>
                <dl class="space-y-2 text-sm">
                    <div class="flex justify-between">
                        <dt class="text-gray-500">"예정 시간"</dt>
                        <dd class="font-medium text-gray-900">"14:00"</dd>
                    </div>
                    <div class="flex justify-between">
                        <dt class="text-gray-500">"반복"</dt>
                        <dd class="font-medium text-gray-900">"매일"</dd>
                    </div>
                    <div class="flex justify-between">
                        <dt class="text-gray-500">"카테고리"</dt>
                        <dd class="font-medium text-gray-900">"건강 체크"</dd>
                    </div>
                    <div class="flex justify-between">
                        <dt class="text-gray-500">"특이사항"</dt>
                        <dd class="font-medium text-gray-900">"치매 초기, 당뇨"</dd>
                    </div>
                </dl>
            </div>

            // Checklist
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"수행 항목"</h3>
                <ul class="space-y-2">
                    {[
                        "혈압계 준비 및 점검",
                        "안정 상태 확인 (5분 휴식)",
                        "혈압 측정 (좌측 상완)",
                        "측정값 기록",
                        "이상 수치 시 보호자 연락",
                    ].into_iter().map(|label| {
                        let checked = RwSignal::new(false);
                        view! {
                            <li class="flex items-center gap-3">
                                <button
                                    class="w-5 h-5 rounded border-2 flex items-center justify-center transition-colors"
                                    class=("border-teal-600", move || checked.get())
                                    class=("bg-teal-600", move || checked.get())
                                    class=("border-gray-300", move || !checked.get())
                                    on:click=move |_| checked.update(|v| *v = !*v)
                                >
                                    <Show when=move || checked.get()>
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
            </div>

            // Complete button
            <button
                class="w-full py-4 font-semibold rounded-xl transition-colors"
                class=("bg-teal-600", move || !is_done.get())
                class=("text-white", move || !is_done.get())
                class=("hover:bg-teal-700", move || !is_done.get())
                class=("bg-gray-200", move || is_done.get())
                class=("text-gray-500", move || is_done.get())
                on:click=move |_| is_done.set(true)
            >
                {move || if is_done.get() { "완료됨" } else { "완료 처리" }}
            </button>
        </div>
    }
}

// =============================================================================
// NotesListPage — observation notes list with "새 기록" button
// =============================================================================

#[component]
pub fn NotesListPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-xl font-bold text-gray-900">"관찰 기록"</h1>
                <a
                    href="/caregiver/notes/new"
                    class="px-4 py-2 bg-teal-600 text-white text-sm font-medium rounded-lg hover:bg-teal-700"
                >"새 기록"</a>
            </div>

            <div class="space-y-3">
                <NoteCard
                    date="2026.03.18 14:30"
                    client="김복순님"
                    preview="오후 혈압 측정 결과 140/85mmHg. 평소보다 약간 높은 수치로 안정 후 재측정 예정."
                    category="건강"
                />
                <NoteCard
                    date="2026.03.18 12:00"
                    client="이순자님"
                    preview="점심 식사 보조. 식욕이 좋으며 밥 한 공기와 반찬을 골고루 드셨음."
                    category="식사"
                />
                <NoteCard
                    date="2026.03.17 16:00"
                    client="최영희님"
                    preview="산책 중 기분이 좋아 보이셨으며 30분간 공원을 걸으심. 다리 통증 호소 없음."
                    category="활동"
                />
                <NoteCard
                    date="2026.03.17 11:00"
                    client="박영자님"
                    preview="목욕 후 피부 상태 양호. 등 부위 약간의 건조함 확인, 보습제 도포 완료."
                    category="건강"
                />
                <NoteCard
                    date="2026.03.16 15:30"
                    client="김복순님"
                    preview="가족 사진을 보며 대화. 과거 기억을 잘 회상하시며 정서적으로 안정된 모습."
                    category="정서"
                />
            </div>
        </div>
    }
}

// =============================================================================
// NoteNewPage — create observation note form
// =============================================================================

#[component]
pub fn NoteNewPage() -> impl IntoView {
    let selected_client = RwSignal::new(String::new());
    let selected_category = RwSignal::new(String::new());
    let content = RwSignal::new(String::new());

    let clients = vec!["김복순님", "이순자님", "박영자님", "최영희님", "정순옥님"];
    let categories = vec!["건강", "식사", "정서", "활동", "기타"];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/notes" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"새 관찰 기록"</h1>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                // Client select
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"고객 선택"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <select
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 bg-white"
                        on:change=move |ev| selected_client.set(event_target_value(&ev))
                    >
                        <option value="" disabled selected>"고객을 선택하세요"</option>
                        {clients.into_iter().map(|c| {
                            view! { <option value=c>{c}</option> }
                        }).collect_view()}
                    </select>
                </div>

                // Category chips
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"카테고리"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <div class="flex flex-wrap gap-2">
                        {categories.into_iter().map(|cat| {
                            let cat_str = cat.to_string();
                            let on_click = {
                                let cat_str = cat_str.clone();
                                move |_| selected_category.set(cat_str.clone())
                            };
                            let s1 = cat_str.clone();
                            let s2 = cat_str.clone();
                            let s3 = cat_str.clone();
                            let s4 = cat_str.clone();
                            let s5 = cat_str.clone();
                            let s6 = cat_str;
                            view! {
                                <button
                                    type="button"
                                    class="px-3 py-1.5 text-sm rounded-full border transition-colors"
                                    class=("bg-teal-600", move || selected_category.get() == s1)
                                    class=("text-white", move || selected_category.get() == s2)
                                    class=("border-teal-600", move || selected_category.get() == s3)
                                    class=("bg-white", move || selected_category.get() != s4)
                                    class=("text-gray-600", move || selected_category.get() != s5)
                                    class=("border-gray-300", move || selected_category.get() != s6)
                                    on:click=on_click
                                >{cat}</button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Content textarea
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"내용"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <textarea
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 resize-none"
                        rows="6"
                        placeholder="관찰 내용을 상세히 기록하세요..."
                        prop:value=move || content.get()
                        on:input=move |ev| content.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <button class="w-full py-4 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700">"기록 저장"</button>
        </div>
    }
}

// =============================================================================
// IncidentPage — incident report form
// =============================================================================

#[component]
pub fn IncidentPage() -> impl IntoView {
    let selected_client = RwSignal::new(String::new());
    let incident_type = RwSignal::new(String::new());
    let severity = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let actions_taken = RwSignal::new(String::new());

    let clients = vec!["김복순님", "이순자님", "박영자님", "최영희님", "정순옥님"];
    let incident_types = vec!["낙상", "투약 오류", "피부 손상", "행동 변화", "기타"];
    let severity_levels: Vec<(&str, &str)> = vec![
        ("경미", "bg-green-100 text-green-700 border-green-300"),
        ("보통", "bg-yellow-100 text-yellow-700 border-yellow-300"),
        ("심각", "bg-orange-100 text-orange-700 border-orange-300"),
        ("응급", "bg-red-100 text-red-700 border-red-300"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"사고 보고"</h1>
            </div>

            // Warning banner
            <div class="bg-red-50 border border-red-200 rounded-xl p-4">
                <p class="text-sm font-medium text-red-800">"응급 상황 시 즉시 119에 연락하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                // Client select
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"고객"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <select
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 bg-white"
                        on:change=move |ev| selected_client.set(event_target_value(&ev))
                    >
                        <option value="" disabled selected>"고객을 선택하세요"</option>
                        {clients.into_iter().map(|c| {
                            view! { <option value=c>{c}</option> }
                        }).collect_view()}
                    </select>
                </div>

                // Incident type select
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"사고 유형"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <select
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 bg-white"
                        on:change=move |ev| incident_type.set(event_target_value(&ev))
                    >
                        <option value="" disabled selected>"유형을 선택하세요"</option>
                        {incident_types.into_iter().map(|t| {
                            view! { <option value=t>{t}</option> }
                        }).collect_view()}
                    </select>
                </div>

                // Severity buttons
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"심각도"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <div class="grid grid-cols-4 gap-2">
                        {severity_levels.into_iter().map(|(level, active_cls)| {
                            let level_str = level.to_string();
                            let active_cls_owned = active_cls.to_string();
                            let cls = {
                                let level_str = level_str.clone();
                                move || {
                                    let base = "py-2 text-sm font-medium rounded-lg border transition-colors text-center";
                                    if severity.get() == level_str {
                                        format!("{base} {}", active_cls_owned)
                                    } else {
                                        format!("{base} bg-white text-gray-600 border-gray-300")
                                    }
                                }
                            };
                            let on_click = {
                                let level_str = level_str.clone();
                                move |_| severity.set(level_str.clone())
                            };
                            view! {
                                <button
                                    type="button"
                                    class=cls
                                    on:click=on_click
                                >{level}</button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Description
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"상황 설명"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <textarea
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 resize-none"
                        rows="4"
                        placeholder="사고 상황을 상세히 기록하세요..."
                        prop:value=move || description.get()
                        on:input=move |ev| description.set(event_target_value(&ev))
                    />
                </div>

                // Actions taken
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"조치 사항"</label>
                    <textarea
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 resize-none"
                        rows="3"
                        placeholder="취한 조치를 기록하세요..."
                        prop:value=move || actions_taken.get()
                        on:input=move |ev| actions_taken.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <button class="w-full py-4 bg-red-600 text-white font-semibold rounded-xl hover:bg-red-700">"보고서 제출"</button>
        </div>
    }
}
