import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const VISIT_DATA: Record<string, {
  id: string; clientName: string; clientAge: number; clientId: string;
  date: string; startTime: string; endTime: string;
  address: string; addressDetail: string; phone: string;
  services: string[]; notes: string; status: string;
  careplanItems: { label: string; done: boolean }[];
}> = {
  v001: {
    id: 'v001',
    clientName: '이순자',
    clientAge: 82,
    clientId: 'c001',
    date: '2026-03-15',
    startTime: '10:00',
    endTime: '13:00',
    address: '서울 강남구 대치동 123-45',
    addressDetail: '현대아파트 101동 1502호',
    phone: '010-2345-6789',
    services: ['목욕 지원', '식사 지원', '배변 지원'],
    notes: '혈압 약 10시에 복용 확인 필요. 미끄럼 방지 매트 사용 중. 낙상 위험 높음.',
    status: 'upcoming',
    careplanItems: [
      { label: '혈압약 복용 확인', done: false },
      { label: '입욕 보조 (30분)', done: false },
      { label: '점심 식사 준비 및 보조', done: false },
      { label: '낙상 위험 평가 기록', done: false },
    ],
  },
};

interface Props {
  params: Promise<{ visitId: string }>;
}

export default async function VisitDetailPage({ params }: Props) {
  const { visitId } = await params;
  const visit = VISIT_DATA[visitId] ?? VISIT_DATA.v001;

  const statusMap: Record<string, { label: string; color: string }> = {
    upcoming: { label: '예정', color: 'badge-info' },
    checkedin: { label: '방문 중', color: 'badge-success' },
    completed: { label: '완료', color: 'badge-success' },
    cancelled: { label: '취소', color: 'badge-danger' },
  };

  const statusInfo = statusMap[visit.status] ?? statusMap.upcoming;

  return (
    <CaregiverAppShell
      activeTab="schedule"
      title="방문 상세"
      showBackButton
      backHref="/schedule"
    >
      <div className="px-4 py-4 space-y-4">
        {/* Client Header */}
        <div className="card">
          <div className="flex items-start justify-between mb-4">
            <div className="flex items-center gap-3">
              <div className="w-14 h-14 bg-blue-100 rounded-2xl flex items-center justify-center text-2xl flex-shrink-0">
                👵
              </div>
              <div>
                <h2 className="text-lg font-bold text-slate-900">{visit.clientName} 어르신</h2>
                <p className="text-sm text-slate-500">{visit.clientAge}세</p>
              </div>
            </div>
            <span className={statusInfo.color}>{statusInfo.label}</span>
          </div>

          <div className="space-y-2.5">
            <div className="flex items-start gap-2.5">
              <svg className="w-4 h-4 text-slate-400 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <span className="text-sm text-slate-700">{visit.date} {visit.startTime} – {visit.endTime}</span>
            </div>
            <div className="flex items-start gap-2.5">
              <svg className="w-4 h-4 text-slate-400 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                <path strokeLinecap="round" strokeLinejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <div>
                <p className="text-sm text-slate-700">{visit.address}</p>
                <p className="text-xs text-slate-400">{visit.addressDetail}</p>
              </div>
            </div>
            <div className="flex items-center gap-2.5">
              <svg className="w-4 h-4 text-slate-400 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
              </svg>
              <a href={`tel:${visit.phone}`} className="text-sm text-blue-600 font-medium">{visit.phone}</a>
            </div>
          </div>
        </div>

        {/* Services */}
        <div className="card">
          <h3 className="section-title">제공 서비스</h3>
          <div className="flex flex-wrap gap-2">
            {visit.services.map((s) => (
              <span key={s} className="badge-info">{s}</span>
            ))}
          </div>
        </div>

        {/* Care Plan Checklist */}
        <div className="card">
          <div className="flex items-center justify-between mb-3">
            <h3 className="section-title mb-0">오늘의 케어 항목</h3>
            <Link href={`/clients/${visit.clientId}/care-plan`} className="text-xs text-blue-600 font-medium">
              전체 케어플랜
            </Link>
          </div>
          <div className="space-y-3">
            {visit.careplanItems.map((item, idx) => (
              <div key={idx} className="flex items-center gap-3">
                <div className={`w-5 h-5 rounded border-2 flex items-center justify-center flex-shrink-0 ${
                  item.done ? 'border-green-500 bg-green-500' : 'border-slate-300'
                }`}>
                  {item.done && (
                    <svg className="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                      <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                    </svg>
                  )}
                </div>
                <span className={`text-sm ${item.done ? 'line-through text-slate-400' : 'text-slate-700'}`}>
                  {item.label}
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Notes */}
        <div className="card">
          <h3 className="section-title">특이사항 / 주의사항</h3>
          <p className="text-sm text-slate-700 leading-relaxed bg-amber-50 rounded-xl p-3 border border-amber-100">
            ⚠️ {visit.notes}
          </p>
        </div>

        {/* Quick Links */}
        <div className="grid grid-cols-2 gap-3">
          <Link href={`/clients/${visit.clientId}`}>
            <div className="card flex items-center gap-2 active:scale-95 transition-transform">
              <span className="text-xl">👤</span>
              <span className="text-sm font-medium text-slate-700">이용자 프로필</span>
            </div>
          </Link>
          <Link href={`/clients/${visit.clientId}/medications`}>
            <div className="card flex items-center gap-2 active:scale-95 transition-transform">
              <span className="text-xl">💊</span>
              <span className="text-sm font-medium text-slate-700">투약 정보</span>
            </div>
          </Link>
        </div>

        {/* Check In/Out Actions */}
        <div className="space-y-3 pb-6">
          <Link href={`/schedule/${visit.id}/checkin`} className="btn-primary block text-center">
            체크인
          </Link>
          <Link href="/notes/new" className="btn-secondary block text-center">
            케어일지 작성
          </Link>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
