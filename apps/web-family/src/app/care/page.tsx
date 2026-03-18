import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const CARE_PLAN = {
  seniorName: '김복순',
  grade: '3등급',
  planPeriod: '2026.01.01 ~ 2026.03.31',
  caseManager: '케어매니저 최지원',
  caseManagerPhone: '010-1234-5678',
  reviewDate: '2026-03-20',
  goals: [
    '낙상 예방 및 안전한 보행 능력 유지',
    '혈압 및 혈당 정상 범위 유지',
    '인지 기능 유지 및 사회적 고립 방지',
    '일상생활 자립 능력 최대한 유지',
  ],
};

const SERVICES = [
  {
    id: 'svc-001',
    category: '재가급여',
    name: '방문 요양',
    icon: '🏥',
    provider: '행복케어 복지센터 · 박미영 요양보호사',
    schedule: '주 5회 (월~금) 08:30~11:00',
    duration: '2시간 30분/회',
    monthlyHours: '50시간',
    status: 'active',
    statusLabel: '진행 중',
    tasks: ['세면 및 위생 관리', '식사 준비 및 보조', '이동 보조', '투약 확인', '말벗·정서 지원'],
    progress: 80,
  },
  {
    id: 'svc-002',
    category: '재가급여',
    name: '방문 물리치료',
    icon: '🦵',
    provider: '하이케어 의원 · 김도현 물리치료사',
    schedule: '주 2회 (화/목) 14:00~15:00',
    duration: '60분/회',
    monthlyHours: '8시간',
    status: 'active',
    statusLabel: '진행 중',
    tasks: ['하지 근력 강화 운동', '보행 훈련', '균형 감각 훈련', '통증 관리'],
    progress: 60,
  },
  {
    id: 'svc-003',
    category: '재가급여',
    name: '인지 재활 프로그램',
    icon: '🧠',
    provider: '함께케어 센터',
    schedule: '주 3회 (월/수/금) 오후 2시',
    duration: '90분/회',
    monthlyHours: '18시간',
    status: 'pending',
    statusLabel: '승인 대기',
    tasks: ['인지 자극 활동', '기억력 훈련', '사회 참여 프로그램'],
    progress: 0,
  },
];

const statusConfig: Record<string, string> = {
  active: 'bg-green-50 text-green-700 border border-green-200',
  pending: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
  ended: 'bg-gray-100 text-gray-600 border border-gray-200',
};

export default function CarePlanPage() {
  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">케어 플랜</h1>
          <p className="text-sm text-gray-500 mt-1">김복순 어머님의 장기요양 서비스 계획</p>
        </div>

        {/* Plan Overview */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-5">
          <div className="flex items-start justify-between gap-3 mb-4">
            <div>
              <h2 className="font-bold text-gray-900">{CARE_PLAN.seniorName} 어머님 케어 플랜</h2>
              <p className="text-sm text-gray-500 mt-0.5">
                노인장기요양 {CARE_PLAN.grade} · {CARE_PLAN.planPeriod}
              </p>
            </div>
            <span className="flex-shrink-0 text-xs font-semibold px-2 py-1 bg-blue-50 text-blue-700 border border-blue-200 rounded-full">
              유효
            </span>
          </div>

          <div className="flex items-center justify-between py-3 border-t border-gray-100">
            <div>
              <p className="text-xs text-gray-400">담당 케어매니저</p>
              <p className="font-medium text-gray-800 text-sm">{CARE_PLAN.caseManager}</p>
            </div>
            <a
              href={`tel:${CARE_PLAN.caseManagerPhone}`}
              className="px-3 py-1.5 bg-blue-50 text-blue-700 border border-blue-200 rounded-lg text-sm font-medium hover:bg-blue-100 transition-colors"
            >
              📞 연락
            </a>
          </div>

          <div className="pt-3 border-t border-gray-100">
            <p className="text-xs text-amber-600 flex items-center gap-1">
              <span>⏰</span>
              <span>다음 케어 계획 검토일: {CARE_PLAN.reviewDate} (5일 후)</span>
            </p>
          </div>
        </div>

        {/* Goals */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-5">
          <h2 className="font-bold text-gray-900 mb-3">케어 목표</h2>
          <ul className="space-y-2">
            {CARE_PLAN.goals.map((goal) => (
              <li key={goal} className="flex items-start gap-2 text-sm">
                <span className="text-blue-500 flex-shrink-0 mt-0.5">✦</span>
                <span className="text-gray-700">{goal}</span>
              </li>
            ))}
          </ul>
        </div>

        {/* Services */}
        <h2 className="text-lg font-bold text-gray-900 mb-4">제공 서비스</h2>
        <div className="space-y-4">
          {SERVICES.map((svc) => (
            <div key={svc.id} className="bg-white border border-gray-200 rounded-xl p-5">
              <div className="flex items-start justify-between gap-3 mb-3">
                <div className="flex items-center gap-3">
                  <span className="text-3xl">{svc.icon}</span>
                  <div>
                    <h3 className="font-bold text-gray-900">{svc.name}</h3>
                    <p className="text-xs text-gray-500 mt-0.5">{svc.provider}</p>
                  </div>
                </div>
                <span className={`flex-shrink-0 text-xs font-semibold px-2 py-1 rounded-full ${statusConfig[svc.status]}`}>
                  {svc.statusLabel}
                </span>
              </div>

              <div className="grid sm:grid-cols-3 gap-2 mb-3">
                <div className="bg-gray-50 rounded-lg p-2.5">
                  <p className="text-xs text-gray-400">스케줄</p>
                  <p className="text-xs font-medium text-gray-800 mt-0.5">{svc.schedule}</p>
                </div>
                <div className="bg-gray-50 rounded-lg p-2.5">
                  <p className="text-xs text-gray-400">1회 시간</p>
                  <p className="text-xs font-medium text-gray-800 mt-0.5">{svc.duration}</p>
                </div>
                <div className="bg-gray-50 rounded-lg p-2.5">
                  <p className="text-xs text-gray-400">월 제공 시간</p>
                  <p className="text-xs font-medium text-gray-800 mt-0.5">{svc.monthlyHours}</p>
                </div>
              </div>

              <div className="mb-3">
                <p className="text-xs font-medium text-gray-600 mb-1.5">서비스 내용</p>
                <div className="flex flex-wrap gap-1.5">
                  {svc.tasks.map((task) => (
                    <span key={task} className="text-xs bg-blue-50 text-blue-700 px-2 py-0.5 rounded-full border border-blue-200">
                      {task}
                    </span>
                  ))}
                </div>
              </div>

              {svc.status === 'active' && (
                <div>
                  <div className="flex justify-between text-xs mb-1">
                    <span className="text-gray-500">이번 달 이행률</span>
                    <span className="font-medium text-gray-800">{svc.progress}%</span>
                  </div>
                  <div className="w-full bg-gray-100 rounded-full h-1.5">
                    <div
                      className="bg-blue-500 h-1.5 rounded-full"
                      style={{ width: `${svc.progress}%` }}
                    />
                  </div>
                </div>
              )}

              {svc.status === 'pending' && (
                <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-2.5">
                  <p className="text-xs text-yellow-700 font-medium">
                    승인 대기 중 — 승인 후 서비스 시작 예정
                  </p>
                  <Link href="/approvals" className="text-xs text-yellow-700 underline mt-0.5 inline-block">
                    승인 처리하기 →
                  </Link>
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </FamilyAppShell>
  );
}
