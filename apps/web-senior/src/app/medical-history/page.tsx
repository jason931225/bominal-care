// 건강 기록 — Medical History Timeline
// Chronological view of medical events, diagnoses, procedures, and lab results

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

type RecordType = 'diagnosis' | 'procedure' | 'lab' | 'hospital' | 'vaccination';

interface MedicalRecord {
  id: string;
  date: string;
  type: RecordType;
  title: string;
  doctor: string;
  hospital: string;
  summary: string;
}

const MEDICAL_RECORDS: MedicalRecord[] = [
  {
    id: 'rec-1',
    date: '2026년 3월 1일',
    type: 'lab',
    title: '혈액 검사 결과',
    doctor: '김민준 원장',
    hospital: '서울 중앙 의원',
    summary: '공복 혈당 138mg/dL, 당화혈색소 7.2%, 혈압 142/88mmHg. 당뇨 조절 목표 미달 — 약 용량 조정 예정.',
  },
  {
    id: 'rec-2',
    date: '2026년 2월 15일',
    type: 'procedure',
    title: '무릎 X-ray 촬영',
    doctor: '이정훈 원장',
    hospital: '한양 정형외과',
    summary: '양측 무릎 퇴행성 관절염 3기 확인. 물리치료 주 2회 권고. 심한 통증 시 스테로이드 주사 고려.',
  },
  {
    id: 'rec-3',
    date: '2026년 1월 20일',
    type: 'vaccination',
    title: '독감 예방접종',
    doctor: '박영수 원장',
    hospital: '동네 가정의학과',
    summary: '2025-2026 인플루엔자 백신 접종 완료. 다음 접종 예정: 2026년 10월.',
  },
  {
    id: 'rec-4',
    date: '2026년 1월 10일',
    type: 'diagnosis',
    title: '고혈압 진단 업데이트',
    doctor: '김민준 원장',
    hospital: '서울 중앙 의원',
    summary: '암로디핀 5mg 신규 처방. 저염식 식이요법 및 규칙적 운동(하루 30분 걷기) 권고.',
  },
  {
    id: 'rec-5',
    date: '2025년 11월 5일',
    type: 'hospital',
    title: '응급실 내원',
    doctor: '응급의학과',
    hospital: '강남 세브란스 병원',
    summary: '낙상으로 인한 손목 타박상. X-ray 결과 골절 없음. 붕대 처치 후 귀가. 정형외과 외래 추적 권고.',
  },
  {
    id: 'rec-6',
    date: '2025년 10월 12일',
    type: 'lab',
    title: '안저 검사',
    doctor: '박수연 원장',
    hospital: '밝은 안과 클리닉',
    summary: '당뇨 망막증 초기 징후. 6개월마다 추적 관찰 필요. 혈당 조절 중요.',
  },
];

const TYPE_CONFIG: Record<RecordType, { label: string; color: string; icon: string }> = {
  diagnosis: { label: '진단', color: 'bg-primary-100 text-primary-700', icon: '🩺' },
  procedure: { label: '처치', color: 'bg-secondary-100 text-secondary-700', icon: '🔬' },
  lab: { label: '검사', color: 'bg-info-50 text-info-700', icon: '🧪' },
  hospital: { label: '입원/응급', color: 'bg-danger-50 text-danger-700', icon: '🏥' },
  vaccination: { label: '예방접종', color: 'bg-success-50 text-success-700', icon: '💉' },
};

export default function MedicalHistoryPage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-5">건강 기록</h1>

        {/* Legend */}
        <div className="flex flex-wrap gap-2 mb-5">
          {Object.entries(TYPE_CONFIG).map(([key, config]) => (
            <span key={key} className={`${config.color} text-senior-sm font-medium px-3 py-1 rounded-full`}>
              {config.icon} {config.label}
            </span>
          ))}
        </div>

        {/* Timeline */}
        <div className="relative">
          {/* Vertical timeline line */}
          <div className="absolute left-7 top-0 bottom-0 w-0.5 bg-gray-200" aria-hidden="true" />

          <div className="space-y-4">
            {MEDICAL_RECORDS.map((record) => {
              const config = TYPE_CONFIG[record.type];
              return (
                <Link
                  key={record.id}
                  href={`/medical-history/${record.id}`}
                  className="relative flex gap-4 group"
                >
                  {/* Timeline node */}
                  <div className={`relative z-10 w-14 h-14 rounded-full ${config.color} flex items-center justify-center text-2xl flex-shrink-0 border-2 border-white shadow-sm`} aria-hidden="true">
                    {config.icon}
                  </div>

                  {/* Card */}
                  <div className="flex-1 senior-card group-hover:shadow-md transition-shadow pb-4">
                    <div className="flex items-start justify-between mb-1">
                      <div>
                        <p className="text-senior-lg font-bold text-gray-900">{record.title}</p>
                        <p className="text-senior-sm text-gray-500">{record.hospital} · {record.doctor}</p>
                      </div>
                      <span className={`${config.color} text-senior-sm font-semibold px-2 py-0.5 rounded-full flex-shrink-0 ml-2`}>
                        {config.label}
                      </span>
                    </div>
                    <p className="text-senior-sm text-primary-600 font-medium mb-2">{record.date}</p>
                    <p className="text-senior-sm text-gray-600 line-clamp-2">{record.summary}</p>
                  </div>
                </Link>
              );
            })}
          </div>
        </div>
      </div>
    </SeniorAppShell>
  );
}
