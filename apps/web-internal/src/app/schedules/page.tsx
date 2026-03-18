import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const WEEK_DAYS = ['월 (3/11)', '화 (3/12)', '수 (3/13)', '목 (3/14)', '금 (3/15)', '토 (3/16)', '일 (3/17)'];

const SCHEDULE_DATA = [
  {
    caregiver: '이민정',
    slots: [
      { day: 0, time: '09:00-12:00', client: '박순자', service: '방문요양', status: '완료' },
      { day: 0, time: '14:00-16:00', client: '이정자', service: '방문요양', status: '완료' },
      { day: 2, time: '09:00-11:00', client: '박순자', service: '방문목욕', status: '완료' },
      { day: 3, time: '09:00-12:00', client: '박순자', service: '방문요양', status: '완료' },
      { day: 4, time: '09:00-12:00', client: '박순자', service: '방문요양', status: '진행중' },
      { day: 4, time: '14:00-16:00', client: '이정자', service: '방문요양', status: '예정' },
    ],
  },
  {
    caregiver: '최수진',
    slots: [
      { day: 0, time: '10:00-13:00', client: '김복동', service: '방문요양', status: '완료' },
      { day: 1, time: '10:00-12:00', client: '김영희', service: '방문목욕', status: '완료' },
      { day: 3, time: '10:00-13:00', client: '김복동', service: '방문요양', status: '완료' },
      { day: 4, time: '10:00-13:00', client: '김복동', service: '방문요양', status: '예정' },
    ],
  },
  {
    caregiver: '정미영',
    slots: [
      { day: 0, time: '13:00-16:00', client: '이철수', service: '방문요양', status: '완료' },
      { day: 2, time: '09:00-12:00', client: '이정자', service: '방문요양', status: '완료' },
      { day: 4, time: '13:00-16:00', client: '이철수', service: '방문요양', status: '진행중' },
    ],
  },
  {
    caregiver: '한지영',
    slots: [
      { day: 1, time: '09:00-12:00', client: '강명순', service: '방문요양', status: '완료' },
      { day: 3, time: '09:00-12:00', client: '강명순', service: '방문요양', status: '완료' },
      { day: 4, time: '09:00-12:00', client: '강명순', service: '방문요양', status: '예정' },
    ],
  },
];

const STATUS_COLORS: Record<string, string> = {
  '완료': 'bg-green-100 text-green-800 border-green-200',
  '진행중': 'bg-blue-100 text-blue-800 border-blue-200',
  '예정': 'bg-gray-100 text-gray-700 border-gray-200',
  '취소': 'bg-red-100 text-red-800 border-red-200',
};

const UPCOMING_VISITS = [
  { time: '09:00', client: '박순자', caregiver: '이민정', service: '방문요양', address: '역삼동' },
  { time: '10:00', client: '김복동', caregiver: '최수진', service: '방문요양', address: '삼성동' },
  { time: '13:00', client: '이철수', caregiver: '정미영', service: '방문요양', address: '청담동' },
  { time: '14:00', client: '강명순', caregiver: '한지영', service: '방문요양', address: '수서동' },
  { time: '14:00', client: '이정자', caregiver: '이민정', service: '방문요양', address: '도곡동' },
  { time: '15:30', client: '홍길자', caregiver: '오혜진', service: '방문목욕', address: '대치동' },
];

export default function SchedulesPage() {
  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">일정 관리</h1>
            <p className="text-sm text-gray-500 mt-1">2026년 3월 11일 - 17일</p>
          </div>
          <div className="flex gap-2">
            <Link href="/schedules/conflicts" className="btn-secondary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
              충돌 일정 보기
            </Link>
            <button className="btn-primary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
              </svg>
              일정 추가
            </button>
          </div>
        </div>

        {/* Stats row */}
        <div className="grid grid-cols-4 gap-4">
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">이번 주 총 방문</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">142</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">오늘 방문</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">24</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">완료율</p>
            <p className="text-3xl font-bold text-green-600 mt-1">94%</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">미이행</p>
            <p className="text-3xl font-bold text-red-600 mt-1">3</p>
          </div>
        </div>

        <div className="grid grid-cols-3 gap-6">
          {/* Calendar view */}
          <div className="col-span-2 card overflow-hidden">
            <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
              <h2 className="section-title">주간 일정표</h2>
              <div className="flex items-center gap-2">
                <button className="p-1.5 hover:bg-gray-100 rounded">
                  <svg className="w-4 h-4 text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
                  </svg>
                </button>
                <span className="text-sm font-medium text-gray-700">3월 2주</span>
                <button className="p-1.5 hover:bg-gray-100 rounded">
                  <svg className="w-4 h-4 text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                  </svg>
                </button>
              </div>
            </div>
            <div className="overflow-x-auto">
              <table className="w-full text-xs">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-3 py-2 text-left text-xs font-semibold text-gray-500 w-24">요양보호사</th>
                    {WEEK_DAYS.map((day) => (
                      <th key={day} className="px-2 py-2 text-center text-xs font-semibold text-gray-500">{day}</th>
                    ))}
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {SCHEDULE_DATA.map((row) => (
                    <tr key={row.caregiver} className="hover:bg-gray-50">
                      <td className="px-3 py-2 font-medium text-gray-900 text-xs">{row.caregiver}</td>
                      {WEEK_DAYS.map((_, dayIdx) => {
                        const daySlots = row.slots.filter(s => s.day === dayIdx);
                        return (
                          <td key={dayIdx} className="px-1 py-1 align-top">
                            {daySlots.map((slot, si) => (
                              <div
                                key={si}
                                className={`mb-1 px-1.5 py-1 rounded border text-xs leading-tight ${STATUS_COLORS[slot.status]}`}
                              >
                                <div className="font-semibold">{slot.time}</div>
                                <div>{slot.client}</div>
                              </div>
                            ))}
                          </td>
                        );
                      })}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Today's list */}
          <div className="card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
              <h2 className="section-title">오늘 방문 목록</h2>
              <span className="text-xs text-gray-500">3월 15일</span>
            </div>
            <ul className="divide-y divide-gray-100">
              {UPCOMING_VISITS.map((visit, idx) => (
                <li key={idx} className="px-5 py-3">
                  <div className="flex items-start gap-3">
                    <div className="text-xs font-semibold text-blue-600 w-12 flex-shrink-0 mt-0.5">{visit.time}</div>
                    <div className="flex-1">
                      <p className="text-sm font-medium text-gray-900">{visit.client}</p>
                      <p className="text-xs text-gray-500">{visit.caregiver} · {visit.service}</p>
                      <p className="text-xs text-gray-400">{visit.address}</p>
                    </div>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
