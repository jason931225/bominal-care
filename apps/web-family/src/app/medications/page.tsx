import FamilyAppShell from '@/components/FamilyAppShell';

const MEDICATIONS = [
  {
    id: 'med-001',
    name: '암로디핀 5mg',
    category: '혈압약',
    categoryColor: 'red',
    icon: '💊',
    prescribedBy: '최병철 내과 원장',
    hospital: '강남성모병원',
    frequency: '1일 1회',
    timing: '아침 식후',
    dosage: '1정',
    startDate: '2025-06-01',
    endDate: '지속',
    purpose: '고혈압 치료',
    sideEffects: '두통, 부종 (드물게)',
    todayStatus: 'taken',
    todayTime: '오전 9:15',
    adherenceRate: 96,
  },
  {
    id: 'med-002',
    name: '메트포르민 500mg',
    category: '당뇨약',
    categoryColor: 'blue',
    icon: '💊',
    prescribedBy: '최병철 내과 원장',
    hospital: '강남성모병원',
    frequency: '1일 2회',
    timing: '아침·저녁 식후',
    dosage: '1정',
    startDate: '2024-03-15',
    endDate: '지속',
    purpose: '제2형 당뇨 조절',
    sideEffects: '소화불량 (식후 복용으로 감소)',
    todayStatus: 'partial',
    todayTime: '아침 완료 / 저녁 대기',
    adherenceRate: 91,
  },
  {
    id: 'med-003',
    name: '아스피린 100mg',
    category: '혈전예방',
    categoryColor: 'orange',
    icon: '💊',
    prescribedBy: '최병철 내과 원장',
    hospital: '강남성모병원',
    frequency: '1일 1회',
    timing: '아침 식후',
    dosage: '1정',
    startDate: '2025-11-01',
    endDate: '지속',
    purpose: '심뇌혈관 질환 예방',
    sideEffects: '위장 장애 (장용정 사용)',
    todayStatus: 'taken',
    todayTime: '오전 9:15',
    adherenceRate: 98,
  },
  {
    id: 'med-004',
    name: '칼슘+비타민D',
    category: '영양제',
    categoryColor: 'yellow',
    icon: '🌟',
    prescribedBy: '김은정 정형외과',
    hospital: '연세정형외과의원',
    frequency: '1일 1회',
    timing: '점심 식후',
    dosage: '1정',
    startDate: '2026-01-10',
    endDate: '2026-07-10',
    purpose: '골다공증 예방',
    sideEffects: '변비 (수분 섭취 권장)',
    todayStatus: 'missed',
    todayTime: '미복용',
    adherenceRate: 78,
  },
];

const statusConfig: Record<string, { label: string; color: string; icon: string }> = {
  taken: { label: '복용 완료', color: 'bg-green-50 text-green-700 border border-green-200', icon: '✓' },
  partial: { label: '일부 완료', color: 'bg-yellow-50 text-yellow-700 border border-yellow-200', icon: '◐' },
  missed: { label: '미복용', color: 'bg-red-50 text-red-700 border border-red-200', icon: '✗' },
  pending: { label: '복용 대기', color: 'bg-gray-100 text-gray-600 border border-gray-200', icon: '○' },
};

const categoryColor: Record<string, string> = {
  red: 'bg-red-50 text-red-700 border border-red-200',
  blue: 'bg-blue-50 text-blue-700 border border-blue-200',
  orange: 'bg-orange-50 text-orange-700 border border-orange-200',
  yellow: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
};

export default function MedicationsPage() {
  const taken = MEDICATIONS.filter((m) => m.todayStatus === 'taken').length;
  const missed = MEDICATIONS.filter((m) => m.todayStatus === 'missed').length;

  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">복약 현황</h1>
          <p className="text-sm text-gray-500 mt-1">
            김복순 어머님의 처방 약물 및 복약 이행 현황 (읽기 전용)
          </p>
        </div>

        {/* Today Summary */}
        <div className="grid grid-cols-3 gap-3 mb-6">
          <div className="bg-green-50 border border-green-200 rounded-xl p-4 text-center">
            <p className="text-2xl font-bold text-green-700">{taken}</p>
            <p className="text-xs text-green-600 mt-0.5">오늘 복용 완료</p>
          </div>
          <div className="bg-red-50 border border-red-200 rounded-xl p-4 text-center">
            <p className="text-2xl font-bold text-red-700">{missed}</p>
            <p className="text-xs text-red-600 mt-0.5">미복용</p>
          </div>
          <div className="bg-blue-50 border border-blue-200 rounded-xl p-4 text-center">
            <p className="text-2xl font-bold text-blue-700">{MEDICATIONS.length}종</p>
            <p className="text-xs text-blue-600 mt-0.5">총 처방 약물</p>
          </div>
        </div>

        {/* Missed Alert */}
        {missed > 0 && (
          <div className="bg-red-50 border border-red-200 rounded-xl p-4 flex items-start gap-3 mb-5">
            <span className="text-xl flex-shrink-0">⚠️</span>
            <div>
              <p className="font-semibold text-red-800">미복용 약물이 있습니다</p>
              <p className="text-sm text-red-600 mt-0.5">
                칼슘+비타민D 점심 복용이 누락되었습니다. 담당 요양보호사에게 확인해 주세요.
              </p>
            </div>
          </div>
        )}

        {/* Medication List */}
        <div className="space-y-4">
          {MEDICATIONS.map((med) => {
            const status = statusConfig[med.todayStatus];
            return (
              <div key={med.id} className="bg-white border border-gray-200 rounded-xl p-5">
                <div className="flex items-start justify-between gap-3 mb-3">
                  <div className="flex items-center gap-3">
                    <span className="text-3xl">{med.icon}</span>
                    <div>
                      <h3 className="font-bold text-gray-900">{med.name}</h3>
                      <div className="flex items-center gap-1.5 mt-0.5">
                        <span className={`text-xs font-medium px-2 py-0.5 rounded-full ${categoryColor[med.categoryColor]}`}>
                          {med.category}
                        </span>
                      </div>
                    </div>
                  </div>
                  <div className="text-right flex-shrink-0">
                    <span className={`text-xs font-semibold px-2 py-1 rounded-full ${status.color}`}>
                      {status.icon} {status.label}
                    </span>
                    <p className="text-xs text-gray-400 mt-1">{med.todayTime}</p>
                  </div>
                </div>

                <div className="grid grid-cols-2 sm:grid-cols-3 gap-2 text-sm mb-3">
                  <div className="bg-gray-50 rounded-lg p-2">
                    <p className="text-xs text-gray-400">복용 빈도</p>
                    <p className="font-medium text-gray-800">{med.frequency}</p>
                  </div>
                  <div className="bg-gray-50 rounded-lg p-2">
                    <p className="text-xs text-gray-400">복용 시기</p>
                    <p className="font-medium text-gray-800">{med.timing}</p>
                  </div>
                  <div className="bg-gray-50 rounded-lg p-2">
                    <p className="text-xs text-gray-400">1회 용량</p>
                    <p className="font-medium text-gray-800">{med.dosage}</p>
                  </div>
                </div>

                <div className="text-xs text-gray-500 space-y-1 mb-3">
                  <p>처방: {med.prescribedBy} · {med.hospital}</p>
                  <p>목적: {med.purpose}</p>
                  <p className="text-amber-600">주의: {med.sideEffects}</p>
                </div>

                {/* Adherence Bar */}
                <div>
                  <div className="flex justify-between text-xs mb-1">
                    <span className="text-gray-500">복약 이행률 (30일)</span>
                    <span className={`font-bold ${med.adherenceRate >= 90 ? 'text-green-600' : med.adherenceRate >= 75 ? 'text-yellow-600' : 'text-red-600'}`}>
                      {med.adherenceRate}%
                    </span>
                  </div>
                  <div className="w-full bg-gray-100 rounded-full h-1.5">
                    <div
                      className={`h-1.5 rounded-full ${
                        med.adherenceRate >= 90 ? 'bg-green-500' :
                        med.adherenceRate >= 75 ? 'bg-yellow-500' : 'bg-red-500'
                      }`}
                      style={{ width: `${med.adherenceRate}%` }}
                    />
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </FamilyAppShell>
  );
}
