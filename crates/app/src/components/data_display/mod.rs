use leptos::prelude::*;

// ---------------------------------------------------------------------------
// TimelineItem — data struct for Timeline entries
// ---------------------------------------------------------------------------

/// A single entry in a vertical timeline.
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineItem {
    /// Display time (e.g. "14:30", "2026-03-18").
    pub time: String,
    /// Short headline for the event.
    pub title: String,
    /// Optional longer description.
    pub description: String,
}

// ---------------------------------------------------------------------------
// DataTable
// ---------------------------------------------------------------------------

/// Shadow-card wrapper around an HTML `<table>`.
///
/// Provides consistent rounded corners, subtle shadow, and themed header/divider
/// colors. Pass `<thead>` / `<tbody>` as children.
#[component]
pub fn DataTable(children: Children) -> impl IntoView {
    view! {
        <div class="overflow-x-auto shadow-sm rounded-2xl overflow-hidden">
            <table class="min-w-full divide-y divide-gray-100 text-sm">
                {children()}
            </table>
        </div>
    }
}

/// Convenience header row that applies the `bg-surface-subtle` background.
///
/// Wrap your `<th>` cells inside this component so every table gets a
/// consistent header style without repeating the class.
#[component]
pub fn DataTableHead(children: Children) -> impl IntoView {
    view! {
        <thead class="bg-surface-subtle">
            {children()}
        </thead>
    }
}

// ---------------------------------------------------------------------------
// DataCard
// ---------------------------------------------------------------------------

/// A compact metric card showing a title, large value, and optional subtitle.
///
/// Uses shadow-only elevation (no border) and responds to hover with a
/// slightly deeper shadow for tactile feedback.
#[component]
pub fn DataCard(
    #[prop(into)] title: String,
    #[prop(into)] value: String,
    #[prop(into, optional)] subtitle: String,
) -> impl IntoView {
    view! {
        <div class="bg-surface-card rounded-2xl shadow-sm p-6 hover:shadow-md transition-shadow duration-200">
            <p class="text-sm text-txt-tertiary">{title}</p>
            <p class="mt-1 text-2xl font-bold text-txt-primary">{value}</p>
            {if !subtitle.is_empty() {
                Some(view! {
                    <p class="mt-1 text-xs text-txt-disabled">{subtitle}</p>
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

/// A statistics widget with an optional icon, large value, label, and
/// percentage-change indicator.
///
/// The icon container is accent-aware: it reads `--portal-accent` and
/// `--portal-accent-light` from the nearest ancestor so each portal
/// (senior, family, caregiver, etc.) gets the correct tint automatically.
#[component]
pub fn StatWidget(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional)] change_percent: Option<f64>,
    #[prop(into, optional)] icon: String,
) -> impl IntoView {
    let change_view = change_percent.map(|pct| {
        let (color, arrow) = if pct >= 0.0 {
            ("text-success", "\u{2191}")
        } else {
            ("text-danger", "\u{2193}")
        };
        let cls = format!("text-xs font-medium {color}");
        view! {
            <span class=cls>
                {arrow} {format!("{:.1}%", pct.abs())}
            </span>
        }
    });

    view! {
        <div class="bg-surface-card rounded-2xl shadow-sm p-6 flex items-start gap-4">
            {if !icon.is_empty() {
                Some(view! {
                    <div class="flex items-center justify-center h-10 w-10 \
                                rounded-lg bg-[var(--portal-accent-light)] \
                                text-[var(--portal-accent)] text-xl shrink-0">
                        {icon}
                    </div>
                })
            } else {
                None
            }}
            <div class="flex flex-col">
                <p class="text-sm text-txt-tertiary">{label}</p>
                <div class="flex items-baseline gap-2">
                    <p class="text-2xl font-bold text-txt-primary">{value}</p>
                    {change_view}
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// StatusBadge
// ---------------------------------------------------------------------------

/// A pill-shaped badge that maps common Korean and English status strings to
/// semantic token colors (success, warning, danger, primary, or neutral).
#[component]
pub fn StatusBadge(
    #[prop(into)] status: String,
) -> impl IntoView {
    let variant_cls = match status.as_str() {
        "active" | "활성" | "승인" | "완료" => "bg-success-light text-success",
        "pending" | "대기" | "보류" => "bg-warning-light text-warning",
        "inactive" | "비활성" | "거부" | "취소" => "bg-danger-light text-danger",
        "scheduled" | "예정" => "bg-primary-light text-primary",
        _ => "bg-surface-subtle text-txt-secondary",
    };

    let cls = format!(
        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {variant_cls}"
    );

    view! { <span class=cls>{status}</span> }
}

// ---------------------------------------------------------------------------
// Timeline
// ---------------------------------------------------------------------------

/// A vertical timeline showing a series of events with accent-colored dots
/// and connecting lines.
///
/// Each item renders its time, title, and description. The dots and connector
/// line use the portal accent color so they adapt per-portal automatically.
#[component]
pub fn Timeline(
    #[prop(into)] items: Vec<TimelineItem>,
) -> impl IntoView {
    let len = items.len();

    view! {
        <div class="relative flex flex-col">
            {items
                .into_iter()
                .enumerate()
                .map(|(idx, item)| {
                    let is_last = idx == len - 1;

                    view! {
                        <div class="relative flex gap-4 pb-6">
                            // Dot + connector column
                            <div class="flex flex-col items-center">
                                // Accent dot
                                <div class="relative z-10 mt-1 h-3 w-3 shrink-0 rounded-full \
                                            bg-[var(--portal-accent)] ring-4 ring-[var(--portal-accent-light)]" />
                                // Vertical connector (hidden on last item)
                                {if !is_last {
                                    Some(view! {
                                        <div class="w-0.5 grow bg-[var(--portal-accent-light)]" />
                                    })
                                } else {
                                    None
                                }}
                            </div>

                            // Content column
                            <div class="flex flex-col min-w-0 pt-0.5">
                                <span class="text-xs text-txt-disabled">{item.time}</span>
                                <p class="text-sm font-medium text-txt-primary mt-0.5">{item.title}</p>
                                {if !item.description.is_empty() {
                                    Some(view! {
                                        <p class="text-xs text-txt-tertiary mt-1">{item.description}</p>
                                    })
                                } else {
                                    None
                                }}
                            </div>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// EmptyState
// ---------------------------------------------------------------------------

/// A full-width empty-state placeholder with an inline SVG illustration,
/// message, and an optional action button.
///
/// When `action_label` and `action_href` are both provided, a primary-colored
/// CTA button is rendered below the message.
#[component]
pub fn EmptyState(
    #[prop(into, optional, default = "데이터가 없습니다".into())] message: String,
    #[prop(into, optional)] action_label: String,
    #[prop(into, optional)] action_href: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-16 text-center">
            // Inline SVG inbox / empty illustration
            <div class="text-txt-disabled mb-4">
                <svg
                    class="h-16 w-16 mx-auto"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 48 48"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    // Outer box
                    <rect x="6" y="10" width="36" height="28" rx="4" />
                    // Inbox tray shelf
                    <path d="M6 30h10l3 4h10l3-4h10" />
                    // Down-arrow into tray
                    <path d="M24 16v10" />
                    <path d="M20 22l4 4 4-4" />
                </svg>
            </div>

            <p class="text-sm text-txt-tertiary">{message}</p>

            {if !action_label.is_empty() && !action_href.is_empty() {
                Some(view! {
                    <a
                        href=action_href
                        class="mt-4 inline-block px-5 py-2.5 bg-primary text-white \
                               text-sm font-medium rounded-xl \
                               hover:bg-primary-hover active:scale-[0.98] \
                               transition-all duration-150"
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
