use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

// ---------------------------------------------------------------------------
// LoadingSpinner
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Animated SVG spinner with configurable size.
///
/// Uses `text-primary` design token for color.
#[component]
pub fn LoadingSpinner(
    #[prop(into, optional)] size: SpinnerSize,
) -> impl IntoView {
    let size_cls = match size {
        SpinnerSize::Sm => "h-4 w-4",
        SpinnerSize::Md => "h-8 w-8",
        SpinnerSize::Lg => "h-12 w-12",
    };

    let cls = format!("animate-spin {size_cls} text-primary");

    view! {
        <svg class=cls xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10"
                stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
        </svg>
    }
}

// ---------------------------------------------------------------------------
// LoadingPage
// ---------------------------------------------------------------------------

/// Full-screen loading state with a centered spinner on the page background.
#[component]
pub fn LoadingPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-surface-page">
            <LoadingSpinner size=SpinnerSize::Lg />
        </div>
    }
}

// ---------------------------------------------------------------------------
// Skeleton
// ---------------------------------------------------------------------------

/// Generic skeleton placeholder with configurable width and height.
///
/// Uses the `.skeleton` CSS class for shimmer animation.
#[component]
pub fn Skeleton(
    #[prop(into, optional, default = "100%".into())] width: String,
    #[prop(into, optional, default = "1rem".into())] height: String,
) -> impl IntoView {
    let style = format!("width:{width};height:{height}");

    view! {
        <div class="skeleton" style=style />
    }
}

/// Multiple skeleton text lines with varying widths.
///
/// Renders `lines` rows (default 3) with the last line shorter to mimic
/// natural paragraph endings.
#[component]
pub fn SkeletonText(
    #[prop(optional, default = 3)] lines: u8,
) -> impl IntoView {
    let widths: Vec<&str> = (0..lines)
        .map(|i| {
            if i == lines - 1 {
                "60%"
            } else if i % 2 == 0 {
                "100%"
            } else {
                "85%"
            }
        })
        .collect();

    view! {
        <div class="flex flex-col gap-2">
            {widths
                .into_iter()
                .map(|w| {
                    let style = format!("width:{w};height:0.75rem");
                    view! { <div class="skeleton" style=style /> }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

/// Circular skeleton placeholder, commonly used for avatars.
#[component]
pub fn SkeletonCircle(
    #[prop(into, optional, default = "3rem".into())] size: String,
) -> impl IntoView {
    let style = format!("width:{size};height:{size}");

    view! {
        <div class="skeleton rounded-full" style=style />
    }
}

/// Card-shaped skeleton with a header block and three text lines.
#[component]
pub fn SkeletonCard() -> impl IntoView {
    view! {
        <div class="bg-surface-card rounded-2xl shadow-sm p-5 space-y-4">
            <div class="skeleton" style="width:40%;height:1.25rem" />
            <SkeletonText lines=3 />
        </div>
    }
}

// ---------------------------------------------------------------------------
// Toast
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Error,
    Warning,
}

/// Notification toast that auto-dismisses after 3 seconds.
///
/// Position is controlled by the `.toast-container` CSS class (fixed,
/// centered at top). Uses `animate-slide-up` for the entrance animation.
///
/// # Props
/// - `message` — display text
/// - `variant` — color scheme (`Info`, `Success`, `Error`, `Warning`)
/// - `visible` — reactive boolean controlling visibility
#[component]
pub fn Toast(
    #[prop(into)] message: String,
    #[prop(into, optional)] variant: ToastVariant,
    visible: RwSignal<bool>,
) -> impl IntoView {
    let (icon, accent) = match variant {
        ToastVariant::Info => ("\u{2139}\u{fe0f}", "text-primary"),
        ToastVariant::Success => ("\u{2705}", "text-success"),
        ToastVariant::Error => ("\u{274c}", "text-danger"),
        ToastVariant::Warning => ("\u{26a0}\u{fe0f}", "text-warning"),
    };

    // Auto-dismiss after 3 seconds whenever the toast becomes visible.
    Effect::new(move |_| {
        if visible.get() {
            if let Some(window) = leptos::web_sys::window() {
                let cb = leptos::wasm_bindgen::prelude::Closure::once_into_js(move || {
                    visible.set(false);
                });
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    3000,
                );
            }
        }
    });

    let cls = format!(
        "toast-container"
    );

    let item_cls = format!(
        "flex items-center gap-3 px-5 py-3 bg-surface-card rounded-2xl \
         shadow-lg animate-slide-up {accent}"
    );

    view! {
        <Show when=move || visible.get()>
            <div class=cls.clone()>
                <div class=item_cls.clone()>
                    <span class="text-lg">{icon}</span>
                    <p class="text-sm font-medium text-txt-primary">{message.clone()}</p>
                    <button
                        class="ml-auto text-txt-tertiary hover:text-txt-primary"
                        on:click=move |_| visible.set(false)
                    >
                        "\u{2715}"
                    </button>
                </div>
            </div>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// AlertBanner
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AlertVariant {
    #[default]
    Info,
    Warning,
    Error,
    Success,
}

/// Dismissible alert banner with icon, message, and optional close button.
///
/// Uses token colors with shadow instead of borders:
/// - Info: `bg-primary-light text-primary`
/// - Warning: `bg-warning-light text-warning`
/// - Error: `bg-danger-light text-danger`
/// - Success: `bg-success-light text-success`
#[component]
pub fn AlertBanner(
    #[prop(into, optional)] variant: AlertVariant,
    #[prop(into)] message: String,
    #[prop(optional)] dismissible: bool,
) -> impl IntoView {
    let dismissed = RwSignal::new(false);

    let (bg, text, icon) = match variant {
        AlertVariant::Info => ("bg-primary-light", "text-primary", "\u{2139}\u{fe0f}"),
        AlertVariant::Warning => ("bg-warning-light", "text-warning", "\u{26a0}\u{fe0f}"),
        AlertVariant::Error => ("bg-danger-light", "text-danger", "\u{274c}"),
        AlertVariant::Success => ("bg-success-light", "text-success", "\u{2705}"),
    };

    let cls = format!(
        "flex items-center gap-3 p-4 rounded-2xl shadow-sm {bg} {text}"
    );

    view! {
        <Show when=move || !dismissed.get()>
            <div class=cls.clone()>
                <span>{icon}</span>
                <p class="flex-1 text-sm">{message.clone()}</p>
                {if dismissible {
                    Some(view! {
                        <button
                            class="ml-auto text-current opacity-50 hover:opacity-100"
                            on:click=move |_| dismissed.set(true)
                        >
                            "\u{2715}"
                        </button>
                    })
                } else {
                    None
                }}
            </div>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// ConfirmDialog
// ---------------------------------------------------------------------------

/// Modal confirmation dialog with backdrop blur and fade-in animation.
///
/// Renders a centered card over a semi-transparent backdrop. The caller
/// controls visibility via `open` and receives confirmation through
/// `on_confirm`. Cancellation simply closes the dialog.
///
/// # Props
/// - `title` — dialog heading
/// - `message` — body text
/// - `confirm_label` — confirm button text (default "확인")
/// - `cancel_label` — cancel button text (default "취소")
/// - `open` — reactive boolean controlling visibility
/// - `on_confirm` — callback fired when the confirm button is clicked
#[component]
pub fn ConfirmDialog(
    #[prop(into)] title: String,
    #[prop(into)] message: String,
    #[prop(into, optional, default = "확인".into())] confirm_label: String,
    #[prop(into, optional, default = "취소".into())] cancel_label: String,
    open: RwSignal<bool>,
    on_confirm: Callback<()>,
) -> impl IntoView {
    let on_cancel = move |_| {
        open.set(false);
    };

    let on_confirm_click = move |_| {
        on_confirm.run(());
        open.set(false);
    };

    view! {
        <Show when=move || open.get()>
            <div
                class="fixed inset-0 z-50 flex items-center justify-center \
                       bg-black/40 backdrop-blur-sm animate-fade-in"
                on:click=on_cancel
            >
                <div
                    class="bg-surface-card rounded-2xl shadow-xl max-w-sm mx-auto \
                           w-full p-6 animate-slide-up"
                    on:click=move |ev| ev.stop_propagation()
                >
                    <h2 class="text-lg font-semibold text-txt-primary">
                        {title.clone()}
                    </h2>
                    <p class="mt-2 text-sm text-txt-secondary">
                        {message.clone()}
                    </p>
                    <div class="mt-6 flex justify-end gap-3">
                        <button
                            class="px-4 py-2 text-sm font-medium text-txt-secondary \
                                   rounded-xl hover:bg-surface-subtle \
                                   active:scale-[0.98] transition-all"
                            on:click=on_cancel
                        >
                            {cancel_label.clone()}
                        </button>
                        <button
                            class="px-4 py-2 text-sm font-medium text-white \
                                   bg-primary rounded-xl hover:bg-primary-hover \
                                   active:scale-[0.98] transition-all"
                            on:click=on_confirm_click
                        >
                            {confirm_label.clone()}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// ErrorPage
// ---------------------------------------------------------------------------

/// Full-screen error page displaying an HTTP status code and message.
///
/// Includes a "홈으로 돌아가기" (go home) link styled with design tokens.
#[component]
pub fn ErrorPage(
    #[prop(optional, default = 500)] status_code: u16,
    #[prop(into, optional)] message: String,
) -> impl IntoView {
    let msg = if message.is_empty() {
        "오류가 발생했습니다".to_string()
    } else {
        message
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-surface-page">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-txt-disabled">
                    {status_code.to_string()}
                </h1>
                <p class="mt-4 text-lg text-txt-secondary">{msg}</p>
                <a
                    href="/"
                    class="mt-6 inline-block px-6 py-3 bg-primary text-white \
                           rounded-xl hover:bg-primary-hover active:scale-[0.98] \
                           transition-all"
                >
                    "홈으로 돌아가기"
                </a>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// NotFoundPage
// ---------------------------------------------------------------------------

/// Convenience wrapper around `ErrorPage` for 404 responses.
#[component]
pub fn NotFoundPage(
    #[prop(into, optional)] message: String,
) -> impl IntoView {
    let msg = if message.is_empty() {
        "페이지를 찾을 수 없습니다".to_string()
    } else {
        message
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-surface-page">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-txt-disabled">"404"</h1>
                <p class="mt-4 text-lg text-txt-secondary">{msg}</p>
                <a
                    href="/"
                    class="mt-6 inline-block px-6 py-3 bg-primary text-white \
                           rounded-xl hover:bg-primary-hover active:scale-[0.98] \
                           transition-all"
                >
                    "홈으로 돌아가기"
                </a>
            </div>
        </div>
    }
}
