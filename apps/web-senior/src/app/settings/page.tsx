'use client';

// 설정 — Settings page for senior app
// Text size, high contrast, language, notification preferences

import { useState } from 'react';
import SeniorAppShell from '@/components/SeniorAppShell';

interface Settings {
  textSize: 'default' | 'large';
  highContrast: boolean;
  language: 'ko' | 'en';
  notifyMedication: boolean;
  notifyVisit: boolean;
  notifyFamily: boolean;
  notifySystem: boolean;
}

const INITIAL_SETTINGS: Settings = {
  textSize: 'large',
  highContrast: false,
  language: 'ko',
  notifyMedication: true,
  notifyVisit: true,
  notifyFamily: true,
  notifySystem: false,
};

function ToggleSwitch({
  checked,
  onChange,
  id,
}: {
  checked: boolean;
  onChange: (val: boolean) => void;
  id: string;
}) {
  return (
    <button
      id={id}
      role="switch"
      aria-checked={checked}
      onClick={() => onChange(!checked)}
      className={`relative inline-flex w-14 h-8 rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 ${
        checked ? 'bg-primary-600' : 'bg-gray-300'
      }`}
    >
      <span
        className={`inline-block w-6 h-6 bg-white rounded-full shadow-sm transition-transform m-1 ${
          checked ? 'translate-x-6' : 'translate-x-0'
        }`}
      />
    </button>
  );
}

export default function SettingsPage() {
  const [settings, setSettings] = useState<Settings>(INITIAL_SETTINGS);
  const [saved, setSaved] = useState(false);

  const update = <K extends keyof Settings>(key: K, value: Settings[K]) => {
    setSettings((prev) => ({ ...prev, [key]: value }));
    setSaved(false);
  };

  const handleSave = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-6">설정</h1>

        {/* Text Size */}
        <section className="senior-card mb-4" aria-labelledby="text-size-heading">
          <h2 id="text-size-heading" className="text-senior-lg font-bold text-gray-800 mb-4">글자 크기</h2>
          <div className="flex gap-3">
            <button
              onClick={() => update('textSize', 'default')}
              className={`flex-1 py-3 rounded-xl border-2 font-semibold text-senior-base transition-colors ${
                settings.textSize === 'default'
                  ? 'border-primary-500 bg-primary-50 text-primary-700'
                  : 'border-gray-200 text-gray-600 hover:border-gray-300'
              }`}
              aria-pressed={settings.textSize === 'default'}
            >
              <span className="text-sm">가</span>
              <span className="block text-senior-xs mt-0.5">기본</span>
            </button>
            <button
              onClick={() => update('textSize', 'large')}
              className={`flex-1 py-3 rounded-xl border-2 font-semibold text-senior-base transition-colors ${
                settings.textSize === 'large'
                  ? 'border-primary-500 bg-primary-50 text-primary-700'
                  : 'border-gray-200 text-gray-600 hover:border-gray-300'
              }`}
              aria-pressed={settings.textSize === 'large'}
            >
              <span className="text-xl">가</span>
              <span className="block text-senior-xs mt-0.5">크게</span>
            </button>
          </div>
        </section>

        {/* High Contrast */}
        <section className="senior-card mb-4">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-senior-lg font-bold text-gray-800">고대비 모드</h2>
              <p className="text-senior-sm text-gray-500 mt-0.5">화면을 더 선명하게 표시합니다</p>
            </div>
            <ToggleSwitch
              id="high-contrast"
              checked={settings.highContrast}
              onChange={(val) => update('highContrast', val)}
            />
          </div>
        </section>

        {/* Language */}
        <section className="senior-card mb-4" aria-labelledby="language-heading">
          <h2 id="language-heading" className="text-senior-lg font-bold text-gray-800 mb-4">언어</h2>
          <div className="flex gap-3">
            {([
              { value: 'ko', label: '한국어', flag: '🇰🇷' },
              { value: 'en', label: 'English', flag: '🇺🇸' },
            ] as const).map((lang) => (
              <button
                key={lang.value}
                onClick={() => update('language', lang.value)}
                className={`flex-1 py-3 rounded-xl border-2 font-semibold text-senior-base transition-colors flex items-center justify-center gap-2 ${
                  settings.language === lang.value
                    ? 'border-primary-500 bg-primary-50 text-primary-700'
                    : 'border-gray-200 text-gray-600 hover:border-gray-300'
                }`}
                aria-pressed={settings.language === lang.value}
              >
                <span>{lang.flag}</span>
                <span>{lang.label}</span>
              </button>
            ))}
          </div>
        </section>

        {/* Notification Preferences */}
        <section className="senior-card mb-6" aria-labelledby="notifications-heading">
          <h2 id="notifications-heading" className="text-senior-lg font-bold text-gray-800 mb-4">알림 설정</h2>
          <div className="space-y-4">
            {([
              { key: 'notifyMedication', label: '약 복용 알림', desc: '복용 시간 30분 전 알림', icon: '💊' },
              { key: 'notifyVisit', label: '방문 알림', desc: '요양보호사 방문 1시간 전 알림', icon: '🏠' },
              { key: 'notifyFamily', label: '가족 메시지 알림', desc: '보호자 메시지 수신 시 알림', icon: '👨‍👩‍👧' },
              { key: 'notifySystem', label: '시스템 알림', desc: '공지사항 및 업데이트 알림', icon: '📢' },
            ] as const).map((item) => (
              <div key={item.key} className="flex items-center justify-between gap-3">
                <div className="flex items-center gap-3 flex-1 min-w-0">
                  <span className="text-2xl flex-shrink-0">{item.icon}</span>
                  <div>
                    <p className="text-senior-base font-semibold text-gray-800">{item.label}</p>
                    <p className="text-senior-xs text-gray-500">{item.desc}</p>
                  </div>
                </div>
                <ToggleSwitch
                  id={item.key}
                  checked={settings[item.key]}
                  onChange={(val) => update(item.key, val)}
                />
              </div>
            ))}
          </div>
        </section>

        {/* Save Button */}
        <button
          onClick={handleSave}
          className={`w-full py-4 rounded-2xl text-senior-lg font-bold transition-colors ${
            saved
              ? 'bg-success-500 text-white'
              : 'bg-primary-600 hover:bg-primary-700 text-white'
          }`}
        >
          {saved ? '저장되었습니다 ✓' : '설정 저장'}
        </button>

        <div className="mt-6 pt-5 border-t border-gray-200 space-y-4">
          <button className="w-full text-senior-base text-gray-500 hover:text-gray-700 py-2 min-h-touch font-medium">
            로그아웃
          </button>
          <p className="text-center text-senior-xs text-gray-400">버전 1.0.0 · 개인정보 처리방침</p>
        </div>
      </div>
    </SeniorAppShell>
  );
}
