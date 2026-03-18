import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const TODAY = '2026년 3월 15일 (일)';

const NEXT_VISIT = {
  id: 'v001',
  clientName: '이순자',
  address: '서울 강남구 대치동 123-45',
  startTime: '10:00',
  endTime: '13:00',
  services: ['목욕 지원', '식사 지원'],
  distance: '1.2km',
  remainingMin: 42,
};

const TODAY_SCHEDULE = [
  { id: 'v001', clientName: '이순자', time: '10:00–13:00', status: 'upcoming', type: '목욕·식사' },
  { id: 'v002', clientName: '박영철', time: '14:30–17:30', status: 'upcoming', type: '가사·투약' },
];

const ALERTS = [
  { id: 'a1', type: 'medication', message: '이순자님 오전 투약 확인 필요', time: '09:45', urgent: true },
  { id: 'a2', type: 'schedule', message: '내일 오전 일정 변경 요청이 도착했습니다.', time: '08:30', urgent: false },
  { id: 'a3', type: 'task', message: '박영철님 케어플랜 업데이트 완료', time: '08:00', urgent: false },
];

const WEEKLY_STATS = {
  completedVisits: 12,
  totalHours: 28.5,
  earnedAmount: '434,655원',
  rating: 4.9,
};

export default function DashboardPage() {
  return (
    <CaregiverAppShell activeTab="schedule">
      <div className="px-4 pt-5 pb-4 space-y-5">
        {/* Greeting */}
        <div className="flex items-start justify-between">
          <div>
            <p className="text-sm text-slate-500">{TODAY}</p>
            <h1 className="text-xl font-bold text-slate-900 mt-0.5">
              안녕하세요, <span className="text-blue-600">김요양</span>님 👋
            </h1>
          </div>
          <Link href="/notifications" className="relative p-2 bg-white rounded-xl shadow-sm border border-slate-100">
            <svg className="w-5 h-5 text-slate-600" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
            </svg>
            <span className="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full" />
          </Link>
        </div>

        {/* Next Visit Card */}
        <div className="bg-gradient-to-br from-blue-600 to-indigo-600 rounded-2xl p-5 text-white">
          <div className="flex items-center justify-between mb-3">
            <span className="text-blue-200 text-xs font-medium">다음 방문</span>
            <span className="bg-white/20 text-white text-xs font-semibold px-2.5 py-1 rounded-full">
              {NEXT_VISIT.remainingMin}분 후
            </span>
          </div>
          <div className="flex items-start justify-between">
            <div>
              <p className="text-xl font-bold">{NEXT_VISIT.clientName} 어르신</p>
              <p className="text-blue-200 text-sm mt-1">
                {NEXT_VISIT.startTime} – {NEXT_VISIT.endTime}
              </p>
              <p className="text-blue-200 text-xs mt-1 truncate max-w-48">{NEXT_VISIT.address}</p>
              <div className="flex gap-1.5 mt-2 flex-wrap">
                {NEXT_VISIT.services.map((s) => (
                  <span key={s} className="bg-white/20 text-white text-xs px-2 py-0.5 rounded-full">{s}</span>
                ))}
              </div>
            </div>
          </div>
          <div className="flex gap-2 mt-4">
            <Link
              href={`/schedule/${NEXT_VISIT.id}`}
              className="flex-1 bg-white text-blue-700 font-semibold text-sm py-2.5 rounded-xl text-center active:scale-95 transition-transform"
            >
              방문 상세
            </Link>
            <Link
              href={`/schedule/${NEXT_VISIT.id}/checkin`}
              className="flex-1 bg-blue-500 text-white font-semibold text-sm py-2.5 rounded-xl text-center active:scale-95 transition-transform"
            >
              체크인
            </Link>
          </div>
        </div>

        {/* Alerts */}
        {ALERTS.length > 0 && (
          <div>
            <div className="flex items-center justify-between mb-3">
              <h2 className="section-title mb-0">알림</h2>
              <Link href="/notifications" className="text-xs text-blue-600 font-medium">전체 보기</Link>
            </div>
            <div className="space-y-2">
              {ALERTS.map((alert) => (
                <div
                  key={alert.id}
                  className={`card flex items-start gap-3 ${alert.urgent ? 'border-red-200 bg-red-50' : ''}`}
                >
                  <div className={`w-8 h-8 rounded-xl flex items-center justify-center flex-shrink-0 ${
                    alert.urgent ? 'bg-red-100' : 'bg-slate-100'
                  }`}>
                    {alert.type === 'medication' ? (
                      <span className="text-base">💊</span>
                    ) : alert.type === 'schedule' ? (
                      <span className="text-base">📅</span>
                    ) : (
                      <span className="text-base">✅</span>
                    )}
                  </div>
                  <div className="flex-1 min-w-0">
                    <p className={`text-sm leading-snug ${alert.urgent ? 'text-red-700 font-medium' : 'text-slate-700'}`}>
                      {alert.message}
                    </p>
                    <p className="text-xs text-slate-400 mt-0.5">{alert.time}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Today Schedule */}
        <div>
          <div className="flex items-center justify-between mb-3">
            <h2 className="section-title mb-0">오늘 일정</h2>
            <Link href="/schedule" className="text-xs text-blue-600 font-medium">전체 보기</Link>
          </div>
          <div className="space-y-2">
            {TODAY_SCHEDULE.map((visit) => (
              <Link key={visit.id} href={`/schedule/${visit.id}`}>
                <div className="card flex items-center gap-4 active:scale-98 transition-transform">
                  <div className="w-12 h-12 bg-blue-50 rounded-xl flex flex-col items-center justify-center flex-shrink-0">
                    <span className="text-xs font-bold text-blue-600 leading-none">{visit.time.split('–')[0]}</span>
                  </div>
                  <div className="flex-1">
                    <p className="text-sm font-semibold text-slate-800">{visit.clientName} 어르신</p>
                    <p className="text-xs text-slate-500 mt-0.5">{visit.time} · {visit.type}</p>
                  </div>
                  <span className={`badge-info`}>예정</span>
                </div>
              </Link>
            ))}
          </div>
        </div>

        {/* Weekly Stats */}
        <div>
          <h2 className="section-title">이번 주 현황</h2>
          <div className="grid grid-cols-2 gap-3">
            <div className="card text-center py-4">
              <p className="text-2xl font-bold text-blue-600">{WEEKLY_STATS.completedVisits}</p>
              <p className="text-xs text-slate-500 mt-1">완료 방문</p>
            </div>
            <div className="card text-center py-4">
              <p className="text-2xl font-bold text-blue-600">{WEEKLY_STATS.totalHours}h</p>
              <p className="text-xs text-slate-500 mt-1">근무 시간</p>
            </div>
            <div className="card text-center py-4">
              <p className="text-xl font-bold text-green-600">{WEEKLY_STATS.earnedAmount}</p>
              <p className="text-xs text-slate-500 mt-1">예상 수당</p>
            </div>
            <div className="card text-center py-4">
              <p className="text-2xl font-bold text-amber-500">{WEEKLY_STATS.rating}</p>
              <p className="text-xs text-slate-500 mt-1">평균 평점 ⭐</p>
            </div>
          </div>
        </div>

        {/* Quick Actions */}
        <div>
          <h2 className="section-title">빠른 실행</h2>
          <div className="grid grid-cols-4 gap-3">
            {[
              { label: '케어일지', href: '/notes/new', icon: '📝' },
              { label: '사고보고', href: '/notes/incident', icon: '🚨' },
              { label: '투약기록', href: '/medications', icon: '💊' },
              { label: '내 일정', href: '/profile/availability', icon: '🗓️' },
            ].map((action) => (
              <Link key={action.href} href={action.href}>
                <div className="card flex flex-col items-center gap-2 py-4 active:scale-95 transition-transform">
                  <span className="text-2xl">{action.icon}</span>
                  <span className="text-xs text-slate-600 font-medium text-center">{action.label}</span>
                </div>
              </Link>
            ))}
          </div>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
