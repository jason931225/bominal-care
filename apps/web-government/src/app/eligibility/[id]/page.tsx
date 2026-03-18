import Link from 'next/link';
import GovernmentAppShell from '@/components/GovernmentAppShell';

interface EligibilityDetailPageProps {
  params: Promise<{ id: string }>;
}

const MOCK_CASE = {
  id: 'EC-2026-001',
  applicantName: '박영희',
  age: 72,
  gender: '여',
  birthDate: '1954-05-12',
  address: '서울 강남구 역삼동 123-45',
  phone: '010-1234-5678',
  guardianName: '박철수 (아들)',
  guardianPhone: '010-9876-5432',
  type: '장기요양 수급 신청',
  grade: '3등급 신청',
  status: '심사중',
  submittedAt: '2026-03-14',
  reviewer: '이담당자',
  medicalConditions: '고혈압, 당뇨, 경도인지장애',
  currentCareLevel: '재가 서비스 미이용',
  notes: '독거 상태이며 일상생활 보조 필요. 자녀가 대리 신청.',
};

const APPROVAL_STEPS = [
  {
    step: 1,
    title: '신청 접수',
    description: '온라인 신청서 접수 완료',
    status: 'completed',
    date: '2026-03-14',
    handler: '시스템 자동',
  },
  {
    step: 2,
    title: '서류 검토',
    description: '진단서, 주민등록등본 등 필수 서류 확인',
    status: 'completed',
    date: '2026-03-15',
    handler: '이담당자',
  },
  {
    step: 3,
    title: '방문 조사',
    description: '조사원 방문하여 장기요양 인정 조사 실시',
    status: 'current',
    date: '2026-03-18 (예정)',
    handler: '김조사원',
  },
  {
    step: 4,
    title: '등급판정위원회',
    description: '조사 결과 기반 등급 판정 심의',
    status: 'pending',
    date: '-',
    handler: '미배정',
  },
  {
    step: 5,
    title: '결과 통보',
    description: '최종 등급 판정 결과 통보',
    status: 'pending',
    date: '-',
    handler: '미배정',
  },
];

const STEP_STYLES: Record<string, { dot: string; line: string; text: string }> = {
  completed: {
    dot: 'bg-green-500 border-green-500',
    line: 'bg-green-500',
    text: 'text-green-700',
  },
  current: {
    dot: 'bg-yellow-500 border-yellow-500 ring-4 ring-yellow-100',
    line: 'bg-gray-200',
    text: 'text-yellow-700',
  },
  pending: {
    dot: 'bg-gray-200 border-gray-300',
    line: 'bg-gray-200',
    text: 'text-gray-400',
  },
};

const STEP_BADGE: Record<string, string> = {
  completed: 'badge-green',
  current: 'badge-yellow',
  pending: 'badge-gray',
};

const STEP_LABEL: Record<string, string> = {
  completed: '완료',
  current: '진행중',
  pending: '대기',
};

export default async function EligibilityDetailPage({ params }: EligibilityDetailPageProps) {
  const { id } = await params;
  // In a real app, fetch case data from API using the id
  const caseData = { ...MOCK_CASE, id };

  return (
    <GovernmentAppShell>
      <div className="space-y-6">
        {/* Breadcrumb */}
        <nav className="flex items-center gap-2 text-sm text-gray-500">
          <Link href="/eligibility" className="hover:text-indigo-600">수급 자격 심사</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">{caseData.id}</span>
        </nav>

        {/* Header */}
        <div className="flex items-center justify-between">
          <div>
            <div className="flex items-center gap-3">
              <h1 className="page-title">{caseData.id}</h1>
              <span className="badge-yellow">{caseData.status}</span>
            </div>
            <p className="text-sm text-gray-500 mt-1">
              {caseData.type} · {caseData.applicantName} ({caseData.age}세)
            </p>
          </div>
          <div className="flex gap-2">
            <button className="btn-secondary">반려</button>
            <button className="btn-primary">승인 처리</button>
          </div>
        </div>

        <div className="grid grid-cols-3 gap-6">
          {/* Case Information */}
          <div className="col-span-2 space-y-6">
            {/* Applicant Details */}
            <div className="card">
              <div className="px-5 py-4 border-b border-slate-100">
                <h2 className="section-title">신청자 정보</h2>
              </div>
              <div className="p-5">
                <dl className="grid grid-cols-2 gap-x-6 gap-y-4 text-sm">
                  <div>
                    <dt className="text-gray-500">성명</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.applicantName}</dd>
                  </div>
                  <div>
                    <dt className="text-gray-500">생년월일 / 나이</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.birthDate} ({caseData.age}세, {caseData.gender})</dd>
                  </div>
                  <div>
                    <dt className="text-gray-500">주소</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.address}</dd>
                  </div>
                  <div>
                    <dt className="text-gray-500">연락처</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.phone}</dd>
                  </div>
                  <div>
                    <dt className="text-gray-500">보호자</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.guardianName}</dd>
                  </div>
                  <div>
                    <dt className="text-gray-500">보호자 연락처</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.guardianPhone}</dd>
                  </div>
                  <div className="col-span-2">
                    <dt className="text-gray-500">기저질환</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.medicalConditions}</dd>
                  </div>
                  <div className="col-span-2">
                    <dt className="text-gray-500">현재 케어 상태</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.currentCareLevel}</dd>
                  </div>
                  <div className="col-span-2">
                    <dt className="text-gray-500">비고</dt>
                    <dd className="font-medium text-gray-900 mt-0.5">{caseData.notes}</dd>
                  </div>
                </dl>
              </div>
            </div>

            {/* Approval Timeline */}
            <div className="card">
              <div className="px-5 py-4 border-b border-slate-100">
                <h2 className="section-title">심사 진행 단계</h2>
              </div>
              <div className="p-5">
                <div className="space-y-0">
                  {APPROVAL_STEPS.map((step, idx) => {
                    const styles = STEP_STYLES[step.status];
                    const isLast = idx === APPROVAL_STEPS.length - 1;

                    return (
                      <div key={step.step} className="flex gap-4">
                        {/* Timeline dot + line */}
                        <div className="flex flex-col items-center">
                          <div className={`w-4 h-4 rounded-full border-2 flex-shrink-0 ${styles.dot}`} />
                          {!isLast && (
                            <div className={`w-0.5 flex-1 min-h-12 ${styles.line}`} />
                          )}
                        </div>

                        {/* Content */}
                        <div className="pb-6 flex-1">
                          <div className="flex items-center gap-2 mb-1">
                            <p className={`text-sm font-semibold ${step.status === 'pending' ? 'text-gray-400' : 'text-gray-900'}`}>
                              {step.title}
                            </p>
                            <span className={STEP_BADGE[step.status]}>
                              {STEP_LABEL[step.status]}
                            </span>
                          </div>
                          <p className={`text-xs ${step.status === 'pending' ? 'text-gray-400' : 'text-gray-500'}`}>
                            {step.description}
                          </p>
                          <div className="flex items-center gap-3 mt-1.5 text-xs text-gray-400">
                            <span>담당: {step.handler}</span>
                            <span>{step.date}</span>
                          </div>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            </div>
          </div>

          {/* Sidebar */}
          <div className="space-y-4">
            {/* Case Summary */}
            <div className="card p-5">
              <h3 className="text-sm font-semibold text-gray-900 mb-3">사건 요약</h3>
              <dl className="space-y-3 text-sm">
                <div className="flex justify-between">
                  <dt className="text-gray-500">사건번호</dt>
                  <dd className="font-medium text-gray-900">{caseData.id}</dd>
                </div>
                <div className="flex justify-between">
                  <dt className="text-gray-500">신청 유형</dt>
                  <dd className="font-medium text-gray-900">{caseData.type}</dd>
                </div>
                <div className="flex justify-between">
                  <dt className="text-gray-500">신청 등급</dt>
                  <dd className="font-medium text-gray-900">{caseData.grade}</dd>
                </div>
                <div className="flex justify-between">
                  <dt className="text-gray-500">접수일</dt>
                  <dd className="font-medium text-gray-900">{caseData.submittedAt}</dd>
                </div>
                <div className="flex justify-between">
                  <dt className="text-gray-500">담당자</dt>
                  <dd className="font-medium text-gray-900">{caseData.reviewer}</dd>
                </div>
              </dl>
            </div>

            {/* Actions */}
            <div className="card p-5">
              <h3 className="text-sm font-semibold text-gray-900 mb-3">심사 조치</h3>
              <div className="space-y-2">
                <button className="w-full py-2 text-sm font-medium text-indigo-600 border border-indigo-200 rounded-lg hover:bg-indigo-50 transition-colors">
                  방문 조사 배정
                </button>
                <button className="w-full py-2 text-sm font-medium text-indigo-600 border border-indigo-200 rounded-lg hover:bg-indigo-50 transition-colors">
                  추가 서류 요청
                </button>
                <button className="w-full py-2 text-sm font-medium text-indigo-600 border border-indigo-200 rounded-lg hover:bg-indigo-50 transition-colors">
                  심사관 변경
                </button>
                <button className="w-full py-2 text-sm font-medium text-gray-600 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">
                  메모 추가
                </button>
              </div>
            </div>

            {/* Attached Documents */}
            <div className="card p-5">
              <h3 className="text-sm font-semibold text-gray-900 mb-3">첨부 서류</h3>
              <ul className="space-y-2">
                {[
                  { name: '장기요양인정 신청서.pdf', size: '245KB' },
                  { name: '의사소견서.pdf', size: '380KB' },
                  { name: '주민등록등본.pdf', size: '120KB' },
                ].map((doc) => (
                  <li key={doc.name} className="flex items-center justify-between p-2 bg-slate-50 rounded-lg">
                    <div className="flex items-center gap-2">
                      <svg className="w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                        <path strokeLinecap="round" strokeLinejoin="round" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
                      </svg>
                      <div>
                        <p className="text-xs font-medium text-gray-900">{doc.name}</p>
                        <p className="text-xs text-gray-400">{doc.size}</p>
                      </div>
                    </div>
                    <button className="text-xs text-indigo-600 hover:text-indigo-800 font-medium">열기</button>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      </div>
    </GovernmentAppShell>
  );
}
