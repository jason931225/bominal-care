// 건강 기록 상세 — Medical History Entry Detail

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const MOCK_RECORD = {
  id: 'rec-1',
  date: '2026년 3월 1일',
  type: 'lab' as const,
  title: '혈액 검사 결과',
  doctor: '김민준 원장',
  specialty: '내과',
  hospital: '서울 중앙 의원',
  summary: '공복 혈당 138mg/dL, 당화혈색소 7.2%, 혈압 142/88mmHg. 당뇨 조절 목표 미달 — 약 용량 조정 예정.',
  labResults: [
    { name: '공복 혈당', value: '138 mg/dL', normal: '70–100 mg/dL', status: 'high' as const },
    { name: '당화혈색소 (HbA1c)', value: '7.2%', normal: '< 5.7%', status: 'high' as const },
    { name: '수축기 혈압', value: '142 mmHg', normal: '< 120 mmHg', status: 'high' as const },
    { name: '이완기 혈압', value: '88 mmHg', normal: '< 80 mmHg', status: 'high' as const },
    { name: '총 콜레스테롤', value: '195 mg/dL', normal: '< 200 mg/dL', status: 'normal' as const },
    { name: '신장기능 (크레아티닌)', value: '0.9 mg/dL', normal: '0.6–1.2 mg/dL', status: 'normal' as const },
  ],
  doctorNotes: '당화혈색소가 목표치(7.0%)를 다소 초과합니다. 메트포르민 용량을 500mg → 1000mg으로 증량합니다. 저탄수화물 식이 유지를 강조드립니다. 3개월 후 재검 예정.',
  followUp: '2026년 6월 첫째 주 혈액 재검',
  attachments: ['혈액검사_결과지_20260301.pdf'],
};

const RESULT_STATUS = {
  normal: { label: '정상', bg: 'bg-success-50', text: 'text-success-700', dot: 'bg-success-500' },
  high: { label: '높음', bg: 'bg-danger-50', text: 'text-danger-700', dot: 'bg-danger-500' },
  low: { label: '낮음', bg: 'bg-warning-50', text: 'text-warning-700', dot: 'bg-warning-500' },
};

interface PageProps {
  params: Promise<{ id: string }>;
}

export default async function MedicalHistoryDetailPage({ params: _params }: PageProps) {
  const record = MOCK_RECORD;

  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Back */}
        <Link
          href="/medical-history"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          건강 기록으로
        </Link>

        {/* Header */}
        <div className="senior-card mb-4">
          <div className="flex items-start gap-3 mb-3">
            <span className="text-3xl" aria-hidden="true">🧪</span>
            <div>
              <h1 className="text-senior-2xl font-bold text-gray-900">{record.title}</h1>
              <p className="text-senior-base text-gray-600">{record.hospital} · {record.doctor}</p>
              <p className="text-senior-sm text-primary-600 font-medium mt-1">{record.date}</p>
            </div>
          </div>
          <p className="text-senior-base text-gray-700 leading-relaxed">{record.summary}</p>
        </div>

        {/* Lab results table */}
        {record.labResults && (
          <section className="senior-card mb-4" aria-labelledby="lab-results-heading">
            <h2 id="lab-results-heading" className="text-senior-lg font-bold text-gray-800 mb-3">검사 수치</h2>
            <div className="space-y-2">
              {record.labResults.map((result, i) => {
                const statusConfig = RESULT_STATUS[result.status];
                return (
                  <div key={i} className={`${statusConfig.bg} rounded-xl p-3 flex items-center gap-3`}>
                    <div className={`w-2.5 h-2.5 rounded-full ${statusConfig.dot} flex-shrink-0`} aria-hidden="true" />
                    <div className="flex-1">
                      <p className="text-senior-base font-semibold text-gray-800">{result.name}</p>
                      <p className="text-senior-sm text-gray-500">정상범위: {result.normal}</p>
                    </div>
                    <div className="text-right flex-shrink-0">
                      <p className={`text-senior-base font-bold ${statusConfig.text}`}>{result.value}</p>
                      <p className={`text-senior-sm font-medium ${statusConfig.text}`}>{statusConfig.label}</p>
                    </div>
                  </div>
                );
              })}
            </div>
          </section>
        )}

        {/* Doctor notes */}
        <section className="senior-card mb-4" aria-labelledby="doctor-notes-heading">
          <h2 id="doctor-notes-heading" className="text-senior-lg font-bold text-gray-800 mb-2">의사 소견</h2>
          <p className="text-senior-base text-gray-700 leading-relaxed">{record.doctorNotes}</p>
        </section>

        {/* Follow-up */}
        <div className="bg-primary-50 border border-primary-200 rounded-2xl p-4 mb-4 flex items-center gap-3">
          <span className="text-2xl" aria-hidden="true">📅</span>
          <div>
            <p className="text-senior-sm font-bold text-primary-700">다음 추적 검사</p>
            <p className="text-senior-base text-primary-700">{record.followUp}</p>
          </div>
        </div>

        {/* Attachments */}
        {record.attachments.length > 0 && (
          <section className="senior-card mb-5" aria-labelledby="attachments-heading">
            <h2 id="attachments-heading" className="text-senior-lg font-bold text-gray-800 mb-2">첨부 파일</h2>
            {record.attachments.map((file, i) => (
              <div key={i} className="flex items-center gap-3 py-2">
                <span className="text-2xl" aria-hidden="true">📄</span>
                <span className="text-senior-base text-primary-600 font-medium">{file}</span>
              </div>
            ))}
          </section>
        )}

        <Link href="/medical-history" className="senior-btn-secondary w-full">
          건강 기록으로 돌아가기
        </Link>
      </div>
    </SeniorAppShell>
  );
}
