// 케어 개요 — Care Overview
// Shows the active care plan summary, assigned care manager, and upcoming visits

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const CARE_PLAN = {
  planName: '재가 돌봄 서비스 플랜 A',
  level: '장기요양 3등급',
  manager: {
    name: '이수진',
    title: '케어매니저',
    phone: '010-9876-5432',
    agency: '행복 케어 센터',
    photo: null,
  },
  startDate: '2025년 11월 1일',
  endDate: '2026년 10월 31일',
  status: 'active' as const,
  nextVisit: {
    date: '2026년 3월 15일',
    time: '오후 2:00',
    type: '방문 요양',
  },
  monthlyStats: {
    totalVisits: 12,
    completedVisits: 3,
    hoursProvided: 9,
    hoursPlanned: 36,
  },
};

const CARE_SERVICES = [
  { id: 'svc-1', name: '방문 요양', icon: '🏠', frequency: '주 3회', provider: '한빛 요양원', active: true },
  { id: 'svc-2', name: '방문 목욕', icon: '🛁', frequency: '주 1회', provider: '청결 재가서비스', active: true },
  { id: 'svc-3', name: '방문 간호', icon: '💉', frequency: '격주 1회', provider: '서울 방문간호센터', active: true },
  { id: 'svc-4', name: '주야간 보호', icon: '☀️', frequency: '미이용', provider: '—', active: false },
];

const UPCOMING_VISITS = [
  { id: 'v-1', date: '3월 15일 (오늘)', time: '오후 2:00', type: '방문 요양', worker: '김복순' },
  { id: 'v-2', date: '3월 17일 (화)', time: '오전 10:00', type: '방문 목욕', worker: '박미경' },
  { id: 'v-3', date: '3월 19일 (목)', time: '오후 2:00', type: '방문 요양', worker: '김복순' },
  { id: 'v-4', date: '3월 22일 (일)', time: '오후 2:00', type: '방문 요양', worker: '김복순' },
];

export default function CarePage() {
  const stats = CARE_PLAN.monthlyStats;
  const progressPct = Math.round((stats.completedVisits / stats.totalVisits) * 100);

  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-5">케어 서비스</h1>

        {/* Active plan summary */}
        <div className="bg-gradient-to-br from-secondary-600 to-secondary-700 rounded-2xl p-5 text-white mb-5">
          <div className="flex items-start justify-between mb-3">
            <div>
              <span className="text-senior-sm text-secondary-200 uppercase tracking-wide">현재 플랜</span>
              <h2 className="text-senior-xl font-bold mt-0.5">{CARE_PLAN.planName}</h2>
              <p className="text-senior-base text-secondary-100">{CARE_PLAN.level}</p>
            </div>
            <span className="bg-white/20 text-white text-senior-sm font-semibold px-3 py-1 rounded-full">
              이용 중
            </span>
          </div>
          <p className="text-senior-sm text-secondary-200">
            {CARE_PLAN.startDate} ~ {CARE_PLAN.endDate}
          </p>
          <Link
            href="/care/plan"
            className="inline-flex items-center gap-1 mt-3 text-secondary-100 hover:text-white text-senior-sm font-medium"
          >
            플랜 상세 보기
            <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
            </svg>
          </Link>
        </div>

        {/* Next visit alert */}
        <div className="bg-primary-50 border-2 border-primary-300 rounded-2xl p-4 mb-5 flex items-center gap-3">
          <span className="text-3xl" aria-hidden="true">📋</span>
          <div>
            <p className="text-senior-base font-bold text-primary-800">다음 방문 예정</p>
            <p className="text-senior-base text-primary-700">
              {CARE_PLAN.nextVisit.date} {CARE_PLAN.nextVisit.time} — {CARE_PLAN.nextVisit.type}
            </p>
          </div>
        </div>

        {/* Monthly progress */}
        <section className="senior-card mb-5" aria-labelledby="monthly-progress">
          <h2 id="monthly-progress" className="text-senior-lg font-bold text-gray-800 mb-3">이번 달 이용 현황</h2>
          <div className="grid grid-cols-2 gap-3 mb-4">
            <div className="bg-gray-50 rounded-xl p-3 text-center">
              <p className="text-senior-2xl font-bold text-secondary-600">{stats.completedVisits}</p>
              <p className="text-senior-sm text-gray-500">완료 방문</p>
            </div>
            <div className="bg-gray-50 rounded-xl p-3 text-center">
              <p className="text-senior-2xl font-bold text-secondary-600">{stats.hoursProvided}h</p>
              <p className="text-senior-sm text-gray-500">제공 시간</p>
            </div>
          </div>
          {/* Progress bar */}
          <div>
            <div className="flex justify-between text-senior-sm text-gray-600 mb-1">
              <span>이용 진행률</span>
              <span>{stats.completedVisits} / {stats.totalVisits}회</span>
            </div>
            <div className="h-4 bg-gray-200 rounded-full overflow-hidden" role="progressbar" aria-valuenow={progressPct} aria-valuemin={0} aria-valuemax={100}>
              <div
                className="h-full bg-secondary-500 rounded-full transition-all"
                style={{ width: `${progressPct}%` }}
              />
            </div>
            <p className="text-senior-sm text-gray-500 mt-1 text-right">{progressPct}% 완료</p>
          </div>
        </section>

        {/* Care manager card */}
        <section className="senior-card mb-5" aria-labelledby="care-manager-heading">
          <h2 id="care-manager-heading" className="text-senior-lg font-bold text-gray-800 mb-3">담당 케어매니저</h2>
          <div className="flex items-center gap-4">
            <div className="w-16 h-16 rounded-full bg-secondary-100 flex items-center justify-center text-2xl flex-shrink-0" aria-hidden="true">
              👩‍⚕️
            </div>
            <div className="flex-1">
              <p className="text-senior-xl font-bold text-gray-900">{CARE_PLAN.manager.name}</p>
              <p className="text-senior-base text-gray-600">{CARE_PLAN.manager.title} · {CARE_PLAN.manager.agency}</p>
            </div>
          </div>
          <a
            href={`tel:${CARE_PLAN.manager.phone}`}
            className="senior-btn-secondary w-full mt-4"
          >
            📞 {CARE_PLAN.manager.phone} 전화하기
          </a>
        </section>

        {/* Active services */}
        <section aria-labelledby="services-heading">
          <h2 id="services-heading" className="senior-section-title">이용 중인 서비스</h2>
          <div className="space-y-2 mb-5">
            {CARE_SERVICES.map((svc) => (
              <div
                key={svc.id}
                className={`senior-card flex items-center gap-4 ${!svc.active ? 'opacity-50' : ''}`}
              >
                <span className="text-2xl" aria-hidden="true">{svc.icon}</span>
                <div className="flex-1">
                  <p className="text-senior-base font-bold text-gray-800">{svc.name}</p>
                  <p className="text-senior-sm text-gray-500">{svc.frequency} · {svc.provider}</p>
                </div>
                <span className={`text-senior-sm font-semibold px-2.5 py-1 rounded-full ${svc.active ? 'bg-success-50 text-success-700' : 'bg-gray-100 text-gray-400'}`}>
                  {svc.active ? '이용 중' : '미이용'}
                </span>
              </div>
            ))}
          </div>
        </section>

        {/* Upcoming visits */}
        <section aria-labelledby="upcoming-visits-heading">
          <h2 id="upcoming-visits-heading" className="senior-section-title">예정된 방문 일정</h2>
          <div className="space-y-2">
            {UPCOMING_VISITS.map((visit) => (
              <div key={visit.id} className="flex items-center gap-3 bg-white rounded-xl border border-gray-100 p-3">
                <div className="w-2 h-2 rounded-full bg-secondary-500 flex-shrink-0" aria-hidden="true" />
                <div className="flex-1">
                  <p className="text-senior-base font-semibold text-gray-800">{visit.date} {visit.time}</p>
                  <p className="text-senior-sm text-gray-500">{visit.type} — {visit.worker}</p>
                </div>
              </div>
            ))}
          </div>
        </section>
      </div>
    </SeniorAppShell>
  );
}
