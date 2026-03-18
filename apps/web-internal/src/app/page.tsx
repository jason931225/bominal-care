import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const STATS = [
  { label: '활성 이용자', value: '48', sub: '전월 대비 +3', color: 'blue' },
  { label: '소속 요양보호사', value: '31', sub: '대기 2명 포함', color: 'green' },
  { label: '오늘 방문 예정', value: '24', sub: '완료 18 / 미완 6', color: 'purple' },
  { label: '미처리 알림', value: '7', sub: '긴급 2건 포함', color: 'red' },
];

const EXCEPTIONS = [
  { id: 'EXC-001', type: '방문 미이행', client: '박순자 (72세)', caregiver: '이민정', time: '오전 09:00', severity: 'high', status: '미처리' },
  { id: 'EXC-002', type: '일정 충돌', client: '김복동 (68세)', caregiver: '최수진', time: '오후 02:00', severity: 'medium', status: '검토중' },
  { id: 'EXC-003', type: '건강 이상', client: '이정자 (81세)', caregiver: '정미영', time: '오전 11:30', severity: 'high', status: '미처리' },
  { id: 'EXC-004', type: '서류 미비', client: '조길동 (75세)', caregiver: '-', time: '-', severity: 'low', status: '대기' },
  { id: 'EXC-005', type: '요양보호사 지각', client: '강명순 (69세)', caregiver: '한지영', time: '오전 08:00', severity: 'medium', status: '처리완료' },
];

const RECENT_VISITS = [
  { client: '박순자', caregiver: '이민정', service: '방문요양', time: '09:00 - 11:00', status: '완료' },
  { client: '김영희', caregiver: '최수진', service: '방문목욕', time: '10:00 - 12:00', status: '완료' },
  { client: '이철수', caregiver: '정미영', service: '방문요양', time: '13:00 - 15:00', status: '진행중' },
  { client: '홍길자', caregiver: '김은지', service: '방문요양', time: '14:00 - 16:00', status: '예정' },
  { client: '장미숙', caregiver: '오혜진', service: '방문목욕', time: '15:30 - 17:30', status: '예정' },
];

const SEVERITY_BADGE: Record<string, string> = {
  high: 'badge-red',
  medium: 'badge-yellow',
  low: 'badge-blue',
};

const SEVERITY_LABEL: Record<string, string> = { high: '높음', medium: '보통', low: '낮음' };

const STATUS_BADGE: Record<string, string> = {
  '미처리': 'badge-red',
  '검토중': 'badge-yellow',
  '대기': 'badge-blue',
  '처리완료': 'badge-green',
};

const VISIT_STATUS_BADGE: Record<string, string> = {
  '완료': 'badge-green',
  '진행중': 'badge-blue',
  '예정': 'badge-gray',
};

const STAT_COLORS: Record<string, string> = {
  blue: 'bg-blue-50',
  green: 'bg-green-50',
  purple: 'bg-purple-50',
  red: 'bg-red-50',
};

const STAT_TEXT_COLORS: Record<string, string> = {
  blue: 'text-blue-700',
  green: 'text-green-700',
  purple: 'text-purple-700',
  red: 'text-red-700',
};

export default function DashboardPage() {
  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">대시보드</h1>
            <p className="text-sm text-gray-500 mt-1">2026년 3월 15일 일요일 기준</p>
          </div>
          <Link href="/reports" className="btn-secondary">
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            보고서 보기
          </Link>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-4 gap-4">
          {STATS.map((stat) => (
            <div key={stat.label} className="stat-card">
              <p className="text-sm font-medium text-gray-500">{stat.label}</p>
              <p className="text-3xl font-bold text-gray-900 mt-1">{stat.value}</p>
              <p className={`text-xs mt-2 font-medium px-2 py-0.5 rounded-full inline-block ${STAT_COLORS[stat.color]} ${STAT_TEXT_COLORS[stat.color]}`}>
                {stat.sub}
              </p>
            </div>
          ))}
        </div>

        {/* Exceptions + Today's visits */}
        <div className="grid grid-cols-3 gap-6">
          <div className="col-span-2 card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
              <h2 className="section-title">예외 사항</h2>
              <span className="text-xs text-gray-500">미처리 {EXCEPTIONS.filter(e => e.status !== '처리완료').length}건</span>
            </div>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="table-header">구분</th>
                    <th className="table-header">이용자</th>
                    <th className="table-header">요양보호사</th>
                    <th className="table-header">시간</th>
                    <th className="table-header">심각도</th>
                    <th className="table-header">상태</th>
                    <th className="table-header"></th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {EXCEPTIONS.map((exc) => (
                    <tr key={exc.id} className="hover:bg-gray-50 transition-colors">
                      <td className="table-cell font-medium">{exc.type}</td>
                      <td className="table-cell">{exc.client}</td>
                      <td className="table-cell">{exc.caregiver}</td>
                      <td className="table-cell text-gray-500">{exc.time}</td>
                      <td className="table-cell">
                        <span className={SEVERITY_BADGE[exc.severity]}>{SEVERITY_LABEL[exc.severity]}</span>
                      </td>
                      <td className="table-cell">
                        <span className={STATUS_BADGE[exc.status]}>{exc.status}</span>
                      </td>
                      <td className="table-cell">
                        <button className="text-blue-600 hover:text-blue-800 text-xs font-medium">처리</button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          <div className="card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
              <h2 className="section-title">오늘 방문</h2>
              <Link href="/schedules" className="text-xs text-blue-600 hover:text-blue-800 font-medium">전체보기</Link>
            </div>
            <ul className="divide-y divide-gray-100">
              {RECENT_VISITS.map((visit, idx) => (
                <li key={idx} className="px-5 py-3">
                  <div className="flex items-start justify-between gap-2">
                    <div>
                      <p className="text-sm font-medium text-gray-900">{visit.client}</p>
                      <p className="text-xs text-gray-500">{visit.caregiver} · {visit.service}</p>
                      <p className="text-xs text-gray-400 mt-0.5">{visit.time}</p>
                    </div>
                    <span className={`${VISIT_STATUS_BADGE[visit.status]} flex-shrink-0`}>{visit.status}</span>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* Quick links */}
        <div className="grid grid-cols-4 gap-4">
          {[
            { href: '/clients', label: '이용자 등록', desc: '새 이용자 추가', emoji: '👤' },
            { href: '/caregivers/applications', label: '채용 신청 검토', desc: '대기 중 2건', emoji: '📋' },
            { href: '/referrals/new', label: '의뢰 생성', desc: '타 기관 의뢰', emoji: '↗' },
            { href: '/compliance', label: '규정 점검', desc: '다음 점검 D-12', emoji: '✓' },
          ].map((link) => (
            <Link key={link.href} href={link.href} className="card p-4 hover:border-blue-300 hover:shadow-md transition-all group">
              <div className="text-2xl mb-2">{link.emoji}</div>
              <p className="text-sm font-semibold text-gray-900 group-hover:text-blue-700">{link.label}</p>
              <p className="text-xs text-gray-500 mt-0.5">{link.desc}</p>
            </Link>
          ))}
        </div>
      </div>
    </InternalAppShell>
  );
}
