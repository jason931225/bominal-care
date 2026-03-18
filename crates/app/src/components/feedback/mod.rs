use leptos::prelude::*;

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

#[component]
pub fn LoadingSpinner(
    #[prop(into, optional)] size: SpinnerSize,
) -> impl IntoView {
    let size_cls = match size {
        SpinnerSize::Sm => "h-4 w-4",
        SpinnerSize::Md => "h-8 w-8",
        SpinnerSize::Lg => "h-12 w-12",
    };

    let cls = format!("animate-spin {size_cls} text-blue-600");

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

#[component]
pub fn LoadingPage(
    #[prop(into, optional)] message: String,
) -> impl IntoView {
    let msg = if message.is_empty() {
        "로딩 중...".to_string()
    } else {
        message
    };

    view! {
        <div class="min-h-screen flex flex-col items-center justify-center bg-gray-50 gap-4">
            <LoadingSpinner size=SpinnerSize::Lg />
            <p class="text-gray-500 text-sm">{msg}</p>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Skeleton
// ---------------------------------------------------------------------------

#[component]
pub fn Skeleton(
    #[prop(into, optional, default = "100%".into())] width: String,
    #[prop(into, optional, default = "1rem".into())] height: String,
) -> impl IntoView {
    let style = format!("width:{width};height:{height}");

    view! {
        <div
            class="animate-pulse bg-gray-200 rounded"
            style=style
        />
    }
}

// ---------------------------------------------------------------------------
// Toast (stub)
// ---------------------------------------------------------------------------

#[component]
pub fn Toast() -> impl IntoView {
    view! {
        <div class="text-sm text-gray-500 italic">"Toast – 추후 구현 예정"</div>
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

#[component]
pub fn AlertBanner(
    #[prop(into, optional)] variant: AlertVariant,
    #[prop(into)] message: String,
    #[prop(optional)] dismissible: bool,
) -> impl IntoView {
    let dismissed = RwSignal::new(false);

    let (bg, border, text, icon) = match variant {
        AlertVariant::Info => (
            "bg-blue-50",
            "border-blue-300",
            "text-blue-800",
            "\u{2139}\u{fe0f}",
        ),
        AlertVariant::Warning => (
            "bg-yellow-50",
            "border-yellow-300",
            "text-yellow-800",
            "\u{26a0}\u{fe0f}",
        ),
        AlertVariant::Error => (
            "bg-red-50",
            "border-red-300",
            "text-red-800",
            "\u{274c}",
        ),
        AlertVariant::Success => (
            "bg-green-50",
            "border-green-300",
            "text-green-800",
            "\u{2705}",
        ),
    };

    let cls = format!(
        "flex items-center gap-3 p-4 border rounded-lg {bg} {border} {text}"
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
// ConfirmDialog (stub)
// ---------------------------------------------------------------------------

#[component]
pub fn ConfirmDialog() -> impl IntoView {
    view! {
        <div class="text-sm text-gray-500 italic">"ConfirmDialog – 추후 구현 예정"</div>
    }
}

// ---------------------------------------------------------------------------
// ErrorPage
// ---------------------------------------------------------------------------

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
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-gray-300">
                    {status_code.to_string()}
                </h1>
                <p class="mt-4 text-lg text-gray-600">{msg}</p>
                <a
                    href="/"
                    class="mt-6 inline-block px-6 py-3 bg-blue-600 text-white \
                           rounded-lg hover:bg-blue-700"
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
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-gray-300">"404"</h1>
                <p class="mt-4 text-lg text-gray-600">{msg}</p>
                <a
                    href="/"
                    class="mt-6 inline-block px-6 py-3 bg-blue-600 text-white \
                           rounded-lg hover:bg-blue-700"
                >
                    "홈으로 돌아가기"
                </a>
            </div>
        </div>
    }
}
