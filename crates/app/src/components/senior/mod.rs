use leptos::prelude::*;

// ---------------------------------------------------------------------------
// EmergencyButton — red FAB in bottom-right corner
// ---------------------------------------------------------------------------

#[component]
pub fn EmergencyButton() -> impl IntoView {
    view! {
        <a
            href="/emergency"
            class="fixed bottom-20 right-4 z-50 flex items-center justify-center \
                   h-16 w-16 rounded-full bg-red-600 text-white shadow-lg \
                   hover:bg-red-700 active:scale-95 transition-transform \
                   md:bottom-6"
            aria-label="긴급 호출"
        >
            <svg class="h-8 w-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 \
                       0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 \
                       5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 \
                       0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
            </svg>
        </a>
    }
}

// ---------------------------------------------------------------------------
// ConsentToggle
// ---------------------------------------------------------------------------

#[component]
pub fn ConsentToggle(
    #[prop(into)] label: String,
    #[prop(into, optional)] purpose: String,
    #[prop(into, optional)] is_active: RwSignal<bool>,
) -> impl IntoView {
    let track = move || {
        if is_active.get() {
            "bg-blue-600"
        } else {
            "bg-gray-300"
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
        <div class="flex items-center justify-between p-4 bg-white rounded-xl \
                    border border-gray-100 shadow-sm">
            <div class="flex flex-col gap-0.5">
                <span class="text-sm font-medium text-gray-800">{label}</span>
                {if !purpose.is_empty() {
                    Some(view! {
                        <span class="text-xs text-gray-400">{purpose}</span>
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
                        "relative inline-flex h-6 w-11 shrink-0 rounded-full \
                         transition-colors focus:outline-none focus:ring-2 \
                         focus:ring-blue-500 focus:ring-offset-2 {}",
                        track(),
                    )
                }
                on:click=move |_| is_active.set(!is_active.get_untracked())
            >
                <span
                    class=move || {
                        format!(
                            "pointer-events-none inline-block h-5 w-5 \
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

// ---------------------------------------------------------------------------
// ScheduleBlock
// ---------------------------------------------------------------------------

#[component]
pub fn ScheduleBlock(
    #[prop(into)] time: String,
    #[prop(into)] title: String,
    #[prop(into, optional)] location: String,
) -> impl IntoView {
    view! {
        <div class="flex gap-4 p-4 bg-white rounded-xl border border-gray-100 shadow-sm">
            <div class="flex flex-col items-center justify-center min-w-[3.5rem] \
                        text-center">
                <span class="text-sm font-semibold text-blue-600">{time}</span>
            </div>
            <div class="flex flex-col gap-0.5">
                <span class="text-sm font-medium text-gray-800">{title}</span>
                {if !location.is_empty() {
                    Some(view! {
                        <span class="text-xs text-gray-400">{location}</span>
                    })
                } else {
                    None
                }}
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// QuickAction
// ---------------------------------------------------------------------------

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
                   bg-white rounded-xl border border-gray-100 shadow-sm \
                   hover:bg-gray-50 active:scale-[0.98] transition-transform \
                   min-h-[5rem]"
        >
            {if !icon.is_empty() {
                Some(view! { <span class="text-2xl">{icon}</span> })
            } else {
                None
            }}
            <span class="text-sm font-medium text-gray-700 text-center">{label}</span>
        </a>
    }
}

// ---------------------------------------------------------------------------
// LargeButton — extra-large touch target for seniors
// ---------------------------------------------------------------------------

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
            class="flex items-center justify-center gap-3 w-full px-6 py-5 \
                   bg-blue-600 text-white text-lg font-semibold rounded-2xl \
                   shadow hover:bg-blue-700 active:scale-[0.98] \
                   transition-transform min-h-[4rem]"
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

// ---------------------------------------------------------------------------
// MedicationCard
// ---------------------------------------------------------------------------

#[component]
pub fn MedicationCard(
    #[prop(into)] name: String,
    #[prop(into, optional)] dosage: String,
    #[prop(into, optional)] time: String,
    #[prop(optional)] is_taken: bool,
) -> impl IntoView {
    let border = if is_taken {
        "border-green-200"
    } else {
        "border-gray-100"
    };

    let badge = if is_taken {
        ("bg-green-100 text-green-700", "\u{2705} 복용 완료")
    } else {
        ("bg-yellow-100 text-yellow-700", "\u{23f0} 미복용")
    };

    let card_cls = format!(
        "flex items-center justify-between p-4 bg-white rounded-xl \
         border shadow-sm {border}"
    );

    let badge_cls = format!(
        "inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full \
         text-xs font-medium {}",
        badge.0
    );

    view! {
        <div class=card_cls>
            <div class="flex flex-col gap-0.5">
                <span class="text-sm font-medium text-gray-800">{name}</span>
                {if !dosage.is_empty() {
                    Some(view! {
                        <span class="text-xs text-gray-400">{dosage}</span>
                    })
                } else {
                    None
                }}
                {if !time.is_empty() {
                    Some(view! {
                        <span class="text-xs text-gray-400">{time}</span>
                    })
                } else {
                    None
                }}
            </div>
            <span class=badge_cls>{badge.1}</span>
        </div>
    }
}
