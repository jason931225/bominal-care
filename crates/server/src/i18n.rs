// =============================================================================
// Internationalization — translation message store
// Ported from packages/i18n/src/
// =============================================================================

use std::collections::HashMap;

use once_cell::sync::Lazy;

pub const DEFAULT_LOCALE: &str = "ko";
pub const LOCALES: &[&str] = &["ko", "en"];

/// Nested message map: section -> key -> translated string.
pub type Messages = HashMap<&'static str, HashMap<&'static str, &'static str>>;

// =============================================================================
// Korean messages
// =============================================================================

pub static KO_MESSAGES: Lazy<Messages> = Lazy::new(|| {
    let mut m: Messages = HashMap::new();

    // common
    let mut common = HashMap::new();
    common.insert("app_name", "Bominal Care");
    common.insert("loading", "로딩 중...");
    common.insert("error", "오류가 발생했습니다");
    common.insert("save", "저장");
    common.insert("cancel", "취소");
    common.insert("confirm", "확인");
    common.insert("delete", "삭제");
    common.insert("edit", "수정");
    common.insert("back", "뒤로");
    common.insert("next", "다음");
    common.insert("search", "검색");
    common.insert("no_results", "결과가 없습니다");
    m.insert("common", common);

    // auth
    let mut auth = HashMap::new();
    auth.insert("sign_in", "로그인");
    auth.insert("sign_out", "로그아웃");
    auth.insert("sign_up", "회원가입");
    auth.insert("email", "이메일");
    auth.insert("password", "비밀번호");
    auth.insert("forgot_password", "비밀번호 찾기");
    m.insert("auth", auth);

    // nav
    let mut nav = HashMap::new();
    nav.insert("home", "홈");
    nav.insert("appointments", "예약");
    nav.insert("care", "케어");
    nav.insert("medications", "약물");
    nav.insert("medical_history", "병력");
    nav.insert("services", "서비스");
    nav.insert("housing", "주거");
    nav.insert("opportunities", "기회");
    nav.insert("emergency", "긴급");
    nav.insert("consent", "동의");
    nav.insert("profile", "프로필");
    nav.insert("notifications", "알림");
    nav.insert("settings", "설정");
    m.insert("nav", nav);

    m
});

// =============================================================================
// English messages
// =============================================================================

pub static EN_MESSAGES: Lazy<Messages> = Lazy::new(|| {
    let mut m: Messages = HashMap::new();

    // common
    let mut common = HashMap::new();
    common.insert("app_name", "Bominal Care");
    common.insert("loading", "Loading...");
    common.insert("error", "An error occurred");
    common.insert("save", "Save");
    common.insert("cancel", "Cancel");
    common.insert("confirm", "Confirm");
    common.insert("delete", "Delete");
    common.insert("edit", "Edit");
    common.insert("back", "Back");
    common.insert("next", "Next");
    common.insert("search", "Search");
    common.insert("no_results", "No results found");
    m.insert("common", common);

    // auth
    let mut auth = HashMap::new();
    auth.insert("sign_in", "Sign In");
    auth.insert("sign_out", "Sign Out");
    auth.insert("sign_up", "Sign Up");
    auth.insert("email", "Email");
    auth.insert("password", "Password");
    auth.insert("forgot_password", "Forgot Password");
    m.insert("auth", auth);

    // nav
    let mut nav = HashMap::new();
    nav.insert("home", "Home");
    nav.insert("appointments", "Appointments");
    nav.insert("care", "Care");
    nav.insert("medications", "Medications");
    nav.insert("medical_history", "Medical History");
    nav.insert("services", "Services");
    nav.insert("housing", "Housing");
    nav.insert("opportunities", "Opportunities");
    nav.insert("emergency", "Emergency");
    nav.insert("consent", "Consent");
    nav.insert("profile", "Profile");
    nav.insert("notifications", "Notifications");
    nav.insert("settings", "Settings");
    m.insert("nav", nav);

    m
});

// =============================================================================
// Translation lookup
// =============================================================================

/// Get a translated message by locale, section, and key.
///
/// Falls back to the Korean message map when the locale is not `"en"`.
/// Returns `None` if the section or key is not found.
///
/// # Examples
///
/// ```
/// use bominal_server::i18n::t;
///
/// assert_eq!(t("ko", "common", "save"), Some("저장"));
/// assert_eq!(t("en", "common", "save"), Some("Save"));
/// assert_eq!(t("ko", "unknown", "key"), None);
/// ```
pub fn t(locale: &str, section: &str, key: &str) -> Option<&'static str> {
    let messages = if locale == "en" {
        &*EN_MESSAGES
    } else {
        &*KO_MESSAGES
    };

    messages.get(section).and_then(|s| s.get(key)).copied()
}

/// Get a translated message, falling back to the key itself if not found.
///
/// This is a convenience wrapper around [`t`] that returns the `key` when no
/// translation exists.
pub fn t_or_key<'a>(locale: &str, section: &str, key: &'a str) -> &'a str {
    // SAFETY: 'static outlives 'a, so this coercion is sound.
    match t(locale, section, key) {
        Some(v) => v,
        None => key,
    }
}
