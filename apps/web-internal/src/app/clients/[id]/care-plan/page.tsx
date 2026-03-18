import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const CARE_PLAN = {
  clientName: '박순자',
  clientId: 'C001',
  grade: '2등급',
  createdAt: '2024-01-20',
  updatedAt: '2026-01-10',
  reviewDate: '2026-07-10',
  status: '유효',
  goals: [
    '일상생활 독립성 유지 및 향상',
    '만성질환(고혈압, 당뇨) 관리 지원',
    '사회적 고립 예방 및 정서적 안정 지원',
    '낙상 예방 및 안전한 이동 지원',
  ],
  services: [
    {
      category: '신체활동 지원',
      items: ['세면 및 목욕 도움', '식사 준비 및 배식 도움', '이동 및 체위변경 지원', '외출 동행'],
      frequency: '주 5회',
      duration: '3시간/회',
    },
    {
      category: '인지활동 지원',
      items: ['인지자극 활동', '일상적 대화 및 정서 지원'],
      frequency: '주 3회',
      duration: '30분/회',
    },
    {
      category: '가사활동 지원',
      items: ['청소 및 주거 환경 관리', '세탁 및 의류 정리'],
      frequency: '주 2회',
      duration: '1시간/회',
    },
    {
      category: '건강 관리',
      items: ['복약 확인', '혈압 및 혈당 측정 기록', '건강 상태 모니터링'],
      frequency: '매 방문',
      duration: '15분/회',
    },
  ],
  healthNotes: '고혈압 약 (아침 복용), 당뇨 약 (식후 복용). 저혈당 증상 관찰 필요. 혈압 160 이상 시 즉시 보고.',
  restrictions: '격렬한 운동 금지. 계단 이동 시 반드시 지원. 혼자 외출 불가.',
  emergency: '박민수 (아들) 010-9876-5432 → 강남세브란스병원 응급실 02-1234-5678',
};

export default async function CarePlanPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  return (
    <InternalAppShell>
      <div className="space-y-6">
        {/* Breadcrumb */}
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <Link href="/clients" className="hover:text-blue-600">이용자 관리</Link>
          <span>/</span>
          <Link href={`/clients/${id}`} className="hover:text-blue-600">{CARE_PLAN.clientName}</Link>
          <span>/</span>
          <span className="text-gray-900">케어 플랜</span>
        </div>

        {/* Header */}
        <div className="flex items-start justify-between">
          <div>
            <h1 className="page-title">{CARE_PLAN.clientName} 케어 플랜</h1>
            <div className="flex items-center gap-3 mt-2">
              <span className="badge-green">{CARE_PLAN.status}</span>
              <span className="text-sm text-gray-500">최종 수정: {CARE_PLAN.updatedAt}</span>
              <span className="text-sm text-gray-500">다음 검토: {CARE_PLAN.reviewDate}</span>
            </div>
          </div>
          <Link href={`/clients/${id}/care-plan/edit`} className="btn-primary">
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
            케어 플랜 수정
          </Link>
        </div>

        {/* Goals */}
        <div className="card p-5">
          <h2 className="section-title mb-4">케어 목표</h2>
          <ul className="space-y-2">
            {CARE_PLAN.goals.map((goal, idx) => (
              <li key={idx} className="flex items-center gap-3">
                <span className="w-6 h-6 rounded-full bg-blue-100 text-blue-700 text-xs font-bold flex items-center justify-center flex-shrink-0">
                  {idx + 1}
                </span>
                <span className="text-sm text-gray-800">{goal}</span>
              </li>
            ))}
          </ul>
        </div>

        {/* Services */}
        <div className="card p-5">
          <h2 className="section-title mb-4">서비스 계획</h2>
          <div className="grid grid-cols-2 gap-4">
            {CARE_PLAN.services.map((svc, idx) => (
              <div key={idx} className="bg-gray-50 rounded-xl p-4">
                <div className="flex items-center justify-between mb-3">
                  <h3 className="font-semibold text-gray-900">{svc.category}</h3>
                  <div className="flex items-center gap-2">
                    <span className="badge-blue">{svc.frequency}</span>
                    <span className="text-xs text-gray-500">{svc.duration}</span>
                  </div>
                </div>
                <ul className="space-y-1">
                  {svc.items.map((item, i) => (
                    <li key={i} className="flex items-center gap-2 text-sm text-gray-700">
                      <svg className="w-3.5 h-3.5 text-blue-500 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                      </svg>
                      {item}
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        </div>

        {/* Health notes + Restrictions + Emergency */}
        <div className="grid grid-cols-3 gap-4">
          <div className="card p-5">
            <h2 className="section-title mb-3">건강 특이사항</h2>
            <p className="text-sm text-gray-700 leading-relaxed">{CARE_PLAN.healthNotes}</p>
          </div>
          <div className="card p-5">
            <h2 className="section-title mb-3">제한 사항</h2>
            <p className="text-sm text-gray-700 leading-relaxed">{CARE_PLAN.restrictions}</p>
          </div>
          <div className="card p-5 border-l-4 border-l-red-500">
            <h2 className="section-title text-red-700 mb-3">응급 연락처</h2>
            <p className="text-sm text-gray-700 leading-relaxed">{CARE_PLAN.emergency}</p>
          </div>
        </div>

        {/* Meta */}
        <div className="card p-4 bg-gray-50">
          <div className="flex items-center gap-6 text-sm text-gray-500">
            <span>최초 작성: {CARE_PLAN.createdAt}</span>
            <span>최종 수정: {CARE_PLAN.updatedAt}</span>
            <span>다음 검토 예정: {CARE_PLAN.reviewDate}</span>
            <span className="badge-green ml-auto">{CARE_PLAN.status}</span>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
