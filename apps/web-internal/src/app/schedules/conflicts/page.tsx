import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const CONFLICTS = [
  {
    id: 'CON-001',
    type: '시간 겹침',
    caregiver: '최수진',
    date: '2026-03-17',
    slot1: { time: '09:00-12:00', client: '김복동', service: '방문요양' },
    slot2: { time: '10:00-12:00', client: '김영희', service: '방문목욕' },
    severity: 'high',
    status: '미처리',
  },
  {
    id: 'CON-002',
    type: '요양보호사 공백',
    caregiver: '-',
    date: '2026-03-18',
    slot1: { time: '13:00-16:00', client: '이정자', service: '방문요양' },
    slot2: { time: '-', client: '-', service: '-' },
    severity: 'high',
    status: '검토중',
  },
  {
    id: 'CON-003',
    type: '이동 불가',
    caregiver: '이민정',
    date: '2026-03-17',
    slot1: { time: '11:00-14:00', client: '박순자 (역삼동)', service: '방문요양' },
    slot2: { time: '11:30-13:30', client: '홍길자 (수서동)', service: '방문요양' },
    severity: 'medium',
    status: '미처리',
  },
  {
    id: 'CON-004',
    type: '초과 배정',
    caregiver: '정미영',
    date: '2026-03-19',
    slot1: { time: '09:00-18:00', client: '5명 배정', service: '방문요양' },
    slot2: { time: '-', client: '법정 최대 초과', service: '-' },
    severity: 'medium',
    status: '대기',
  },
];

const GAPS = [
  { client: '강명순', date: '2026-03-18', time: '09:00-12:00', service: '방문요양', reason: '담당 요양보호사 휴가' },
  { client: '윤영희', date: '2026-03-19', time: '10:00-12:00', service: '방문목욕', reason: '대체 인력 미배정' },
  { client: '조길동', date: '2026-03-20', time: '14:00-17:00', service: '방문요양', reason: '요양보호사 신규 배정 대기' },
];

const SEVERITY_BADGE: Record<string, string> = { high: 'badge-red', medium: 'badge-yellow', low: 'badge-blue' };
const SEVERITY_LABEL: Record<string, string> = { high: '높음', medium: '보통', low: '낮음' };
const STATUS_BADGE: Record<string, string> = { '미처리': 'badge-red', '검토중': 'badge-yellow', '대기': 'badge-blue', '처리완료': 'badge-green' };

export default function ScheduleConflictsPage() {
  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">일정 충돌 및 공백</h1>
            <p className="text-sm text-gray-500 mt-1">미처리 충돌 {CONFLICTS.filter(c => c.status !== '처리완료').length}건 · 서비스 공백 {GAPS.length}건</p>
          </div>
          <Link href="/schedules" className="btn-secondary">
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
            </svg>
            일정 관리로
          </Link>
        </div>

        {/* Summary */}
        <div className="grid grid-cols-4 gap-4">
          <div className="stat-card border-l-4 border-l-red-500">
            <p className="text-sm font-medium text-gray-500">시간 겹침</p>
            <p className="text-3xl font-bold text-red-600 mt-1">{CONFLICTS.filter(c => c.type === '시간 겹침').length}</p>
          </div>
          <div className="stat-card border-l-4 border-l-orange-400">
            <p className="text-sm font-medium text-gray-500">요양보호사 공백</p>
            <p className="text-3xl font-bold text-orange-600 mt-1">{CONFLICTS.filter(c => c.type === '요양보호사 공백').length}</p>
          </div>
          <div className="stat-card border-l-4 border-l-yellow-400">
            <p className="text-sm font-medium text-gray-500">이동 불가</p>
            <p className="text-3xl font-bold text-yellow-600 mt-1">{CONFLICTS.filter(c => c.type === '이동 불가').length}</p>
          </div>
          <div className="stat-card border-l-4 border-l-purple-400">
            <p className="text-sm font-medium text-gray-500">서비스 공백</p>
            <p className="text-3xl font-bold text-purple-600 mt-1">{GAPS.length}</p>
          </div>
        </div>

        {/* Conflicts table */}
        <div className="card">
          <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
            <h2 className="section-title">충돌 일정 목록</h2>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50">
                <tr>
                  <th className="table-header">유형</th>
                  <th className="table-header">요양보호사</th>
                  <th className="table-header">날짜</th>
                  <th className="table-header">일정 1</th>
                  <th className="table-header">일정 2</th>
                  <th className="table-header">심각도</th>
                  <th className="table-header">상태</th>
                  <th className="table-header"></th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {CONFLICTS.map((c) => (
                  <tr key={c.id} className="hover:bg-gray-50">
                    <td className="table-cell font-medium">{c.type}</td>
                    <td className="table-cell">{c.caregiver}</td>
                    <td className="table-cell">{c.date}</td>
                    <td className="table-cell">
                      <div className="text-xs">
                        <p className="font-medium">{c.slot1.time}</p>
                        <p className="text-gray-500">{c.slot1.client} · {c.slot1.service}</p>
                      </div>
                    </td>
                    <td className="table-cell">
                      <div className="text-xs">
                        <p className="font-medium">{c.slot2.time}</p>
                        <p className="text-gray-500">{c.slot2.client} · {c.slot2.service}</p>
                      </div>
                    </td>
                    <td className="table-cell">
                      <span className={SEVERITY_BADGE[c.severity]}>{SEVERITY_LABEL[c.severity]}</span>
                    </td>
                    <td className="table-cell">
                      <span className={STATUS_BADGE[c.status]}>{c.status}</span>
                    </td>
                    <td className="table-cell">
                      <button className="text-blue-600 hover:text-blue-800 text-xs font-medium">해결</button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Gaps table */}
        <div className="card">
          <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
            <h2 className="section-title">서비스 공백</h2>
            <span className="text-xs text-gray-500">요양보호사 미배정 일정</span>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50">
                <tr>
                  <th className="table-header">이용자</th>
                  <th className="table-header">날짜</th>
                  <th className="table-header">시간</th>
                  <th className="table-header">서비스</th>
                  <th className="table-header">사유</th>
                  <th className="table-header"></th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {GAPS.map((gap, idx) => (
                  <tr key={idx} className="hover:bg-gray-50">
                    <td className="table-cell font-medium">{gap.client}</td>
                    <td className="table-cell">{gap.date}</td>
                    <td className="table-cell">{gap.time}</td>
                    <td className="table-cell">{gap.service}</td>
                    <td className="table-cell text-gray-500">{gap.reason}</td>
                    <td className="table-cell">
                      <button className="text-blue-600 hover:text-blue-800 text-xs font-medium">배정</button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
