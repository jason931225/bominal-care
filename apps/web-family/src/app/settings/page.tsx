'use client';

import { useState } from 'react';
import FamilyAppShell from '@/components/FamilyAppShell';

const NOTIFICATION_SETTINGS = [
  { id: 'med_reminder', label: '복약 알림', desc: '어르신 복약 완료 및 누락 시 알림', defaultOn: true },
  { id: 'visit_complete', label: '방문 케어 완료 알림', desc: '케어 방문 시작/종료 시 알림', defaultOn: true },
  { id: 'health_alert', label: '건강 이상 경보', desc: '혈압, 혈당 등 이상 감지 시 즉시 알림', defaultOn: true },
  { id: 'approval_request', label: '승인 요청 알림', desc: '새 승인 요청 도착 시 알림', defaultOn: true },
  { id: 'payment_complete', label: '결제 처리 알림', desc: '결제 완료 및 청구 예정 알림', defaultOn: false },
  { id: 'match_recommend', label: '매칭 추천 알림', desc: '신규 제공자 추천 시 알림', defaultOn: true },
  { id: 'weekly_report', label: '주간 케어 요약', desc: '매주 월요일 어르신 케어 요약 리포트', defaultOn: false },
];

const PRIVACY_SETTINGS = [
  { id: 'share_care', label: '케어 기록 공유', desc: '다른 가족 구성원과 케어 기록 공유', defaultOn: true },
  { id: 'location', label: '위치 공유', desc: '케어 제공자 방문 위치 확인', defaultOn: false },
  { id: 'analytics', label: '이용 통계 수집', desc: '서비스 개선을 위한 익명 통계', defaultOn: true },
];

export default function SettingsPage() {
  const [notifSettings, setNotifSettings] = useState<Record<string, boolean>>(
    Object.fromEntries(NOTIFICATION_SETTINGS.map((s) => [s.id, s.defaultOn]))
  );
  const [privacySettings, setPrivacySettings] = useState<Record<string, boolean>>(
    Object.fromEntries(PRIVACY_SETTINGS.map((s) => [s.id, s.defaultOn]))
  );
  const [language, setLanguage] = useState('ko');
  const [theme, setTheme] = useState('light');
  const [saved, setSaved] = useState(false);

  const toggleNotif = (id: string) => {
    setNotifSettings((prev) => ({ ...prev, [id]: !prev[id] }));
  };

  const togglePrivacy = (id: string) => {
    setPrivacySettings((prev) => ({ ...prev, [id]: !prev[id] }));
  };

  const handleSave = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  return (
    <FamilyAppShell>
      <div className="max-w-2xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-bold text-gray-900">설정</h1>
          <button
            onClick={handleSave}
            className={`px-4 py-2 text-sm font-semibold rounded-lg transition-colors ${
              saved
                ? 'bg-green-600 text-white'
                : 'bg-blue-600 text-white hover:bg-blue-700'
            }`}
          >
            {saved ? '✓ 저장됨' : '저장'}
          </button>
        </div>

        {/* Notification Settings */}
        <section className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-4">알림 설정</h2>
          <div className="space-y-4">
            {NOTIFICATION_SETTINGS.map((setting) => (
              <div key={setting.id} className="flex items-start justify-between gap-3">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-800">{setting.label}</p>
                  <p className="text-xs text-gray-500 mt-0.5">{setting.desc}</p>
                </div>
                <button
                  onClick={() => toggleNotif(setting.id)}
                  className={`relative inline-flex flex-shrink-0 h-6 w-11 items-center rounded-full transition-colors ${
                    notifSettings[setting.id] ? 'bg-blue-600' : 'bg-gray-300'
                  }`}
                >
                  <span
                    className={`inline-block w-4 h-4 bg-white rounded-full shadow transform transition-transform ${
                      notifSettings[setting.id] ? 'translate-x-6' : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>
            ))}
          </div>
        </section>

        {/* Privacy Settings */}
        <section className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-4">개인정보 및 공유</h2>
          <div className="space-y-4">
            {PRIVACY_SETTINGS.map((setting) => (
              <div key={setting.id} className="flex items-start justify-between gap-3">
                <div className="flex-1">
                  <p className="text-sm font-medium text-gray-800">{setting.label}</p>
                  <p className="text-xs text-gray-500 mt-0.5">{setting.desc}</p>
                </div>
                <button
                  onClick={() => togglePrivacy(setting.id)}
                  className={`relative inline-flex flex-shrink-0 h-6 w-11 items-center rounded-full transition-colors ${
                    privacySettings[setting.id] ? 'bg-blue-600' : 'bg-gray-300'
                  }`}
                >
                  <span
                    className={`inline-block w-4 h-4 bg-white rounded-full shadow transform transition-transform ${
                      privacySettings[setting.id] ? 'translate-x-6' : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>
            ))}
          </div>
        </section>

        {/* Appearance */}
        <section className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-4">화면 설정</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-800 mb-2">언어</label>
              <div className="flex gap-2">
                {[
                  { value: 'ko', label: '한국어' },
                  { value: 'en', label: 'English' },
                ].map((lang) => (
                  <button
                    key={lang.value}
                    onClick={() => setLanguage(lang.value)}
                    className={`px-4 py-2 rounded-lg text-sm font-medium border transition-colors ${
                      language === lang.value
                        ? 'bg-blue-600 text-white border-blue-600'
                        : 'bg-white text-gray-600 border-gray-300 hover:border-blue-300'
                    }`}
                  >
                    {lang.label}
                  </button>
                ))}
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-800 mb-2">테마</label>
              <div className="flex gap-2">
                {[
                  { value: 'light', label: '라이트', icon: '☀️' },
                  { value: 'dark', label: '다크', icon: '🌙' },
                  { value: 'system', label: '시스템', icon: '💻' },
                ].map((t) => (
                  <button
                    key={t.value}
                    onClick={() => setTheme(t.value)}
                    className={`flex items-center gap-1.5 px-3 py-2 rounded-lg text-sm font-medium border transition-colors ${
                      theme === t.value
                        ? 'bg-blue-600 text-white border-blue-600'
                        : 'bg-white text-gray-600 border-gray-300 hover:border-blue-300'
                    }`}
                  >
                    <span>{t.icon}</span>
                    <span>{t.label}</span>
                  </button>
                ))}
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-800 mb-2">글자 크기</label>
              <div className="flex gap-2">
                {['작게', '보통', '크게'].map((size) => (
                  <button
                    key={size}
                    className={`px-4 py-2 rounded-lg text-sm border transition-colors ${
                      size === '보통'
                        ? 'bg-blue-600 text-white border-blue-600 font-semibold'
                        : 'bg-white text-gray-600 border-gray-300 hover:border-blue-300'
                    }`}
                  >
                    {size}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </section>

        {/* Account */}
        <section className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-4">계정 관리</h2>
          <div className="space-y-2">
            {[
              { label: '비밀번호 변경', icon: '🔒', action: '변경' },
              { label: '2단계 인증', icon: '🔐', action: '설정' },
              { label: '연결된 소셜 계정', icon: '🔗', action: '관리' },
              { label: '알림 수신 이메일', icon: '📧', action: '변경' },
            ].map((item) => (
              <div key={item.label} className="flex items-center justify-between py-2.5 border-b border-gray-100 last:border-0">
                <div className="flex items-center gap-2.5">
                  <span>{item.icon}</span>
                  <span className="text-sm text-gray-700">{item.label}</span>
                </div>
                <button className="text-sm text-blue-600 hover:underline font-medium">
                  {item.action}
                </button>
              </div>
            ))}
          </div>
        </section>

        {/* Danger Zone */}
        <section className="bg-red-50 border border-red-200 rounded-xl p-5">
          <h2 className="font-bold text-red-800 mb-3">위험 구역</h2>
          <div className="space-y-2">
            <button className="w-full text-left py-2 text-sm text-red-700 hover:text-red-900 border-b border-red-100 pb-3">
              모든 데이터 삭제 요청
            </button>
            <button className="w-full text-left py-2 text-sm text-red-700 hover:text-red-900 font-semibold">
              계정 탈퇴
            </button>
          </div>
        </section>
      </div>
    </FamilyAppShell>
  );
}
