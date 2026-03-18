import FamilyAppShell from '@/components/FamilyAppShell';

const EVENTS = [
  {
    id: 1,
    date: '2026-03-15',
    dateLabel: '오늘',
    time: '09:15',
    type: '복약',
    typeColor: 'blue',
    icon: '💊',
    title: '아침 복약 완료',
    desc: '혈압약(암로디핀 5mg), 당뇨약(메트포르민 500mg), 혈전예방제(아스피린 100mg) 복용 확인',
    actor: '요양보호사 박미영',
    detail: '식사 후 정상 복용, 이상 반응 없음',
  },
  {
    id: 2,
    date: '2026-03-15',
    dateLabel: '오늘',
    time: '08:30',
    type: '방문',
    typeColor: 'green',
    icon: '🏥',
    title: '방문 케어 시작',
    desc: '오전 케어 방문 — 세면 보조, 식사 준비, 기저귀 교체',
    actor: '요양보호사 박미영',
    detail: '2시간 30분 방문 서비스',
  },
  {
    id: 3,
    date: '2026-03-14',
    dateLabel: '어제',
    time: '18:00',
    type: '관찰',
    typeColor: 'yellow',
    icon: '📝',
    title: '혈압 측정',
    desc: '수축기 145 / 이완기 88 mmHg — 정상 범위 초과',
    actor: '요양보호사 이순자',
    detail: '담당 간호사 보고 완료, 주치의 상담 예약',
  },
  {
    id: 4,
    date: '2026-03-14',
    dateLabel: '어제',
    time: '14:00',
    type: '승인요청',
    typeColor: 'purple',
    icon: '✅',
    title: '추가 서비스 승인 요청',
    desc: '물리치료 주 2회 → 주 4회 증가 요청',
    actor: '케어매니저 최지원',
    detail: '낙상 후 재활 목적, 보험공단 사전 승인 필요',
  },
  {
    id: 5,
    date: '2026-03-14',
    dateLabel: '어제',
    time: '09:00',
    type: '복약',
    typeColor: 'blue',
    icon: '💊',
    title: '아침 복약 완료',
    desc: '전 복약 정상 복용',
    actor: '요양보호사 이순자',
    detail: '',
  },
  {
    id: 6,
    date: '2026-03-13',
    dateLabel: '3월 13일',
    time: '10:00',
    type: '방문',
    typeColor: 'green',
    icon: '🏥',
    title: '물리치료',
    desc: '하지 근력 강화 운동, 보행 훈련 30분',
    actor: '물리치료사 김도현',
    detail: '근력 향상 추세, 다음 주 보행 보조기 검토',
  },
  {
    id: 7,
    date: '2026-03-13',
    dateLabel: '3월 13일',
    time: '14:30',
    type: '문서',
    typeColor: 'gray',
    icon: '📄',
    title: '계약 갱신 완료',
    desc: '방문 요양 서비스 계약 3개월 연장',
    actor: '케어매니저 최지원',
    detail: '계약 기간: 2026.04.01 ~ 2026.06.30',
  },
];

const TYPE_FILTERS = ['전체', '방문', '복약', '관찰', '승인요청', '문서'];

const colorBadge: Record<string, string> = {
  blue: 'bg-blue-50 text-blue-700 border border-blue-200',
  green: 'bg-green-50 text-green-700 border border-green-200',
  yellow: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
  purple: 'bg-purple-50 text-purple-700 border border-purple-200',
  gray: 'bg-gray-100 text-gray-600 border border-gray-200',
};

const dotColor: Record<string, string> = {
  blue: 'bg-blue-500',
  green: 'bg-green-500',
  yellow: 'bg-yellow-500',
  purple: 'bg-purple-500',
  gray: 'bg-gray-400',
};

const groupedEvents = EVENTS.reduce<Record<string, typeof EVENTS>>((acc, ev) => {
  if (!acc[ev.dateLabel]) acc[ev.dateLabel] = [];
  acc[ev.dateLabel].push(ev);
  return acc;
}, {});

export default function TimelinePage() {
  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">케어 타임라인</h1>
          <p className="text-sm text-gray-500 mt-1">김복순 어머님의 모든 케어 활동 기록</p>
        </div>

        {/* Filters */}
        <div className="flex gap-2 overflow-x-auto pb-2 mb-6 scrollbar-hide">
          {TYPE_FILTERS.map((f) => (
            <button
              key={f}
              className={`flex-shrink-0 px-3 py-1.5 rounded-full text-sm font-medium border transition-colors ${
                f === '전체'
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-white text-gray-600 border-gray-200 hover:border-blue-300'
              }`}
            >
              {f}
            </button>
          ))}
        </div>

        {/* Timeline Groups */}
        {Object.entries(groupedEvents).map(([dateLabel, events]) => (
          <div key={dateLabel} className="mb-8">
            <div className="flex items-center gap-3 mb-4">
              <h2 className="text-sm font-bold text-gray-500 uppercase tracking-wide">{dateLabel}</h2>
              <div className="flex-1 h-px bg-gray-200" />
            </div>

            <div className="relative">
              {/* Vertical line */}
              <div className="absolute left-5 top-0 bottom-0 w-0.5 bg-gray-200" />

              <div className="space-y-4">
                {events.map((event) => (
                  <div key={event.id} className="relative flex gap-4">
                    {/* Dot */}
                    <div className={`flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center text-lg z-10 ${dotColor[event.typeColor]} bg-opacity-10 border-2 border-white shadow-sm`}
                      style={{ background: 'white' }}>
                      {event.icon}
                    </div>

                    {/* Card */}
                    <div className="flex-1 bg-white border border-gray-200 rounded-xl p-4 shadow-sm">
                      <div className="flex items-start justify-between gap-2 mb-2">
                        <div className="flex items-center gap-2 flex-wrap">
                          <span className={`text-xs font-semibold px-2 py-0.5 rounded-full ${colorBadge[event.typeColor]}`}>
                            {event.type}
                          </span>
                          <span className="text-xs text-gray-400">{event.time}</span>
                        </div>
                      </div>
                      <p className="font-semibold text-gray-900">{event.title}</p>
                      <p className="text-sm text-gray-600 mt-1">{event.desc}</p>
                      {event.detail && (
                        <p className="text-xs text-gray-400 mt-1.5 italic">{event.detail}</p>
                      )}
                      <div className="flex items-center gap-1 mt-2">
                        <span className="text-xs text-gray-400">담당:</span>
                        <span className="text-xs font-medium text-gray-600">{event.actor}</span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        ))}

        {/* Load More */}
        <div className="text-center mt-4">
          <button className="px-6 py-2.5 border border-gray-300 text-gray-600 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors">
            이전 기록 더 보기
          </button>
        </div>
      </div>
    </FamilyAppShell>
  );
}
