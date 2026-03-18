'use client';

import { useState } from 'react';
import GovernmentAppShell from '@/components/GovernmentAppShell';

interface GeneralSettings {
  region: string;
  department: string;
  teamName: string;
  managerName: string;
  managerEmail: string;
  managerPhone: string;
}

interface NotificationPreferences {
  emailAlerts: boolean;
  smsAlerts: boolean;
  providerViolation: boolean;
  eligibilityCaseUpdate: boolean;
  programDeadline: boolean;
  auditLogCritical: boolean;
  observabilityCritical: boolean;
  weeklyReport: boolean;
}

const INITIAL_SETTINGS: GeneralSettings = {
  region: '서울특별시',
  department: '강남구청',
  teamName: '노인복지과 장기요양팀',
  managerName: '이담당자',
  managerEmail: 'manager@gangnam.go.kr',
  managerPhone: '02-3423-XXXX',
};

const INITIAL_NOTIFICATIONS: NotificationPreferences = {
  emailAlerts: true,
  smsAlerts: false,
  providerViolation: true,
  eligibilityCaseUpdate: true,
  programDeadline: true,
  auditLogCritical: true,
  observabilityCritical: true,
  weeklyReport: true,
};

function Toggle({
  enabled,
  onToggle,
}: {
  enabled: boolean;
  onToggle: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onToggle}
      className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
        enabled ? 'bg-indigo-600' : 'bg-gray-200'
      }`}
    >
      <span
        className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
          enabled ? 'translate-x-6' : 'translate-x-1'
        }`}
      />
    </button>
  );
}

export default function SettingsPage() {
  const [settings, setSettings] = useState<GeneralSettings>(INITIAL_SETTINGS);
  const [notifications, setNotifications] = useState<NotificationPreferences>(INITIAL_NOTIFICATIONS);
  const [saved, setSaved] = useState(false);

  const handleSettingChange = (field: keyof GeneralSettings, value: string) => {
    setSettings((prev) => ({ ...prev, [field]: value }));
    setSaved(false);
  };

  const handleNotificationToggle = (field: keyof NotificationPreferences) => {
    setNotifications((prev) => ({ ...prev, [field]: !prev[field] }));
    setSaved(false);
  };

  const handleSave = () => {
    // In a real app, save to API
    setSaved(true);
    setTimeout(() => setSaved(false), 3000);
  };

  return (
    <GovernmentAppShell>
      <div className="space-y-6 max-w-4xl">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">설정</h1>
            <p className="text-sm text-gray-500 mt-1">포털 일반 설정 및 알림 환경설정</p>
          </div>
          <div className="flex items-center gap-3">
            {saved && (
              <span className="text-sm text-green-600 font-medium">저장되었습니다</span>
            )}
            <button onClick={handleSave} className="btn-primary">
              설정 저장
            </button>
          </div>
        </div>

        {/* General Settings */}
        <div className="card">
          <div className="px-5 py-4 border-b border-slate-100">
            <h2 className="section-title">일반 설정</h2>
            <p className="text-xs text-gray-500 mt-0.5">지역, 부서 및 담당자 정보를 설정합니다</p>
          </div>
          <div className="p-5">
            <div className="grid grid-cols-2 gap-x-6 gap-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">광역시/도</label>
                <select
                  className="input"
                  value={settings.region}
                  onChange={(e) => handleSettingChange('region', e.target.value)}
                >
                  <option>서울특별시</option>
                  <option>부산광역시</option>
                  <option>대구광역시</option>
                  <option>인천광역시</option>
                  <option>광주광역시</option>
                  <option>대전광역시</option>
                  <option>울산광역시</option>
                  <option>세종특별자치시</option>
                  <option>경기도</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">구/군/시</label>
                <select
                  className="input"
                  value={settings.department}
                  onChange={(e) => handleSettingChange('department', e.target.value)}
                >
                  <option>강남구청</option>
                  <option>서초구청</option>
                  <option>송파구청</option>
                  <option>강동구청</option>
                  <option>강서구청</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">팀명</label>
                <input
                  type="text"
                  className="input"
                  value={settings.teamName}
                  onChange={(e) => handleSettingChange('teamName', e.target.value)}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">담당자</label>
                <input
                  type="text"
                  className="input"
                  value={settings.managerName}
                  onChange={(e) => handleSettingChange('managerName', e.target.value)}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">이메일</label>
                <input
                  type="email"
                  className="input"
                  value={settings.managerEmail}
                  onChange={(e) => handleSettingChange('managerEmail', e.target.value)}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">연락처</label>
                <input
                  type="tel"
                  className="input"
                  value={settings.managerPhone}
                  onChange={(e) => handleSettingChange('managerPhone', e.target.value)}
                />
              </div>
            </div>
          </div>
        </div>

        {/* Notification Preferences */}
        <div className="card">
          <div className="px-5 py-4 border-b border-slate-100">
            <h2 className="section-title">알림 환경설정</h2>
            <p className="text-xs text-gray-500 mt-0.5">알림 수신 방법 및 알림 종류를 설정합니다</p>
          </div>
          <div className="p-5">
            {/* Channels */}
            <div className="mb-6">
              <h3 className="text-sm font-semibold text-gray-900 mb-3">알림 채널</h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-sm font-medium text-gray-900">이메일 알림</p>
                    <p className="text-xs text-gray-500">중요 알림을 이메일로 수신합니다</p>
                  </div>
                  <Toggle
                    enabled={notifications.emailAlerts}
                    onToggle={() => handleNotificationToggle('emailAlerts')}
                  />
                </div>
                <div className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-sm font-medium text-gray-900">SMS 알림</p>
                    <p className="text-xs text-gray-500">긴급 알림을 문자 메시지로 수신합니다</p>
                  </div>
                  <Toggle
                    enabled={notifications.smsAlerts}
                    onToggle={() => handleNotificationToggle('smsAlerts')}
                  />
                </div>
              </div>
            </div>

            <hr className="border-slate-100 mb-6" />

            {/* Alert Types */}
            <div>
              <h3 className="text-sm font-semibold text-gray-900 mb-3">알림 유형</h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-sm font-medium text-gray-900">기관 위반 알림</p>
                    <p className="text-xs text-gray-500">제공 기관의 규정 위반 발생 시 알림</p>
                  </div>
                  <Toggle
                    enabled={notifications.providerViolation}
                    onToggle={() => handleNotificationToggle('providerViolation')}
                  />
                </div>
                <div className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-sm font-medium text-gray-900">수급 자격 심사 업데이트</p>
                    <p className="text-xs text-gray-500">심사 상태 변경 및 새 신청 접수 알림</p>
                  </div>
                  <Toggle
                    enabled={notifications.eligibilityCaseUpdate}
                    onToggle={() => handleNotificationToggle('eligibilityCaseUpdate')}
                  />
                </div>
                <div className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-sm font-medium text-gray-900">프로그램 마감 알림</p>
                    <p className="text-xs text-gray-500">프로그램 접수 마감일 임박 시 알림</p>
                  </div>
                  <Toggle
                    enabled={notifications.programDeadline}
                    onToggle={() => handleNotificationToggle('programDeadline')}
                  />
                </div>
                <div className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-sm font-medium text-gray-900">감사 로그 긴급 알림</p>
                    <p className="text-xs text-gray-500">비정상적인 시스템 활동 감지 시 알림</p>
                  </div>
                  <Toggle
                    enabled={notifications.auditLogCritical}
                    onToggle={() => handleNotificationToggle('auditLogCritical')}
                  />
                </div>
                <div className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-sm font-medium text-gray-900">모니터링 긴급 시그널</p>
                    <p className="text-xs text-gray-500">CRITICAL/HIGH 심각도 시그널 발생 시 알림</p>
                  </div>
                  <Toggle
                    enabled={notifications.observabilityCritical}
                    onToggle={() => handleNotificationToggle('observabilityCritical')}
                  />
                </div>
                <div className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-sm font-medium text-gray-900">주간 리포트</p>
                    <p className="text-xs text-gray-500">매주 월요일 주간 현황 요약 보고서 수신</p>
                  </div>
                  <Toggle
                    enabled={notifications.weeklyReport}
                    onToggle={() => handleNotificationToggle('weeklyReport')}
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </GovernmentAppShell>
  );
}
