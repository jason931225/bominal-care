// 동의 관리 센터 — Consent Center
// View and manage data sharing and medical consent settings

'use client';

import { useState } from 'react';
import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

interface ConsentItem {
  id: string;
  title: string;
  description: string;
  category: '의료' | '데이터' | '서비스' | '마케팅';
  required: boolean;
  enabled: boolean;
  lastUpdated: string;
  detail: string;
}

const INITIAL_CONSENTS: ConsentItem[] = [
  {
    id: 'con-1',
    title: '개인정보 수집 및 이용',
    description: '서비스 제공을 위한 필수 개인정보 처리',
    category: '데이터',
    required: true,
    enabled: true,
    lastUpdated: '2025년 11월 1일',
    detail: '성명, 생년월일, 연락처 등 기본 정보를 서비스 제공 목적으로 수집합니다.',
  },
  {
    id: 'con-2',
    title: '의료 정보 활용 동의',
    description: '케어 서비스 개선을 위한 건강 정보 활용',
    category: '의료',
    required: false,
    enabled: true,
    lastUpdated: '2025년 11월 1일',
    detail: '진료 기록, 약물 정보 등을 케어 서비스 품질 향상에 활용합니다.',
  },
  {
    id: 'con-3',
    title: '가족 · 보호자 정보 공유',
    description: '건강 상태 및 일정 알림을 보호자에게 공유',
    category: '서비스',
    required: false,
    enabled: true,
    lastUpdated: '2025년 11월 5일',
    detail: '약 복용 여부, 예약 일정, 긴급 상황을 등록된 보호자에게 알립니다.',
  },
  {
    id: 'con-4',
    title: '위치 정보 수집 동의',
    description: '교통 서비스 및 가까운 의료기관 안내',
    category: '서비스',
    required: false,
    enabled: false,
    lastUpdated: '2025년 11월 1일',
    detail: '이동 서비스 예약 및 주변 의료기관 안내 시 현재 위치를 사용합니다.',
  },
  {
    id: 'con-5',
    title: '마케팅 정보 수신 동의',
    description: '새 서비스 소식 및 혜택 안내 문자/앱 알림',
    category: '마케팅',
    required: false,
    enabled: false,
    lastUpdated: '2025년 11월 1일',
    detail: '이벤트, 할인 혜택, 신규 서비스 안내를 SMS 또는 앱 알림으로 받습니다.',
  },
  {
    id: 'con-6',
    title: '제3자 정보 제공 동의',
    description: '협력 기관(병원, 요양원 등)과의 정보 공유',
    category: '데이터',
    required: false,
    enabled: false,
    lastUpdated: '2025년 11월 1일',
    detail: '서비스 연계를 위해 제휴 의료기관 및 복지 기관과 정보를 공유합니다.',
  },
];

const CATEGORY_COLORS: Record<ConsentItem['category'], string> = {
  '의료': 'bg-danger-50 text-danger-700',
  '데이터': 'bg-primary-50 text-primary-700',
  '서비스': 'bg-secondary-50 text-secondary-700',
  '마케팅': 'bg-gray-100 text-gray-600',
};

export default function ConsentPage() {
  const [consents, setConsents] = useState<ConsentItem[]>(INITIAL_CONSENTS);
  const [saved, setSaved] = useState(false);

  function toggleConsent(id: string) {
    setConsents((prev) =>
      prev.map((c) =>
        c.id === id && !c.required
          ? { ...c, enabled: !c.enabled, lastUpdated: '2026년 3월 15일' }
          : c
      )
    );
    setSaved(false);
  }

  function handleSave() {
    setSaved(true);
    setTimeout(() => setSaved(false), 3000);
  }

  const enabledCount = consents.filter((c) => c.enabled).length;

  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-2">동의 관리</h1>
        <p className="text-senior-base text-gray-500 mb-5">정보 활용 동의를 직접 관리하세요</p>

        {/* Summary */}
        <div className="senior-card mb-5 flex items-center gap-4">
          <div className="text-4xl" aria-hidden="true">🔒</div>
          <div>
            <p className="text-senior-base font-bold text-gray-800">
              현재 {enabledCount}개 항목에 동의됨
            </p>
            <p className="text-senior-sm text-gray-500">
              필수 항목은 서비스 이용에 필요합니다.
            </p>
          </div>
        </div>

        {/* Consent items */}
        <div className="space-y-3 mb-6">
          {consents.map((consent) => (
            <div key={consent.id} className="senior-card">
              <div className="flex items-start gap-3">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <span className={`${CATEGORY_COLORS[consent.category]} text-senior-sm font-bold px-2 py-0.5 rounded-full`}>
                      {consent.category}
                    </span>
                    {consent.required && (
                      <span className="bg-danger-50 text-danger-600 text-senior-sm font-bold px-2 py-0.5 rounded-full">
                        필수
                      </span>
                    )}
                  </div>
                  <p className="text-senior-base font-bold text-gray-900">{consent.title}</p>
                  <p className="text-senior-sm text-gray-600 mt-0.5">{consent.description}</p>
                  <div className="flex items-center gap-2 mt-2">
                    <Link
                      href={`/consent/${consent.id}`}
                      className="text-senior-sm text-primary-600 font-medium min-h-touch flex items-center"
                    >
                      상세 보기
                    </Link>
                    <span className="text-gray-300">|</span>
                    <span className="text-senior-sm text-gray-400">{consent.lastUpdated} 동의</span>
                  </div>
                </div>

                {/* Toggle switch */}
                <button
                  type="button"
                  role="switch"
                  aria-checked={consent.enabled}
                  aria-label={`${consent.title} ${consent.enabled ? '동의 해제' : '동의'}`}
                  onClick={() => toggleConsent(consent.id)}
                  disabled={consent.required}
                  className={`relative inline-flex w-14 h-8 rounded-full transition-colors duration-200 flex-shrink-0 mt-1
                    ${consent.enabled ? 'bg-primary-600' : 'bg-gray-300'}
                    ${consent.required ? 'opacity-60 cursor-not-allowed' : 'cursor-pointer'}`}
                >
                  <span
                    className={`absolute top-1 w-6 h-6 rounded-full bg-white shadow transition-transform duration-200
                      ${consent.enabled ? 'translate-x-7' : 'translate-x-1'}`}
                  />
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* Save button */}
        <button
          onClick={handleSave}
          className={`senior-btn-primary w-full ${saved ? 'bg-success-500 hover:bg-success-500' : ''}`}
        >
          {saved ? '✅ 저장됐습니다' : '변경사항 저장'}
        </button>

        <p className="mt-4 text-center text-senior-sm text-gray-400">
          동의 변경은 언제든지 가능합니다.
        </p>
      </div>
    </SeniorAppShell>
  );
}
