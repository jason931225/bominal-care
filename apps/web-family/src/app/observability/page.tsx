import FamilyAppShell from '@/components/FamilyAppShell';

const SIGNALS = [
  {
    id: 1,
    time: '09:15',
    type: '복약',
    typeColor: 'blue',
    icon: '💊',
    title: '아침 복약 완료',
    value: '3/3',
    status: 'normal',
  },
  {
    id: 2,
    time: '09:10',
    type: '활동',
    typeColor: 'green',
    icon: '🚶',
    title: '기상 및 활동 시작',
    value: '정상',
    status: 'normal',
  },
  {
    id: 3,
    time: '08:45',
    type: '혈압',
    typeColor: 'yellow',
    icon: '❤️',
    title: '혈압 측정',
    value: '145/88',
    status: 'warning',
  },
  {
    id: 4,
    time: '08:00',
    type: '식사',
    typeColor: 'green',
    icon: '🍚',
    title: '아침 식사',
    value: '완료',
    status: 'normal',
  },
  {
    id: 5,
    time: '어제 22:00',
    type: '수면',
    typeColor: 'indigo',
    icon: '😴',
    title: '취침',
    value: '정상',
    status: 'normal',
  },
];

const HEALTH_METRICS = [
  {
    id: 'blood-pressure',
    label: '혈압',
    icon: '❤️',
    current: '145/88',
    unit: 'mmHg',
    normal: '120/80 이하',
    trend: 'up',
    trendLabel: '상승',
    status: 'warning',
    history: [120, 122, 125, 130, 138, 140, 145],
  },
  {
    id: 'blood-sugar',
    label: '혈당',
    icon: '🩸',
    current: '118',
    unit: 'mg/dL',
    normal: '100 이하 (공복)',
    trend: 'stable',
    trendLabel: '안정',
    status: 'normal',
    history: [115, 118, 112, 120, 116, 119, 118],
  },
  {
    id: 'weight',
    label: '체중',
    icon: '⚖️',
    current: '52.3',
    unit: 'kg',
    normal: '50~55kg',
    trend: 'stable',
    trendLabel: '안정',
    status: 'normal',
    history: [53, 52.8, 52.5, 52.6, 52.4, 52.3, 52.3],
  },
  {
    id: 'steps',
    label: '일일 보행 수',
    icon: '🚶',
    current: '1,240',
    unit: '걸음',
    normal: '2,000+ 권장',
    trend: 'down',
    trendLabel: '감소',
    status: 'warning',
    history: [2100, 1900, 1800, 1600, 1450, 1300, 1240],
  },
];

const statusColor: Record<string, string> = {
  normal: 'text-green-600',
  warning: 'text-yellow-600',
  critical: 'text-red-600',
};

const statusBg: Record<string, string> = {
  normal: 'bg-green-50 border-green-200',
  warning: 'bg-yellow-50 border-yellow-200',
  critical: 'bg-red-50 border-red-200',
};

const typeColorMap: Record<string, string> = {
  blue: 'bg-blue-50 text-blue-700 border border-blue-200',
  green: 'bg-green-50 text-green-700 border border-green-200',
  yellow: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
  indigo: 'bg-indigo-50 text-indigo-700 border border-indigo-200',
};

export default function ObservabilityPage() {
  const warningCount = HEALTH_METRICS.filter((m) => m.status === 'warning').length;

  return (
    <FamilyAppShell>
      <div className="max-w-4xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">모니터링 대시보드</h1>
          <p className="text-sm text-gray-500 mt-1">
            김복순 어머님의 건강 신호 및 활동 현황 — 실시간 관찰
          </p>
        </div>

        {/* Alert Banner */}
        {warningCount > 0 && (
          <div className="bg-yellow-50 border border-yellow-300 rounded-xl p-4 flex items-center gap-3 mb-5">
            <span className="text-xl">⚠️</span>
            <p className="text-sm font-semibold text-yellow-800">
              주의가 필요한 지표 {warningCount}개 — 혈압, 보행 수가 권고 범위를 벗어났습니다
            </p>
          </div>
        )}

        <div className="grid lg:grid-cols-3 gap-5">
          {/* Health Metrics */}
          <div className="lg:col-span-2">
            <h2 className="text-lg font-bold text-gray-900 mb-4">건강 지표</h2>
            <div className="grid sm:grid-cols-2 gap-3 mb-6">
              {HEALTH_METRICS.map((metric) => (
                <div
                  key={metric.id}
                  className={`bg-white border rounded-xl p-4 ${statusBg[metric.status]}`}
                >
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-2">
                      <span className="text-xl">{metric.icon}</span>
                      <span className="text-sm font-semibold text-gray-800">{metric.label}</span>
                    </div>
                    <span className={`text-xs font-bold ${statusColor[metric.status]}`}>
                      {metric.trendLabel}
                      {metric.trend === 'up' ? ' ↑' : metric.trend === 'down' ? ' ↓' : ' →'}
                    </span>
                  </div>
                  <p className="text-2xl font-bold text-gray-900">
                    {metric.current}
                    <span className="text-sm font-normal text-gray-500 ml-1">{metric.unit}</span>
                  </p>
                  <p className="text-xs text-gray-500 mt-1">정상 범위: {metric.normal}</p>

                  {/* Mini chart */}
                  <div className="flex items-end gap-0.5 h-8 mt-3">
                    {metric.history.map((val, i) => {
                      const max = Math.max(...metric.history);
                      const min = Math.min(...metric.history);
                      const range = max - min || 1;
                      const height = Math.round(((val - min) / range) * 28) + 4;
                      return (
                        <div
                          key={i}
                          className={`flex-1 rounded-sm ${i === metric.history.length - 1 ? 'bg-blue-500' : 'bg-gray-200'}`}
                          style={{ height: `${height}px` }}
                        />
                      );
                    })}
                  </div>
                  <p className="text-xs text-gray-400 mt-1">최근 7일 추이</p>
                </div>
              ))}
            </div>

            {/* Care Coverage */}
            <div className="bg-white border border-gray-200 rounded-xl p-4">
              <h3 className="font-bold text-gray-900 mb-3">이번 주 케어 이행률</h3>
              <div className="space-y-3">
                {[
                  { label: '방문 요양', done: 4, total: 5, color: 'blue' },
                  { label: '복약', done: 6, total: 7, color: 'green' },
                  { label: '물리치료', done: 2, total: 2, color: 'purple' },
                ].map((item) => (
                  <div key={item.label}>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-gray-600">{item.label}</span>
                      <span className="font-medium text-gray-800">{item.done}/{item.total}회</span>
                    </div>
                    <div className="w-full bg-gray-100 rounded-full h-2">
                      <div
                        className={`h-2 rounded-full ${
                          item.color === 'blue' ? 'bg-blue-500' :
                          item.color === 'green' ? 'bg-green-500' : 'bg-purple-500'
                        }`}
                        style={{ width: `${(item.done / item.total) * 100}%` }}
                      />
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Signal Feed */}
          <div>
            <h2 className="text-lg font-bold text-gray-900 mb-4">오늘 활동 피드</h2>
            <div className="bg-white border border-gray-200 rounded-xl divide-y divide-gray-100">
              {SIGNALS.map((signal) => (
                <div key={signal.id} className="flex items-center gap-3 p-3">
                  <span className="text-xl flex-shrink-0">{signal.icon}</span>
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-gray-800 truncate">{signal.title}</p>
                    <div className="flex items-center gap-1.5 mt-0.5">
                      <span className={`text-xs px-1.5 py-0.5 rounded-full ${typeColorMap[signal.typeColor]}`}>
                        {signal.type}
                      </span>
                      <span className="text-xs text-gray-400">{signal.time}</span>
                    </div>
                  </div>
                  <span className={`flex-shrink-0 text-sm font-bold ${statusColor[signal.status]}`}>
                    {signal.value}
                  </span>
                </div>
              ))}
            </div>

            {/* Emergency Contact */}
            <div className="mt-4 bg-red-50 border border-red-200 rounded-xl p-4">
              <p className="font-semibold text-red-800 text-sm mb-2">긴급 연락</p>
              <div className="space-y-2">
                {[
                  { role: '담당 케어매니저', name: '최지원', phone: '010-1234-5678' },
                  { role: '담당 요양보호사', name: '박미영', phone: '010-9876-5432' },
                ].map((contact) => (
                  <div key={contact.role} className="flex items-center justify-between">
                    <div>
                      <p className="text-xs text-red-600">{contact.role}</p>
                      <p className="text-sm font-medium text-red-900">{contact.name}</p>
                    </div>
                    <a
                      href={`tel:${contact.phone}`}
                      className="px-3 py-1 bg-red-600 text-white text-xs font-semibold rounded-lg hover:bg-red-700 transition-colors"
                    >
                      전화
                    </a>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </FamilyAppShell>
  );
}
