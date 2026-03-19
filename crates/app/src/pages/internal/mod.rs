use leptos::prelude::*;
use uuid::Uuid;
use bominal_types::ObservabilitySignal;

fn demo_client_id() -> String {
    Uuid::new_v5(&Uuid::NAMESPACE_OID, b"demo-client-hong").to_string()
}

// =============================================================================
// Internal (Provider) Portal Pages — 20 components
// =============================================================================

/// Dashboard summary from /api/observability/dashboard.
#[derive(Debug, Clone, serde::Deserialize)]
struct DashboardSummary {
    total_visits_today: i64,
    total_medications_today: i64,
    total_incidents: i64,
    recent_signals: Vec<ObservabilitySignal>,
}


/// Provider dashboard with operational metrics.
/// Shows key KPIs (medications, signals, visits, incidents) loaded from the
/// observability API, plus quick-links to schedule conflicts and referrals.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let dashboard = LocalResource::new(|| {
        crate::api::get::<DashboardSummary>("/api/observability/dashboard")
    });

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"내부 관리 대시보드"</h1>
                <p class="text-sm text-txt-secondary mt-1">"기관 운영 현황을 관리하세요."</p>
            </div>

            <Suspense fallback=move || view! {
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                        <p class="text-sm text-txt-tertiary">"이용자 수"</p>
                        <div class="mt-1"><div class="skeleton h-8 w-16"></div></div>
                    </div>
                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                        <p class="text-sm text-txt-tertiary">"요양보호사"</p>
                        <div class="mt-1"><div class="skeleton h-8 w-16"></div></div>
                    </div>
                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                        <p class="text-sm text-txt-tertiary">"오늘 방문"</p>
                        <div class="mt-1"><div class="skeleton h-8 w-16"></div></div>
                    </div>
                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                        <p class="text-sm text-txt-tertiary">"사고 건수"</p>
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
                                        <p class="text-sm text-txt-tertiary">"이용자 수"</p>
                                        <p class="text-2xl font-bold text-txt-primary mt-1">{summary.total_medications_today.to_string()}</p>
                                        <p class="text-xs text-txt-tertiary mt-1">"복약 건수 (오늘)"</p>
                                    </div>
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                        <p class="text-sm text-txt-tertiary">"최근 신호"</p>
                                        <p class="text-2xl font-bold text-txt-primary mt-1">{summary.recent_signals.len().to_string()}</p>
                                        <p class="text-xs text-txt-tertiary mt-1">"관찰 신호"</p>
                                    </div>
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                        <p class="text-sm text-txt-tertiary">"오늘 방문"</p>
                                        <p class="text-2xl font-bold text-txt-primary mt-1">{summary.total_visits_today.to_string()}</p>
                                        <p class="text-xs text-[var(--portal-accent)] mt-1">"방문 건수"</p>
                                    </div>
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                        <p class="text-sm text-txt-tertiary">"사고 건수"</p>
                                        <p class="text-2xl font-bold text-txt-primary mt-1">{summary.total_incidents.to_string()}</p>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        Ok(resp) => view! { <p class="text-red-500">{resp.error.unwrap_or_default()}</p> }.into_any(),
                        Err(e) => view! { <p class="text-red-500">{e}</p> }.into_any(),
                    }
                })}
            </Suspense>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                    <h2 class="font-semibold text-txt-primary mb-3">"일정 충돌"</h2>
                    <p class="text-sm text-txt-tertiary">"현재 일정 충돌이 없습니다."</p>
                    <a href="/internal/schedules/conflicts" class="text-sm text-[var(--portal-accent)] hover:underline mt-2 inline-block">"상세 보기 →"</a>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                    <h2 class="font-semibold text-txt-primary mb-3">"최근 의뢰"</h2>
                    <p class="text-sm text-txt-tertiary">"새로운 의뢰가 없습니다."</p>
                    <a href="/internal/referrals" class="text-sm text-[var(--portal-accent)] hover:underline mt-2 inline-block">"전체 보기 →"</a>
                </div>
            </div>
        </div>
    }
}

/// Paginated client list.
/// Displays all registered clients for the provider organization.
#[component]
pub fn ClientsListPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"이용자 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"등록된 이용자 목록입니다."</p>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Client detail with care plan summary.
/// Shows personal info and an overview of the assigned care plan for a
/// single client, with a link to the full care plan management page.
#[component]
pub fn ClientDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"이용자 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"이용자 정보와 케어 플랜을 확인하세요."</p>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                    <h2 class="font-semibold text-txt-primary">"기본 정보"</h2>
                    <div class="space-y-2 text-sm">
                        <div class="flex justify-between"><span class="text-txt-tertiary">"이름"</span><span class="text-txt-primary">"홍길동"</span></div>
                        <div class="flex justify-between"><span class="text-txt-tertiary">"등급"</span><span class="text-txt-primary">"3등급"</span></div>
                        <div class="flex justify-between"><span class="text-txt-tertiary">"연락처"</span><span class="text-txt-primary">"010-1234-5678"</span></div>
                        <div class="flex justify-between"><span class="text-txt-tertiary">"주소"</span><span class="text-txt-primary">"서울시 강남구"</span></div>
                    </div>
                </div>
                <div class="lg:col-span-2 bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                    <div class="flex justify-between items-center">
                        <h2 class="font-semibold text-txt-primary">"케어 플랜"</h2>
                        <a href={format!("/internal/clients/{}/care-plan", demo_client_id())} class="text-sm text-[var(--portal-accent)] hover:underline">"관리 →"</a>
                    </div>
                    <div class="bg-surface-subtle rounded-xl p-4">
                        <p class="text-sm font-medium text-txt-primary">"방문요양 주 3회"</p>
                        <p class="text-xs text-txt-tertiary mt-1">"월, 수, 금 10:00 - 12:00"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Care plan management for a client.
/// Displays the current active care plan with service details, schedule,
/// and goals. Provides navigation to the edit form.
#[component]
pub fn ClientCarePlanPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-xl font-bold text-txt-primary">"케어 플랜 관리"</h1>
                    <p class="text-sm text-txt-secondary mt-1">"이용자의 케어 플랜을 관리하세요."</p>
                </div>
                <a href={format!("/internal/clients/{}/care-plan/edit", demo_client_id())} class="bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">"수정"</a>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex justify-between items-center">
                    <h2 class="font-semibold text-txt-primary">"현재 플랜"</h2>
                    <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"활성"</span>
                </div>
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div><p class="text-txt-tertiary">"서비스"</p><p class="font-medium text-txt-primary mt-1">"방문요양"</p></div>
                    <div><p class="text-txt-tertiary">"빈도"</p><p class="font-medium text-txt-primary mt-1">"주 3회"</p></div>
                    <div><p class="text-txt-tertiary">"시작일"</p><p class="font-medium text-txt-primary mt-1">"2026-01-01"</p></div>
                    <div><p class="text-txt-tertiary">"종료일"</p><p class="font-medium text-txt-primary mt-1">"2026-12-31"</p></div>
                </div>
                <div>
                    <p class="text-sm text-txt-tertiary">"목표"</p>
                    <ul class="mt-2 space-y-1 text-sm text-txt-secondary list-disc list-inside">
                        <li>"일상생활 지원 (식사, 이동)"</li>
                        <li>"사회 참여 증진"</li>
                        <li>"인지 기능 유지"</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

/// Edit care plan form.
/// Provides fields for modifying the plan title, description, and date
/// range, with save and cancel actions.
#[component]
pub fn ClientCarePlanEditPage() -> impl IntoView {
    let (title, set_title) = signal(String::from("방문요양 주 3회"));
    let (description, set_description) = signal(String::new());
    let (start_date, set_start_date) = signal(String::from("2026-01-01"));
    let (end_date, set_end_date) = signal(String::from("2026-12-31"));

    view! {
        <div class="space-y-8 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"케어 플랜 수정"</h1>
                <p class="text-sm text-txt-secondary mt-1">"케어 플랜 내용을 수정하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"플랜 제목"</label>
                    <input type="text" class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30" prop:value=move || title.get() on:input=move |ev| set_title.set(event_target_value(&ev)) />
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"설명"</label>
                    <textarea rows=3 class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30" prop:value=move || description.get() on:input=move |ev| set_description.set(event_target_value(&ev))></textarea>
                </div>
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <label class="block text-sm font-medium text-txt-secondary mb-1">"시작일"</label>
                        <input type="date" class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30" prop:value=move || start_date.get() on:input=move |ev| set_start_date.set(event_target_value(&ev)) />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-txt-secondary mb-1">"종료일"</label>
                        <input type="date" class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30" prop:value=move || end_date.get() on:input=move |ev| set_end_date.set(event_target_value(&ev)) />
                    </div>
                </div>
                <div class="flex gap-3 pt-2">
                    <button class="flex-1 bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">"저장"</button>
                    <a href={format!("/internal/clients/{}/care-plan", demo_client_id())} class="flex-1 text-center border border-gray-200 text-txt-secondary rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-surface-subtle active:scale-[0.98] transition-all">"취소"</a>
                </div>
            </div>
        </div>
    }
}

/// Approved caregivers list.
/// Shows all caregivers that have been approved to work under this provider.
#[component]
pub fn CaregiversListPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"요양보호사 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"승인된 요양보호사 목록입니다."</p>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Caregiver profile detail.
/// Displays the caregiver's profile photo placeholder, stats (status,
/// night availability, assigned clients, monthly visits), and certifications.
#[component]
pub fn CaregiverDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"요양보호사 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"요양보호사 프로필 정보입니다."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex items-center gap-4">
                    <div class="w-14 h-14 bg-portal-caregiver/10 rounded-full flex items-center justify-center">
                        <span class="text-xl font-bold text-portal-caregiver">"김"</span>
                    </div>
                    <div>
                        <p class="font-semibold text-txt-primary">"김요양"</p>
                        <p class="text-sm text-txt-tertiary">"경력 8년 · 치매 전문"</p>
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div><p class="text-txt-tertiary">"상태"</p><span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"승인됨"</span></div>
                    <div><p class="text-txt-tertiary">"야간 가능"</p><p class="font-medium text-txt-primary mt-1">"가능"</p></div>
                    <div><p class="text-txt-tertiary">"담당 이용자"</p><p class="font-medium text-txt-primary mt-1">"5명"</p></div>
                    <div><p class="text-txt-tertiary">"이번 달 방문"</p><p class="font-medium text-txt-primary mt-1">"18건"</p></div>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-3">"자격증"</h2>
                <div class="space-y-2">
                    <div class="flex justify-between text-sm">
                        <span class="text-txt-secondary">"요양보호사 자격증"</span>
                        <span class="text-txt-tertiary">"2018-06-15 취득"</span>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-txt-secondary">"치매 전문 교육 수료"</span>
                        <span class="text-txt-tertiary">"2023-03-20 취득"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Pending caregiver applications list.
/// Shows applications from caregivers awaiting review and approval.
#[component]
pub fn ApplicationsListPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"지원서 검토"</h1>
                <p class="text-sm text-txt-secondary mt-1">"대기 중인 요양보호사 지원서입니다."</p>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Review a single application.
/// Displays applicant qualifications, experience, self-introduction,
/// and verified credentials, with approve/reject actions.
#[component]
pub fn ApplicationDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"지원서 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"지원서를 검토하고 승인/반려하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div><p class="text-txt-tertiary">"경력"</p><p class="font-medium text-txt-primary mt-1">"5년"</p></div>
                    <div><p class="text-txt-tertiary">"전문분야"</p><p class="font-medium text-txt-primary mt-1">"방문요양, 방문목욕"</p></div>
                    <div><p class="text-txt-tertiary">"치매 경험"</p><p class="font-medium text-txt-primary mt-1">"있음"</p></div>
                    <div><p class="text-txt-tertiary">"야간 가능"</p><p class="font-medium text-txt-primary mt-1">"가능"</p></div>
                </div>
                <div>
                    <p class="text-sm text-txt-tertiary">"자기소개"</p>
                    <p class="text-sm text-txt-secondary mt-1">"5년간 방문요양 서비스를 제공하며 어르신들의 일상생활을 도왔습니다."</p>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-3">"자격증"</h2>
                <div class="space-y-2 text-sm">
                    <div class="flex justify-between"><span class="text-txt-secondary">"요양보호사 자격증"</span><span class="text-success">"확인됨"</span></div>
                </div>
            </div>

            <div class="flex gap-3">
                <button class="flex-1 bg-success text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">"승인"</button>
                <button class="flex-1 border border-danger text-danger rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-danger-light active:scale-[0.98] transition-all">"반려"</button>
            </div>
        </div>
    }
}

/// Compliance dashboard.
/// Summarizes regulatory compliance status including staffing ratios,
/// training completion, and documentation completeness.
#[component]
pub fn CompliancePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"컴플라이언스"</h1>
                <p class="text-sm text-txt-secondary mt-1">"규정 준수 현황을 확인하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">"인력 기준 충족"</p>
                    <p class="text-2xl font-bold text-success mt-1">"충족"</p>
                    <p class="text-xs text-txt-tertiary mt-1">"12/10 (최소 기준 대비)"</p>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">"교육 이수율"</p>
                    <p class="text-2xl font-bold text-[var(--portal-accent)] mt-1">"95%"</p>
                    <p class="text-xs text-txt-tertiary mt-1">"법정 의무교육"</p>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">"서류 완비율"</p>
                    <p class="text-2xl font-bold text-txt-primary mt-1">"100%"</p>
                    <p class="text-xs text-success mt-1">"모든 필수 서류 제출됨"</p>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-3">"주요 점검 항목"</h2>
                <ul class="space-y-2 text-sm">
                    <li class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-success"></span>"인력 배치 기준 — 충족"</li>
                    <li class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-success"></span>"시설 안전 점검 — 완료"</li>
                    <li class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-success"></span>"감염 예방 관리 — 정상"</li>
                    <li class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-warning"></span>"직원 건강검진 — 1명 미완료"</li>
                </ul>
            </div>
        </div>
    }
}

/// Quality metrics dashboard.
/// Displays service quality KPIs (overall score, satisfaction, punctuality,
/// incident rate) and a simple bar chart of monthly quality trends.
#[component]
pub fn QualityPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"품질 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"서비스 품질 지표를 모니터링하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">"종합 품질 점수"</p>
                    <p class="text-2xl font-bold text-success mt-1">"92"<span class="text-sm font-normal text-txt-tertiary">" / 100"</span></p>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">"이용자 만족도"</p>
                    <p class="text-2xl font-bold text-[var(--portal-accent)] mt-1">"4.6"<span class="text-sm font-normal text-txt-tertiary">" / 5"</span></p>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">"방문 정시율"</p>
                    <p class="text-2xl font-bold text-txt-primary mt-1">"96%"</p>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="text-sm text-txt-tertiary">"사고 발생률"</p>
                    <p class="text-2xl font-bold text-txt-primary mt-1">"0.3%"</p>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-3">"월별 품질 추이"</h2>
                <div class="h-40 flex items-end justify-around gap-2">
                    {["85","88","90","89","92","94","92"].into_iter().enumerate().map(|(i, val)| {
                        let months = ["9월","10월","11월","12월","1월","2월","3월"];
                        let height = format!("{}%", val.parse::<f64>().unwrap_or(0.0));
                        view! {
                            <div class="flex-1 flex flex-col items-center gap-1">
                                <div class="w-full bg-[var(--portal-accent-light)] rounded-t-sm" style={format!("height: {height}")}></div>
                                <span class="text-xs text-txt-tertiary">{months[i]}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

/// Incidents list.
/// Shows all reported incidents for the provider organization.
#[component]
pub fn IncidentsListPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"사고 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"발생한 사고를 관리하세요."</p>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Incident detail view.
/// Displays incident metadata (type, location, description, actions taken)
/// with resolution actions (mark complete or flag for follow-up).
#[component]
pub fn IncidentDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"사고 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"사고 내용과 조치 사항을 확인하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex justify-between items-center">
                    <span class="text-xs px-2 py-1 rounded-full bg-danger-light text-danger">"미처리"</span>
                    <p class="text-xs text-txt-tertiary">"2026-03-15 14:30"</p>
                </div>
                <div class="text-sm space-y-3">
                    <div><p class="text-txt-tertiary">"사고 유형"</p><p class="font-medium text-txt-primary mt-1">"낙상"</p></div>
                    <div><p class="text-txt-tertiary">"발생 장소"</p><p class="font-medium text-txt-primary mt-1">"이용자 자택 거실"</p></div>
                    <div><p class="text-txt-tertiary">"상세 내용"</p><p class="text-txt-secondary mt-1">"방문요양 중 이용자가 거실에서 미끄러져 경미한 타박상을 입었습니다."</p></div>
                    <div><p class="text-txt-tertiary">"조치 사항"</p><p class="text-txt-secondary mt-1">"즉시 응급처치 실시, 보호자 연락 완료"</p></div>
                </div>
            </div>

            <div class="flex gap-3">
                <button class="flex-1 bg-success text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">"처리 완료"</button>
                <button class="flex-1 border border-gray-200 text-txt-secondary rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-surface-subtle active:scale-[0.98] transition-all">"추가 조치 필요"</button>
            </div>
        </div>
    }
}

/// Referrals list.
/// Shows inter-organization referral status with a button to create new ones.
#[component]
pub fn ReferralsListPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-xl font-bold text-txt-primary">"의뢰 관리"</h1>
                    <p class="text-sm text-txt-secondary mt-1">"기관 간 의뢰 현황입니다."</p>
                </div>
                <a href="/internal/referrals/new" class="bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">"새 의뢰"</a>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Create new referral form.
/// Collects target organization, client, reason, and notes to submit
/// an inter-organization referral request.
#[component]
pub fn ReferralNewPage() -> impl IntoView {
    let (reason, set_reason) = signal(String::new());
    let (notes, set_notes) = signal(String::new());

    view! {
        <div class="space-y-8 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"새 의뢰 작성"</h1>
                <p class="text-sm text-txt-secondary mt-1">"다른 기관으로 의뢰서를 작성하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"의뢰 대상 기관"</label>
                    <select class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30">
                        <option value="">"기관을 선택하세요"</option>
                        <option value="1">"행복요양원"</option>
                        <option value="2">"사랑재가센터"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"의뢰 대상 이용자"</label>
                    <select class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30">
                        <option value="">"이용자를 선택하세요"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"의뢰 사유"</label>
                    <input type="text" class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30" placeholder="의뢰 사유를 입력하세요" prop:value=move || reason.get() on:input=move |ev| set_reason.set(event_target_value(&ev)) />
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"비고"</label>
                    <textarea rows=3 class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-[var(--portal-accent)]/30" placeholder="추가 사항을 적어주세요" prop:value=move || notes.get() on:input=move |ev| set_notes.set(event_target_value(&ev))></textarea>
                </div>
                <div class="flex gap-3 pt-2">
                    <button class="flex-1 bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all">"의뢰 전송"</button>
                    <a href="/internal/referrals" class="flex-1 text-center border border-gray-200 text-txt-secondary rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-surface-subtle active:scale-[0.98] transition-all">"취소"</a>
                </div>
            </div>
        </div>
    }
}

/// Reports dashboard.
/// Lists available report types (monthly operations, quality evaluation,
/// workforce management) and recently generated reports.
#[component]
pub fn ReportsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"보고서"</h1>
                <p class="text-sm text-txt-secondary mt-1">"운영 보고서를 생성하고 조회하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer">
                    <h3 class="font-semibold text-txt-primary">"월간 운영 보고서"</h3>
                    <p class="text-sm text-txt-tertiary mt-1">"이용자 현황, 방문 실적, 매출 요약"</p>
                    <button class="mt-3 text-sm text-[var(--portal-accent)] hover:underline">"생성하기 →"</button>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer">
                    <h3 class="font-semibold text-txt-primary">"품질 평가 보고서"</h3>
                    <p class="text-sm text-txt-tertiary mt-1">"서비스 품질, 만족도, 사고 현황"</p>
                    <button class="mt-3 text-sm text-[var(--portal-accent)] hover:underline">"생성하기 →"</button>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer">
                    <h3 class="font-semibold text-txt-primary">"인력 관리 보고서"</h3>
                    <p class="text-sm text-txt-tertiary mt-1">"요양보호사 근무 현황, 자격 관리"</p>
                    <button class="mt-3 text-sm text-[var(--portal-accent)] hover:underline">"생성하기 →"</button>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-3">"최근 생성 보고서"</h2>
                <p class="text-sm text-txt-tertiary">"생성된 보고서가 없습니다."</p>
            </div>
        </div>
    }
}

/// Visit schedules view.
/// Displays caregiver visit schedules for management and coordination.
#[component]
pub fn SchedulesPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"방문 일정"</h1>
                <p class="text-sm text-txt-secondary mt-1">"요양보호사 방문 일정을 관리하세요."</p>
            </div>
            <div class="skeleton h-8 w-full"></div>
        </div>
    }
}

/// Schedule conflicts view.
/// Shows overlapping visit schedules with a resolution guide, or confirms
/// that no conflicts exist.
#[component]
pub fn ScheduleConflictsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"일정 충돌"</h1>
                <p class="text-sm text-txt-secondary mt-1">"겹치는 방문 일정을 확인하고 해결하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <div class="flex items-center gap-3 text-sm text-success bg-success-light rounded-xl p-4">
                    <span class="w-2 h-2 rounded-full bg-success"></span>
                    "현재 일정 충돌이 없습니다."
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <h2 class="font-semibold text-txt-primary mb-3">"충돌 해결 가이드"</h2>
                <ul class="space-y-2 text-sm text-txt-secondary">
                    <li>"1. 충돌이 발생하면 관련 방문 일정이 표시됩니다."</li>
                    <li>"2. 담당 요양보호사를 변경하거나 시간을 조정하세요."</li>
                    <li>"3. 변경 사항은 관련자에게 자동 통보됩니다."</li>
                </ul>
            </div>
        </div>
    }
}

/// Provider settings page.
/// Manages organization info, operational toggles (auto-schedule alerts,
/// incident notifications, auto-reports), and account actions (logout).
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"기관 설정"</h1>
                <p class="text-sm text-txt-secondary mt-1">"기관 정보와 운영 설정을 관리하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">"기관 정보"</h2>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between"><span class="text-txt-tertiary">"기관명"</span><span class="text-txt-primary">"행복재가센터"</span></div>
                    <div class="flex justify-between"><span class="text-txt-tertiary">"기관 유형"</span><span class="text-txt-primary">"재가장기요양기관"</span></div>
                    <div class="flex justify-between"><span class="text-txt-tertiary">"기관 번호"</span><span class="text-txt-primary">"12345678"</span></div>
                    <div class="flex justify-between"><span class="text-txt-tertiary">"주소"</span><span class="text-txt-primary">"서울시 강남구 역삼동"</span></div>
                    <div class="flex justify-between"><span class="text-txt-tertiary">"대표 연락처"</span><span class="text-txt-primary">"02-1234-5678"</span></div>
                </div>
                <button class="w-full border border-gray-200 text-txt-secondary rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-surface-subtle active:scale-[0.98] transition-all">"정보 수정"</button>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">"운영 설정"</h2>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between items-center">
                        <span class="text-txt-secondary">"자동 일정 알림"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"활성"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-txt-secondary">"사고 즉시 알림"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"활성"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-txt-secondary">"월간 보고서 자동 생성"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-surface-subtle text-txt-secondary">"비활성"</span>
                    </div>
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
