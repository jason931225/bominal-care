use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Button
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Button(
    #[prop(into, optional)] variant: ButtonVariant,
    #[prop(into, optional)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center justify-center font-medium rounded-lg \
                transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2";

    let variant_cls = match variant {
        ButtonVariant::Primary => {
            "bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500"
        }
        ButtonVariant::Secondary => {
            "bg-gray-100 text-gray-700 hover:bg-gray-200 focus:ring-gray-400 border border-gray-300"
        }
        ButtonVariant::Danger => {
            "bg-red-600 text-white hover:bg-red-700 focus:ring-red-500"
        }
        ButtonVariant::Ghost => {
            "bg-transparent text-gray-600 hover:bg-gray-100 focus:ring-gray-400"
        }
    };

    let size_cls = match size {
        ButtonSize::Sm => "px-3 py-1.5 text-sm",
        ButtonSize::Md => "px-4 py-2 text-base",
        ButtonSize::Lg => "px-6 py-3 text-lg",
    };

    let disabled_cls = if disabled || loading {
        "opacity-50 cursor-not-allowed"
    } else {
        ""
    };

    let class = format!("{base} {variant_cls} {size_cls} {disabled_cls}");

    view! {
        <button class=class disabled=disabled || loading>
            {move || {
                if loading {
                    Some(view! {
                        <svg
                            class="animate-spin -ml-1 mr-2 h-4 w-4"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <circle class="opacity-25" cx="12" cy="12" r="10"
                                stroke="currentColor" stroke-width="4" />
                            <path class="opacity-75" fill="currentColor"
                                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                        </svg>
                    })
                } else {
                    None
                }
            }}
            {children()}
        </button>
    }
}

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

#[component]
pub fn Input(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] error: String,
    #[prop(into, optional, default = "text".into())] input_type: String,
    #[prop(into, optional)] value: RwSignal<String>,
) -> impl IntoView {
    let has_error = !error.is_empty();
    let ring = if has_error {
        "border-red-500 focus:ring-red-500"
    } else {
        "border-gray-300 focus:ring-blue-500"
    };

    let input_class = format!(
        "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 text-gray-900 {ring}"
    );

    view! {
        <div class="flex flex-col gap-1">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-gray-700">{label}</label>
                })
            } else {
                None
            }}
            <input
                type=input_type
                class=input_class
                placeholder=placeholder
                prop:value=move || value.get()
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                }
            />
            {if has_error {
                Some(view! {
                    <p class="text-sm text-red-600">{error}</p>
                })
            } else {
                None
            }}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Textarea
// ---------------------------------------------------------------------------

#[component]
pub fn Textarea(
    #[prop(into, optional)] label: String,
    #[prop(optional, default = 4)] rows: u32,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-gray-700">{label}</label>
                })
            } else {
                None
            }}
            <textarea
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none \
                       focus:ring-2 focus:ring-blue-500 text-gray-900"
                rows=rows
                placeholder=placeholder
                prop:value=move || value.get()
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                }
            />
        </div>
    }
}

// ---------------------------------------------------------------------------
// Select
// ---------------------------------------------------------------------------

#[component]
pub fn Select(
    #[prop(into, optional)] label: String,
    #[prop(into)] options: Vec<(String, String)>,
    #[prop(into, optional)] value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-gray-700">{label}</label>
                })
            } else {
                None
            }}
            <select
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none \
                       focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                prop:value=move || value.get()
                on:change=move |ev| {
                    value.set(event_target_value(&ev));
                }
            >
                {options
                    .into_iter()
                    .map(|(val, text)| {
                        view! { <option value=val>{text}</option> }
                    })
                    .collect_view()}
            </select>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Checkbox
// ---------------------------------------------------------------------------

#[component]
pub fn Checkbox(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] checked: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <label class="inline-flex items-center gap-2 cursor-pointer">
            <input
                type="checkbox"
                class="h-4 w-4 rounded border-gray-300 text-blue-600 \
                       focus:ring-blue-500"
                prop:checked=move || checked.get()
                on:change=move |ev| {
                    checked.set(event_target_checked(&ev));
                }
            />
            {if !label.is_empty() {
                Some(view! { <span class="text-sm text-gray-700">{label}</span> })
            } else {
                None
            }}
        </label>
    }
}

// ---------------------------------------------------------------------------
// RadioGroup (stub)
// ---------------------------------------------------------------------------

#[component]
pub fn RadioGroup() -> impl IntoView {
    view! {
        <div class="text-sm text-gray-500 italic">"RadioGroup – 추후 구현 예정"</div>
    }
}

// ---------------------------------------------------------------------------
// Switch
// ---------------------------------------------------------------------------

#[component]
pub fn Switch(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] checked: RwSignal<bool>,
) -> impl IntoView {
    let track = move || {
        if checked.get() {
            "bg-blue-600"
        } else {
            "bg-gray-300"
        }
    };

    let knob = move || {
        if checked.get() {
            "translate-x-5"
        } else {
            "translate-x-0"
        }
    };

    view! {
        <label class="inline-flex items-center gap-3 cursor-pointer select-none">
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
                on:click=move |_| checked.set(!checked.get_untracked())
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
            {if !label.is_empty() {
                Some(view! { <span class="text-sm text-gray-700">{label}</span> })
            } else {
                None
            }}
        </label>
    }
}

// ---------------------------------------------------------------------------
// Label
// ---------------------------------------------------------------------------

#[component]
pub fn Label(
    #[prop(into)] text: String,
    #[prop(into, optional)] for_id: String,
) -> impl IntoView {
    view! {
        <label for=for_id class="text-sm font-medium text-gray-700">
            {text}
        </label>
    }
}

// ---------------------------------------------------------------------------
// Avatar
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Avatar(
    #[prop(into, optional)] src: String,
    #[prop(into, optional, default = "avatar".into())] alt: String,
    #[prop(into, optional)] size: AvatarSize,
) -> impl IntoView {
    let size_cls = match size {
        AvatarSize::Sm => "h-8 w-8",
        AvatarSize::Md => "h-10 w-10",
        AvatarSize::Lg => "h-14 w-14",
    };

    let class = format!("{size_cls} rounded-full object-cover bg-gray-200");

    if src.is_empty() {
        // Placeholder circle with initials
        let placeholder_cls =
            format!("{size_cls} rounded-full bg-gray-300 flex items-center justify-center");
        view! {
            <div class=placeholder_cls>
                <span class="text-gray-500 text-sm font-medium">
                    {alt.chars().next().unwrap_or('?').to_string()}
                </span>
            </div>
        }
        .into_any()
    } else {
        view! { <img src=src alt=alt class=class /> }.into_any()
    }
}

// ---------------------------------------------------------------------------
// Badge
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Danger,
    Info,
}

#[component]
pub fn Badge(
    #[prop(into)] text: String,
    #[prop(into, optional)] variant: BadgeVariant,
) -> impl IntoView {
    let variant_cls = match variant {
        BadgeVariant::Default => "bg-gray-100 text-gray-700",
        BadgeVariant::Success => "bg-green-100 text-green-700",
        BadgeVariant::Warning => "bg-yellow-100 text-yellow-800",
        BadgeVariant::Danger => "bg-red-100 text-red-700",
        BadgeVariant::Info => "bg-blue-100 text-blue-700",
    };

    let class = format!(
        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {variant_cls}"
    );

    view! { <span class=class>{text}</span> }
}

// ---------------------------------------------------------------------------
// Utility — checked helper
// ---------------------------------------------------------------------------

fn event_target_checked(ev: &leptos::ev::Event) -> bool {
    use leptos::wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok())
        .map(|el| el.checked())
        .unwrap_or(false)
}
