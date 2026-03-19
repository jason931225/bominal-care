use leptos::prelude::*;
use bominal_types::ObservabilitySignal;

// =============================================================================
// Government Portal Pages — 9 components
// =============================================================================

/// Dashboard summary from /api/observability/dashboard.
#[derive(Debug, Clone, serde::Deserialize)]
struct DashboardSummary {
    total_visits_today: i64,
    total_medications_today: i64,
    total_incidents: i64,
    recent_signals: Vec<ObservabilitySignal>,
}

/// Government overview dashboard displaying key oversight metrics such as
/// eligibility determinations, managed institutions, programs, and observation signals.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let dashboard = LocalResource::new(|| {
        crate::api::get::<DashboardSummary>("/api/observability/dashboard")
    });

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"Bominal Gov 관리 대시보드"</h1>
                <p class="text-sm text-txt-secondary mt-1">"관할 지역 돌봄 현황을 모니터링하세요."</p>
            </div>

            <Suspense fallback=move || view! {
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                        <p class="text-sm text-txt-tertiary">"등급 판정 건수"</p>
                        <div class="mt-1"><div class="skeleton h-8 w-16"></div></div>
                    </div>
                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                        <p class="text-sm text-txt-tertiary">"관할 기관"</p>
                        <div class="mt-1"><div class="skeleton h-8 w-16"></div></div>
                    </div>
                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                        <p class="text-sm text-txt-tertiary">"프로그램"</p>
                        <div class="mt-1"><div class="skeleton h-8 w-16"></div></div>
                    </div>
                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                        <p class="text-sm text-txt-tertiary">"관찰 신호"</p>
                        <div class="mt-1"><div class="skeleton h-8 w-16"></div></div>
                    </div>
                </div>
            }>
                {move || Suspend::new(async move {
                    match dashboard.await {
                        Ok(resp) if resp.success => {
                            let summary = resp.data.unwrap_or(DashboardSummary {
                                total_visits_today: 0,
                                total_medications_today: 0,
                                total_incidents: 0,
                                recent_signals: vec![],
                            });
                            view! {
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                        <p class="text-sm text-txt-tertiary">"등급 판정 건수"</p>
                                        <p class="text-2xl font-bold text-txt-primary mt-1">{summary.total_medications_today.to_string()}</p>
                                        <p class="text-xs text-[var(--portal-accent)] mt-1">"이번 분기"</p>
                                    </div>
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                        <p class="text-sm text-txt-tertiary">"관할 기관"</p>
                                        <p class="text-2xl font-bold text-txt-primary mt-1">"34"</p>
                                        <p class="text-xs text-txt-tertiary mt-1">"활동 중"</p>
                                    </div>
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                        <p class="text-sm text-txt-tertiary">"프로그램"</p>
                                        <p class="text-2xl font-bold text-txt-primary mt-1">"8"</p>
                                        <p class="text-xs text-txt-tertiary mt-1">"운영 중"</p>
                                    </div>
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                        <p class="text-sm text-txt-tertiary">"관찰 신호"</p>
                                        <p class="text-2xl font-bold text-txt-primary mt-1">{summary.recent_signals.len().to_string()}</p>
                                        <p class="text-xs text-txt-tertiary mt-1">"사고: "{summary.total_incidents.to_string()}" 건"</p>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        Ok(resp) => view! { <p class="text-danger">{resp.error.unwrap_or_default()}</p> }.into_any(),
                        Err(e) => view! { <p class="text-danger">{e}</p> }.into_any(),
                    }
                })}
            </Suspense>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <h2 class="font-semibold text-txt-primary mb-3">"기관 현황"</h2>
                    <div class="space-y-2 text-sm">
                        <div class="flex justify-between"><span class="text-txt-secondary">"정상 운영"</span><span class="font-medium text-success">"31 기관"</span></div>
                        <div class="flex justify-between"><span class="text-txt-secondary">"주의 필요"</span><span class="font-medium text-warning">"2 기관"</span></div>
                        <div class="flex justify-between"><span class="text-txt-secondary">"시정 조치 중"</span><span class="font-medium text-danger">"1 기관"</span></div>
                    </div>
                    <a href="/gov/providers" class="text-sm text-[var(--portal-accent)] hover:underline mt-3 inline-block">"전체 보기 →"</a>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <h2 class="font-semibold text-txt-primary mb-3">"최근 감사 로그"</h2>
                    <p class="text-sm text-txt-tertiary">"최근 감사 기록이 여기에 표시됩니다."</p>
                    <a href="/gov/audit" class="text-sm text-[var(--portal-accent)] hover:underline mt-3 inline-block">"감사 로그 →"</a>
                </div>
            </div>
        </div>
    }
}

/// Eligibility cases list page with status filters for managing
/// long-term care grade determination cases.
#[component]
pub fn EligibilityListPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"등급 판정 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"장기요양 등급 판정 사례를 관리하세요."</p>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Eligibility case detail page showing case information and
/// multi-step approval workflow for grade determination.
#[component]
pub fn EligibilityDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"등급 판정 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"판정 사례의 상세 정보와 승인 단계입니다."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex justify-between items-center">
                    <h2 class="font-semibold text-txt-primary">"사례 정보"</h2>
                    <span class="text-xs px-2 py-1 rounded-full bg-warning-light text-warning">"심사 중"</span>
                </div>
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div><p class="text-txt-tertiary">"프로그램"</p><p class="font-medium text-txt-primary mt-1">"장기요양보험"</p></div>
                    <div><p class="text-txt-tertiary">"신청일"</p><p class="font-medium text-txt-primary mt-1">"2026-03-01"</p></div>
                    <div><p class="text-txt-tertiary">"신청인"</p><p class="font-medium text-txt-primary mt-1">"홍길동"</p></div>
                    <div><p class="text-txt-tertiary">"판정 예정일"</p><p class="font-medium text-txt-primary mt-1">"2026-03-30"</p></div>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-4">"승인 단계"</h2>
                <div class="space-y-4">
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-full bg-success-light flex items-center justify-center"><span class="text-success text-sm font-bold">"1"</span></div>
                        <div class="flex-1"><p class="text-sm font-medium text-txt-primary">"서류 접수"</p><p class="text-xs text-success">"완료 — 2026-03-01"</p></div>
                    </div>
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-full bg-[var(--portal-accent-light)] flex items-center justify-center"><span class="text-[var(--portal-accent)] text-sm font-bold">"2"</span></div>
                        <div class="flex-1"><p class="text-sm font-medium text-txt-primary">"방문 조사"</p><p class="text-xs text-[var(--portal-accent)]">"진행 중"</p></div>
                    </div>
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-full bg-surface-subtle flex items-center justify-center"><span class="text-txt-disabled text-sm font-bold">"3"</span></div>
                        <div class="flex-1"><p class="text-sm font-medium text-txt-disabled">"등급 판정 위원회"</p><p class="text-xs text-txt-disabled">"대기"</p></div>
                    </div>
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-full bg-surface-subtle flex items-center justify-center"><span class="text-txt-disabled text-sm font-bold">"4"</span></div>
                        <div class="flex-1"><p class="text-sm font-medium text-txt-disabled">"결과 통보"</p><p class="text-xs text-txt-disabled">"대기"</p></div>
                    </div>
                </div>
            </div>

            <div class="flex gap-3">
                <button class="flex-1 bg-success text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-success/90 active:scale-[0.98] transition-all">"승인"</button>
                <button class="flex-1 border border-danger/30 text-danger rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-danger-light active:scale-[0.98] transition-all">"반려"</button>
            </div>
        </div>
    }
}

/// Registered providers list page showing all care institutions
/// under jurisdiction with search, type filtering, and status indicators.
#[component]
pub fn ProvidersPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"기관 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"관할 지역 등록 기관 현황입니다."</p>
            </div>

            <div class="flex gap-2 mb-4">
                <input type="text" placeholder="기관명 검색..." class="border border-gray-200 rounded-xl px-3 py-2 text-sm flex-1 focus:ring-[var(--portal-accent)]/30" />
                <select class="border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30">
                    <option value="">"전체 유형"</option>
                    <option value="home_care">"재가요양"</option>
                    <option value="facility">"시설요양"</option>
                    <option value="day_care">"주야간보호"</option>
                </select>
            </div>

            <div class="bg-surface-card rounded-2xl shadow-sm overflow-hidden">
                <table class="w-full text-sm">
                    <thead class="bg-surface-subtle border-b border-gray-100">
                        <tr>
                            <th class="text-left px-4 py-3 font-medium text-txt-tertiary">"기관명"</th>
                            <th class="text-left px-4 py-3 font-medium text-txt-tertiary">"유형"</th>
                            <th class="text-left px-4 py-3 font-medium text-txt-tertiary">"이용자"</th>
                            <th class="text-left px-4 py-3 font-medium text-txt-tertiary">"등급"</th>
                            <th class="text-left px-4 py-3 font-medium text-txt-tertiary">"상태"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-100">
                        <tr class="hover:bg-surface-subtle">
                            <td class="px-4 py-3 font-medium text-txt-primary">"행복재가센터"</td>
                            <td class="px-4 py-3 text-txt-secondary">"재가요양"</td>
                            <td class="px-4 py-3 text-txt-secondary">"47명"</td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"A등급"</span></td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"정상"</span></td>
                        </tr>
                        <tr class="hover:bg-surface-subtle">
                            <td class="px-4 py-3 font-medium text-txt-primary">"사랑요양원"</td>
                            <td class="px-4 py-3 text-txt-secondary">"시설요양"</td>
                            <td class="px-4 py-3 text-txt-secondary">"120명"</td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"A등급"</span></td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"정상"</span></td>
                        </tr>
                        <tr class="hover:bg-surface-subtle">
                            <td class="px-4 py-3 font-medium text-txt-primary">"은빛주야간보호"</td>
                            <td class="px-4 py-3 text-txt-secondary">"주야간보호"</td>
                            <td class="px-4 py-3 text-txt-secondary">"35명"</td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-warning-light text-warning">"B등급"</span></td>
                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-warning-light text-warning">"주의"</span></td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

/// Government programs overview page listing all active senior care programs
/// with enrollment counts and institution statistics.
#[component]
pub fn ProgramsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"정부 프로그램"</h1>
                <p class="text-sm text-txt-secondary mt-1">"운영 중인 돌봄 프로그램 현황입니다."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <div class="flex justify-between items-start">
                        <h3 class="font-semibold text-txt-primary">"노인장기요양보험"</h3>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"운영 중"</span>
                    </div>
                    <p class="text-sm text-txt-tertiary mt-2">"65세 이상 또는 노인성 질환자 대상 요양 서비스"</p>
                    <div class="flex gap-4 mt-3 text-xs text-txt-tertiary">
                        <span>"수급자: 1,247명"</span>
                        <span>"기관: 34개"</span>
                    </div>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <div class="flex justify-between items-start">
                        <h3 class="font-semibold text-txt-primary">"노인돌봄종합서비스"</h3>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"운영 중"</span>
                    </div>
                    <p class="text-sm text-txt-tertiary mt-2">"장기요양 등급 외 돌봄 필요 노인 대상 서비스"</p>
                    <div class="flex gap-4 mt-3 text-xs text-txt-tertiary">
                        <span>"이용자: 523명"</span>
                        <span>"기관: 18개"</span>
                    </div>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <div class="flex justify-between items-start">
                        <h3 class="font-semibold text-txt-primary">"치매안심센터"</h3>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"운영 중"</span>
                    </div>
                    <p class="text-sm text-txt-tertiary mt-2">"치매 조기 발견, 예방, 관리 통합 서비스"</p>
                    <div class="flex gap-4 mt-3 text-xs text-txt-tertiary">
                        <span>"등록자: 892명"</span>
                        <span>"센터: 5개"</span>
                    </div>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <div class="flex justify-between items-start">
                        <h3 class="font-semibold text-txt-primary">"응급안전안심서비스"</h3>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"운영 중"</span>
                    </div>
                    <p class="text-sm text-txt-tertiary mt-2">"독거노인 대상 ICT 기반 응급 알림 서비스"</p>
                    <div class="flex gap-4 mt-3 text-xs text-txt-tertiary">
                        <span>"이용자: 1,834명"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// System-wide observability signals page for monitoring platform health,
/// anomalies, and operational metrics across all connected institutions.
#[component]
pub fn ObservabilityPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"시스템 모니터링"</h1>
                <p class="text-sm text-txt-secondary mt-1">"시스템 전체 관찰 신호를 확인하세요."</p>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Audit log viewer page for reviewing system activity records,
/// compliance events, and administrative actions.
#[component]
pub fn AuditPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"감사 로그"</h1>
                <p class="text-sm text-txt-secondary mt-1">"시스템 활동 기록을 조회하세요."</p>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Government portal settings page for managing jurisdiction info,
/// notification preferences, data exports, and account actions.
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"관리 설정"</h1>
                <p class="text-sm text-txt-secondary mt-1">"Bominal Gov 포털 운영 설정을 관리하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">"관할 정보"</h2>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between"><span class="text-txt-tertiary">"관할 지역"</span><span class="text-txt-primary">"서울특별시"</span></div>
                    <div class="flex justify-between"><span class="text-txt-tertiary">"담당 부서"</span><span class="text-txt-primary">"복지정책과"</span></div>
                    <div class="flex justify-between"><span class="text-txt-tertiary">"담당자"</span><span class="text-txt-primary">"박주무관"</span></div>
                    <div class="flex justify-between"><span class="text-txt-tertiary">"연락처"</span><span class="text-txt-primary">"02-2133-XXXX"</span></div>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">"알림 설정"</h2>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between items-center">
                        <span class="text-txt-secondary">"긴급 사고 알림"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"활성"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-txt-secondary">"기관 시정 조치 알림"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"활성"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-txt-secondary">"월간 통계 보고"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"활성"</span>
                    </div>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">"데이터 관리"</h2>
                <div class="space-y-3">
                    <button class="w-full text-left text-sm text-[var(--portal-accent)] hover:underline">"감사 로그 내보내기 (CSV) →"</button>
                    <button class="w-full text-left text-sm text-[var(--portal-accent)] hover:underline">"기관 현황 보고서 다운로드 →"</button>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">"계정"</h2>
                <button
                    class="text-sm text-danger hover:underline"
                    on:click=move |_| {
                        leptos::task::spawn_local(async move {
                            let _ = crate::api::post_no_body("/api/auth/logout").await;
                            if let Some(window) = leptos::web_sys::window() {
                                let _ = window.location().set_href("/auth/signin");
                            }
                        });
                    }
                >"로그아웃"</button>
            </div>
        </div>
    }
}
