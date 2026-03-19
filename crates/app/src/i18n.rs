//! Internationalization module for the Bominal Care frontend.
//!
//! All user-visible text must be retrieved via [`t`] or [`t_with`].
//! Korean (ko) is the default locale. Additional locales can be added
//! by extending the `TRANSLATIONS` table.
//!
//! # Key naming convention
//! - `common.*` — shared across all portals (buttons, labels, status)
//! - `auth.*` — sign-in, sign-out, terms, privacy
//! - `senior.*` — senior portal pages
//! - `family.*` — family portal pages
//! - `caregiver.*` — caregiver portal pages
//! - `internal.*` — internal/provider portal pages
//! - `gov.*` — government portal pages
//! - `nav.*` — navigation labels
//! - `error.*` — error messages
//! - `form.*` — form labels and placeholders

use std::collections::HashMap;
use std::sync::LazyLock;

/// Translation table: key → Korean string.
static TRANSLATIONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // =========================================================================
    // common — shared labels, buttons, status
    // =========================================================================
    m.insert("common.app_name", "Bominal Care");
    m.insert("common.loading", "로딩 중...");
    m.insert("common.save", "저장");
    m.insert("common.cancel", "취소");
    m.insert("common.confirm", "확인");
    m.insert("common.delete", "삭제");
    m.insert("common.edit", "수정");
    m.insert("common.back", "이전");
    m.insert("common.next", "다음");
    m.insert("common.close", "닫기");
    m.insert("common.search", "검색");
    m.insert("common.search_placeholder", "검색...");
    m.insert("common.logout", "로그아웃");
    m.insert("common.login", "로그인");
    m.insert("common.submit", "제출");
    m.insert("common.view_all", "전체 보기 →");
    m.insert("common.view_detail", "상세보기");
    m.insert("common.no_data", "데이터가 없습니다.");
    m.insert("common.error_occurred", "오류가 발생했습니다.");
    m.insert("common.preparing", "준비 중입니다.");
    m.insert("common.total", "총");
    m.insert("common.count_suffix", "건");
    m.insert("common.hours", "시간");
    m.insert("common.active", "활성");
    m.insert("common.inactive", "비활성");
    m.insert("common.status", "상태");
    m.insert("common.date", "날짜");
    m.insert("common.time", "시간");
    m.insert("common.name", "이름");
    m.insert("common.phone", "전화번호");
    m.insert("common.address", "주소");
    m.insert("common.email", "이메일");
    m.insert("common.notes", "메모");
    m.insert("common.description", "설명");
    m.insert("common.type", "유형");
    m.insert("common.home", "홈");
    m.insert("common.settings", "설정");
    m.insert("common.notifications", "알림");
    m.insert("common.profile", "프로필");
    m.insert("common.download", "다운로드");
    m.insert("common.upload", "업로드");
    m.insert("common.file_upload_hint", "파일을 드래그하거나 클릭하여 업로드");
    m.insert("common.approve", "승인");
    m.insert("common.reject", "거부");
    m.insert("common.all", "전체");
    m.insert("common.today", "오늘");
    m.insert("common.version", "v1.0.0");
    m.insert("common.mark_all_read", "모두 읽음");

    // =========================================================================
    // auth — sign-in, sign-out, terms, privacy
    // =========================================================================
    m.insert("auth.signin.title", "Bominal Care");
    m.insert("auth.signin.subtitle", "안전하고 편리한 돌봄 서비스");
    m.insert("auth.signin.passkey", "패스키로 로그인");
    m.insert("auth.signin.google", "Google로 로그인");
    m.insert("auth.signin.naver", "네이버로 로그인");
    m.insert("auth.signin.kakao", "카카오로 로그인");
    m.insert("auth.signin.or", "또는");
    m.insert("auth.signin.demo", "데모 로그인");
    m.insert("auth.signin.role_select", "역할 선택");
    m.insert("auth.signin.role_senior", "어르신 (시니어)");
    m.insert("auth.signin.role_family", "가족 보호자");
    m.insert("auth.signin.role_caregiver", "요양보호사");
    m.insert("auth.signin.role_internal", "기관 관리자");
    m.insert("auth.signin.role_government", "정부 담당자");
    m.insert("auth.signin.pipa_consent", "개인정보 수집 및 이용에 동의합니다 (필수)");
    m.insert("auth.signin.demo_start", "데모 계정으로 시작");
    m.insert("auth.signin.logging_in", "로그인 중...");
    m.insert("auth.signin.login_failed", "로그인에 실패했습니다.");
    m.insert("auth.signin.footer", "로그인하면 ");
    m.insert("auth.signin.terms_link", "이용약관");
    m.insert("auth.signin.privacy_link", "개인정보처리방침");
    m.insert("auth.signin.footer_end", "에 동의하게 됩니다.");
    m.insert("auth.expired", "인증이 만료되었습니다. 다시 로그인해 주세요.");
    m.insert("auth.error.title", "인증 오류");
    m.insert("auth.error.message", "로그인 중 오류가 발생했습니다. 다시 시도해 주세요.");
    m.insert("auth.error.retry", "다시 로그인");
    m.insert("auth.error.go_home", "홈으로 돌아가기");
    m.insert("auth.terms.title", "이용약관");
    m.insert("auth.privacy.title", "개인정보처리방침");
    m.insert("auth.back_to_login", "← 로그인으로 돌아가기");

    // =========================================================================
    // nav — portal names and navigation labels
    // =========================================================================
    m.insert("nav.portal.senior", "Bominal Senior");
    m.insert("nav.portal.family", "Bominal Family");
    m.insert("nav.portal.caregiver", "Bominal Care");
    m.insert("nav.portal.internal", "Bominal Provider");
    m.insert("nav.portal.government", "Bominal Gov");
    m.insert("nav.internal_admin", "내부 관리");
    m.insert("nav.gov_portal", "정부 포털");
    m.insert("nav.home", "홈");
    m.insert("nav.appointments", "예약");
    m.insert("nav.medications", "약");
    m.insert("nav.care", "돌봄");
    m.insert("nav.more", "더보기");
    m.insert("nav.schedule", "스케줄");
    m.insert("nav.clients", "고객");
    m.insert("nav.tasks", "업무");
    m.insert("nav.timeline", "타임라인");
    m.insert("nav.matching", "매칭");
    m.insert("nav.dashboard", "대시보드");
    m.insert("nav.emergency", "긴급");
    m.insert("nav.emergency_contact", "긴급 연락");

    // =========================================================================
    // senior — senior portal pages
    // =========================================================================
    m.insert("senior.dashboard.greeting", "안녕하세요!");
    m.insert("senior.dashboard.greeting_sub", "오늘도 건강한 하루 보내세요.");
    m.insert("senior.dashboard.today_meds", "오늘의 복약");
    m.insert("senior.dashboard.no_meds", "오늘 예정된 복약이 없습니다.");
    m.insert("senior.dashboard.upcoming_appts", "예정된 진료");
    m.insert("senior.dashboard.no_appts", "예정된 진료가 없습니다.");
    m.insert("senior.dashboard.quick_menu", "빠른 메뉴");
    m.insert("senior.medications.title", "약물 관리");
    m.insert("senior.medications.subtitle", "현재 복용 중인 약물 목록");
    m.insert("senior.medications.no_data", "등록된 약물이 없습니다.");
    m.insert("senior.medications.taking", "복용 중");
    m.insert("senior.medications.stopped", "중단");
    m.insert("senior.appointments.title", "진료 예약");
    m.insert("senior.appointments.subtitle", "진료 예약 내역");
    m.insert("senior.appointments.new", "새 예약");
    m.insert("senior.profile.title", "내 프로필");
    m.insert("senior.profile.subtitle", "개인정보 및 건강 기본 정보");
    m.insert("senior.profile.korean_name", "한국 이름");
    m.insert("senior.profile.emergency_name", "긴급 연락처 (이름)");
    m.insert("senior.profile.emergency_phone", "긴급 연락처 (전화)");
    m.insert("senior.profile.no_data", "프로필 정보를 불러올 수 없습니다.");
    m.insert("senior.emergency.title", "긴급 연락");
    m.insert("senior.emergency.subtitle", "긴급 상황 시 사용하세요.");
    m.insert("senior.emergency.call_119", "119 응급 전화");
    m.insert("senior.consent.title", "동의 관리");
    m.insert("senior.notifications.title", "알림");
    m.insert("senior.settings.title", "설정");
    m.insert("senior.more.title", "더보기");

    // =========================================================================
    // family — family portal pages
    // =========================================================================
    m.insert("family.dashboard.title", "가족 케어 대시보드");
    m.insert("family.dashboard.subtitle", "돌봄 대상자의 현황을 한눈에 확인하세요.");
    m.insert("family.dashboard.today_schedule", "오늘의 일정");
    m.insert("family.dashboard.med_status", "복약 상태");
    m.insert("family.dashboard.med_normal", "정상");
    m.insert("family.dashboard.timeline_preview", "케어 타임라인 미리보기");
    m.insert("family.dashboard.no_visits", "예정된 방문이 없습니다.");
    m.insert("family.timeline.title", "케어 타임라인");
    m.insert("family.timeline.subtitle", "최근 30일간의 돌봄 기록입니다.");
    m.insert("family.matching.title", "요양보호사 매칭");
    m.insert("family.matching.subtitle", "조건을 입력하고 매칭을 시작하세요.");
    m.insert("family.matching.search", "매칭 검색");
    m.insert("family.approvals.title", "승인 대기 목록");
    m.insert("family.payments.title", "결제 내역");
    m.insert("family.settings.title", "설정");
    m.insert("family.settings.notification", "알림 설정");
    m.insert("family.settings.push", "푸시 알림");
    m.insert("family.settings.email", "이메일 알림");

    // =========================================================================
    // caregiver — caregiver portal pages
    // =========================================================================
    m.insert("caregiver.dashboard.title", "요양보호사 대시보드");
    m.insert("caregiver.dashboard.subtitle", "오늘의 스케줄과 업무를 확인하세요.");
    m.insert("caregiver.dashboard.today_visits", "오늘 방문");
    m.insert("caregiver.dashboard.work_hours", "근무 시간");
    m.insert("caregiver.dashboard.next_visit", "다음 방문");
    m.insert("caregiver.dashboard.no_visits", "예정된 방문이 없습니다.");
    m.insert("caregiver.dashboard.weekly_summary", "이번 주 요약");
    m.insert("caregiver.dashboard.total_visits", "총 방문");
    m.insert("caregiver.dashboard.client_count", "고객 수");
    m.insert("caregiver.schedule.title", "스케줄");
    m.insert("caregiver.schedule.checkin", "체크인");
    m.insert("caregiver.schedule.checkout", "체크아웃");
    m.insert("caregiver.clients.title", "담당 고객");
    m.insert("caregiver.clients.search", "고객 검색...");
    m.insert("caregiver.tasks.title", "업무 목록");
    m.insert("caregiver.tasks.incomplete", "미완료");
    m.insert("caregiver.tasks.complete", "완료");
    m.insert("caregiver.tasks.mark_complete", "업무 완료");
    m.insert("caregiver.notes.title", "관찰 기록");
    m.insert("caregiver.notes.new", "새 기록");
    m.insert("caregiver.notes.create", "관찰 기록 작성");
    m.insert("caregiver.incident.title", "사고/이상 보고");
    m.insert("caregiver.incident.emergency_note", "응급 상황 시 먼저 119에 연락하시고, 이후 보고서를 작성해주세요.");
    m.insert("caregiver.apply.title", "요양보호사 지원");
    m.insert("caregiver.apply.start", "지원 시작하기");
    m.insert("caregiver.profile.title", "내 프로필");
    m.insert("caregiver.availability.title", "근무 가능 시간");
    m.insert("caregiver.settings.title", "설정");

    // =========================================================================
    // internal — internal/provider portal pages
    // =========================================================================
    m.insert("internal.dashboard.title", "내부 관리 대시보드");
    m.insert("internal.dashboard.subtitle", "기관 운영 현황을 관리하세요.");
    m.insert("internal.clients.title", "이용자 관리");
    m.insert("internal.caregivers.title", "요양보호사 관리");
    m.insert("internal.schedules.title", "방문 일정");
    m.insert("internal.quality.title", "품질 관리");
    m.insert("internal.compliance.title", "컴플라이언스");
    m.insert("internal.reports.title", "보고서");
    m.insert("internal.referrals.title", "의뢰 관리");
    m.insert("internal.referrals.new", "새 의뢰");
    m.insert("internal.settings.title", "기관 설정");

    // =========================================================================
    // gov — government portal pages
    // =========================================================================
    m.insert("gov.dashboard.title", "정부 관리 대시보드");
    m.insert("gov.dashboard.subtitle", "관할 지역 돌봄 현황을 모니터링하세요.");
    m.insert("gov.eligibility.title", "등급 판정 관리");
    m.insert("gov.providers.title", "기관 관리");
    m.insert("gov.programs.title", "정부 프로그램");
    m.insert("gov.audit.title", "감사 로그");
    m.insert("gov.observability.title", "시스템 모니터링");
    m.insert("gov.settings.title", "관리 설정");

    // =========================================================================
    // medical — medical/clinician portal pages
    // =========================================================================
    m.insert("nav.portal.medical", "Bominal Medical");
    m.insert("medical.nav.subtitle", "의료진 포털");
    m.insert("medical.nav.dashboard", "대시보드");
    m.insert("medical.nav.patients", "환자 조회");
    m.insert("medical.nav.prescriptions", "처방전");
    m.insert("medical.nav.appointments", "진료 예약");
    m.insert("medical.nav.history", "진료 이력");
    m.insert("medical.handoff.label", "핸드오프 세션");
    m.insert("medical.handoff.no_patient", "현재 연결된 환자 없음");
    m.insert("medical.user.initial", "의");
    m.insert("medical.user.name", "의료진");
    m.insert("medical.user.role", "담당의사");
    m.insert("medical.topbar.title", "의료진 관리 시스템");
    m.insert("medical.topbar.subtitle", "환자 진료 및 처방 관리");
    m.insert("medical.dashboard.title", "의료진 대시보드");
    m.insert("medical.dashboard.subtitle", "환자 진료 현황을 한눈에 확인하세요.");
    m.insert("medical.dashboard.active_sessions", "활성 세션");
    m.insert("medical.dashboard.active_sessions_sub", "현재 연결된 환자");
    m.insert("medical.dashboard.today_prescriptions", "오늘 처방");
    m.insert("medical.dashboard.today_prescriptions_sub", "발행된 처방전");
    m.insert("medical.dashboard.today_appointments", "오늘 예약");
    m.insert("medical.dashboard.today_appointments_sub", "예정된 진료");
    m.insert("medical.dashboard.patients_seen", "진료 완료");
    m.insert("medical.dashboard.patients_seen_sub", "오늘 진료한 환자");
    m.insert("medical.dashboard.quick_actions", "빠른 작업");
    m.insert("medical.dashboard.action_patient_lookup", "환자 조회");
    m.insert("medical.dashboard.action_patient_lookup_sub", "환자를 검색하고 세션을 시작하세요");
    m.insert("medical.dashboard.action_new_prescription", "처방전 작성");
    m.insert("medical.dashboard.action_new_prescription_sub", "새 처방전을 작성하세요");
    m.insert("medical.dashboard.action_book_appointment", "진료 예약");
    m.insert("medical.dashboard.action_book_appointment_sub", "환자의 진료를 예약하세요");
    m.insert("medical.dashboard.recent_activity", "최근 활동");
    m.insert("medical.dashboard.no_activity", "최근 활동이 없습니다.");
    m.insert("medical.patients.title", "환자 조회");
    m.insert("medical.patients.subtitle", "환자를 검색하여 핸드오프 세션을 시작하세요.");
    m.insert("medical.patients.search_placeholder", "환자 이름 또는 ID로 검색...");
    m.insert("medical.patients.search_hint", "환자를 검색하세요.");
    m.insert("medical.session.title", "핸드오프 세션");
    m.insert("medical.session.subtitle", "현재 연결된 환자의 정보를 확인하세요.");
    m.insert("medical.session.no_active", "활성 세션 없음");
    m.insert("medical.session.no_active_sub", "환자 조회에서 세션을 시작하세요.");
    m.insert("medical.session.demographics", "기본 정보");
    m.insert("medical.session.care_grade", "돌봄 등급");
    m.insert("medical.session.care_plan", "케어 플랜");
    m.insert("medical.session.no_care_plan", "활성 케어 플랜이 없습니다.");
    m.insert("medical.session.write_prescription", "처방전 작성");
    m.insert("medical.session.book_appointment", "진료 예약");
    m.insert("medical.prescriptions.title", "처방전 작성");
    m.insert("medical.prescriptions.subtitle", "환자의 처방전을 작성하세요.");
    m.insert("medical.prescriptions.no_patient", "환자를 먼저 선택해주세요");
    m.insert("medical.prescriptions.med_name", "약품명");
    m.insert("medical.prescriptions.med_name_placeholder", "약품명을 입력하세요");
    m.insert("medical.prescriptions.dosage", "용량");
    m.insert("medical.prescriptions.dosage_placeholder", "예: 500mg");
    m.insert("medical.prescriptions.frequency", "복용 빈도");
    m.insert("medical.prescriptions.freq_once_daily", "1일 1회");
    m.insert("medical.prescriptions.freq_twice_daily", "1일 2회");
    m.insert("medical.prescriptions.freq_three_daily", "1일 3회");
    m.insert("medical.prescriptions.freq_as_needed", "필요 시");
    m.insert("medical.prescriptions.duration", "복용 기간");
    m.insert("medical.prescriptions.duration_placeholder", "예: 7일, 30일");
    m.insert("medical.prescriptions.notes_placeholder", "추가 지시사항");
    m.insert("medical.prescriptions.submit", "처방전 발행");
    m.insert("medical.prescriptions.submitting", "발행 중...");
    m.insert("medical.appointments.title", "진료 예약");
    m.insert("medical.appointments.subtitle", "환자의 진료를 예약하세요.");
    m.insert("medical.appointments.new_booking", "새 예약");
    m.insert("medical.appointments.no_patient", "환자를 먼저 선택해주세요");
    m.insert("medical.appointments.institution", "의료기관명");
    m.insert("medical.appointments.institution_placeholder", "병원/의원 이름");
    m.insert("medical.appointments.purpose", "방문 목적");
    m.insert("medical.appointments.purpose_placeholder", "진료, 검사 등");
    m.insert("medical.appointments.notes_placeholder", "추가 메모");
    m.insert("medical.appointments.submit", "예약 등록");
    m.insert("medical.appointments.submitting", "등록 중...");
    m.insert("medical.appointments.upcoming", "예정된 진료");
    m.insert("medical.appointments.no_upcoming", "예정된 진료가 없습니다.");
    m.insert("medical.history.title", "진료 이력");
    m.insert("medical.history.subtitle", "환자의 진료 이력을 확인하세요.");
    m.insert("medical.history.no_patient", "환자를 먼저 선택해주세요");
    m.insert("medical.history.conditions", "질환 이력");
    m.insert("medical.history.no_conditions", "등록된 질환 이력이 없습니다.");
    m.insert("medical.history.past_prescriptions", "과거 처방 이력");
    m.insert("medical.history.no_prescriptions", "과거 처방 이력이 없습니다.");
    m.insert("medical.history.past_appointments", "과거 진료 이력");
    m.insert("medical.history.no_appointments", "과거 진료 이력이 없습니다.");

    // =========================================================================
    // error — error messages
    // =========================================================================
    m.insert("error.not_found", "페이지를 찾을 수 없습니다");
    m.insert("error.server", "서버 오류");
    m.insert("error.load_failed", "데이터를 불러올 수 없습니다.");
    m.insert("error.profile_load_failed", "프로필 정보를 불러올 수 없습니다.");
    m.insert("error.go_home", "홈으로 돌아가기");

    // =========================================================================
    // form — form labels and placeholders
    // =========================================================================
    m.insert("form.required", "*");
    m.insert("form.select_placeholder", "선택하세요");
    m.insert("form.region", "지역");
    m.insert("form.service_type", "서비스 유형");
    m.insert("form.client_select", "고객 선택");
    m.insert("form.client_select_placeholder", "고객을 선택하세요");
    m.insert("form.category", "분류");
    m.insert("form.content", "내용");
    m.insert("form.severity", "심각도");

    m
});

/// Look up a translation by key. Returns a fallback marker if not found
/// (makes missing translations visible in the UI during development).
pub fn t(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or("[missing]")
}

/// Look up a translation and replace `{name}` placeholders with values.
///
/// # Example
/// ```rust
/// let msg = t_with("greeting", &[("name", "김복순")]);
/// // "안녕하세요, 김복순님!"
/// ```
pub fn t_with(key: &str, params: &[(&str, &str)]) -> String {
    let template = t(key);
    let mut result = template.to_string();
    for (name, value) in params {
        result = result.replace(&format!("{{{name}}}"), value);
    }
    result
}
