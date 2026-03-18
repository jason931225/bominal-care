use leptos::prelude::*;
use crate::server_fns::*;

// =============================================================================
// Government Portal Pages — 8 components
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

/// Government overview dashboard with key oversight metrics.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let stats = Resource::new(|| (), |_| observability::get_dashboard_stats());
    let cases = Resource::new(|| (), |_| eligibility::list_eligibility_cases(
        None, None, 1, 5,
    ));

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"정부 관리 대시보드"</h1>
                <p class="text-sm text-gray-600 mt-1">"관할 지역 돌봄 현황을 모니터링하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"등급 판정 건수"</p>
                    <Suspense fallback=move || view! { <p class="text-2xl font-bold text-gray-300">"..."</p> }>
                        {move || cases.get().map(|res| match res {
                            Ok(list) => view! { <p class="text-2xl font-bold text-gray-900 mt-1">{list.total}</p> }.into_any(),
                            Err(_) => view! { <p class="text-2xl font-bold text-red-400 mt-1">"오류"</p> }.into_any(),
                        })}
                    </Suspense>
                    <p class="text-xs text-blue-600 mt-1">"이번 분기"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"관할 기관"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"34"</p>
                    <p class="text-xs text-gray-500 mt-1">"활동 중"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"프로그램"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"8"</p>
                    <p class="text-xs text-gray-500 mt-1">"운영 중"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"관찰 신호"</p>
                    <Suspense fallback=move || view! { <p class="text-2xl font-bold text-gray-300">"..."</p> }>
                        {move || stats.get().map(|res| match res {
                            Ok(s) => view! {
                                <p class="text-2xl font-bold text-gray-900 mt-1">{s.total}</p>
                                <p class="text-xs text-orange-600 mt-1">"미확인: "{s.unacknowledged}</p>
                            }.into_any(),
                            Err(_) => view! { <p class="text-2xl font-bold text-red-400 mt-1">"오류"</p> }.into_any(),
                        })}
                    </Suspense>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <h2 class="font-semibold text-gray-900 mb-3">"기관 현황"</h2>
                    <div class="space-y-2 text-sm">
                        <div class="flex justify-between"><span class="text-gray-600">"정상 운영"</span><span class="font-medium text-green-600">"31 기관"</span></div>
                        <div class="flex justify-between"><span class="text-gray-600">"주의 필요"</span><span class="font-medium text-yellow-600">"2 기관"</span></div>
                        <div class="flex justify-between"><span class="text-gray-600">"시정 조치 중"</span><span class="font-medium text-red-600">"1 기관"</span></div>
                    </div>
                    <a href="/government/providers" class="text-sm text-blue-600 hover:underline mt-3 inline-block">"전체 보기 →"</a>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <h2 class="font-semibold text-gray-900 mb-3">"최근 감사 로그"</h2>
                    <p class="text-sm text-gray-500">"최근 감사 기록이 여기에 표시됩니다."</p>
                    <a href="/government/audit" class="text-sm text-blue-600 hover:underline mt-3 inline-block">"감사 로그 →"</a>
                </div>
            </div>
        </div>
    }
}

/// Eligibility cases list with status filters.
#[component]
pub fn EligibilityListPage() -> impl IntoView {
    let (status_filter, set_status_filter) = signal(String::new());

    let cases = Resource::new(
        move || status_filter.get(),
        move |filter| {
            let status = if filter.is_empty() { None } else { Some(filter) };
            eligibility::list_eligibility_cases(status, None, 1, 20)
        },
    );

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"등급 판정 관리"</h1>
                <p class="text-sm text-gray-600 mt-1">"장기요양 등급 판정 사례를 관리하세요."</p>
            </div>

            <div class="flex gap-2">
                <button
                    class={move || format!("px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {}", if status_filter.get().is_empty() { "bg-blue-600 text-white" } else { "bg-gray-100 text-gray-600 hover:bg-gray-200" })}
                    on:click=move |_| set_status_filter.set(String::new())
                >"전체"</button>
                <button
                    class={move || format!("px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {}", if status_filter.get() == "submitted" { "bg-blue-600 text-white" } else { "bg-gray-100 text-gray-600 hover:bg-gray-200" })}
                    on:click=move |_| set_status_filter.set("submitted".to_string())
                >"접수됨"</button>
                <button
                    class={move || format!("px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {}", if status_filter.get() == "under_review" { "bg-blue-600 text-white" } else { "bg-gray-100 text-gray-600 hover:bg-gray-200" })}
                    on:click=move |_| set_status_filter.set("under_review".to_string())
                >"심사 중"</button>
                <button
                    class={move || format!("px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {}", if status_filter.get() == "approved" { "bg-blue-600 text-white" } else { "bg-gray-100 text-gray-600 hover:bg-gray-200" })}
                    on:click=move |_| set_status_filter.set("approved".to_string())
                >"승인됨"</button>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || cases.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"해당 건이 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                                    <table class="w-full text-sm">
                                        <thead class="bg-gray-50 border-b border-gray-100">
                                            <tr>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"프로그램"</th>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"상태"</th>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"신청일"</th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-gray-100">
                                            {list.data.into_iter().map(|c| view! {
                                                <tr class="hover:bg-gray-50 cursor-pointer">
                                                    <td class="px-4 py-3">
                                                        <a href={format!("/government/eligibility/{}", c.id)} class="font-medium text-gray-900 hover:text-blue-600">{c.program_name.clone()}</a>
                                                    </td>
                                                    <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-blue-50 text-blue-700">{c.status.to_string()}</span></td>
                                                    <td class="px-4 py-3 text-gray-600">{c.created_at.format("%Y-%m-%d").to_string()}</td>
                                                </tr>
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                    <div class="px-4 py-3 bg-gray-50 border-t border-gray-100 text-sm text-gray-500">"총 "{list.total}" 건"</div>
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"데이터를 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Eligibility case detail with approval steps.
#[component]
pub fn EligibilityDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-6 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"등급 판정 상세"</h1>
                <p class="text-sm text-gray-600 mt-1">"판정 사례의 상세 정보와 승인 단계입니다."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="flex justify-between items-center">
                    <h2 class="font-semibold text-gray-900">"사례 정보"</h2>
                    <span class="text-xs px-2 py-1 rounded-full bg-yellow-50 text-yellow-700">"심사 중"</span>
                </div>
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div><p class="text-gray-500">"프로그램"</p><p class="font-medium text-gray-900 mt-1">"장기요양보험"</p></div>
                    <div><p class="text-gray-500">"신청일"</p><p class="font-medium text-gray-900 mt-1">"2026-03-01"</p></div>
                    <div><p class="text-gray-500">"신청인"</p><p class="font-medium text-gray-900 mt-1">"홍길동"</p></div>
                    <div><p class="text-gray-500">"판정 예정일"</p><p class="font-medium text-gray-900 mt-1">"2026-03-30"</p></div>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-4">"승인 단계"</h2>
                <div class="space-y-4">
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-full bg-green-100 flex items-center justify-center"><span class="text-green-600 text-sm font-bold">"1"</span></div>
                        <div class="flex-1"><p class="text-sm font-medium text-gray-900">"서류 접수"</p><p class="text-xs text-green-600">"완료 — 2026-03-01"</p></div>
                    </div>
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center"><span class="text-blue-600 text-sm font-bold">"2"</span></div>
                        <div class="flex-1"><p class="text-sm font-medium text-gray-900">"방문 조사"</p><p class="text-xs text-blue-600">"진행 중"</p></div>
                    </div>
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-full bg-gray-100 flex items-center justify-center"><span class="text-gray-400 text-sm font-bold">"3"</span></div>
                        <div class="flex-1"><p class="text-sm font-medium text-gray-400">"등급 판정 위원회"</p><p class="text-xs text-gray-400">"대기"</p></div>
                    </div>
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-full bg-gray-100 flex items-center justify-center"><span class="text-gray-400 text-sm font-bold">"4"</span></div>
                        <div class="flex-1"><p class="text-sm font-medium text-gray-400">"결과 통보"</p><p class="text-xs text-gray-400">"대기"</p></div>
                    </div>
                </div>
            </div>

            <div class="flex gap-3">
                <button class="flex-1 bg-green-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-green-700 transition-colors">"승인"</button>
                <button class="flex-1 border border-red-300 text-red-600 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-red-50 transition-colors">"반려"</button>
            </div>
        </div>
    }
}

/// Registered providers list.
#[component]
pub fn ProvidersPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"기관 관리"</h1>
                <p class="text-sm text-gray-600 mt-1">"관할 지역 등록 기관 현황입니다."</p>
            </div>

            <div class="flex gap-2 mb-4">
                <input type="text" placeholder="기관명 검색..." class="border border-gray-300 rounded-lg px-3 py-2 text-sm flex-1" />
                <select class="border border-gray-300 rounded-lg px-3 py-2 text-sm">
                    <option value="">"전체 유형"</option>
                    <option value="home_care">"재가요양"</option>
                    <option value="facility">"시설요양"</option>
                    <option value="day_care">"주야간보호"</option>
                </select>
            </div>

            <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                <table class="w-full text-sm">
                    <thead class="bg-gray-50 border-b border-gray-100">
                        <tr>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"기관명"</th>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"유형"</th>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"이용자"</th>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"등급"</th>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"상태"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-100">
                        <tr class="hover:bg-gray-50">
                            <td class="px-4 py-3 font-medium text-gray-900">"행복재가센터"</td>
                            <td class="px-4 py-3 text-gray-600">"재가요양"</td>
                            <td class="px-4 py-3 text-gray-600">"47명"</td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"A등급"</span></td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"정상"</span></td>
                        </tr>
                        <tr class="hover:bg-gray-50">
                            <td class="px-4 py-3 font-medium text-gray-900">"사랑요양원"</td>
                            <td class="px-4 py-3 text-gray-600">"시설요양"</td>
                            <td class="px-4 py-3 text-gray-600">"120명"</td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"A등급"</span></td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"정상"</span></td>
                        </tr>
                        <tr class="hover:bg-gray-50">
                            <td class="px-4 py-3 font-medium text-gray-900">"은빛주야간보호"</td>
                            <td class="px-4 py-3 text-gray-600">"주야간보호"</td>
                            <td class="px-4 py-3 text-gray-600">"35명"</td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-yellow-50 text-yellow-700">"B등급"</span></td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-yellow-50 text-yellow-700">"주의"</span></td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

/// Government programs overview.
#[component]
pub fn ProgramsPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"정부 프로그램"</h1>
                <p class="text-sm text-gray-600 mt-1">"운영 중인 돌봄 프로그램 현황입니다."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <div class="flex justify-between items-start">
                        <h3 class="font-semibold text-gray-900">"노인장기요양보험"</h3>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"운영 중"</span>
                    </div>
                    <p class="text-sm text-gray-500 mt-2">"65세 이상 또는 노인성 질환자 대상 요양 서비스"</p>
                    <div class="flex gap-4 mt-3 text-xs text-gray-500">
                        <span>"수급자: 1,247명"</span>
                        <span>"기관: 34개"</span>
                    </div>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <div class="flex justify-between items-start">
                        <h3 class="font-semibold text-gray-900">"노인돌봄종합서비스"</h3>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"운영 중"</span>
                    </div>
                    <p class="text-sm text-gray-500 mt-2">"장기요양 등급 외 돌봄 필요 노인 대상 서비스"</p>
                    <div class="flex gap-4 mt-3 text-xs text-gray-500">
                        <span>"이용자: 523명"</span>
                        <span>"기관: 18개"</span>
                    </div>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <div class="flex justify-between items-start">
                        <h3 class="font-semibold text-gray-900">"치매안심센터"</h3>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"운영 중"</span>
                    </div>
                    <p class="text-sm text-gray-500 mt-2">"치매 조기 발견, 예방, 관리 통합 서비스"</p>
                    <div class="flex gap-4 mt-3 text-xs text-gray-500">
                        <span>"등록자: 892명"</span>
                        <span>"센터: 5개"</span>
                    </div>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <div class="flex justify-between items-start">
                        <h3 class="font-semibold text-gray-900">"응급안전안심서비스"</h3>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"운영 중"</span>
                    </div>
                    <p class="text-sm text-gray-500 mt-2">"독거노인 대상 ICT 기반 응급 알림 서비스"</p>
                    <div class="flex gap-4 mt-3 text-xs text-gray-500">
                        <span>"이용자: 1,834명"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// System-wide observability signals.
#[component]
pub fn ObservabilityPage() -> impl IntoView {
    let signals = Resource::new(|| (), |_| observability::list_signals(
        None, None, None, None, 1, 30,
    ));
    let stats = Resource::new(|| (), |_| observability::get_dashboard_stats());

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"시스템 모니터링"</h1>
                <p class="text-sm text-gray-600 mt-1">"시스템 전체 관찰 신호를 확인하세요."</p>
            </div>

            <Suspense fallback=move || view! { <div></div> }>
                {move || stats.get().map(|res| match res {
                    Ok(s) => view! {
                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                <p class="text-sm text-gray-500">"전체 신호"</p>
                                <p class="text-2xl font-bold text-gray-900 mt-1">{s.total}</p>
                            </div>
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                <p class="text-sm text-gray-500">"미확인"</p>
                                <p class="text-2xl font-bold text-orange-600 mt-1">{s.unacknowledged}</p>
                            </div>
                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                <p class="text-sm text-gray-500">"긴급"</p>
                                <p class="text-2xl font-bold text-red-600 mt-1">{s.by_severity.len()}</p>
                            </div>
                        </div>
                    }.into_any(),
                    Err(_) => view! { <p class="text-sm text-red-500">"통계를 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"신호를 불러오는 중..."</p> }>
                {move || signals.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"신호가 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3">
                                    {list.data.into_iter().map(|s| view! {
                                        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                            <div class="flex justify-between items-start">
                                                <div>
                                                    <p class="text-sm font-medium text-gray-900">{s.message.clone()}</p>
                                                    <p class="text-xs text-gray-500 mt-1">{s.event_type.to_string()}" · "{s.created_at.format("%Y-%m-%d %H:%M").to_string()}</p>
                                                </div>
                                                <div class="flex gap-2 items-center">
                                                    <span class={format!("text-xs px-2 py-1 rounded-full {}",
                                                        match s.severity.to_string().as_str() {
                                                            "critical" => "bg-red-50 text-red-700",
                                                            "high" => "bg-orange-50 text-orange-700",
                                                            "medium" => "bg-yellow-50 text-yellow-700",
                                                            _ => "bg-gray-50 text-gray-700",
                                                        }
                                                    )}>{s.severity.to_string()}</span>
                                                    {if s.acknowledged_at.is_none() {
                                                        view! { <button class="text-xs text-blue-600 hover:underline">"확인"</button> }.into_any()
                                                    } else {
                                                        view! { <span class="text-xs text-green-600">"확인됨"</span> }.into_any()
                                                    }}
                                                </div>
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"신호를 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Audit log viewer.
#[component]
pub fn AuditPage() -> impl IntoView {
    let (action_filter, set_action_filter) = signal(String::new());

    let logs = Resource::new(
        move || action_filter.get(),
        move |filter| {
            let action = if filter.is_empty() { None } else { Some(filter) };
            audit::list_audit_logs(action, None, 1, 30)
        },
    );

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"감사 로그"</h1>
                <p class="text-sm text-gray-600 mt-1">"시스템 활동 기록을 조회하세요."</p>
            </div>

            <div class="flex gap-2">
                <select
                    class="border border-gray-300 rounded-lg px-3 py-2 text-sm"
                    on:change=move |ev| set_action_filter.set(event_target_value(&ev))
                >
                    <option value="">"전체 활동"</option>
                    <option value="create">"생성"</option>
                    <option value="update">"수정"</option>
                    <option value="delete">"삭제"</option>
                    <option value="login">"로그인"</option>
                </select>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || logs.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"로그가 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                                    <table class="w-full text-sm">
                                        <thead class="bg-gray-50 border-b border-gray-100">
                                            <tr>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"시간"</th>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"활동"</th>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"대상"</th>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"사용자"</th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-gray-100">
                                            {list.data.into_iter().map(|log| view! {
                                                <tr class="hover:bg-gray-50">
                                                    <td class="px-4 py-3 text-gray-600 whitespace-nowrap">{log.created_at.format("%m/%d %H:%M").to_string()}</td>
                                                    <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-blue-50 text-blue-700">{log.action.to_string()}</span></td>
                                                    <td class="px-4 py-3 text-gray-600">{log.entity_type.clone()}</td>
                                                    <td class="px-4 py-3 text-gray-600">{log.user_id.map(|id| id.to_string()).unwrap_or_else(|| "시스템".into())}</td>
                                                </tr>
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                    <div class="px-4 py-3 bg-gray-50 border-t border-gray-100 text-sm text-gray-500">"총 "{list.total}" 건"</div>
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"로그를 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Government portal settings.
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="space-y-6 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"관리 설정"</h1>
                <p class="text-sm text-gray-600 mt-1">"정부 포털 운영 설정을 관리하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="font-semibold text-gray-900">"관할 정보"</h2>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between"><span class="text-gray-500">"관할 지역"</span><span class="text-gray-900">"서울특별시"</span></div>
                    <div class="flex justify-between"><span class="text-gray-500">"담당 부서"</span><span class="text-gray-900">"복지정책과"</span></div>
                    <div class="flex justify-between"><span class="text-gray-500">"담당자"</span><span class="text-gray-900">"박주무관"</span></div>
                    <div class="flex justify-between"><span class="text-gray-500">"연락처"</span><span class="text-gray-900">"02-2133-XXXX"</span></div>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="font-semibold text-gray-900">"알림 설정"</h2>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"긴급 사고 알림"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"활성"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"기관 시정 조치 알림"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"활성"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"월간 통계 보고"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"활성"</span>
                    </div>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="font-semibold text-gray-900">"데이터 관리"</h2>
                <div class="space-y-3">
                    <button class="w-full text-left text-sm text-blue-600 hover:underline">"감사 로그 내보내기 (CSV) →"</button>
                    <button class="w-full text-left text-sm text-blue-600 hover:underline">"기관 현황 보고서 다운로드 →"</button>
                </div>
            </div>
        </div>
    }
}
