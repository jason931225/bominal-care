// 예약 상세 — Appointment Detail
// Shows full information about a single appointment

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

// In production this data would be fetched by ID from the API
const MOCK_APPOINTMENT = {
  id: 'apt-1',
  doctor: '김민준 원장',
  specialty: '내과',
  hospital: '서울 중앙 의원',
  address: '서울특별시 중구 을지로 123, 3층',
  phone: '02-1234-5678',
  date: '2026년 3월 15일',
  time: '오전 10:30',
  duration: '30분',
  status: 'upcoming' as const,
  notes: '공복 혈액 검사 필요 — 당일 아침 금식',
  reason: '정기 혈압 및 당뇨 관리 검진',
  instructions: [
    '예약 10분 전 도착해 주세요.',
    '당일 아침 금식(물은 가능)이 필요합니다.',
    '현재 복용 중인 약 목록을 지참해 주세요.',
    '건강보험증 또는 신분증을 지참해 주세요.',
  ],
  caregiverNotified: true,
};

const STATUS_DISPLAY = {
  upcoming: { label: '예정', bg: 'bg-primary-100', text: 'text-primary-700' },
  completed: { label: '완료', bg: 'bg-success-50', text: 'text-success-700' },
  cancelled: { label: '취소', bg: 'bg-gray-100', text: 'text-gray-500' },
};

interface PageProps {
  params: Promise<{ id: string }>;
}

export default async function AppointmentDetailPage({ params: _params }: PageProps) {
  // In production: fetch appointment by (await _params).id from API
  const apt = MOCK_APPOINTMENT;
  const status = STATUS_DISPLAY[apt.status];

  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Back navigation */}
        <Link
          href="/appointments"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          예약 목록으로
        </Link>

        {/* Header card */}
        <div className="senior-card mb-4">
          <div className="flex items-start justify-between mb-3">
            <div>
              <h1 className="text-senior-2xl font-bold text-gray-900">{apt.doctor}</h1>
              <p className="text-senior-lg text-gray-600">{apt.specialty}</p>
            </div>
            <span className={`${status.bg} ${status.text} text-senior-sm font-bold px-3 py-1.5 rounded-full`}>
              {status.label}
            </span>
          </div>

          {/* Date and time — large and prominent */}
          <div className="bg-primary-50 rounded-xl p-4 mb-3">
            <div className="flex items-center gap-3">
              <div className="text-3xl" aria-hidden="true">📅</div>
              <div>
                <p className="text-senior-xl font-bold text-primary-800">{apt.date}</p>
                <p className="text-senior-lg text-primary-700">{apt.time} ({apt.duration})</p>
              </div>
            </div>
          </div>

          {/* Hospital info */}
          <div className="space-y-2">
            <div className="flex items-start gap-2">
              <svg className="w-5 h-5 text-gray-400 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
                <path strokeLinecap="round" strokeLinejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                <path strokeLinecap="round" strokeLinejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <div>
                <p className="text-senior-base font-semibold text-gray-800">{apt.hospital}</p>
                <p className="text-senior-sm text-gray-500">{apt.address}</p>
              </div>
            </div>
            <a
              href={`tel:${apt.phone}`}
              className="flex items-center gap-2 text-primary-600 font-medium text-senior-base min-h-touch"
            >
              <svg className="w-5 h-5 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
                <path strokeLinecap="round" strokeLinejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.948V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
              </svg>
              {apt.phone}
            </a>
          </div>
        </div>

        {/* Reason for visit */}
        <section className="senior-card mb-4" aria-labelledby="reason-heading">
          <h2 id="reason-heading" className="text-senior-lg font-bold text-gray-800 mb-2">방문 이유</h2>
          <p className="text-senior-base text-gray-700">{apt.reason}</p>
          {apt.notes && (
            <div className="mt-3 bg-warning-50 rounded-lg p-3">
              <p className="text-senior-sm font-semibold text-warning-700">📝 메모</p>
              <p className="text-senior-sm text-warning-700 mt-1">{apt.notes}</p>
            </div>
          )}
        </section>

        {/* Instructions */}
        <section className="senior-card mb-4" aria-labelledby="instructions-heading">
          <h2 id="instructions-heading" className="text-senior-lg font-bold text-gray-800 mb-3">준비사항</h2>
          <ul className="space-y-2">
            {apt.instructions.map((instruction, index) => (
              <li key={index} className="flex items-start gap-2">
                <span className="w-5 h-5 rounded-full bg-primary-100 text-primary-700 text-xs font-bold flex items-center justify-center flex-shrink-0 mt-0.5">
                  {index + 1}
                </span>
                <span className="text-senior-base text-gray-700">{instruction}</span>
              </li>
            ))}
          </ul>
        </section>

        {/* Caregiver notification */}
        <div className={`rounded-2xl px-4 py-3 mb-5 flex items-center gap-3 ${apt.caregiverNotified ? 'bg-success-50' : 'bg-gray-50'}`}>
          <span className="text-2xl" aria-hidden="true">{apt.caregiverNotified ? '✅' : '🔔'}</span>
          <p className="text-senior-base font-medium text-gray-700">
            {apt.caregiverNotified ? '보호자에게 알림이 전달됐습니다.' : '보호자에게 알림을 보내세요.'}
          </p>
        </div>

        {/* Actions */}
        <div className="space-y-3">
          <a
            href={`tel:${apt.phone}`}
            className="senior-btn-secondary w-full"
          >
            병원에 전화하기
          </a>
          <Link href="/appointments" className="block text-center text-senior-base text-gray-500 py-3 min-h-touch">
            예약 취소하기
          </Link>
        </div>
      </div>
    </SeniorAppShell>
  );
}
