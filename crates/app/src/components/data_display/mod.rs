use leptos::prelude::*;

// ---------------------------------------------------------------------------
// DataTable (stub — thin wrapper around <table>)
// ---------------------------------------------------------------------------

#[component]
pub fn DataTable(children: Children) -> impl IntoView {
    view! {
        <div class="overflow-x-auto rounded-lg border border-gray-200">
            <table class="min-w-full divide-y divide-gray-200 text-sm">
                {children()}
            </table>
        </div>
    }
}

// ---------------------------------------------------------------------------
// DataCard
// ---------------------------------------------------------------------------

#[component]
pub fn DataCard(
    #[prop(into)] title: String,
    #[prop(into)] value: String,
    #[prop(into, optional)] subtitle: String,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-5">
            <p class="text-sm text-gray-500">{title}</p>
            <p class="mt-1 text-2xl font-bold text-gray-900">{value}</p>
            {if !subtitle.is_empty() {
                Some(view! {
                    <p class="mt-1 text-xs text-gray-400">{subtitle}</p>
                })
            } else {
                None
            }}
        </div>
    }
}

// ---------------------------------------------------------------------------
// StatWidget
// ---------------------------------------------------------------------------

#[component]
pub fn StatWidget(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional)] change_percent: Option<f64>,
    #[prop(into, optional)] icon: String,
) -> impl IntoView {
    let change_view = change_percent.map(|pct| {
        let (color, arrow) = if pct >= 0.0 {
            ("text-green-600", "\u{2191}")
        } else {
            ("text-red-600", "\u{2193}")
        };
        let cls = format!("text-xs font-medium {color}");
        view! {
            <span class=cls>
                {arrow} {format!("{:.1}%", pct.abs())}
            </span>
        }
    });

    view! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-5 \
                    flex items-start gap-4">
            {if !icon.is_empty() {
                Some(view! {
                    <div class="flex items-center justify-center h-10 w-10 \
                                rounded-lg bg-blue-50 text-blue-600 text-xl shrink-0">
                        {icon}
                    </div>
                })
            } else {
                None
            }}
            <div class="flex flex-col">
                <p class="text-sm text-gray-500">{label}</p>
                <div class="flex items-baseline gap-2">
                    <p class="text-2xl font-bold text-gray-900">{value}</p>
                    {change_view}
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// StatusBadge
// ---------------------------------------------------------------------------

#[component]
pub fn StatusBadge(
    #[prop(into)] status: String,
) -> impl IntoView {
    let variant_cls = match status.as_str() {
        "active" | "활성" | "승인" | "완료" => "bg-green-100 text-green-700",
        "pending" | "대기" | "보류" => "bg-yellow-100 text-yellow-800",
        "inactive" | "비활성" | "거부" | "취소" => "bg-red-100 text-red-700",
        "scheduled" | "예정" => "bg-blue-100 text-blue-700",
        _ => "bg-gray-100 text-gray-700",
    };

    let cls = format!(
        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {variant_cls}"
    );

    view! { <span class=cls>{status}</span> }
}

// ---------------------------------------------------------------------------
// Timeline (stub)
// ---------------------------------------------------------------------------

#[component]
pub fn Timeline() -> impl IntoView {
    view! {
        <div class="text-sm text-gray-500 italic">"Timeline – 추후 구현 예정"</div>
    }
}

// ---------------------------------------------------------------------------
// EmptyState
// ---------------------------------------------------------------------------

#[component]
pub fn EmptyState(
    #[prop(into, optional, default = "데이터가 없습니다".into())] message: String,
    #[prop(into, optional)] icon: String,
    #[prop(into, optional)] action_label: String,
    #[prop(into, optional)] action_href: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-16 text-center">
            {if !icon.is_empty() {
                view! {
                    <div class="text-4xl text-gray-300 mb-4">{icon}</div>
                }.into_any()
            } else {
                view! {
                    <div class="text-4xl text-gray-300 mb-4">
                        <svg class="h-12 w-12 mx-auto" fill="none" stroke="currentColor"
                            viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                stroke-width="1.5"
                                d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 \
                                   0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 \
                                   00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 \
                                   1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
                        </svg>
                    </div>
                }.into_any()
            }}
            <p class="text-gray-500 text-sm">{message}</p>
            {if !action_label.is_empty() && !action_href.is_empty() {
                Some(view! {
                    <a
                        href=action_href
                        class="mt-4 inline-block px-4 py-2 bg-blue-600 text-white \
                               text-sm rounded-lg hover:bg-blue-700"
                    >
                        {action_label}
                    </a>
                })
            } else {
                None
            }}
        </div>
    }
}
