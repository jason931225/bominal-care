use leptos::prelude::*;
use bominal_types::{Visit, Incident, DailyObservation, PersonProfile};

// =============================================================================
// Helpers
// =============================================================================

/// Extract a path segment by index from the current URL.
fn path_segment(index: usize) -> String {
    leptos::web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_default()
        .split('/')
        .nth(index)
        .unwrap_or("")
        .to_string()
}

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
        "건강" | "BloodPressure" | "Weight" | "Temperature" => "bg-green-100 text-green-700",
        "식사" | "Nutrition" => "bg-orange-100 text-orange-700",
        "정서" | "Mood" | "SocialEngagement" => "bg-purple-100 text-purple-700",
        "활동" | "Activity" | "Mobility" => "bg-blue-100 text-blue-700",
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
    let visits = LocalResource::new(|| crate::api::get::<Vec<Visit>>("/api/visits?status=in_progress"));

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
            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
            }>
                {move || Suspend::new(async move {
                    match visits.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-gray-500 py-8">"진행 중인 업무가 없습니다."</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|visit| {
                                            let id = visit.id.to_string();
                                            let time = visit.scheduled_start.format("%H:%M").to_string();
                                            let status_str = format!("{}", visit.status);
                                            let is_done = status_str == "완료";
                                            view! {
                                                <TaskCard
                                                    id=id
                                                    title=format!("방문 {}", time)
                                                    client="고객".to_string()
                                                    time=time
                                                    priority="보통".to_string()
                                                    done=is_done
                                                />
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-gray-500 py-8">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

// =============================================================================
// TaskDetailPage — single task detail fetched from API visit
// =============================================================================

#[component]
pub fn TaskDetailPage() -> impl IntoView {
    let is_done = RwSignal::new(false);

    // Extract task/visit ID from URL: /caregiver/tasks/{id}
    let task_id = path_segment(3);
    let visit_url = format!("/api/visits/{}", task_id);

    let visit_data = LocalResource::new(move || {
        let url = visit_url.clone();
        async move { crate::api::get::<Visit>(&url).await }
    });

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

            <Suspense fallback=move || view! {
                <div class="animate-pulse space-y-4">
                    <div class="bg-gray-200 rounded-xl h-32" />
                    <div class="bg-gray-200 rounded-xl h-24" />
                </div>
            }>
                {move || Suspend::new(async move {
                    match visit_data.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(v) => {
                                    let time = v.scheduled_start.format("%H:%M").to_string();
                                    let date_str = crate::api::format_date_kr(&v.scheduled_start);
                                    let status = format!("{}", v.status);
                                    let notes_text = v.notes.clone().unwrap_or_else(|| "특이사항 없음".to_string());

                                    view! {
                                        <div class="space-y-5">
                                            // Task info card
                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                <div class="flex items-center justify-between mb-3">
                                                    <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-yellow-100 text-yellow-700">"보통"</span>
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
                                                <h2 class="text-lg font-bold text-gray-900 mb-1">{format!("방문 {} 업무", time)}</h2>
                                                <p class="text-sm text-gray-500">{format!("{} · {}", date_str, status)}</p>
                                            </div>

                                            // Visit info
                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                <h3 class="font-semibold text-gray-900 mb-3">"방문 정보"</h3>
                                                <dl class="space-y-2 text-sm">
                                                    <div class="flex justify-between">
                                                        <dt class="text-gray-500">"예정 시간"</dt>
                                                        <dd class="font-medium text-gray-900">{format!("{} - {}", v.scheduled_start.format("%H:%M"), v.scheduled_end.format("%H:%M"))}</dd>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <dt class="text-gray-500">"상태"</dt>
                                                        <dd class="font-medium text-gray-900">{status}</dd>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <dt class="text-gray-500">"메모"</dt>
                                                        <dd class="font-medium text-gray-900">{notes_text}</dd>
                                                    </div>
                                                </dl>
                                            </div>

                                            // Checklist (template items)
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
                                    }.into_any()
                                }
                                None => view! {
                                    <p class="text-center text-gray-500 py-8">"업무 정보를 찾을 수 없습니다."</p>
                                }.into_any(),
                            }
                        }
                        _ => view! {
                            <p class="text-center text-gray-500 py-8">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

// =============================================================================
// NotesListPage — observation notes from API
// =============================================================================

#[component]
pub fn NotesListPage() -> impl IntoView {
    let notes = LocalResource::new(|| {
        crate::api::get::<Vec<DailyObservation>>("/api/observability")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-xl font-bold text-gray-900">"관찰 기록"</h1>
                <a
                    href="/caregiver/notes/new"
                    class="px-4 py-2 bg-teal-600 text-white text-sm font-medium rounded-lg hover:bg-teal-700"
                >"새 기록"</a>
            </div>

            <Suspense fallback=move || view! {
                <div class="animate-pulse space-y-3">
                    <div class="bg-gray-200 rounded-xl h-20" />
                    <div class="bg-gray-200 rounded-xl h-20" />
                </div>
            }>
                {move || Suspend::new(async move {
                    match notes.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-gray-500 py-8">"관찰 기록이 없습니다."</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|obs| {
                                            let date_str = crate::api::format_datetime_kr(&obs.created_at);
                                            let category = format!("{}", obs.category);
                                            let preview = obs.value.clone();
                                            let notes_text = obs.notes.clone().unwrap_or_default();
                                            let display = if notes_text.is_empty() { preview } else { format!("{} — {}", preview, notes_text) };
                                            view! {
                                                <NoteCard
                                                    date=date_str
                                                    client="기록"
                                                    preview=display
                                                    category=category
                                                />
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-gray-500 py-8">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

// =============================================================================
// NoteNewPage — create observation note form with client list from API
// =============================================================================

#[component]
pub fn NoteNewPage() -> impl IntoView {
    let selected_client = RwSignal::new(String::new());
    let selected_category = RwSignal::new(String::new());
    let content = RwSignal::new(String::new());
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    let categories = vec!["건강", "식사", "정서", "활동", "기타"];

    // Fetch clients from API for the dropdown
    let clients_data = LocalResource::new(|| {
        crate::api::get::<Vec<PersonProfile>>("/api/profile/seniors?caregiver=me")
    });

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
                // Client select — populated from API
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"고객 선택"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <Suspense fallback=move || view! {
                        <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm bg-white" disabled>
                            <option>"불러오는 중..."</option>
                        </select>
                    }>
                        {move || Suspend::new(async move {
                            match clients_data.await {
                                Ok(resp) if resp.success => {
                                    let profiles = resp.data.unwrap_or_default();
                                    view! {
                                        <select
                                            class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 bg-white"
                                            on:change=move |ev| selected_client.set(event_target_value(&ev))
                                        >
                                            <option value="" disabled selected>"고객을 선택하세요"</option>
                                            {profiles.into_iter().map(|p| {
                                                let name = format!("{}님", p.korean_name.unwrap_or_else(|| "이름 없음".to_string()));
                                                let name2 = name.clone();
                                                view! { <option value=name>{name2}</option> }
                                            }).collect_view()}
                                        </select>
                                    }.into_any()
                                }
                                _ => view! {
                                    <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm bg-white">
                                        <option>"고객 목록을 불러올 수 없습니다"</option>
                                    </select>
                                }.into_any(),
                            }
                        })}
                    </Suspense>
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

            // Error message
            {move || error_msg.get().map(|msg| view! {
                <p class="text-sm text-red-600 text-center">{msg}</p>
            })}

            <button
                class="w-full py-4 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700 disabled:opacity-50"
                disabled=move || submitting.get()
                on:click=move |_| {
                    let client = selected_client.get();
                    let category = selected_category.get();
                    let note_content = content.get();
                    leptos::task::spawn_local(async move {
                        submitting.set(true);
                        error_msg.set(None);
                        let body = serde_json::json!({
                            "client": client,
                            "category": category,
                            "content": note_content,
                        });
                        match crate::api::post::<DailyObservation, _>("/api/observability", &body).await {
                            Ok(resp) if resp.success => {
                                if let Some(window) = leptos::web_sys::window() {
                                    let _ = window.location().set_href("/caregiver/notes");
                                }
                            }
                            Ok(resp) => error_msg.set(resp.error),
                            Err(e) => error_msg.set(Some(e)),
                        }
                        submitting.set(false);
                    });
                }
            >
                {move || if submitting.get() { "처리 중..." } else { "기록 저장" }}
            </button>
        </div>
    }
}

// =============================================================================
// IncidentPage — incident report form with client list from API
// =============================================================================

#[component]
pub fn IncidentPage() -> impl IntoView {
    let selected_client = RwSignal::new(String::new());
    let incident_type = RwSignal::new(String::new());
    let severity = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let actions_taken = RwSignal::new(String::new());
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);
    let success_msg = RwSignal::new(None::<String>);

    let incident_types = vec!["낙상", "투약 오류", "피부 손상", "행동 변화", "기타"];
    let severity_levels: Vec<(&str, &str)> = vec![
        ("경미", "bg-green-100 text-green-700 border-green-300"),
        ("보통", "bg-yellow-100 text-yellow-700 border-yellow-300"),
        ("심각", "bg-orange-100 text-orange-700 border-orange-300"),
        ("응급", "bg-red-100 text-red-700 border-red-300"),
    ];

    // Fetch clients from API for the dropdown
    let clients_data = LocalResource::new(|| {
        crate::api::get::<Vec<PersonProfile>>("/api/profile/seniors?caregiver=me")
    });

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
                // Client select — from API
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"고객"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <Suspense fallback=move || view! {
                        <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm bg-white" disabled>
                            <option>"불러오는 중..."</option>
                        </select>
                    }>
                        {move || Suspend::new(async move {
                            match clients_data.await {
                                Ok(resp) if resp.success => {
                                    let profiles = resp.data.unwrap_or_default();
                                    view! {
                                        <select
                                            class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500 bg-white"
                                            on:change=move |ev| selected_client.set(event_target_value(&ev))
                                        >
                                            <option value="" disabled selected>"고객을 선택하세요"</option>
                                            {profiles.into_iter().map(|p| {
                                                let name = format!("{}님", p.korean_name.unwrap_or_else(|| "이름 없음".to_string()));
                                                let name2 = name.clone();
                                                view! { <option value=name>{name2}</option> }
                                            }).collect_view()}
                                        </select>
                                    }.into_any()
                                }
                                _ => view! {
                                    <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm bg-white">
                                        <option>"고객 목록을 불러올 수 없습니다"</option>
                                    </select>
                                }.into_any(),
                            }
                        })}
                    </Suspense>
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

            // Feedback messages
            {move || error_msg.get().map(|msg| view! {
                <p class="text-sm text-red-600 text-center">{msg}</p>
            })}
            {move || success_msg.get().map(|msg| view! {
                <p class="text-sm text-green-600 text-center">{msg}</p>
            })}

            <button
                class="w-full py-4 bg-red-600 text-white font-semibold rounded-xl hover:bg-red-700 disabled:opacity-50"
                disabled=move || submitting.get()
                on:click=move |_| {
                    let client = selected_client.get();
                    let itype = incident_type.get();
                    let sev = severity.get();
                    let desc = description.get();
                    let actions = actions_taken.get();
                    leptos::task::spawn_local(async move {
                        submitting.set(true);
                        error_msg.set(None);
                        success_msg.set(None);
                        let body = serde_json::json!({
                            "client": client,
                            "incident_type": itype,
                            "severity": sev,
                            "description": desc,
                            "actions_taken": actions,
                        });
                        match crate::api::post::<Incident, _>("/api/incidents", &body).await {
                            Ok(resp) if resp.success => {
                                success_msg.set(Some("보고서가 제출되었습니다.".to_string()));
                            }
                            Ok(resp) => error_msg.set(resp.error),
                            Err(e) => error_msg.set(Some(e)),
                        }
                        submitting.set(false);
                    });
                }
            >
                {move || if submitting.get() { "처리 중..." } else { "보고서 제출" }}
            </button>
        </div>
    }
}
