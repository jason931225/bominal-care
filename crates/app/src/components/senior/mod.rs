use leptos::prelude::*;

/// Fixed-position emergency call button displayed in the bottom-right corner.
///
/// Renders as a large pulsing red circle with a phone icon, linking to
/// `/emergency`. Uses the `animate-pulse-ring` animation for visual urgency
/// and includes a white border ring for contrast against any background.
#[component]
pub fn EmergencyButton() -> impl IntoView {
    view! {
        <a
            href="/emergency"
            class="fixed bottom-20 right-4 z-50 flex items-center justify-center \
                   h-[4.5rem] w-[4.5rem] rounded-full bg-danger text-white \
                   shadow-lg shadow-danger/40 border-[3px] border-white \
                   animate-pulse-ring active:scale-95 transition-transform \
                   md:bottom-6"
            aria-label="긴급 호출"
        >
            <svg class="h-9 w-9" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 \
                       0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 \
                       5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 \
                       0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
            </svg>
        </a>
    }
}

/// Toggle switch for managing user consent preferences.
///
/// Displays a label, optional purpose description, and an accessible toggle
/// switch. The switch track uses the portal accent color when active and a
/// neutral gray when inactive.
#[component]
pub fn ConsentToggle(
    #[prop(into)] label: String,
    #[prop(into, optional)] purpose: String,
    #[prop(into, optional)] is_active: RwSignal<bool>,
) -> impl IntoView {
    let track = move || {
        if is_active.get() {
            "bg-[var(--portal-accent)]"
        } else {
            "bg-gray-200"
        }
    };

    let knob = move || {
        if is_active.get() {
            "translate-x-5"
        } else {
            "translate-x-0"
        }
    };

    view! {
        <div class="flex items-center justify-between p-4 bg-surface-card rounded-2xl shadow-sm">
            <div class="flex flex-col gap-0.5">
                <span class="text-sm font-medium text-txt-primary">{label}</span>
                {if !purpose.is_empty() {
                    Some(view! {
                        <span class="text-xs text-txt-tertiary">{purpose}</span>
                    })
                } else {
                    None
                }}
            </div>
            <button
                type="button"
                role="switch"
                class=move || {
                    format!(
                        "relative inline-flex h-7 w-12 shrink-0 rounded-full \
                         transition-colors focus:outline-none focus:ring-2 \
                         focus:ring-primary focus:ring-offset-2 {}",
                        track(),
                    )
                }
                on:click=move |_| is_active.set(!is_active.get_untracked())
            >
                <span
                    class=move || {
                        format!(
                            "pointer-events-none inline-block h-6 w-6 \
                             translate-y-0.5 rounded-full bg-white shadow \
                             ring-0 transition-transform {}",
                            knob(),
                        )
                    }
                />
            </button>
        </div>
    }
}

/// A single time-block entry for the senior's daily schedule.
///
/// Shows the time in the portal accent color alongside the event title and
/// an optional location. Uses the `card-interactive` CSS class for hover
/// and press feedback.
#[component]
pub fn ScheduleBlock(
    #[prop(into)] time: String,
    #[prop(into)] title: String,
    #[prop(into, optional)] location: String,
) -> impl IntoView {
    view! {
        <div class="card-interactive flex gap-4 p-4 rounded-2xl shadow-sm">
            <div class="flex flex-col items-center justify-center min-w-[3.5rem] \
                        text-center">
                <span class="text-sm font-semibold text-[var(--portal-accent)]">{time}</span>
            </div>
            <div class="flex flex-col gap-0.5">
                <span class="text-sm font-medium text-txt-primary">{title}</span>
                {if !location.is_empty() {
                    Some(view! {
                        <span class="text-xs text-txt-tertiary">{location}</span>
                    })
                } else {
                    None
                }}
            </div>
        </div>
    }
}

/// Grid-friendly quick action tile linking to a feature page.
///
/// Renders an icon inside an accent-tinted container above a text label.
/// The card elevates on hover and compresses slightly on press for tactile
/// feedback.
#[component]
pub fn QuickAction(
    #[prop(into, optional)] icon: String,
    #[prop(into)] label: String,
    #[prop(into)] href: String,
) -> impl IntoView {
    view! {
        <a
            href=href
            class="flex flex-col items-center justify-center gap-2 p-4 \
                   bg-surface-card rounded-2xl shadow-sm \
                   hover:shadow-md active:scale-[0.98] transition-shadow \
                   min-h-[5rem]"
        >
            {if !icon.is_empty() {
                Some(view! {
                    <span class="flex items-center justify-center h-12 w-12 \
                                 rounded-2xl bg-[var(--portal-accent-light)] \
                                 text-[var(--portal-accent)] text-2xl">
                        {icon}
                    </span>
                })
            } else {
                None
            }}
            <span class="text-sm font-medium text-txt-primary text-center">{label}</span>
        </a>
    }
}

/// Extra-large call-to-action button designed for senior-friendly touch targets.
///
/// Full-width with generous padding, a primary background, and a subtle
/// glow shadow. Supports an optional leading icon.
#[component]
pub fn LargeButton(
    #[prop(into)] label: String,
    #[prop(into, optional)] href: String,
    #[prop(into, optional)] icon: String,
) -> impl IntoView {
    let target = if href.is_empty() { "#".to_string() } else { href };

    view! {
        <a
            href=target
            class="flex items-center justify-center gap-3 w-full px-8 py-6 \
                   bg-primary text-white text-xl font-semibold rounded-2xl \
                   shadow-md shadow-primary/20 hover:bg-primary-hover \
                   active:scale-[0.98] transition-transform min-h-[4rem]"
        >
            {if !icon.is_empty() {
                Some(view! { <span class="text-2xl">{icon}</span> })
            } else {
                None
            }}
            <span>{label}</span>
        </a>
    }
}

/// Card displaying a single medication with its dosage, scheduled time, and
/// intake status.
///
/// When `is_taken` is true the card background shifts to a light success tint
/// and the status badge shows a green "taken" indicator. Otherwise the badge
/// uses a warning palette to draw attention.
#[component]
pub fn MedicationCard(
    #[prop(into)] name: String,
    #[prop(into, optional)] dosage: String,
    #[prop(into, optional)] time: String,
    #[prop(optional)] is_taken: bool,
) -> impl IntoView {
    let card_bg = if is_taken {
        "bg-success-light/30"
    } else {
        "bg-surface-card"
    };

    let (badge_cls_colors, badge_label) = if is_taken {
        ("bg-success-light text-success", "\u{2705} 복용 완료")
    } else {
        ("bg-warning-light text-warning", "\u{23f0} 미복용")
    };

    let card_cls = format!(
        "flex items-center justify-between p-4 rounded-2xl shadow-sm {card_bg}"
    );

    let badge_cls = format!(
        "inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full \
         text-xs font-medium {badge_cls_colors}"
    );

    view! {
        <div class=card_cls>
            <div class="flex flex-col gap-0.5">
                <span class="text-sm font-medium text-txt-primary">{name}</span>
                {if !dosage.is_empty() {
                    Some(view! {
                        <span class="text-xs text-txt-tertiary">{dosage}</span>
                    })
                } else {
                    None
                }}
                {if !time.is_empty() {
                    Some(view! {
                        <span class="text-xs text-txt-tertiary">{time}</span>
                    })
                } else {
                    None
                }}
            </div>
            <span class=badge_cls>{badge_label}</span>
        </div>
    }
}
