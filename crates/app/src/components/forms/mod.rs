use leptos::prelude::*;

// ---------------------------------------------------------------------------
// FormField
// ---------------------------------------------------------------------------

/// A wrapper that renders a label, child content, and an optional error message.
///
/// Uses design-token colors for the label (`text-txt-secondary`), error text
/// (`text-danger`), and the required asterisk (`text-danger`).
#[component]
pub fn FormField(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] error: String,
    #[prop(optional)] required: bool,
    children: Children,
) -> impl IntoView {
    let has_error = !error.is_empty();

    view! {
        <div class="flex flex-col gap-1.5">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-txt-secondary">
                        {label}
                        {if required {
                            Some(view! { <span class="text-danger ml-0.5">"*"</span> })
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
                    <p class="text-sm text-danger">{error}</p>
                })
            } else {
                None
            }}
        </div>
    }
}

// ---------------------------------------------------------------------------
// DatePicker
// ---------------------------------------------------------------------------

/// A native date input styled with the Toss design-token palette.
///
/// Renders an optional label in `text-txt-secondary` and an `<input type="date">`
/// with `rounded-xl`, token border/focus colors, and `text-txt-primary`.
#[component]
pub fn DatePicker(
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional)] label: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1.5">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-txt-secondary">{label}</label>
                })
            } else {
                None
            }}
            <input
                type="date"
                class="w-full px-3 py-2.5 border border-gray-200 rounded-xl \
                       focus:outline-none focus:ring-2 focus:ring-primary/30 \
                       focus:border-primary text-txt-primary \
                       transition-colors"
                prop:value=move || value.get()
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                }
            />
        </div>
    }
}

// ---------------------------------------------------------------------------
// PhoneInput
// ---------------------------------------------------------------------------

/// A Korean phone-number input that auto-formats digits into `010-1234-5678`.
///
/// Shares the same styling as `DatePicker`: `rounded-xl`, token border/focus
/// colors, and `text-txt-primary`. The label reads "전화번호" by default.
#[component]
pub fn PhoneInput(
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(into, optional, default = "010-0000-0000".into())] placeholder: String,
    #[prop(into, optional, default = "전화번호".into())] label: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1.5">
            {if !label.is_empty() {
                Some(view! {
                    <label class="text-sm font-medium text-txt-secondary">{label}</label>
                })
            } else {
                None
            }}
            <input
                type="tel"
                class="w-full px-3 py-2.5 border border-gray-200 rounded-xl \
                       focus:outline-none focus:ring-2 focus:ring-primary/30 \
                       focus:border-primary text-txt-primary \
                       transition-colors"
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

/// Format digits into Korean phone format: `010-1234-5678`.
///
/// Strips all non-digit characters, then inserts hyphens at positions 3 and 7
/// (for 11-digit numbers) following the standard Korean mobile format.
pub fn format_korean_phone(input: &str) -> String {
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

/// A Toss-style search bar with an inline magnifying-glass icon.
///
/// Starts with a subtle background (`bg-surface-subtle`, `border-0`) and gains
/// a white background with a primary focus ring on focus. Fires `on_search`
/// when the user presses Enter.
#[component]
pub fn SearchInput(
    #[prop(into, optional, default = "검색...".into())] placeholder: String,
    #[prop(into, optional)] on_search: Option<Callback<String>>,
) -> impl IntoView {
    let query = RwSignal::new(String::new());

    view! {
        <div class="relative">
            <span class="absolute inset-y-0 left-3 flex items-center text-txt-disabled pointer-events-none">
                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M21 21l-4.35-4.35M11 19a8 8 0 100-16 8 8 0 000 16z" />
                </svg>
            </span>
            <input
                type="search"
                class="w-full pl-10 pr-4 py-2.5 \
                       bg-surface-subtle border-0 rounded-xl \
                       focus:bg-surface-card focus:ring-2 focus:ring-primary/30 \
                       focus:outline-none \
                       text-txt-primary placeholder:text-txt-disabled \
                       transition-colors"
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
// FileUpload
// ---------------------------------------------------------------------------

/// A drag-and-drop file upload zone with an upload-cloud icon.
///
/// Renders a dashed border area that highlights on hover and accepts clicks to
/// open the native file picker via a hidden `<input type="file">`. When a file
/// is selected, the `on_file` callback fires with the filename.
///
/// Props:
/// - `label` — optional descriptive text shown below the icon.
/// - `accept` — optional MIME filter (e.g. `"image/*"`).
/// - `on_file` — callback receiving the selected filename.
#[component]
pub fn FileUpload(
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] accept: String,
    #[prop(into, optional)] on_file: Option<Callback<String>>,
) -> impl IntoView {
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let is_dragging = RwSignal::new(false);
    let selected_name = RwSignal::new(String::new());

    let display_label = if label.is_empty() {
        "파일을 드래그하거나 클릭하여 업로드".to_string()
    } else {
        label
    };

    let on_file_selected = {
        let on_file = on_file.clone();
        move |ev: leptos::ev::Event| {
            let val = event_target_value(&ev);
            let name = val.rsplit(['/', '\\']).next().unwrap_or(&val).to_string();
            if !name.is_empty() {
                selected_name.set(name.clone());
                if let Some(cb) = &on_file {
                    cb.run(name);
                }
            }
        }
    };

    let zone_class = move || {
        let base = "relative flex flex-col items-center justify-center gap-3 \
                    p-8 rounded-2xl cursor-pointer transition-colors";
        if is_dragging.get() {
            format!("{base} border-2 border-dashed border-primary bg-primary-light/50")
        } else {
            format!("{base} border-2 border-dashed border-gray-200 \
                     hover:border-primary hover:bg-primary-light/50")
        }
    };

    view! {
        <div
            class=zone_class
            on:click=move |_| {
                if let Some(el) = input_ref.get() {
                    let _ = el.click();
                }
            }
            on:dragover=move |ev| {
                ev.prevent_default();
                is_dragging.set(true);
            }
            on:dragleave=move |_| {
                is_dragging.set(false);
            }
            on:drop=move |ev| {
                ev.prevent_default();
                is_dragging.set(false);
            }
        >
            // Upload cloud icon (inline SVG)
            <svg class="h-10 w-10 text-txt-tertiary" fill="none" stroke="currentColor"
                 viewBox="0 0 24 24" stroke-width="1.5">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 \
                     5.25 5.25 0 0110.338-2.32 3.75 3.75 0 013.572 5.345A4.5 4.5 0 0118 19.5H6.75z" />
            </svg>

            <p class="text-sm text-txt-tertiary">{display_label}</p>

            {move || {
                let name = selected_name.get();
                if name.is_empty() {
                    None
                } else {
                    Some(view! {
                        <p class="text-xs text-txt-secondary font-medium">{name}</p>
                    })
                }
            }}

            <input
                node_ref=input_ref
                type="file"
                class="hidden"
                accept=accept
                on:change=on_file_selected
            />
        </div>
    }
}

// ---------------------------------------------------------------------------
// StepWizard
// ---------------------------------------------------------------------------

/// A horizontal step indicator with numbered circles and animated connectors.
///
/// Completed steps show a filled accent circle; the current step shows an
/// outlined accent circle; upcoming steps are gray. The connector bar between
/// steps animates its fill width with `transition-all duration-300`.
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

    let step_count = steps.len();

    view! {
        <div class="flex items-center w-full">
            {steps
                .into_iter()
                .enumerate()
                .map(|(idx, label)| {
                    let (circle_cls, text_cls) = if idx < current_step {
                        // completed
                        (
                            "flex items-center justify-center h-8 w-8 rounded-full \
                             bg-[var(--portal-accent)] text-white text-sm font-semibold \
                             transition-all duration-300",
                            "text-xs mt-1 text-[var(--portal-accent)] font-medium",
                        )
                    } else if idx == current_step {
                        // current
                        (
                            "flex items-center justify-center h-8 w-8 rounded-full \
                             border-2 border-[var(--portal-accent)] \
                             text-[var(--portal-accent)] text-sm font-semibold \
                             transition-all duration-300",
                            "text-xs mt-1 text-[var(--portal-accent)] font-medium",
                        )
                    } else {
                        // upcoming
                        (
                            "flex items-center justify-center h-8 w-8 rounded-full \
                             border-2 border-gray-200 text-txt-disabled text-sm \
                             transition-all duration-300",
                            "text-xs mt-1 text-txt-disabled",
                        )
                    };

                    let connector_fill_pct = if idx < current_step {
                        "100%"
                    } else {
                        "0%"
                    };

                    let show_connector = idx < step_count.saturating_sub(1);

                    view! {
                        <div class="flex flex-col items-center shrink-0">
                            <div class=circle_cls>{(idx + 1).to_string()}</div>
                            <span class=text_cls>{label}</span>
                        </div>

                        {if show_connector {
                            Some(view! {
                                <div class="flex-1 h-0.5 mx-1 bg-gray-200 rounded-full \
                                            relative min-w-[1.5rem] self-start mt-4">
                                    <div
                                        class="absolute inset-y-0 left-0 bg-[var(--portal-accent)] \
                                               rounded-full transition-all duration-300"
                                        style:width=connector_fill_pct
                                    />
                                </div>
                            })
                        } else {
                            None
                        }}
                    }
                })
                .collect_view()}
        </div>
    }
}
