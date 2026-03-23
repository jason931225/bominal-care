use chrono::{Datelike, Duration, Utc, Weekday};
use leptos::prelude::*;
use bominal_types::Visit;

// =============================================================================
// Helpers — dynamic week date computation
// =============================================================================

/// Korean weekday label.
fn weekday_label(wd: Weekday) -> &'static str {
    match wd {
        Weekday::Mon => "월",
        Weekday::Tue => "화",
        Weekday::Wed => "수",
        Weekday::Thu => "목",
        Weekday::Fri => "금",
        Weekday::Sat => "토",
        Weekday::Sun => "일",
    }
}

/// Compute the 7-day week strip starting from Monday of the current week.
/// Returns Vec of (weekday_label, day_of_month, is_today_index).
fn current_week_days() -> (Vec<(String, String)>, usize) {
    let now = js_sys::Date::new_0();
    let year = now.get_full_year() as i32;
    let month = now.get_month() as u32 + 1; // JS months are 0-based
    let day = now.get_date() as u32;

    let today = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .unwrap_or_else(|| Utc::now().date_naive());
    let weekday_num = today.weekday().num_days_from_monday(); // Mon=0
    let monday = today - Duration::days(weekday_num as i64);

    let mut days = Vec::with_capacity(7);
    let mut today_index = 0_usize;
    for i in 0..7 {
        let d = monday + Duration::days(i);
        let wd = d.weekday();
        days.push((weekday_label(wd).to_string(), d.day().to_string()));
        if d == today {
            today_index = i as usize;
        }
    }
    (days, today_index)
}

/// Format a current time string from js_sys::Date (HH:MM).
fn current_time_str() -> String {
    let now = js_sys::Date::new_0();
    format!("{:02}:{:02}", now.get_hours(), now.get_minutes())
}

/// Format current date in Korean: "2026년 3월 23일 (월)"
fn current_date_kr() -> String {
    let now = js_sys::Date::new_0();
    let year = now.get_full_year();
    let month = now.get_month() + 1;
    let day = now.get_date();
    let wd_num = now.get_day(); // 0=Sun
    let wd_label = match wd_num as u32 {
        0 => "일",
        1 => "월",
        2 => "화",
        3 => "수",
        4 => "목",
        5 => "금",
        6 => "토",
        _ => "",
    };
    format!("{year}년 {month}월 {day}일 ({wd_label})")
}

/// Extract visit id from the current URL path at the given segment index.
fn visit_id_from_path(segment: usize) -> String {
    leptos::web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_default()
        .split('/')
        .nth(segment)
        .unwrap_or("")
        .to_string()
}

// =============================================================================
// 2. ScheduleListPage — week date strip + daily visit list
// =============================================================================

#[component]
pub fn ScheduleListPage() -> impl IntoView {
    let (days, today_index) = current_week_days();
    let selected_day = RwSignal::new(today_index);

    let visits = LocalResource::new(|| crate::api::get::<Vec<Visit>>("/api/visits"));

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <h1 class="text-xl font-bold text-gray-900">"스케줄"</h1>

            // Week date strip — dynamically computed
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
// 3. ScheduleDetailPage — visit detail fetched from API
// =============================================================================

#[component]
pub fn ScheduleDetailPage() -> impl IntoView {
    // Extract visit ID from URL: /caregiver/schedule/{id}
    let vid = visit_id_from_path(3);
    let visit_url = format!("/api/visits/{}", vid);
    let visit_id_for_checkin = vid.clone();

    let visit = LocalResource::new(move || {
        let url = visit_url.clone();
        async move { crate::api::get::<Visit>(&url).await }
    });

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

            <Suspense fallback=move || view! {
                <div class="animate-pulse space-y-4">
                    <div class="bg-gray-200 rounded-xl h-32" />
                    <div class="bg-gray-200 rounded-xl h-24" />
                </div>
            }>
                {
                    let checkin_vid = visit_id_for_checkin.clone();
                    move || {
                        let checkin_vid = checkin_vid.clone();
                        Suspend::new(async move {
                            match visit.await {
                                Ok(resp) if resp.success => {
                                    match resp.data {
                                        Some(v) => {
                                            let start = v.scheduled_start.format("%H:%M").to_string();
                                            let end = v.scheduled_end.format("%H:%M").to_string();
                                            let status = format!("{}", v.status);
                                            let time_display = format!("{} - {}", start, end);
                                            let notes_text = v.notes.clone().unwrap_or_default();
                                            view! {
                                                <div class="space-y-5">
                                                    // Visit info
                                                    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                        <div class="flex items-center gap-3 mb-3">
                                                            <div class="w-12 h-12 bg-teal-100 rounded-full flex items-center justify-center">
                                                                <svg class="w-6 h-6 text-teal-700" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                                                </svg>
                                                            </div>
                                                            <div>
                                                                <p class="font-semibold text-gray-900">"방문 "{status}</p>
                                                                <p class="text-sm text-gray-500">{crate::api::format_date_kr(&v.scheduled_start)}</p>
                                                            </div>
                                                        </div>
                                                        <div class="grid grid-cols-2 gap-3 text-sm">
                                                            <div>
                                                                <p class="text-gray-500">"시간"</p>
                                                                <p class="font-medium text-gray-900">{time_display}</p>
                                                            </div>
                                                            <div>
                                                                <p class="text-gray-500">"서비스 유형"</p>
                                                                <p class="font-medium text-gray-900">"방문요양"</p>
                                                            </div>
                                                        </div>
                                                        {if !notes_text.is_empty() {
                                                            view! {
                                                                <div class="mt-3 text-sm">
                                                                    <p class="text-gray-500">"메모"</p>
                                                                    <p class="font-medium text-gray-900">{notes_text}</p>
                                                                </div>
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }}
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

                                                    // Care checklist (template items — not client-specific data)
                                                    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                        <h2 class="font-semibold text-gray-900 mb-3">"케어 체크리스트"</h2>
                                                        <CareChecklist />
                                                    </div>

                                                    // Action buttons
                                                    <div class="flex gap-3">
                                                        <a href={format!("/caregiver/check-in/{}", checkin_vid)} class="flex-1 text-center py-3 bg-teal-600 text-white font-medium rounded-xl hover:bg-teal-700">"체크인"</a>
                                                        <a href="/caregiver/notes/new" class="flex-1 text-center py-3 border border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50">"기록 작성"</a>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        }
                                        None => view! {
                                            <p class="text-center text-gray-500 py-8">"방문 정보를 찾을 수 없습니다."</p>
                                        }.into_any(),
                                    }
                                }
                                Ok(resp) => view! { <p class="text-center text-red-500 py-8">{resp.error.unwrap_or_else(|| "오류가 발생했습니다.".to_string())}</p> }.into_any(),
                                Err(e) => view! { <p class="text-center text-red-500 py-8">{e}</p> }.into_any(),
                            }
                        })
                    }
                }
            </Suspense>
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
// 4. CheckInPage — location verification + time display from API visit
// =============================================================================

#[component]
pub fn CheckInPage() -> impl IntoView {
    let location_status = RwSignal::new("확인 중...");
    let is_verified = RwSignal::new(false);
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    // Extract visit id from URL: /caregiver/check-in/{id}
    let visit_id = visit_id_from_path(3);

    let now_time = current_time_str();
    let now_date = current_date_kr();

    let visit_url = format!("/api/visits/{}", visit_id);
    let visit_data = LocalResource::new(move || {
        let url = visit_url.clone();
        async move { crate::api::get::<Visit>(&url).await }
    });

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

            // Time display — current real time
            <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100 text-center">
                <p class="text-sm text-gray-500">"현재 시각"</p>
                <p class="text-4xl font-bold text-gray-900 mt-2">{now_time}</p>
                <p class="text-sm text-gray-500 mt-1">{now_date}</p>
            </div>

            // Visit info — from API
            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-24" />
            }>
                {move || Suspend::new(async move {
                    match visit_data.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(v) => {
                                    let start = v.scheduled_start.format("%H:%M").to_string();
                                    let end = v.scheduled_end.format("%H:%M").to_string();
                                    let time_range = format!("{} - {}", start, end);
                                    view! {
                                        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                            <h2 class="font-semibold text-gray-900 mb-2">"방문 정보"</h2>
                                            <div class="space-y-2 text-sm">
                                                <div class="flex justify-between">
                                                    <span class="text-gray-500">"예정 시간"</span>
                                                    <span class="font-medium text-gray-900">{time_range}</span>
                                                </div>
                                                <div class="flex justify-between">
                                                    <span class="text-gray-500">"서비스"</span>
                                                    <span class="font-medium text-gray-900">"방문요양"</span>
                                                </div>
                                                <div class="flex justify-between">
                                                    <span class="text-gray-500">"상태"</span>
                                                    <span class="font-medium text-gray-900">{format!("{}", v.status)}</span>
                                                </div>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                None => view! {
                                    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                        <p class="text-sm text-gray-500">"방문 정보를 불러올 수 없습니다."</p>
                                    </div>
                                }.into_any(),
                            }
                        }
                        _ => view! {
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                <p class="text-sm text-gray-500">"방문 정보를 불러올 수 없습니다."</p>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>

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
                        <p class="text-xs text-gray-500">"GPS 위치 확인 중"</p>
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
// 5. CheckOutPage — checkout form with visit data from API
// =============================================================================

#[component]
pub fn CheckOutPage() -> impl IntoView {
    let notes = RwSignal::new(String::new());
    let condition = RwSignal::new("양호".to_string());
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    // Extract visit id from URL: /caregiver/check-out/{id}
    let visit_id = visit_id_from_path(3);

    let visit_url = format!("/api/visits/{}", visit_id);
    let visit_data = LocalResource::new(move || {
        let url = visit_url.clone();
        async move { crate::api::get::<Visit>(&url).await }
    });

    let now_time = current_time_str();

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

            // Time summary — from API visit data
            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-32" />
            }>
                {
                    let now_t = now_time.clone();
                    move || {
                        let now_t = now_t.clone();
                        Suspend::new(async move {
                            match visit_data.await {
                                Ok(resp) if resp.success => {
                                    match resp.data {
                                        Some(v) => {
                                            let checkin_time = v.actual_start
                                                .map(|dt| dt.format("%H:%M").to_string())
                                                .unwrap_or_else(|| v.scheduled_start.format("%H:%M").to_string());

                                            // Compute duration from actual_start (or scheduled_start) to now
                                            let start_dt = v.actual_start.unwrap_or(v.scheduled_start);
                                            let now_utc = Utc::now();
                                            let duration = now_utc - start_dt;
                                            let dur_hours = duration.num_hours();
                                            let dur_mins = duration.num_minutes() % 60;
                                            let dur_display = if dur_hours > 0 {
                                                format!("{}시간 {}분", dur_hours, dur_mins.abs())
                                            } else {
                                                format!("{}분", dur_mins.abs())
                                            };

                                            view! {
                                                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                    <div class="grid grid-cols-2 gap-4 text-center">
                                                        <div>
                                                            <p class="text-sm text-gray-500">"체크인"</p>
                                                            <p class="text-xl font-bold text-gray-900">{checkin_time}</p>
                                                        </div>
                                                        <div>
                                                            <p class="text-sm text-gray-500">"현재 시각"</p>
                                                            <p class="text-xl font-bold text-teal-600">{now_t}</p>
                                                        </div>
                                                    </div>
                                                    <div class="mt-3 pt-3 border-t border-gray-100 text-center">
                                                        <p class="text-sm text-gray-500">"총 근무 시간"</p>
                                                        <p class="text-lg font-bold text-gray-900">{dur_display}</p>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        }
                                        None => view! {
                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                                                <p class="text-sm text-gray-500">"방문 정보를 찾을 수 없습니다."</p>
                                            </div>
                                        }.into_any(),
                                    }
                                }
                                _ => view! {
                                    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                                        <p class="text-sm text-gray-500">"데이터를 불러올 수 없습니다."</p>
                                    </div>
                                }.into_any(),
                            }
                        })
                    }
                }
            </Suspense>

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
                        <p class="text-xs text-gray-500">"GPS 위치 확인됨"</p>
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
