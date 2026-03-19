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
    Outline,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// A general-purpose button with multiple variants and sizes.
///
/// # Props
/// - `variant` – Visual style: `Primary` (filled blue), `Secondary` (gray),
///   `Danger` (red), `Ghost` (transparent), or `Outline` (bordered blue).
/// - `size` – Padding / font size: `Sm`, `Md` (default), `Lg`.
/// - `disabled` – Renders the button as non-interactive with reduced opacity.
/// - `loading` – Shows a spinner and disables the button.
/// - `children` – Button label / inner content.
///
/// # Usage
/// ```rust
/// view! { <Button variant=ButtonVariant::Primary>"저장"</Button> }
/// ```
#[component]
pub fn Button(
    #[prop(into, optional)] variant: ButtonVariant,
    #[prop(into, optional)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center justify-center font-medium rounded-xl \
                active:scale-[0.98] transition-all duration-200 \
                focus:outline-none focus:ring-2 focus:ring-offset-2";

    let variant_cls = match variant {
        ButtonVariant::Primary => {
            "bg-primary text-white hover:bg-primary-hover focus:ring-primary/30"
        }
        ButtonVariant::Secondary => {
            "bg-gray-100 text-txt-secondary hover:bg-gray-200 focus:ring-gray-400 \
             border border-gray-300"
        }
        ButtonVariant::Danger => {
            "bg-danger text-white hover:bg-red-700 focus:ring-danger/30"
        }
        ButtonVariant::Ghost => {
            "bg-transparent text-txt-secondary hover:bg-gray-100 focus:ring-gray-400"
        }
        ButtonVariant::Outline => {
            "border-2 border-primary text-primary hover:bg-primary-light \
             focus:ring-primary/30"
        }
    };

    let size_cls = match size {
        ButtonSize::Sm => "px-4 py-2 text-sm min-h-[44px]",
        ButtonSize::Md => "px-5 py-2.5 text-base min-h-[44px]",
        ButtonSize::Lg => "px-6 py-3 text-lg min-h-[48px]",
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

/// A styled text input with optional label and inline error message.
///
/// # Props
/// - `label` – Text displayed above the input field.
/// - `placeholder` – Placeholder text inside the input.
/// - `error` – When non-empty, renders a red border and error message below.
/// - `input_type` – HTML input type attribute (defaults to `"text"`).
/// - `value` – Two-way bound `RwSignal<String>` for the input value.
///
/// # Usage
/// ```rust
/// let name = RwSignal::new(String::new());
/// view! { <Input label="이름" placeholder="이름을 입력하세요" value=name /> }
/// ```
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
        "border-danger focus:ring-danger/30"
    } else {
        "border-gray-200 focus:ring-primary/30 focus:border-primary"
    };

    let input_class = format!(
        "w-full px-4 py-2.5 min-h-[44px] border rounded-xl focus:outline-none focus:ring-2 \
         text-txt-primary {ring}"
    );

    view! {
        <div class="flex flex-col gap-1">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-txt-secondary">{label}</label>
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
                    <p class="text-sm text-danger">{error}</p>
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

/// A multi-line text input with optional label, using design-token colors.
///
/// # Props
/// - `label` – Text displayed above the textarea.
/// - `rows` – Number of visible text lines (defaults to 4).
/// - `placeholder` – Placeholder text inside the textarea.
/// - `error` – When non-empty, renders a red border and error message below.
/// - `value` – Two-way bound `RwSignal<String>` for the textarea content.
///
/// # Usage
/// ```rust
/// let memo = RwSignal::new(String::new());
/// view! { <Textarea label="메모" rows=6 value=memo /> }
/// ```
#[component]
pub fn Textarea(
    #[prop(into, optional)] label: String,
    #[prop(optional, default = 4)] rows: u32,
    #[prop(into, optional)] placeholder: String,
    #[prop(into, optional)] error: String,
    #[prop(into, optional)] value: RwSignal<String>,
) -> impl IntoView {
    let has_error = !error.is_empty();
    let ring = if has_error {
        "border-danger focus:ring-danger/30"
    } else {
        "border-gray-200 focus:ring-primary/30 focus:border-primary"
    };

    let textarea_class = format!(
        "w-full px-4 py-2.5 min-h-[44px] border rounded-xl focus:outline-none focus:ring-2 \
         text-txt-primary {ring}"
    );

    view! {
        <div class="flex flex-col gap-1">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-txt-secondary">{label}</label>
                })
            } else {
                None
            }}
            <textarea
                class=textarea_class
                rows=rows
                placeholder=placeholder
                prop:value=move || value.get()
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                }
            />
            {if has_error {
                Some(view! {
                    <p class="text-sm text-danger">{error}</p>
                })
            } else {
                None
            }}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Select
// ---------------------------------------------------------------------------

/// A dropdown select input with optional label, using design-token colors.
///
/// # Props
/// - `label` – Text displayed above the select.
/// - `options` – List of `(value, display_text)` pairs for `<option>` elements.
/// - `value` – Two-way bound `RwSignal<String>` for the selected value.
///
/// # Usage
/// ```rust
/// let region = RwSignal::new(String::new());
/// let opts = vec![("seoul".into(), "서울".into()), ("busan".into(), "부산".into())];
/// view! { <Select label="지역" options=opts value=region /> }
/// ```
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
                    <label class="text-sm font-medium text-txt-secondary">{label}</label>
                })
            } else {
                None
            }}
            <select
                class="w-full px-4 py-2.5 min-h-[44px] border border-gray-200 rounded-xl \
                       focus:outline-none focus:ring-2 focus:ring-primary/30 \
                       focus:border-primary text-txt-primary bg-surface-card"
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

/// A styled checkbox with an optional inline label.
///
/// # Props
/// - `label` – Text displayed beside the checkbox.
/// - `checked` – Two-way bound `RwSignal<bool>` for the checked state.
///
/// # Usage
/// ```rust
/// let agree = RwSignal::new(false);
/// view! { <Checkbox label="약관에 동의합니다" checked=agree /> }
/// ```
#[component]
pub fn Checkbox(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] checked: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <label class="inline-flex items-center gap-3 cursor-pointer min-h-[44px]">
            <input
                type="checkbox"
                class="h-5 w-5 rounded border-gray-300 text-primary \
                       focus:ring-primary/30"
                prop:checked=move || checked.get()
                on:change=move |ev| {
                    checked.set(event_target_checked(&ev));
                }
            />
            {if !label.is_empty() {
                Some(view! { <span class="text-sm text-txt-secondary">{label}</span> })
            } else {
                None
            }}
        </label>
    }
}

// ---------------------------------------------------------------------------
// RadioGroup
// ---------------------------------------------------------------------------

/// A group of radio buttons rendered from a list of options.
///
/// # Props
/// - `label` – Optional group label displayed above the radio buttons.
/// - `name` – HTML `name` attribute shared by all radios in the group.
/// - `options` – List of `(value, display_text)` pairs for each radio button.
/// - `value` – Two-way bound `RwSignal<String>` for the currently selected value.
///
/// # Usage
/// ```rust
/// let gender = RwSignal::new(String::new());
/// let opts = vec![("male".into(), "남성".into()), ("female".into(), "여성".into())];
/// view! { <RadioGroup name="gender" options=opts value=gender /> }
/// ```
#[component]
pub fn RadioGroup(
    #[prop(into, optional)] label: String,
    #[prop(into)] name: String,
    #[prop(into)] options: Vec<(String, String)>,
    #[prop(into, optional)] value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <fieldset class="flex flex-col gap-2">
            {if !label.is_empty() {
                Some(view! {
                    <legend class="text-sm font-medium text-txt-secondary mb-1">{label}</legend>
                })
            } else {
                None
            }}
            {options
                .into_iter()
                .map(|(opt_val, opt_text)| {
                    let name = name.clone();
                    let val_for_check = opt_val.clone();
                    let val_for_set = opt_val.clone();
                    view! {
                        <label class="inline-flex items-center gap-2 cursor-pointer">
                            <input
                                type="radio"
                                name=name
                                value=opt_val
                                class="h-4 w-4 border-gray-300 text-primary \
                                       focus:ring-primary/30"
                                prop:checked=move || value.get() == val_for_check
                                on:change={
                                    let val = val_for_set.clone();
                                    move |_| value.set(val.clone())
                                }
                            />
                            <span class="text-sm text-txt-primary">{opt_text}</span>
                        </label>
                    }
                })
                .collect_view()}
        </fieldset>
    }
}

// ---------------------------------------------------------------------------
// Switch
// ---------------------------------------------------------------------------

/// A toggle switch that acts as an on/off control, accent-color aware.
///
/// # Props
/// - `label` – Text displayed beside the switch.
/// - `checked` – Two-way bound `RwSignal<bool>` for the toggle state.
///
/// The active track color uses `var(--portal-accent)` so each portal
/// (senior, family, caregiver, etc.) gets its own accent automatically.
///
/// # Usage
/// ```rust
/// let notifications = RwSignal::new(true);
/// view! { <Switch label="알림 받기" checked=notifications /> }
/// ```
#[component]
pub fn Switch(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] checked: RwSignal<bool>,
) -> impl IntoView {
    let track = move || {
        if checked.get() {
            "bg-[var(--portal-accent)]"
        } else {
            "bg-gray-200"
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
        <label class="inline-flex items-center gap-3 cursor-pointer select-none min-h-[44px]">
            <button
                type="button"
                role="switch"
                class=move || {
                    format!(
                        "relative inline-flex h-7 w-12 shrink-0 rounded-full \
                         transition-colors focus:outline-none focus:ring-2 \
                         focus:ring-[var(--portal-accent)]/30 focus:ring-offset-2 {}",
                        track(),
                    )
                }
                on:click=move |_| checked.set(!checked.get_untracked())
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
            {if !label.is_empty() {
                Some(view! { <span class="text-sm text-txt-secondary">{label}</span> })
            } else {
                None
            }}
        </label>
    }
}

// ---------------------------------------------------------------------------
// Label
// ---------------------------------------------------------------------------

/// A standalone form label element using the `txt-secondary` design token.
///
/// # Props
/// - `text` – The label content.
/// - `for_id` – Optional HTML `for` attribute linking to an input's `id`.
///
/// # Usage
/// ```rust
/// view! { <Label text="이메일" for_id="email-input" /> }
/// ```
#[component]
pub fn Label(
    #[prop(into)] text: String,
    #[prop(into, optional)] for_id: String,
) -> impl IntoView {
    view! {
        <label for=for_id class="text-sm font-medium text-txt-secondary">
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
    Xl,
}

/// A circular avatar that displays an image or a placeholder initial.
///
/// # Props
/// - `src` – URL of the avatar image. When empty, renders a placeholder circle
///   with the first character of `alt` using portal-accent colors.
/// - `alt` – Alt text for the image / initial source (defaults to `"avatar"`).
/// - `size` – Diameter: `Sm` (32px), `Md` (40px), `Lg` (56px), `Xl` (64px).
///
/// # Usage
/// ```rust
/// view! { <Avatar src="/img/user.jpg" alt="김민수" size=AvatarSize::Lg /> }
/// ```
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
        AvatarSize::Xl => "h-16 w-16",
    };

    let class = format!("{size_cls} rounded-full object-cover");

    if src.is_empty() {
        let placeholder_cls = format!(
            "{size_cls} rounded-full bg-[var(--portal-accent-light)] \
             flex items-center justify-center"
        );
        view! {
            <div class=placeholder_cls>
                <span class="text-[var(--portal-accent)] text-sm font-medium">
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

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BadgeStyle {
    #[default]
    Filled,
    Dot,
}

/// A small status indicator badge with semantic color variants.
///
/// # Props
/// - `text` – The badge label.
/// - `variant` – Color scheme: `Default` (gray), `Success` (green), `Warning`
///   (amber), `Danger` (red), or `Info` (blue). Each uses design-token colors.
/// - `style` – `Filled` (default) or `Dot` (prepends a small colored circle).
///
/// # Usage
/// ```rust
/// view! { <Badge text="활성" variant=BadgeVariant::Success /> }
/// view! { <Badge text="검토 중" variant=BadgeVariant::Warning style=BadgeStyle::Dot /> }
/// ```
#[component]
pub fn Badge(
    #[prop(into)] text: String,
    #[prop(into, optional)] variant: BadgeVariant,
    #[prop(into, optional)] style: BadgeStyle,
) -> impl IntoView {
    let (bg_cls, text_cls, dot_cls) = match variant {
        BadgeVariant::Default => (
            "bg-gray-100",
            "text-txt-secondary",
            "bg-txt-tertiary",
        ),
        BadgeVariant::Success => (
            "bg-success-light",
            "text-success",
            "bg-success",
        ),
        BadgeVariant::Warning => (
            "bg-warning-light",
            "text-warning",
            "bg-warning",
        ),
        BadgeVariant::Danger => (
            "bg-danger-light",
            "text-danger",
            "bg-danger",
        ),
        BadgeVariant::Info => (
            "bg-primary-light",
            "text-primary",
            "bg-primary",
        ),
    };

    let class = format!(
        "inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full \
         text-xs font-medium {bg_cls} {text_cls}"
    );

    let show_dot = style == BadgeStyle::Dot;

    view! {
        <span class=class>
            {if show_dot {
                Some(view! {
                    <span class=format!("inline-block h-1.5 w-1.5 rounded-full {dot_cls}") />
                })
            } else {
                None
            }}
            {text}
        </span>
    }
}

// ---------------------------------------------------------------------------
// Utility -- checked helper
// ---------------------------------------------------------------------------

fn event_target_checked(ev: &leptos::ev::Event) -> bool {
    use leptos::wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok())
        .map(|el| el.checked())
        .unwrap_or(false)
}
