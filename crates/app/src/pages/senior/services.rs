use leptos::prelude::*;

use crate::components::layout::PageHeader;
use super::{InfoRow, ServiceInfoItem};

/// Housing options list (static content).
#[component]
pub fn HousingPage() -> impl IntoView {
    let options = vec![
        ("senior-housing", "시니어 주택", "어르신 전용 주거 시설"),
        ("care-home", "요양원", "전문 간호 요양 시설"),
        ("group-home", "공동 생활 가정", "소규모 돌봄 시설"),
        ("day-care", "주간보호센터", "주간 돌봄 프로그램"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="주거 서비스" subtitle="다양한 주거 옵션을 살펴보세요" />
            <div class="space-y-3">
                {options.into_iter().map(|(slug, title, desc)| {
                    view! {
                        <a href=format!("/housing/{slug}")
                           class="block bg-surface-card rounded-2xl p-5 shadow-sm \
                                  hover:shadow-md transition-shadow duration-200">
                            <p class="text-lg font-medium text-txt-primary">{title}</p>
                            <p class="text-base text-txt-tertiary mt-1">{desc}</p>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Housing option detail.
#[component]
pub fn HousingDetailPage(
    #[prop(into)] housing_type: String,
) -> impl IntoView {
    let (title, description, features) = match housing_type.as_str() {
        "senior-housing" => (
            "시니어 주택",
            "어르신이 독립적으로 생활할 수 있는 전용 주거 시설입니다.",
            vec!["개인 공간 보장", "공동 활동 프로그램", "응급 호출 시스템", "식사 서비스"],
        ),
        "care-home" => (
            "요양원",
            "전문 간호 인력이 24시간 돌봄을 제공하는 시설입니다.",
            vec!["24시간 간호", "의료 서비스", "재활 프로그램", "여가 활동"],
        ),
        "group-home" => (
            "공동 생활 가정",
            "소규모 가정 환경에서 함께 생활하는 돌봄 시설입니다.",
            vec!["가정적 환경", "소규모 운영", "개별 맞춤 돌봄", "지역사회 연계"],
        ),
        _ => (
            "주간보호센터",
            "낮 시간 동안 다양한 프로그램과 돌봄을 제공합니다.",
            vec!["주간 돌봄", "식사 제공", "건강 체크", "사회 활동"],
        ),
    };

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/housing" class="text-primary text-lg">"< 주거 서비스"</a>
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h1 class="text-2xl font-bold text-txt-primary">{title}</h1>
                <p class="text-lg text-txt-secondary">{description}</p>
                <h2 class="text-xl font-semibold text-txt-primary">"주요 특징"</h2>
                <ul class="space-y-2">
                    {features.into_iter().map(|f| view! {
                        <li class="flex items-center gap-2 text-lg text-txt-secondary">
                            <span class="text-success font-bold">"·"</span>
                            {f}
                        </li>
                    }).collect_view()}
                </ul>
            </div>
        </div>
    }
}

/// Available services menu.
#[component]
pub fn ServicesPage() -> impl IntoView {
    let services = vec![
        ("/services/meals", "식사 배달", "정기적인 식사 배달 서비스", "\u{1f371}"),
        ("/services/partners", "협력 서비스", "파트너 기관 연계 서비스", "\u{1f91d}"),
        ("/services/rides", "이동 서비스", "병원 및 외출 교통 지원", "\u{1f697}"),
        ("/housing", "주거 서비스", "주거 관련 정보 및 지원", "\u{1f3e0}"),
        ("/opportunities", "사회 참여", "자원봉사 및 사회활동", "\u{1f31f}"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="서비스" subtitle="이용 가능한 서비스를 선택하세요" />
            <div class="space-y-3">
                {services.into_iter().map(|(href, title, desc, icon)| {
                    view! {
                        <a href=href
                           class="flex items-center gap-4 bg-surface-card rounded-2xl p-5 shadow-sm \
                                  hover:shadow-md transition-shadow duration-200">
                            <span class="text-3xl">{icon}</span>
                            <div>
                                <p class="text-lg font-medium text-txt-primary">{title}</p>
                                <p class="text-base text-txt-tertiary">{desc}</p>
                            </div>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Meal delivery service info.
#[component]
pub fn ServicesMealsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/services" class="text-primary text-lg">"< 서비스"</a>
            <PageHeader title="식사 배달" subtitle="정기적인 식사 배달 서비스" />
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="text-xl font-semibold text-txt-primary">"서비스 안내"</h2>
                <ul class="space-y-3">
                    <ServiceInfoItem label="배달 시간" value="오전 11:30 ~ 오후 12:30" />
                    <ServiceInfoItem label="식사 종류" value="한식 정식 (저염/저당 옵션)" />
                    <ServiceInfoItem label="배달 지역" value="서울 전 지역" />
                    <ServiceInfoItem label="이용 요금" value="1식 5,000원 (지원금 적용 가능)" />
                </ul>
            </div>
            <a href="tel:1588-9191"
               class="block w-full bg-primary text-white text-center text-lg font-semibold \
                      rounded-xl py-4 hover:bg-primary-hover active:scale-[0.98] transition-all">
                "신청 전화하기 (1588-9191)"
            </a>
        </div>
    }
}

/// Partner services list.
#[component]
pub fn ServicesPartnersPage() -> impl IntoView {
    let partners = vec![
        ("복지관 연계", "지역 복지관 프로그램 안내"),
        ("건강검진", "무료 건강검진 서비스"),
        ("법률 상담", "무료 법률 상담 지원"),
        ("심리 상담", "정신건강 상담 서비스"),
        ("일자리 지원", "시니어 일자리 매칭"),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/services" class="text-primary text-lg">"< 서비스"</a>
            <PageHeader title="협력 서비스" subtitle="파트너 기관 연계 서비스" />
            <div class="space-y-3">
                {partners.into_iter().map(|(title, desc)| {
                    view! {
                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                            <p class="text-lg font-medium text-txt-primary">{title}</p>
                            <p class="text-base text-txt-tertiary mt-1">{desc}</p>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Transport / ride services.
#[component]
pub fn ServicesRidesPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/services" class="text-primary text-lg">"< 서비스"</a>
            <PageHeader title="이동 서비스" subtitle="병원 방문 및 외출 교통 지원" />
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="text-xl font-semibold text-txt-primary">"이용 안내"</h2>
                <ul class="space-y-3">
                    <ServiceInfoItem label="이용 시간" value="오전 8:00 ~ 오후 6:00" />
                    <ServiceInfoItem label="예약 방법" value="최소 1일 전 전화 예약" />
                    <ServiceInfoItem label="이용 요금" value="편도 3,000원 (지원금 적용 가능)" />
                    <ServiceInfoItem label="이용 범위" value="병원, 관공서, 복지시설" />
                </ul>
            </div>
            <a href="tel:1588-9191"
               class="block w-full bg-primary text-white text-center text-lg font-semibold \
                      rounded-xl py-4 hover:bg-primary-hover active:scale-[0.98] transition-all">
                "예약 전화하기 (1588-9191)"
            </a>
        </div>
    }
}

// =============================================================================
// Community activities — API-driven with static fallback
// =============================================================================

/// Static fallback activities when the API returns empty or fails.
fn static_activities() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    vec![
        ("opp-1", "노인 대학", "평생학습 프로그램", "매주 화/목"),
        ("opp-2", "건강 체조", "어르신 건강 체조 모임", "매주 월/수/금"),
        ("opp-3", "봉사 활동", "지역사회 봉사 프로그램", "매월 둘째 토요일"),
        ("opp-4", "문화 교실", "서예, 그림, 음악 수업", "매주 수요일"),
        ("opp-5", "걷기 모임", "동네 산책 및 걷기 운동", "매일 오전 7시"),
    ]
}

/// Render a single activity card (shared by API and static paths).
fn activity_card_view(
    id: String,
    title: String,
    description: String,
    schedule: Option<String>,
    location: Option<String>,
) -> impl IntoView {
    view! {
        <a href=format!("/opportunities/{id}")
           class="block bg-surface-card rounded-2xl p-5 shadow-sm \
                  hover:shadow-md transition-shadow duration-200">
            <p class="text-lg font-medium text-txt-primary">{title}</p>
            <p class="text-base text-txt-tertiary mt-1">{description}</p>
            {schedule.map(|s| view! { <p class="text-base text-primary mt-1">{s}</p> })}
            {location.map(|l| view! {
                <p class="text-sm text-txt-tertiary mt-1">{format!("\u{1f4cd} {l}")}</p>
            })}
        </a>
    }
}

/// Volunteer and social opportunities list (API-driven with static fallback).
#[component]
pub fn OpportunitiesPage() -> impl IntoView {
    let activities = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/community/activities")
    });
    let holidays = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/community/holidays")
    });
    let alerts = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/community/alerts")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="사회 참여" subtitle="다양한 활동에 참여하세요" />

            // Active alerts banner
            <Suspense fallback=move || ()>
                {move || Suspend::new(async move {
                    let alert_items = match alerts.await {
                        Ok(resp) if resp.success => resp.data.unwrap_or_default(),
                        _ => vec![],
                    };
                    if alert_items.is_empty() {
                        view! { <div /> }.into_any()
                    } else {
                        view! {
                            <div class="space-y-2">
                                {alert_items.into_iter().map(|alert| {
                                    let title = alert.get("title")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("알림")
                                        .to_string();
                                    let desc = alert.get("description")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    view! {
                                        <div class="bg-yellow-50 border border-yellow-200 rounded-2xl p-4 flex items-start gap-3">
                                            <span class="text-xl shrink-0">{"\u{26a0}\u{fe0f}"}</span>
                                            <div>
                                                <p class="text-base font-semibold text-yellow-800">{title}</p>
                                                <p class="text-sm text-yellow-700 mt-0.5">{desc}</p>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>

            // Activities (API-driven with static fallback)
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    let api_items = match activities.await {
                        Ok(resp) if resp.success => resp.data.unwrap_or_default(),
                        _ => vec![],
                    };

                    if api_items.is_empty() {
                        // Fallback to static activities
                        view! {
                            <div class="space-y-3">
                                {static_activities().into_iter().map(|(id, title, desc, schedule)| {
                                    activity_card_view(
                                        id.to_string(),
                                        title.to_string(),
                                        desc.to_string(),
                                        Some(schedule.to_string()),
                                        None,
                                    )
                                }).collect_view()}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-3">
                                {api_items.into_iter().map(|item| {
                                    let id = item.get("id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown")
                                        .to_string();
                                    let title = item.get("title")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("활동")
                                        .to_string();
                                    let desc = item.get("description")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let schedule = item.get("schedule")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    let location = item.get("location")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    activity_card_view(id, title, desc, schedule, location)
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>

            // Upcoming holidays section
            <Suspense fallback=move || ()>
                {move || Suspend::new(async move {
                    let holiday_items = match holidays.await {
                        Ok(resp) if resp.success => resp.data.unwrap_or_default(),
                        _ => vec![],
                    };
                    if holiday_items.is_empty() {
                        view! { <div /> }.into_any()
                    } else {
                        view! {
                            <section class="space-y-3">
                                <h2 class="text-xl font-semibold text-txt-primary">
                                    "다가오는 공휴일"
                                </h2>
                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                    <ul class="space-y-2">
                                        {holiday_items.into_iter().map(|h| {
                                            let date = h.get("date")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                            let name = h.get("name")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("공휴일")
                                                .to_string();
                                            view! {
                                                <li class="flex items-center justify-between py-2 border-b border-surface-subtle last:border-b-0">
                                                    <span class="text-base text-txt-primary font-medium">{name}</span>
                                                    <span class="text-sm text-txt-tertiary">{date}</span>
                                                </li>
                                            }
                                        }).collect_view()}
                                    </ul>
                                </div>
                            </section>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Static fallback data for opportunity detail pages.
fn static_opportunity_detail(
    id: &str,
) -> (&'static str, &'static str, &'static str, &'static str, &'static str) {
    match id {
        "opp-1" => (
            "노인 대학",
            "평생학습 프로그램으로 다양한 주제의 강좌를 수강할 수 있습니다.",
            "매주 화/목 10:00-12:00",
            "서울시 종로구 복지관",
            "해당 기관에 문의하세요",
        ),
        "opp-2" => (
            "건강 체조",
            "전문 강사와 함께하는 어르신 맞춤 건강 체조 프로그램입니다.",
            "매주 월/수/금 09:00-10:00",
            "동네 공원 or 복지관",
            "해당 기관에 문의하세요",
        ),
        "opp-3" => (
            "봉사 활동",
            "지역사회를 위한 봉사 프로그램에 참여할 수 있습니다.",
            "매월 둘째 토요일 09:00-12:00",
            "지역 복지관",
            "해당 기관에 문의하세요",
        ),
        "opp-4" => (
            "문화 교실",
            "서예, 그림, 음악 등 다양한 문화 수업을 제공합니다.",
            "매주 수요일 14:00-16:00",
            "문화센터",
            "해당 기관에 문의하세요",
        ),
        _ => (
            "걷기 모임",
            "동네 이웃과 함께 산책하며 건강을 챙기는 모임입니다.",
            "매일 오전 7:00-8:00",
            "동네 공원 입구",
            "해당 기관에 문의하세요",
        ),
    }
}

/// Render the opportunity detail card content.
fn opportunity_detail_card(
    title: String,
    desc: String,
    schedule: String,
    location: String,
    contact: String,
) -> impl IntoView {
    let is_phone = contact.chars().any(|c| c.is_ascii_digit());
    let tel_href = format!("tel:{contact}");
    let contact_display = contact.clone();
    view! {
        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
            <h1 class="text-2xl font-bold text-txt-primary">{title}</h1>
            <p class="text-lg text-txt-secondary">{desc}</p>
            <div class="space-y-2">
                <InfoRow label="일정" value=schedule />
                <InfoRow label="장소" value=location />
                <InfoRow label="문의" value=contact_display />
            </div>
        </div>
        {if is_phone {
            view! {
                <a href=tel_href
                   class="block w-full bg-primary text-white text-center text-lg font-semibold \
                          rounded-xl py-4 hover:bg-primary-hover active:scale-[0.98] transition-all">
                    "문의 전화하기"
                </a>
            }.into_any()
        } else {
            view! {
                <div class="w-full bg-surface-subtle text-txt-tertiary text-center text-base font-medium \
                            rounded-xl py-4">
                    "해당 기관에 직접 문의하세요"
                </div>
            }.into_any()
        }}
    }
}

/// Opportunity detail (API-driven with static fallback).
#[component]
pub fn OpportunityDetailPage(
    #[prop(into)] opportunity_id: String,
) -> impl IntoView {
    let opp_id = opportunity_id.clone();
    let activity = LocalResource::new(move || {
        let id = opp_id.clone();
        async move {
            crate::api::get::<Vec<serde_json::Value>>("/api/community/activities")
                .await
                .ok()
                .and_then(|resp| {
                    resp.data
                        .unwrap_or_default()
                        .into_iter()
                        .find(|item| item.get("id").and_then(|v| v.as_str()) == Some(&id))
                })
        }
    });

    let fallback_id = opportunity_id.clone();

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/opportunities" class="text-primary text-lg">"< 사회 참여"</a>
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || {
                    let fb_id = fallback_id.clone();
                    Suspend::new(async move {
                        let api_item = activity.await;
                        match api_item {
                            Some(item) => {
                                let title = item
                                    .get("title")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("활동")
                                    .to_string();
                                let desc = item
                                    .get("description")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let schedule = item
                                    .get("schedule")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let location = item
                                    .get("location")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let contact = item
                                    .get("contact")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                opportunity_detail_card(title, desc, schedule, location, contact)
                                    .into_any()
                            }
                            None => {
                                let (title, desc, schedule, location, contact) =
                                    static_opportunity_detail(&fb_id);
                                opportunity_detail_card(
                                    title.to_string(),
                                    desc.to_string(),
                                    schedule.to_string(),
                                    location.to_string(),
                                    contact.to_string(),
                                )
                                .into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
