import Link from 'next/link';
import CaregiverAppShell from '@/components/CaregiverAppShell';

const WEEK_DAYS = [
  { day: '일', date: 10, active: false },
  { day: '월', date: 11, active: false },
  { day: '화', date: 12, active: false },
  { day: '수', date: 13, active: false },
  { day: '목', date: 14, active: false },
  { day: '금', date: 15, active: true },  // today
  { day: '토', date: 16, active: false },
];

const VISITS = [
  {
    id: 'v001',
    clientName: '이순자',
    clientAge: 82,
    date: 15,
    startTime: '10:00',
    endTime: '13:00',
    address: '강남구 대치동',
    services: ['목욕 지원', '식사 지원'],
    status: 'upcoming',
    color: 'blue',
  },
  {
    id: 'v002',
    clientName: '박영철',
    clientAge: 76,
    date: 15,
    startTime: '14:30',
    endTime: '17:30',
    address: '강남구 역삼동',
    services: ['가사 지원', '투약 보조'],
    status: 'upcoming',
    color: 'indigo',
  },
  {
    id: 'v003',
    clientName: '최말순',
    clientAge: 88,
    date: 16,
    startTime: '09:00',
    endTime: '12:00',
    address: '서초구 반포동',
    services: ['목욕 지원', '이동 지원'],
    status: 'upcoming',
    color: 'purple',
  },
  {
    id: 'v004',
    clientName: '이순자',
    clientAge: 82,
    date: 17,
    startTime: '10:00',
    endTime: '13:00',
    address: '강남구 대치동',
    services: ['목욕 지원', '식사 지원'],
    status: 'upcoming',
    color: 'blue',
  },
];

const colorMap: Record<string, string> = {
  blue: 'bg-blue-50 border-blue-200',
  indigo: 'bg-indigo-50 border-indigo-200',
  purple: 'bg-purple-50 border-purple-200',
};

const dotMap: Record<string, string> = {
  blue: 'bg-blue-500',
  indigo: 'bg-indigo-500',
  purple: 'bg-purple-500',
};

const textMap: Record<string, string> = {
  blue: 'text-blue-700',
  indigo: 'text-indigo-700',
  purple: 'text-purple-700',
};

export default function SchedulePage() {
  const selectedDate = 15;
  const todayVisits = VISITS.filter((v) => v.date === selectedDate);

  return (
    <CaregiverAppShell activeTab="schedule" title="일정">
      <div className="space-y-0">
        {/* Week Strip */}
        <div className="bg-white px-4 pt-4 pb-3 border-b border-slate-100">
          <div className="flex items-center justify-between mb-3">
            <h2 className="text-base font-semibold text-slate-800">2026년 3월</h2>
            <div className="flex gap-2">
              <button type="button" className="p-1.5 rounded-lg active:bg-slate-100">
                <svg className="w-4 h-4 text-slate-500" fill="none" stroke="currentColor" strokeWidth={2.5} viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
                </svg>
              </button>
              <button type="button" className="p-1.5 rounded-lg active:bg-slate-100">
                <svg className="w-4 h-4 text-slate-500" fill="none" stroke="currentColor" strokeWidth={2.5} viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                </svg>
              </button>
            </div>
          </div>
          <div className="grid grid-cols-7 gap-1">
            {WEEK_DAYS.map((d) => {
              const hasVisit = VISITS.some((v) => v.date === d.date);
              const isSelected = d.date === selectedDate;
              return (
                <button key={d.date} type="button" className="flex flex-col items-center gap-1 py-2 rounded-xl active:bg-slate-50">
                  <span className={`text-xs font-medium ${d.day === '일' ? 'text-red-500' : d.day === '토' ? 'text-blue-500' : 'text-slate-500'}`}>
                    {d.day}
                  </span>
                  <div className={`w-8 h-8 rounded-full flex items-center justify-center ${
                    isSelected ? 'bg-blue-600' : ''
                  }`}>
                    <span className={`text-sm font-semibold ${isSelected ? 'text-white' : 'text-slate-800'}`}>
                      {d.date}
                    </span>
                  </div>
                  <div className={`w-1.5 h-1.5 rounded-full ${hasVisit ? 'bg-blue-400' : 'bg-transparent'}`} />
                </button>
              );
            })}
          </div>
        </div>

        {/* Visit List */}
        <div className="px-4 pt-4 pb-4">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-sm font-semibold text-slate-700">
              3월 {selectedDate}일 일정 ({todayVisits.length}건)
            </h3>
            <span className="text-xs text-slate-400">{todayVisits.reduce((acc, v) => {
              const [sh, sm] = v.startTime.split(':').map(Number);
              const [eh, em] = v.endTime.split(':').map(Number);
              return acc + (eh * 60 + em - (sh * 60 + sm));
            }, 0) / 60}시간 근무</span>
          </div>

          {todayVisits.length === 0 ? (
            <div className="text-center py-12 text-slate-400">
              <div className="text-4xl mb-3">📅</div>
              <p className="text-sm">이 날은 일정이 없습니다.</p>
            </div>
          ) : (
            <div className="space-y-3">
              {todayVisits.map((visit) => (
                <Link key={visit.id} href={`/schedule/${visit.id}`}>
                  <div className={`card border ${colorMap[visit.color] ?? 'bg-white border-slate-200'} active:scale-98 transition-transform`}>
                    <div className="flex items-start gap-3">
                      <div className="flex flex-col items-center gap-1 flex-shrink-0">
                        <div className={`w-2 h-2 rounded-full mt-1.5 ${dotMap[visit.color] ?? 'bg-slate-400'}`} />
                      </div>
                      <div className="flex-1">
                        <div className="flex items-start justify-between">
                          <div>
                            <p className={`text-base font-bold ${textMap[visit.color] ?? 'text-slate-800'}`}>
                              {visit.clientName} 어르신
                            </p>
                            <p className="text-xs text-slate-500 mt-0.5">{visit.clientAge}세 · {visit.address}</p>
                          </div>
                          <div className="text-right">
                            <p className="text-sm font-semibold text-slate-800">{visit.startTime}</p>
                            <p className="text-xs text-slate-400">{visit.endTime}</p>
                          </div>
                        </div>
                        <div className="flex flex-wrap gap-1.5 mt-2">
                          {visit.services.map((s) => (
                            <span key={s} className="text-xs bg-white/80 text-slate-600 border border-slate-200 px-2 py-0.5 rounded-full">
                              {s}
                            </span>
                          ))}
                        </div>
                      </div>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          )}
        </div>

        {/* Upcoming */}
        <div className="px-4 pb-4">
          <h3 className="text-sm font-semibold text-slate-700 mb-3">이번 주 예정</h3>
          <div className="space-y-2">
            {VISITS.filter((v) => v.date > selectedDate).map((visit) => (
              <Link key={`upcoming-${visit.id}-${visit.date}`} href={`/schedule/${visit.id}`}>
                <div className="card flex items-center gap-3 active:scale-98 transition-transform">
                  <div className="flex-shrink-0 text-center w-12">
                    <p className="text-xs text-slate-400">3월</p>
                    <p className="text-base font-bold text-slate-700">{visit.date}</p>
                  </div>
                  <div className="w-px h-10 bg-slate-200" />
                  <div className="flex-1">
                    <p className="text-sm font-medium text-slate-800">{visit.clientName} · {visit.startTime}</p>
                    <p className="text-xs text-slate-400">{visit.services.join(', ')}</p>
                  </div>
                </div>
              </Link>
            ))}
          </div>
        </div>
      </div>
    </CaregiverAppShell>
  );
}
