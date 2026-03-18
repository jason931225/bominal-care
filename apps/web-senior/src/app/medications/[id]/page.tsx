// 약물 상세 — Medication Detail
// Full information about a single medication including instructions and side effects

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

// Mock data — in production fetched by (await params).id
const MOCK_MED = {
  id: 'med-1',
  name: '암로디핀',
  nameEn: 'Amlodipine 5mg',
  category: '칼슘 채널 차단제',
  dosage: '5mg 1정',
  frequency: '하루 1회 — 아침 식후',
  prescribedBy: '김민준 원장',
  prescribedDate: '2026년 1월 10일',
  purpose: '고혈압 치료 및 혈압 조절',
  color: '흰색',
  shape: '원형',
  manufacturer: '한미약품',
  storage: '직사광선을 피해 실온(1~30°C) 보관',
  remaining: 24,
  refillDate: '2026년 4월 8일',
  instructions: [
    '아침 식사 후 30분 이내에 복용하세요.',
    '물 한 컵(200ml) 이상과 함께 복용하세요.',
    '자몽 주스와 함께 복용하지 마세요.',
    '임의로 복용을 중단하지 마세요.',
  ],
  sideEffects: [
    { severity: '흔한', description: '발목 부종, 얼굴 홍조, 두통' },
    { severity: '드문', description: '어지러움, 피로감, 메스꺼움' },
    { severity: '즉시 상담', description: '극심한 어지러움, 심한 두통, 흉통' },
  ],
  interactions: ['자몽 / 자몽 주스', '특정 항진균제 (의사 상담 필요)'],
};

interface PageProps {
  params: Promise<{ id: string }>;
}

export default async function MedicationDetailPage({ params: _params }: PageProps) {
  const med = MOCK_MED;
  const lowStock = med.remaining <= 10;

  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Back */}
        <Link
          href="/medications"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          약물 목록으로
        </Link>

        {/* Header */}
        <div className="senior-card mb-4">
          <div className="flex items-start gap-4 mb-4">
            <div className="w-16 h-16 rounded-2xl bg-warning-50 border-2 border-warning-200 flex items-center justify-center text-3xl flex-shrink-0" aria-hidden="true">
              💊
            </div>
            <div>
              <h1 className="text-senior-2xl font-bold text-gray-900">{med.name}</h1>
              <p className="text-senior-base text-gray-500">{med.nameEn}</p>
              <p className="text-senior-sm text-gray-400">{med.category}</p>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div className="bg-gray-50 rounded-xl p-3">
              <p className="text-senior-sm text-gray-500">용량</p>
              <p className="text-senior-base font-bold text-gray-800">{med.dosage}</p>
            </div>
            <div className="bg-gray-50 rounded-xl p-3">
              <p className="text-senior-sm text-gray-500">복용 횟수</p>
              <p className="text-senior-base font-bold text-gray-800">하루 1회</p>
            </div>
            <div className="bg-gray-50 rounded-xl p-3">
              <p className="text-senior-sm text-gray-500">모양 / 색상</p>
              <p className="text-senior-base font-bold text-gray-800">{med.shape} / {med.color}</p>
            </div>
            <div className={`rounded-xl p-3 ${lowStock ? 'bg-danger-50' : 'bg-gray-50'}`}>
              <p className="text-senior-sm text-gray-500">잔여량</p>
              <p className={`text-senior-base font-bold ${lowStock ? 'text-danger-600' : 'text-gray-800'}`}>
                {lowStock && '⚠️ '}{med.remaining}정
              </p>
            </div>
          </div>
        </div>

        {/* Refill warning */}
        <div className="bg-warning-50 border border-warning-300 rounded-2xl p-3 mb-4 flex items-center gap-3">
          <span className="text-2xl" aria-hidden="true">📅</span>
          <div>
            <p className="text-senior-sm font-bold text-warning-700">재처방 예정일</p>
            <p className="text-senior-base text-warning-700">{med.refillDate}</p>
          </div>
        </div>

        {/* Purpose */}
        <section className="senior-card mb-4" aria-labelledby="purpose-heading">
          <h2 id="purpose-heading" className="text-senior-lg font-bold text-gray-800 mb-2">복용 목적</h2>
          <p className="text-senior-base text-gray-700">{med.purpose}</p>
          <div className="mt-3 flex items-center gap-2 text-senior-sm text-gray-500">
            <span>처방: {med.prescribedBy}</span>
            <span>·</span>
            <span>{med.prescribedDate}</span>
          </div>
        </section>

        {/* Instructions */}
        <section className="senior-card mb-4" aria-labelledby="instructions-heading">
          <h2 id="instructions-heading" className="text-senior-lg font-bold text-gray-800 mb-3">복용 방법</h2>
          <ul className="space-y-2">
            {med.instructions.map((inst, i) => (
              <li key={i} className="flex items-start gap-2">
                <span className="text-primary-500 flex-shrink-0 mt-0.5" aria-hidden="true">•</span>
                <span className="text-senior-base text-gray-700">{inst}</span>
              </li>
            ))}
          </ul>
          <div className="mt-3 bg-gray-50 rounded-lg p-3">
            <p className="text-senior-sm text-gray-600">
              🗄 보관: {med.storage}
            </p>
          </div>
        </section>

        {/* Side effects */}
        <section className="senior-card mb-4" aria-labelledby="side-effects-heading">
          <h2 id="side-effects-heading" className="text-senior-lg font-bold text-gray-800 mb-3">주의사항 및 부작용</h2>
          <div className="space-y-3">
            {med.sideEffects.map((effect, i) => {
              const isUrgent = effect.severity === '즉시 상담';
              return (
                <div key={i} className={`rounded-xl p-3 ${isUrgent ? 'bg-danger-50' : 'bg-gray-50'}`}>
                  <p className={`text-senior-sm font-bold mb-1 ${isUrgent ? 'text-danger-700' : 'text-gray-600'}`}>
                    {isUrgent ? '🚨 ' : ''}{effect.severity}
                  </p>
                  <p className={`text-senior-base ${isUrgent ? 'text-danger-700' : 'text-gray-700'}`}>
                    {effect.description}
                  </p>
                </div>
              );
            })}
          </div>
        </section>

        {/* Drug interactions */}
        <section className="senior-card mb-5" aria-labelledby="interactions-heading">
          <h2 id="interactions-heading" className="text-senior-lg font-bold text-gray-800 mb-2">주의 성분 / 음식</h2>
          <ul className="space-y-1">
            {med.interactions.map((item, i) => (
              <li key={i} className="flex items-center gap-2 text-senior-base text-danger-700">
                <span aria-hidden="true">⛔</span>
                {item}
              </li>
            ))}
          </ul>
        </section>

        <Link href="/medications" className="senior-btn-secondary w-full">
          약물 목록으로 돌아가기
        </Link>
      </div>
    </SeniorAppShell>
  );
}
