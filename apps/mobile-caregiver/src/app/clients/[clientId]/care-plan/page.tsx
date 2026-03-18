import CaregiverAppShell from '@/components/CaregiverAppShell';

const CARE_PLAN = {
  clientName: '이순자',
  clientId: 'c001',
  lastUpdated: '2026-03-01',
  updatedBy: '김요양 (담당)',
  goals: [
    '낙상 예방 및 안전한 일상생활 유지',
    '고혈압 관리 및 복약 순응도 향상',
    '경증 치매 진행 속도 완화',
    '사회적 고립감 해소 및 삶의 질 향상',
  ],
  sections: [
    {
      id: 'physical',
      title: '신체 케어',
      icon: '🏃',
      items: [
        { task: '입욕 지원 (주 3회, 화·목·토)', frequency: '주 3회', duration: '30–45분', notes: '미끄럼 방지 매트 사용. 욕조 이동 시 반드시 부축.' },
        { task: '구강 위생 관리', frequency: '매 방문 시', duration: '10분', notes: '전동 칫솔 사용. 의치는 소독 후 보관.' },
        { task: '피부 상태 확인 및 로션 도포', frequency: '매 방문 시', duration: '15분', notes: '욕창 방지를 위해 발뒤꿈치, 등 확인.' },
        { task: '체위 변경 (침상 시)', frequency: '2시간마다', duration: '5분', notes: '방문 중 2회 이상 수행.' },
      ],
    },
    {
      id: 'nutrition',
      title: '영양·식사',
      icon: '🍱',
      items: [
        { task: '점심 식사 준비 및 보조', frequency: '매 방문 시', duration: '45분', notes: '저염식 원칙. 딱딱한 음식 자제. 소량씩 천천히 섭취 유도.' },
        { task: '수분 섭취 확인', frequency: '매 방문 시', duration: '상시', notes: '방문 중 최소 500ml 이상 음용 확인.' },
        { task: '체중 측정', frequency: '주 1회 (수요일)', duration: '5분', notes: '이전 측정 대비 2kg 이상 변화 시 가족 연락.' },
      ],
    },
    {
      id: 'medication',
      title: '투약 관리',
      icon: '💊',
      items: [
        { task: '암로디핀 5mg 복용 확인', frequency: '매 방문 시 (10시)', duration: '5분', notes: '반드시 방문 시작 30분 내 확인. 복용 여부 기록 필수.' },
        { task: '아리셉트 저녁 복용 준비', frequency: '저녁 방문 시', duration: '5분', notes: '21시에 복용. 낮 방문 시 저녁 약 미리 준비.' },
        { task: '칼슘+비타민D 점심 복용', frequency: '매 방문 시 (13시)', duration: '5분', notes: '식후 복용 원칙.' },
      ],
    },
    {
      id: 'cognitive',
      title: '인지·정서 활동',
      icon: '🧩',
      items: [
        { task: '인지활동 프로그램 (퍼즐, 단어 맞추기)', frequency: '주 2회 (화·목)', duration: '20–30분', notes: '과거 사진 활용 대화도 효과적. 성취감 충분히 표현해 줄 것.' },
        { task: '말벗 및 정서 지원', frequency: '매 방문 시', duration: '15분 이상', notes: '최근 관심사: 손주, 뉴스, 노래. 가급적 긍정적 주제로 대화.' },
      ],
    },
    {
      id: 'safety',
      title: '안전·환경',
      icon: '🛡️',
      items: [
        { task: '낙상 위험 환경 점검', frequency: '매 방문 시', duration: '10분', notes: '이동 경로에 장애물 없는지 확인. 야간 조명 작동 확인.' },
        { task: '가스·전기 안전 확인', frequency: '방문 종료 시', duration: '5분', notes: '가스 밸브, 콘센트 상태 확인 후 퇴실.' },
      ],
    },
  ],
};

export default async function CarePlanPage({ params }: { params: Promise<{ clientId: string }> }) {
  const { clientId: _clientId } = await params;

  return (
    <CaregiverAppShell
      activeTab="clients"
      title={`${CARE_PLAN.clientName} 케어플랜`}
      showBackButton
      backHref={`/clients/${CARE_PLAN.clientId}`}
    >
      <div className="px-4 py-4 space-y-4">
        {/* Meta */}
        <div className="card bg-blue-50 border-blue-200">
          <div className="flex items-start justify-between">
            <div>
              <p className="text-xs text-blue-500 font-medium">케어플랜</p>
              <p className="text-base font-bold text-blue-800 mt-0.5">{CARE_PLAN.clientName} 어르신</p>
            </div>
            <div className="text-right">
              <p className="text-xs text-blue-500">최종 업데이트</p>
              <p className="text-xs text-blue-700 font-medium">{CARE_PLAN.lastUpdated}</p>
              <p className="text-xs text-blue-500">{CARE_PLAN.updatedBy}</p>
            </div>
          </div>
        </div>

        {/* Goals */}
        <div className="card">
          <h3 className="section-title">케어 목표</h3>
          <div className="space-y-2">
            {CARE_PLAN.goals.map((goal, idx) => (
              <div key={idx} className="flex items-start gap-2.5">
                <div className="w-5 h-5 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0 mt-0.5">
                  <span className="text-xs font-bold text-blue-600">{idx + 1}</span>
                </div>
                <p className="text-sm text-slate-700 leading-relaxed">{goal}</p>
              </div>
            ))}
          </div>
        </div>

        {/* Sections */}
        {CARE_PLAN.sections.map((section) => (
          <div key={section.id} className="card">
            <h3 className="flex items-center gap-2 text-base font-bold text-slate-800 mb-4">
              <span>{section.icon}</span>
              {section.title}
            </h3>
            <div className="space-y-4">
              {section.items.map((item, idx) => (
                <div key={idx} className="pb-4 border-b border-slate-100 last:border-0 last:pb-0">
                  <div className="flex items-start justify-between gap-3 mb-2">
                    <p className="text-sm font-semibold text-slate-800">{item.task}</p>
                  </div>
                  <div className="flex gap-3 flex-wrap mb-2">
                    <div className="flex items-center gap-1">
                      <svg className="w-3.5 h-3.5 text-slate-400" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                      </svg>
                      <span className="text-xs text-slate-500">{item.frequency}</span>
                    </div>
                    <div className="flex items-center gap-1">
                      <svg className="w-3.5 h-3.5 text-slate-400" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                      </svg>
                      <span className="text-xs text-slate-500">{item.duration}</span>
                    </div>
                  </div>
                  {item.notes && (
                    <p className="text-xs text-slate-500 bg-slate-50 rounded-lg p-2 leading-relaxed">
                      📌 {item.notes}
                    </p>
                  )}
                </div>
              ))}
            </div>
          </div>
        ))}

        <div className="pb-4" />
      </div>
    </CaregiverAppShell>
  );
}
