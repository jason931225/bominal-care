use leptos::prelude::*;

/// Sign-in page with passkey, OAuth buttons, and demo login form.
#[component]
pub fn SignInPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 px-4">
            <div class="w-full max-w-md space-y-8">
                // Logo and title
                <div class="text-center">
                    <div class="mx-auto w-16 h-16 bg-blue-600 rounded-2xl flex items-center justify-center mb-4">
                        <svg class="w-10 h-10 text-white" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z" />
                        </svg>
                    </div>
                    <h1 class="text-2xl font-bold text-gray-900">"시니어케어 포털"</h1>
                    <p class="mt-2 text-sm text-gray-600">"안전하고 편리한 돌봄 서비스"</p>
                </div>

                <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-8 space-y-6">
                    // Passkey login
                    <button class="w-full flex items-center justify-center gap-3 px-4 py-3 bg-blue-600 text-white font-medium rounded-xl hover:bg-blue-700 transition-colors">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z" />
                        </svg>
                        "패스키로 로그인"
                    </button>

                    // Divider
                    <div class="relative">
                        <div class="absolute inset-0 flex items-center">
                            <div class="w-full border-t border-gray-200" />
                        </div>
                        <div class="relative flex justify-center text-sm">
                            <span class="px-4 bg-white text-gray-500">"또는"</span>
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
                            "Google로 로그인"
                        </button>

                        <button class="w-full flex items-center justify-center gap-3 px-4 py-3 bg-[#03C75A] text-white rounded-xl text-sm font-medium hover:bg-[#02b351] transition-colors">
                            <span class="font-bold text-lg">"N"</span>
                            "네이버로 로그인"
                        </button>

                        <button class="w-full flex items-center justify-center gap-3 px-4 py-3 bg-[#FEE500] text-[#191919] rounded-xl text-sm font-medium hover:bg-[#fdd800] transition-colors">
                            <svg class="w-5 h-5" viewBox="0 0 24 24" fill="#191919">
                                <path d="M12 3C6.48 3 2 6.69 2 11.2c0 2.89 1.93 5.42 4.83 6.86l-1.23 4.56c-.11.42.36.76.72.52l5.45-3.61c.07 0 .15.01.23.01 5.52 0 10-3.69 10-8.34C22 6.69 17.52 3 12 3z" />
                            </svg>
                            "카카오로 로그인"
                        </button>
                    </div>

                    // Divider
                    <div class="relative">
                        <div class="absolute inset-0 flex items-center">
                            <div class="w-full border-t border-gray-200" />
                        </div>
                        <div class="relative flex justify-center text-sm">
                            <span class="px-4 bg-white text-gray-500">"데모 로그인"</span>
                        </div>
                    </div>

                    // Demo login form
                    <form class="space-y-4" action="/auth/signin" method="post">
                        <div>
                            <label for="demo-role" class="block text-sm font-medium text-gray-700 mb-1">
                                "역할 선택"
                            </label>
                            <select
                                id="demo-role"
                                name="role"
                                class="w-full px-3 py-2.5 border border-gray-300 rounded-xl text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            >
                                <option value="senior">"어르신 (시니어)"</option>
                                <option value="family">"가족 보호자"</option>
                                <option value="caregiver">"요양보호사"</option>
                                <option value="internal">"기관 관리자"</option>
                                <option value="government">"정부 담당자"</option>
                            </select>
                        </div>
                        <button
                            type="submit"
                            class="w-full px-4 py-3 bg-gray-900 text-white font-medium rounded-xl hover:bg-gray-800 transition-colors"
                        >
                            "데모 계정으로 시작"
                        </button>
                    </form>
                </div>

                // Footer
                <p class="text-center text-xs text-gray-500">
                    "로그인하면 "
                    <a href="#" class="underline hover:text-gray-700">"이용약관"</a>
                    " 및 "
                    <a href="#" class="underline hover:text-gray-700">"개인정보처리방침"</a>
                    "에 동의하게 됩니다."
                </p>
            </div>
        </div>
    }
}

/// Error page displaying an auth error message.
#[component]
pub fn ErrorPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 px-4">
            <div class="w-full max-w-md text-center">
                <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-8 space-y-6">
                    // Error icon
                    <div class="mx-auto w-16 h-16 bg-red-100 rounded-full flex items-center justify-center">
                        <svg class="w-8 h-8 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                        </svg>
                    </div>

                    <div>
                        <h1 class="text-xl font-bold text-gray-900">"인증 오류"</h1>
                        <p class="mt-2 text-sm text-gray-600">
                            "로그인 중 오류가 발생했습니다. 다시 시도해 주세요."
                        </p>
                    </div>

                    <div class="space-y-3">
                        <a
                            href="/auth/signin"
                            class="block w-full px-4 py-3 bg-blue-600 text-white font-medium rounded-xl hover:bg-blue-700 transition-colors text-center"
                        >
                            "다시 로그인"
                        </a>
                        <a
                            href="/"
                            class="block w-full px-4 py-3 border border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50 transition-colors text-center"
                        >
                            "홈으로 돌아가기"
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
