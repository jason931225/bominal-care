// 케어 플랜 상세 — Care Plan Detail
// Displays the full care plan including goals, visit schedule, and service details

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const CARE_PLAN = {
  id: 'plan-2025-001',
  name: '재가 돌봄 서비스 플랜 A',
  level: '장기요양 3등급',
  certExpiry: '2027년 6월 30일',
  benefitNumber: 'LTC-2024-789012',
  startDate: '2025년 11월 1일',
  endDate: '2026년 10월 31일',
  monthlyBudget: 1_456_700,
  goals: [
    '일상생활 수행 능력 유지 및 향상',
    '낙상 예방 및 안전한 생활환경 조성',
    '사회적 고립 예방 및 정서적 지원',
    '만성질환(고혈압, 당뇨) 안정적 관리',
  ],
  weeklySchedule: [
    { day: '월요일', time: '오후 2:00 ~ 4:00', type: '방문 요양', worker: '김복순' },
    { day: '수요일', time: '오전 10:00 ~ 11:00', type: '방문 목욕', worker: '박미경' },
    { day: '수요일', time: '오후 2:00 ~ 4:00', type: '방문 요양', worker: '김복순' },
    { day: '금요일', time: '오후 2:00 ~ 4:00', type: '방문 요양', worker: '김복순' },
    { day: '격주 화요일', time: '오전 11:00 ~ 12:00', type: '방문 간호', worker: '최간호사' },
  ],
  visitContents: [
    '신체 기능 지원 — 세면, 구강 위생, 옷 갈아입기',
    '가사 지원 — 식사 준비, 청소, 세탁',
    '건강 관리 — 혈압 측정, 혈당 체크, 약 복용 확인',
    '정서 지원 — 말벗, 여가활동, 산책',
  ],
  copayment: { rate: '15%', monthly: 218_505 },
};

const DAY_COLORS: Record<string, string> = {
  '월요일': 'bg-primary-100 text-primary-700',
  '수요일': 'bg-secondary-100 text-secondary-700',
  '금요일': 'bg-warning-50 text-warning-700',
  '격주 화요일': 'bg-info-50 text-info-700',
};

function formatKRW(amount: number) {
  return amount.toLocaleString('ko-KR') + '원';
}

export default function CarePlanPage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Back */}
        <Link
          href="/care"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          케어 서비스로
        </Link>

        <h1 className="text-senior-2xl font-bold text-gray-900 mb-5">케어 플랜 상세</h1>

        {/* Plan overview */}
        <div className="senior-card mb-4">
          <div className="flex items-start justify-between mb-4">
            <div>
              <p className="text-senior-sm text-gray-500 mb-0.5">장기요양 등급</p>
              <p className="text-senior-xl font-bold text-secondary-700">{CARE_PLAN.level}</p>
            </div>
            <span className="bg-success-50 text-success-700 text-senior-sm font-bold px-3 py-1 rounded-full">
              이용 중
            </span>
          </div>

          <div className="space-y-2 text-senior-base">
            <div className="flex justify-between">
              <span className="text-gray-500">플랜명</span>
              <span className="font-semibold text-gray-800">{CARE_PLAN.name}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">이용 기간</span>
              <span className="font-semibold text-gray-800">{CARE_PLAN.startDate} ~</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">만료일</span>
              <span className="font-semibold text-gray-800">{CARE_PLAN.endDate}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">급여 번호</span>
              <span className="font-semibold text-gray-800 text-senior-sm">{CARE_PLAN.benefitNumber}</span>
            </div>
          </div>
        </div>

        {/* Budget and copayment */}
        <section className="senior-card mb-4" aria-labelledby="budget-heading">
          <h2 id="budget-heading" className="text-senior-lg font-bold text-gray-800 mb-3">월 급여 및 본인부담금</h2>
          <div className="grid grid-cols-2 gap-3">
            <div className="bg-secondary-50 rounded-xl p-3 text-center">
              <p className="text-senior-sm text-gray-500 mb-1">월 한도액</p>
              <p className="text-senior-lg font-bold text-secondary-700">{formatKRW(CARE_PLAN.monthlyBudget)}</p>
            </div>
            <div className="bg-warning-50 rounded-xl p-3 text-center">
              <p className="text-senior-sm text-gray-500 mb-1">본인부담 (15%)</p>
              <p className="text-senior-lg font-bold text-warning-700">{formatKRW(CARE_PLAN.copayment.monthly)}</p>
            </div>
          </div>
        </section>

        {/* Care goals */}
        <section className="senior-card mb-4" aria-labelledby="goals-heading">
          <h2 id="goals-heading" className="text-senior-lg font-bold text-gray-800 mb-3">케어 목표</h2>
          <ul className="space-y-2">
            {CARE_PLAN.goals.map((goal, i) => (
              <li key={i} className="flex items-start gap-2">
                <span className="text-secondary-500 mt-0.5 flex-shrink-0" aria-hidden="true">✓</span>
                <span className="text-senior-base text-gray-700">{goal}</span>
              </li>
            ))}
          </ul>
        </section>

        {/* Weekly schedule */}
        <section className="senior-card mb-4" aria-labelledby="schedule-heading">
          <h2 id="schedule-heading" className="text-senior-lg font-bold text-gray-800 mb-3">주간 방문 일정</h2>
          <div className="space-y-3">
            {CARE_PLAN.weeklySchedule.map((entry, i) => {
              const dayColor = DAY_COLORS[entry.day] ?? 'bg-gray-100 text-gray-700';
              return (
                <div key={i} className="flex items-center gap-3">
                  <span className={`${dayColor} text-senior-sm font-bold px-2.5 py-1 rounded-lg min-w-[80px] text-center flex-shrink-0`}>
                    {entry.day}
                  </span>
                  <div className="flex-1">
                    <p className="text-senior-base font-semibold text-gray-800">{entry.time}</p>
                    <p className="text-senior-sm text-gray-500">{entry.type} — {entry.worker}</p>
                  </div>
                </div>
              );
            })}
          </div>
        </section>

        {/* Visit contents */}
        <section className="senior-card mb-5" aria-labelledby="contents-heading">
          <h2 id="contents-heading" className="text-senior-lg font-bold text-gray-800 mb-3">서비스 내용</h2>
          <ul className="space-y-2">
            {CARE_PLAN.visitContents.map((item, i) => (
              <li key={i} className="flex items-start gap-2 text-senior-base text-gray-700">
                <span className="text-secondary-400 mt-0.5 flex-shrink-0" aria-hidden="true">•</span>
                {item}
              </li>
            ))}
          </ul>
        </section>

        {/* Contact care manager */}
        <Link href="/care" className="senior-btn-secondary w-full">
          케어매니저에게 연락하기
        </Link>
      </div>
    </SeniorAppShell>
  );
}
