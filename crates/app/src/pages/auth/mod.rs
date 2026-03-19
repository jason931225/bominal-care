use leptos::prelude::*;

use crate::i18n::t;

/// Demo login request body sent to POST /api/auth/demo.
#[derive(serde::Serialize)]
struct DemoLoginBody {
    email: String,
}

/// Map role selection to demo email.
fn role_to_email(role: &str) -> &'static str {
    match role {
        "senior" => "senior@demo.com",
        "family" => "family@demo.com",
        "caregiver" => "caregiver@demo.com",
        "internal" => "provider@demo.com",
        "government" => "government@demo.com",
        _ => "senior@demo.com",
    }
}

/// Map role selection to redirect path after login.
fn role_to_redirect(role: &str) -> &'static str {
    match role {
        "senior" => "/",
        "family" => "/family",
        "caregiver" => "/caregiver",
        "internal" => "/internal",
        "government" => "/gov",
        _ => "/",
    }
}

/// Sign-in page with passkey authentication, OAuth social login buttons,
/// and a demo login form for development and testing purposes.
#[component]
pub fn SignInPage() -> impl IntoView {
    let (selected_role, set_selected_role) = signal("senior".to_string());
    let (pipa_agreed, set_pipa_agreed) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    // Capture auth signal outside spawn_local (reactive context required)
    let auth = crate::use_auth();

    let on_demo_login = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let role = selected_role.get_untracked();
        let email = role_to_email(&role).to_string();
        let redirect = role_to_redirect(&role).to_string();

        set_loading.set(true);
        set_error_msg.set(None);

        leptos::task::spawn_local(async move {
            let body = DemoLoginBody { email };
            match crate::api::post::<crate::AuthUser, _>("/api/auth/demo", &body).await {
                Ok(resp) if resp.success => {
                    // Update auth context
                    if let Some(user) = resp.data {
                        auth.set(Some(user));
                    }
                    // Redirect to portal
                    if let Some(window) = leptos::web_sys::window() {
                        let _ = window.location().set_href(&redirect);
                    }
                }
                Ok(resp) => {
                    set_error_msg.set(Some(
                        resp.error.unwrap_or_else(|| t("auth.signin.login_failed").to_string()),
                    ));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error_msg.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-surface-page px-4">
            <div class="w-full max-w-md space-y-8">
                // Logo and title
                <div class="text-center">
                    <div class="mx-auto w-20 h-20 bg-primary rounded-3xl flex items-center justify-center mb-4">
                        <svg class="w-10 h-10 text-white" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z" />
                        </svg>
                    </div>
                    <h1 class="text-2xl font-bold text-txt-primary">{t("auth.signin.title")}</h1>
                    <p class="mt-2 text-sm text-txt-secondary">{t("auth.signin.subtitle")}</p>
                </div>

                <div class="bg-surface-card rounded-3xl shadow-lg p-8 space-y-6">
                    // Passkey login
                    <button class="w-full flex items-center justify-center gap-3 px-4 py-3 bg-primary text-white font-medium rounded-xl hover:bg-primary-hover active:scale-[0.98] transition-all">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z" />
                        </svg>
                        {t("auth.signin.passkey")}
                    </button>

                    // Divider
                    <div class="relative">
                        <div class="absolute inset-0 flex items-center">
                            <div class="w-full border-t border-gray-200" />
                        </div>
                        <div class="relative flex justify-center text-sm">
                            <span class="px-4 bg-surface-card text-txt-tertiary">{t("auth.signin.or")}</span>
                        </div>
                    </div>

                    // OAuth buttons
                    <div class="space-y-3">
                        <button class="w-full flex items-center justify-center gap-3 px-4 py-3 border border-gray-300 rounded-xl text-sm font-medium text-gray-700 hover:bg-gray-50 transition-colors">
                            <svg class="w-5 h-5" viewBox="0 0 24 24">
                                <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92a5.06 5.06 0 01-2.2 3.32v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.1z" />
                                <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
                                <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" />
                                <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" />
                            </svg>
                            {t("auth.signin.google")}
                        </button>

                        <button class="w-full flex items-center justify-center gap-3 px-4 py-3 bg-[#03C75A] text-white rounded-xl text-sm font-medium hover:bg-[#02b351] transition-colors">
                            <span class="font-bold text-lg">"N"</span>
                            {t("auth.signin.naver")}
                        </button>

                        <button class="w-full flex items-center justify-center gap-3 px-4 py-3 bg-[#FEE500] text-[#191919] rounded-xl text-sm font-medium hover:bg-[#fdd800] transition-colors">
                            <svg class="w-5 h-5" viewBox="0 0 24 24" fill="#191919">
                                <path d="M12 3C6.48 3 2 6.69 2 11.2c0 2.89 1.93 5.42 4.83 6.86l-1.23 4.56c-.11.42.36.76.72.52l5.45-3.61c.07 0 .15.01.23.01 5.52 0 10-3.69 10-8.34C22 6.69 17.52 3 12 3z" />
                            </svg>
                            {t("auth.signin.kakao")}
                        </button>
                    </div>

                    // Divider
                    <div class="relative">
                        <div class="absolute inset-0 flex items-center">
                            <div class="w-full border-t border-gray-200" />
                        </div>
                        <div class="relative flex justify-center text-sm">
                            <span class="px-4 bg-surface-card text-txt-tertiary">{t("auth.signin.demo")}</span>
                        </div>
                    </div>

                    // Demo login form — Leptos-controlled
                    <form class="space-y-4" on:submit=on_demo_login>
                        <div>
                            <label for="demo-role" class="block text-sm font-medium text-txt-primary mb-1">
                                {t("auth.signin.role_select")}
                            </label>
                            <select
                                id="demo-role"
                                class="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:ring-2 focus:ring-primary/30 focus:border-primary"
                                on:change=move |ev| set_selected_role.set(event_target_value(&ev))
                            >
                                <option value="senior">"어르신 (시니어)"</option>
                                <option value="family">"가족 보호자"</option>
                                <option value="caregiver">"요양보호사"</option>
                                <option value="internal">"기관 관리자"</option>
                                <option value="government">"정부 담당자"</option>
                            </select>
                        </div>

                        // PIPA consent checkbox
                        <div class="flex items-start gap-2">
                            <input
                                type="checkbox"
                                id="pipa-consent"
                                class="mt-1 h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary/30"
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_pipa_agreed.set(checked);
                                }
                            />
                            <label for="pipa-consent" class="text-sm text-txt-secondary">
                                {t("auth.signin.pipa_consent")}
                            </label>
                        </div>

                        // Error message
                        {move || error_msg.get().map(|msg| view! {
                            <p class="text-sm text-danger bg-danger-light rounded-xl px-3 py-2">{msg}</p>
                        })}

                        <button
                            type="submit"
                            class="w-full px-4 py-3 font-medium rounded-xl transition-all"
                            class=("bg-primary", move || pipa_agreed.get() && !loading.get())
                            class=("text-white", move || pipa_agreed.get() && !loading.get())
                            class=("hover:bg-primary-hover", move || pipa_agreed.get() && !loading.get())
                            class=("active:scale-[0.98]", move || pipa_agreed.get() && !loading.get())
                            class=("bg-surface-subtle", move || !pipa_agreed.get() || loading.get())
                            class=("text-txt-disabled", move || !pipa_agreed.get() || loading.get())
                            class=("cursor-not-allowed", move || !pipa_agreed.get() || loading.get())
                            disabled=move || !pipa_agreed.get() || loading.get()
                        >
                            {move || if loading.get() { t("auth.signin.logging_in") } else { t("auth.signin.demo_start") }}
                        </button>
                    </form>
                </div>

                // Footer
                <p class="text-center text-xs text-txt-tertiary">
                    "로그인하면 "
                    <a href="/terms" class="underline hover:text-txt-primary">{t("auth.signin.terms_link")}</a>
                    " 및 "
                    <a href="/privacy" class="underline hover:text-txt-primary">{t("auth.signin.privacy_link")}</a>
                    "에 동의하게 됩니다."
                </p>
            </div>
        </div>
    }
}

/// Helper to get checkbox checked state from an event.
fn event_target_checked(ev: &leptos::ev::Event) -> bool {
    use wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok())
        .map(|el| el.checked())
        .unwrap_or(false)
}

/// Terms of service page displaying the platform usage agreement
/// in accordance with Korean electronic commerce regulations.
#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-surface-page px-4 py-12">
            <div class="max-w-2xl mx-auto bg-surface-card rounded-2xl shadow-sm p-8">
                <h1 class="text-2xl font-bold text-txt-primary mb-6">{t("auth.terms.title")}</h1>
                <div class="prose prose-sm text-txt-secondary space-y-4">
                    <h2 class="text-lg font-semibold text-txt-primary">"제1조 (목적)"</h2>
                    <p>"본 약관은 Bominal Care(이하 '서비스')의 이용 조건 및 절차, 이용자와 서비스 제공자의 권리, 의무 및 책임사항을 규정함을 목적으로 합니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"제2조 (서비스의 내용)"</h2>
                    <p>"서비스는 노인장기요양보험법에 따른 돌봄 서비스 관리, 요양보호사 매칭, 케어 플랜 관리, 복약 관리 등의 기능을 제공합니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"제3조 (이용자의 의무)"</h2>
                    <p>"이용자는 본인의 정보를 정확하게 제공하여야 하며, 타인의 정보를 도용하거나 허위 정보를 입력해서는 안 됩니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"제4조 (서비스 이용 제한)"</h2>
                    <p>"서비스는 관련 법령 및 본 약관에 따라 이용자의 서비스 이용을 제한할 수 있습니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"제5조 (면책)"</h2>
                    <p>"서비스는 천재지변, 시스템 장애 등 불가항력으로 인한 서비스 중단에 대해 책임을 지지 않습니다."</p>
                </div>
                <div class="mt-8">
                    <a href="/auth/signin" class="text-sm text-primary hover:underline">{t("auth.back_to_login")}</a>
                </div>
            </div>
        </div>
    }
}

/// Privacy policy page describing personal data collection and handling
/// in compliance with PIPA (Personal Information Protection Act).
#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-surface-page px-4 py-12">
            <div class="max-w-2xl mx-auto bg-surface-card rounded-2xl shadow-sm p-8">
                <h1 class="text-2xl font-bold text-txt-primary mb-6">{t("auth.privacy.title")}</h1>
                <div class="prose prose-sm text-txt-secondary space-y-4">
                    <h2 class="text-lg font-semibold text-txt-primary">"1. 수집하는 개인정보 항목"</h2>
                    <p>"서비스는 다음과 같은 개인정보를 수집합니다: 이름, 연락처, 주소, 생년월일, 건강 정보, 장기요양등급 정보, 긴급 연락처."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"2. 개인정보의 수집 및 이용 목적"</h2>
                    <p>"수집된 개인정보는 돌봄 서비스 제공, 요양보호사 매칭, 케어 플랜 관리, 복약 알림, 긴급 상황 대응, 서비스 품질 개선을 위해 이용됩니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"3. 개인정보의 보유 및 이용 기간"</h2>
                    <p>"개인정보는 서비스 이용 기간 동안 보유하며, 이용자 탈퇴 시 지체 없이 파기합니다. 단, 관련 법령에 따라 일정 기간 보관이 필요한 경우 해당 기간 동안 보관합니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"4. 개인정보의 제3자 제공"</h2>
                    <p>"서비스는 이용자의 동의 없이 개인정보를 제3자에게 제공하지 않습니다. 단, 법령에 의한 요청이 있는 경우 예외로 합니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"5. 개인정보의 안전성 확보 조치"</h2>
                    <p>"서비스는 개인정보보호법 제29조에 따라 개인정보의 안전성 확보를 위해 암호화, 접근 제한, 접속 기록 보관 등의 조치를 취합니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"6. 이용자의 권리"</h2>
                    <p>"이용자는 언제든지 자신의 개인정보에 대해 열람, 정정, 삭제, 처리 정지를 요청할 수 있습니다. 동의 관리 페이지에서 데이터 공유 설정을 변경할 수 있습니다."</p>
                    <h2 class="text-lg font-semibold text-txt-primary">"7. 개인정보 보호 책임자"</h2>
                    <p>"개인정보 보호 책임자: Bominal Care 운영팀"</p>
                    <p>"연락처: privacy@bominalcare.kr"</p>
                </div>
                <div class="mt-8">
                    <a href="/auth/signin" class="text-sm text-primary hover:underline">{t("auth.back_to_login")}</a>
                </div>
            </div>
        </div>
    }
}

/// Error page displayed when authentication fails, providing options
/// to retry login or navigate back to the home page.
#[component]
pub fn ErrorPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-surface-page px-4">
            <div class="w-full max-w-md text-center">
                <div class="bg-surface-card rounded-2xl shadow-sm p-8 space-y-6">
                    // Error icon
                    <div class="mx-auto w-16 h-16 bg-danger-light rounded-full flex items-center justify-center">
                        <svg class="w-8 h-8 text-danger" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                        </svg>
                    </div>

                    <div>
                        <h1 class="text-xl font-bold text-txt-primary">{t("auth.error.title")}</h1>
                        <p class="mt-2 text-sm text-txt-secondary">
                            "로그인 중 오류가 발생했습니다. 다시 시도해 주세요."
                        </p>
                    </div>

                    <div class="space-y-3">
                        <a
                            href="/auth/signin"
                            class="block w-full px-4 py-3 bg-primary text-white font-medium rounded-xl hover:bg-primary-hover active:scale-[0.98] transition-all text-center"
                        >
                            {t("auth.error.retry")}
                        </a>
                        <a
                            href="/"
                            class="block w-full px-4 py-3 border-2 border-gray-200 text-txt-secondary font-medium rounded-xl hover:bg-surface-subtle transition-all text-center"
                        >
                            {t("error.go_home")}
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
