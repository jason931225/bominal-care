use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Step data for the 7-step application process
// ---------------------------------------------------------------------------

/// Metadata for a single step in the multi-step application form.
struct StepInfo {
    number: u8,
    label: &'static str,
}

/// The seven steps of the long-term care application process.
const STEPS: &[StepInfo] = &[
    StepInfo { number: 1, label: "기본 정보" },
    StepInfo { number: 2, label: "건강 상태" },
    StepInfo { number: 3, label: "돌봄 필요도" },
    StepInfo { number: 4, label: "보호자 정보" },
    StepInfo { number: 5, label: "거주 환경" },
    StepInfo { number: 6, label: "서비스 선택" },
    StepInfo { number: 7, label: "확인 및 제출" },
];

// ---------------------------------------------------------------------------
// Progress bar
// ---------------------------------------------------------------------------

/// Horizontal progress bar showing completion percentage across all steps.
///
/// Uses `--portal-accent` for the fill color and `bg-surface-subtle` for the
/// track, with a smooth width transition.
#[component]
fn ProgressBar(
    /// Current step (1-based).
    current_step: u8,
    /// Total number of steps.
    total_steps: u8,
) -> impl IntoView {
    let pct = if total_steps > 0 {
        ((current_step as f64) / (total_steps as f64) * 100.0) as u32
    } else {
        0
    };
    let width_style = format!("width: {}%", pct);

    view! {
        <div class="w-full bg-surface-subtle rounded-full h-2">
            <div
                class="bg-[var(--portal-accent)] h-2 rounded-full transition-all duration-300"
                style=width_style
            />
        </div>
    }
}

// ---------------------------------------------------------------------------
// Step indicators
// ---------------------------------------------------------------------------

/// Row of numbered circles representing each application step.
///
/// Visual states:
/// - **Completed**: accent background with white checkmark.
/// - **Current**: accent-light background with accent ring and text.
/// - **Upcoming**: subtle background with disabled text.
#[component]
fn StepIndicators(
    /// Current step (1-based).
    current_step: u8,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between gap-1 overflow-x-auto py-2">
            {STEPS.iter().map(|step| {
                let is_done = step.number < current_step;
                let is_current = step.number == current_step;

                let circle_class = if is_done {
                    "w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold \
                     bg-[var(--portal-accent)] text-white"
                } else if is_current {
                    "w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold \
                     bg-[var(--portal-accent-light)] text-[var(--portal-accent)] \
                     ring-2 ring-[var(--portal-accent)]"
                } else {
                    "w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold \
                     bg-surface-subtle text-txt-disabled"
                };

                let label_class = if is_current {
                    "text-xs font-medium mt-1 text-center whitespace-nowrap \
                     text-[var(--portal-accent)]"
                } else if is_done {
                    "text-xs mt-1 text-center whitespace-nowrap \
                     text-[var(--portal-accent)]"
                } else {
                    "text-xs mt-1 text-center whitespace-nowrap \
                     text-txt-disabled"
                };

                view! {
                    <div class="flex flex-col items-center min-w-[3.5rem]">
                        <div class=circle_class>
                            {if is_done {
                                view! {
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                    </svg>
                                }.into_any()
                            } else {
                                view! { <span>{step.number}</span> }.into_any()
                            }}
                        </div>
                        <span class=label_class>{step.label}</span>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Applicant layout — multi-step form
// ---------------------------------------------------------------------------

/// Applicant portal layout for the multi-step long-term care application form.
///
/// Structure:
/// - **Top bar** (sticky): back button, title, step counter, progress bar.
///   Uses `shadow-sm` instead of a hard border.
/// - **Step indicators**: visual step circles beneath the header.
/// - **Main content**: animated with `animate-fade-in`.
/// - **Bottom action bar** (fixed): Previous / Next navigation buttons with
///   an upward shadow instead of a hard border.
#[component]
pub fn ApplicantLayout(children: Children) -> impl IntoView {
    let current_step: u8 = 1;
    let total_steps: u8 = STEPS.len() as u8;

    view! {
        <div class="min-h-screen bg-surface-page">
            // Top bar with back button and step counter
            <header class="sticky top-0 z-40 bg-surface-card shadow-sm">
                <div class="max-w-2xl mx-auto flex items-center justify-between px-4 h-14">
                    // Back button
                    <button
                        class="flex items-center gap-1 text-txt-secondary hover:text-txt-primary transition-colors"
                        aria-label="이전 단계"
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                        </svg>
                        <span class="text-sm font-medium">"이전"</span>
                    </button>

                    // Title
                    <h1 class="text-sm font-semibold text-txt-primary">"장기요양 신청"</h1>

                    // Step counter
                    <span class="text-sm text-txt-tertiary">
                        {format!("{current_step} / {total_steps}")}
                    </span>
                </div>

                // Progress bar
                <div class="max-w-2xl mx-auto px-4 pb-2">
                    <ProgressBar current_step=current_step total_steps=total_steps />
                </div>
            </header>

            // Step indicators
            <div class="max-w-2xl mx-auto px-4 pt-4">
                <StepIndicators current_step=current_step />
            </div>

            // Page content
            <main class="max-w-2xl mx-auto px-4 py-6 animate-fade-in">
                {children()}
            </main>

            // Bottom action bar
            <div class="fixed bottom-0 inset-x-0 bg-surface-card shadow-[0_-2px_8px_rgba(0,0,0,0.06)] z-40">
                <div class="max-w-2xl mx-auto flex items-center justify-between px-4 py-3">
                    <button class="px-6 py-2.5 text-sm font-medium text-txt-secondary bg-surface-subtle rounded-xl hover:bg-gray-200 transition-colors">
                        "이전 단계"
                    </button>
                    <button class="px-6 py-2.5 text-sm font-medium text-white bg-primary rounded-xl hover:bg-primary-hover active:scale-[0.98] transition-all">
                        "다음 단계"
                    </button>
                </div>
            </div>
        </div>
    }
}
