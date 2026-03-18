// 동의 기록 상세 — Consent Record Detail
// Shows full legal text and history for a single consent record

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const MOCK_CONSENT = {
  id: 'con-2',
  title: '의료 정보 활용 동의',
  category: '의료',
  required: false,
  enabled: true,
  version: 'v2.1',
  lastUpdated: '2025년 11월 1일',
  expiresAt: '2026년 10월 31일',
  collector: '주식회사 시니어케어',
  purpose: '개인화된 케어 서비스 제공 및 건강 관리 품질 향상',
  items: [
    '진료 기록 (진단명, 처방전, 검사 결과)',
    '약물 복용 이력',
    '건강 측정 데이터 (혈압, 혈당)',
    '케어 서비스 이용 기록',
  ],
  sharedWith: [
    '담당 케어매니저',
    '방문 요양 서비스 제공기관',
    '담당 의사 (진료 연속성 목적)',
  ],
  retentionPeriod: '서비스 이용 종료 후 5년',
  withdrawalMethod: '앱 내 동의 관리 페이지에서 언제든 철회 가능',
  history: [
    { date: '2025년 11월 1일', action: '최초 동의', by: '본인' },
    { date: '2026년 1월 10일', action: '내용 확인', by: '본인' },
  ],
  legalBasis: '개인정보 보호법 제15조, 의료법 제21조의2',
};

interface PageProps {
  params: Promise<{ id: string }>;
}

export default async function ConsentDetailPage({ params: _params }: PageProps) {
  const consent = MOCK_CONSENT;

  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Back */}
        <Link
          href="/consent"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          동의 관리로
        </Link>

        {/* Header */}
        <div className="senior-card mb-4">
          <div className="flex items-start justify-between mb-3">
            <div>
              <span className="bg-danger-50 text-danger-700 text-senior-sm font-bold px-2 py-0.5 rounded-full">
                {consent.category}
              </span>
              <h1 className="text-senior-2xl font-bold text-gray-900 mt-2">{consent.title}</h1>
            </div>
            <span className={`text-senior-sm font-bold px-3 py-1.5 rounded-full flex-shrink-0 ml-2 ${consent.enabled ? 'bg-success-50 text-success-700' : 'bg-gray-100 text-gray-500'}`}>
              {consent.enabled ? '동의됨' : '미동의'}
            </span>
          </div>

          <div className="grid grid-cols-2 gap-3 text-senior-sm">
            <div className="bg-gray-50 rounded-lg p-2">
              <p className="text-gray-500">버전</p>
              <p className="font-semibold text-gray-800">{consent.version}</p>
            </div>
            <div className="bg-gray-50 rounded-lg p-2">
              <p className="text-gray-500">동의일</p>
              <p className="font-semibold text-gray-800">{consent.lastUpdated}</p>
            </div>
            <div className="bg-gray-50 rounded-lg p-2">
              <p className="text-gray-500">만료일</p>
              <p className="font-semibold text-gray-800">{consent.expiresAt}</p>
            </div>
            <div className="bg-gray-50 rounded-lg p-2">
              <p className="text-gray-500">법적 근거</p>
              <p className="font-semibold text-gray-800 text-xs">{consent.legalBasis}</p>
            </div>
          </div>
        </div>

        {/* Purpose */}
        <section className="senior-card mb-4" aria-labelledby="purpose-heading">
          <h2 id="purpose-heading" className="text-senior-lg font-bold text-gray-800 mb-2">수집 목적</h2>
          <p className="text-senior-base text-gray-700">{consent.purpose}</p>
          <p className="text-senior-sm text-gray-500 mt-2">수집 기관: {consent.collector}</p>
        </section>

        {/* Items collected */}
        <section className="senior-card mb-4" aria-labelledby="items-heading">
          <h2 id="items-heading" className="text-senior-lg font-bold text-gray-800 mb-2">수집 항목</h2>
          <ul className="space-y-1">
            {consent.items.map((item, i) => (
              <li key={i} className="flex items-start gap-2 text-senior-base text-gray-700">
                <span className="text-primary-500 mt-0.5 flex-shrink-0" aria-hidden="true">•</span>
                {item}
              </li>
            ))}
          </ul>
        </section>

        {/* Shared with */}
        <section className="senior-card mb-4" aria-labelledby="shared-heading">
          <h2 id="shared-heading" className="text-senior-lg font-bold text-gray-800 mb-2">공유 대상</h2>
          <ul className="space-y-1">
            {consent.sharedWith.map((s, i) => (
              <li key={i} className="flex items-start gap-2 text-senior-base text-gray-700">
                <span className="text-secondary-500 mt-0.5 flex-shrink-0" aria-hidden="true">→</span>
                {s}
              </li>
            ))}
          </ul>
        </section>

        {/* Retention and withdrawal */}
        <section className="senior-card mb-4" aria-labelledby="retention-heading">
          <h2 id="retention-heading" className="text-senior-lg font-bold text-gray-800 mb-2">보유 기간 및 철회</h2>
          <p className="text-senior-base text-gray-700 mb-2">
            <span className="font-semibold">보유 기간:</span> {consent.retentionPeriod}
          </p>
          <p className="text-senior-base text-gray-700">
            <span className="font-semibold">철회 방법:</span> {consent.withdrawalMethod}
          </p>
        </section>

        {/* History */}
        <section className="senior-card mb-5" aria-labelledby="history-heading">
          <h2 id="history-heading" className="text-senior-lg font-bold text-gray-800 mb-3">동의 이력</h2>
          <div className="space-y-2">
            {consent.history.map((h, i) => (
              <div key={i} className="flex items-center gap-3">
                <div className="w-2 h-2 rounded-full bg-primary-400 flex-shrink-0" aria-hidden="true" />
                <span className="text-senior-sm text-gray-500 w-32 flex-shrink-0">{h.date}</span>
                <span className="text-senior-base font-medium text-gray-700">{h.action}</span>
                <span className="text-senior-sm text-gray-400">({h.by})</span>
              </div>
            ))}
          </div>
        </section>

        <Link href="/consent" className="senior-btn-secondary w-full">
          동의 관리로 돌아가기
        </Link>
      </div>
    </SeniorAppShell>
  );
}
