use leptos::prelude::*;

// ---------------------------------------------------------------------------
// FormField
// ---------------------------------------------------------------------------

#[component]
pub fn FormField(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] error: String,
    #[prop(optional)] required: bool,
    children: Children,
) -> impl IntoView {
    let has_error = !error.is_empty();

    view! {
        <div class="flex flex-col gap-1">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-gray-700">
                        {label}
                        {if required {
                            Some(view! { <span class="text-red-500 ml-0.5">"*"</span> })
                        } else {
                            None
                        }}
                    </label>
                })
            } else {
                None
            }}
            {children()}
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
// DatePicker (stub — falls back to native date input)
// ---------------------------------------------------------------------------

#[component]
pub fn DatePicker(
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional)] label: String,
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
            <input
                type="date"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg \
                       focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900"
                prop:value=move || value.get()
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                }
            />
        </div>
    }
}

// ---------------------------------------------------------------------------
// PhoneInput (Korean phone format)
// ---------------------------------------------------------------------------

#[component]
pub fn PhoneInput(
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional, default = "010-0000-0000".into())] placeholder: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <label class="text-sm font-medium text-gray-700">"전화번호"</label>
            <input
                type="tel"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg \
                       focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900"
                placeholder=placeholder
                maxlength="13"
                prop:value=move || value.get()
                on:input=move |ev| {
                    let raw = event_target_value(&ev);
                    let formatted = format_korean_phone(&raw);
                    value.set(formatted);
                }
            />
        </div>
    }
}

/// Format digits into Korean phone format: 010-1234-5678
fn format_korean_phone(input: &str) -> String {
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    let len = digits.len();
    if len <= 3 {
        digits
    } else if len <= 7 {
        format!("{}-{}", &digits[..3], &digits[3..])
    } else {
        let mid = if len >= 11 { 7 } else { len.min(7) };
        let end = len.min(11);
        format!("{}-{}-{}", &digits[..3], &digits[3..mid], &digits[mid..end])
    }
}

// ---------------------------------------------------------------------------
// SearchInput
// ---------------------------------------------------------------------------

#[component]
pub fn SearchInput(
    #[prop(into, optional, default = "검색...".into())] placeholder: String,
    #[prop(into, optional)] on_search: Option<Callback<String>>,
) -> impl IntoView {
    let query = RwSignal::new(String::new());

    view! {
        <div class="relative">
            <span class="absolute inset-y-0 left-3 flex items-center text-gray-400">
                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M21 21l-4.35-4.35M11 19a8 8 0 100-16 8 8 0 000 16z" />
                </svg>
            </span>
            <input
                type="search"
                class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg \
                       focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900"
                placeholder=placeholder
                prop:value=move || query.get()
                on:input=move |ev| {
                    query.set(event_target_value(&ev));
                }
                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
                        if let Some(cb) = &on_search {
                            cb.run(query.get_untracked());
                        }
                    }
                }
            />
        </div>
    }
}

// ---------------------------------------------------------------------------
// FileUpload (stub)
// ---------------------------------------------------------------------------

#[component]
pub fn FileUpload() -> impl IntoView {
    view! {
        <div class="border-2 border-dashed border-gray-300 rounded-lg p-8 \
                    text-center text-gray-400 cursor-pointer hover:border-gray-400">
            <p class="text-sm">"파일을 드래그하거나 클릭하여 업로드"</p>
            <p class="text-xs mt-1">"FileUpload – 추후 구현 예정"</p>
        </div>
    }
}

// ---------------------------------------------------------------------------
// StepWizard
// ---------------------------------------------------------------------------

#[component]
pub fn StepWizard(
    #[prop(optional, default = 0)] current_step: usize,
    #[prop(optional, default = 1)] total_steps: usize,
    #[prop(into, optional)] step_labels: Vec<String>,
) -> impl IntoView {
    let steps = if step_labels.is_empty() {
        (1..=total_steps)
            .map(|i| format!("단계 {i}"))
            .collect::<Vec<_>>()
    } else {
        step_labels
    };

    view! {
        <div class="flex items-center gap-2">
            {steps
                .into_iter()
                .enumerate()
                .map(|(idx, label)| {
                    let (circle_cls, text_cls) = if idx < current_step {
                        // completed
                        (
                            "flex items-center justify-center h-8 w-8 rounded-full \
                             bg-blue-600 text-white text-sm font-medium",
                            "text-sm text-blue-600 font-medium",
                        )
                    } else if idx == current_step {
                        // current
                        (
                            "flex items-center justify-center h-8 w-8 rounded-full \
                             border-2 border-blue-600 text-blue-600 text-sm font-medium",
                            "text-sm text-blue-600 font-medium",
                        )
                    } else {
                        // upcoming
                        (
                            "flex items-center justify-center h-8 w-8 rounded-full \
                             border-2 border-gray-300 text-gray-400 text-sm",
                            "text-sm text-gray-400",
                        )
                    };

                    let connector = if idx < total_steps.saturating_sub(1) {
                        Some(view! {
                            <div class="flex-1 h-0.5 bg-gray-300 min-w-[1rem]" />
                        })
                    } else {
                        None
                    };

                    view! {
                        <div class="flex flex-col items-center gap-1">
                            <div class=circle_cls>{(idx + 1).to_string()}</div>
                            <span class=text_cls>{label}</span>
                        </div>
                        {connector}
                    }
                })
                .collect_view()}
        </div>
    }
}
