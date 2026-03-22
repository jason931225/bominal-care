use leptos::prelude::*;
use bominal_types::Visit;

use super::demo_visit_id;

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

    let visits = LocalResource::new(|| crate::api::get::<Vec<Visit>>("/api/visits"));

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
            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
            }>
                {move || Suspend::new(async move {
                    match visits.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-gray-500 py-8">"오늘 예정된 방문이 없습니다."</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|visit| {
                                            let id = visit.id.to_string();
                                            let time = format!(
                                                "{} - {}",
                                                visit.scheduled_start.format("%H:%M"),
                                                visit.scheduled_end.format("%H:%M")
                                            );
                                            let status = format!("{}", visit.status);
                                            view! {
                                                <ScheduleVisitCard
                                                    id=id
                                                    time=time
                                                    client="고객"
                                                    service="방문요양"
                                                    status=status
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

#[component]
fn ScheduleVisitCard(
    #[prop(into)] id: String,
    #[prop(into)] time: String,
    #[prop(into)] client: String,
    #[prop(into)] service: String,
    #[prop(into)] status: String,
) -> impl IntoView {
    let badge_cls = match status.as_str() {
        "완료" | "Completed" => "text-xs font-medium px-2 py-0.5 rounded-full bg-green-100 text-green-700",
        "진행중" | "InProgress" => "text-xs font-medium px-2 py-0.5 rounded-full bg-blue-100 text-blue-700",
        _ => "text-xs font-medium px-2 py-0.5 rounded-full bg-gray-100 text-gray-600",
    };
    let href = format!("/caregiver/schedule/{id}");

    view! {
        <a href=href class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
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
                <a href={format!("/caregiver/check-in/{}", demo_visit_id())} class="flex-1 text-center py-3 bg-teal-600 text-white font-medium rounded-xl hover:bg-teal-700">"체크인"</a>
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
                            class=("border-teal-600", move || is_checked.get())
                            class=("bg-teal-600", move || is_checked.get())
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
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    // Extract visit id from URL path — use a placeholder for now; in production
    // this would come from a route param.
    let visit_id = demo_visit_id();

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

            // Error message
            {move || error_msg.get().map(|msg| view! {
                <p class="text-sm text-red-600 text-center">{msg}</p>
            })}

            // Check-in button
            {
                let visit_id = visit_id.clone();
                view! {
                    <button
                        class="w-full py-4 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700 disabled:opacity-50"
                        disabled=move || !is_verified.get() || submitting.get()
                        on:click=move |_| {
                            let vid = visit_id.clone();
                            leptos::task::spawn_local(async move {
                                submitting.set(true);
                                error_msg.set(None);
                                let body = serde_json::json!({"latitude": 0.0, "longitude": 0.0});
                                let url = format!("/api/visits/{}/check-in", vid);
                                match crate::api::post::<Visit, _>(&url, &body).await {
                                    Ok(resp) if resp.success => {
                                        if let Some(window) = leptos::web_sys::window() {
                                            let _ = window.location().set_href("/caregiver/schedule");
                                        }
                                    }
                                    Ok(resp) => error_msg.set(resp.error),
                                    Err(e) => error_msg.set(Some(e)),
                                }
                                submitting.set(false);
                            });
                        }
                    >
                        {move || if submitting.get() { "처리 중..." } else { "체크인 완료" }}
                    </button>
                }
            }
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
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    let visit_id = demo_visit_id();

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
                        let on_click = {
                            let opt_str = opt_str.clone();
                            move |_| condition.set(opt_str.clone())
                        };
                        let s1 = opt_str.clone();
                        let s2 = opt_str.clone();
                        let s3 = opt_str.clone();
                        let s4 = opt_str.clone();
                        let s5 = opt_str.clone();
                        let s6 = opt_str;
                        view! {
                            <button
                                class="flex-1 py-2 text-sm rounded-lg border transition-colors"
                                class=("bg-teal-600", move || condition.get() == s1)
                                class=("text-white", move || condition.get() == s2)
                                class=("border-teal-600", move || condition.get() == s3)
                                class=("bg-white", move || condition.get() != s4)
                                class=("text-gray-700", move || condition.get() != s5)
                                class=("border-gray-300", move || condition.get() != s6)
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

            // Error message
            {move || error_msg.get().map(|msg| view! {
                <p class="text-sm text-red-600 text-center">{msg}</p>
            })}

            {
                let visit_id = visit_id.clone();
                view! {
                    <button
                        class="w-full py-4 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700 disabled:opacity-50"
                        disabled=move || submitting.get()
                        on:click=move |_| {
                            let vid = visit_id.clone();
                            let notes_val = notes.get();
                            let cond_val = condition.get();
                            leptos::task::spawn_local(async move {
                                submitting.set(true);
                                error_msg.set(None);
                                let body = serde_json::json!({
                                    "latitude": 0.0,
                                    "longitude": 0.0,
                                    "notes": notes_val,
                                    "condition": cond_val,
                                });
                                let url = format!("/api/visits/{}/check-out", vid);
                                match crate::api::post::<Visit, _>(&url, &body).await {
                                    Ok(resp) if resp.success => {
                                        if let Some(window) = leptos::web_sys::window() {
                                            let _ = window.location().set_href("/caregiver/schedule");
                                        }
                                    }
                                    Ok(resp) => error_msg.set(resp.error),
                                    Err(e) => error_msg.set(Some(e)),
                                }
                                submitting.set(false);
                            });
                        }
                    >
                        {move || if submitting.get() { "처리 중..." } else { "체크아웃 완료" }}
                    </button>
                }
            }
        </div>
    }
}
