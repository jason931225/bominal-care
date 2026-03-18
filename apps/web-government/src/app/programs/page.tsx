import GovernmentAppShell from '@/components/GovernmentAppShell';

const PROGRAMS = [
  {
    id: 'PRG-001',
    name: '강남구 경로당 활성화 프로그램',
    category: '여가·문화',
    operator: '강남구청 노인복지과',
    sites: 12,
    participants: 482,
    budget: '4,800만원',
    period: '2026-01-01 ~ 2026-12-31',
    status: '운영중',
    description: '구내 경로당 연계 여가 문화 프로그램 제공. 노래교실, 공예, 체조 등.',
    target: '65세 이상 구민',
  },
  {
    id: 'PRG-002',
    name: '노인 일자리 및 사회활동 지원',
    category: '일자리',
    operator: '강남구 시니어클럽',
    sites: 8,
    participants: 210,
    budget: '12,500만원',
    period: '2026-01-01 ~ 2026-12-31',
    status: '운영중',
    description: '노인 적합형 일자리 창출 및 사회 참여 활동 지원.',
    target: '만 60세 이상 취업 희망자',
  },
  {
    id: 'PRG-003',
    name: '치매 예방 및 인지 강화 프로그램',
    category: '건강',
    operator: '강남구 치매안심센터',
    sites: 6,
    participants: 143,
    budget: '2,200만원',
    period: '2026-01-01 ~ 2026-12-31',
    status: '운영중',
    description: '경도인지장애 어르신 대상 인지 강화 활동 및 가족 교육.',
    target: '경도인지장애 진단 어르신',
  },
  {
    id: 'PRG-004',
    name: '독거노인 생활 관리사 파견 사업',
    category: '돌봄',
    operator: '강남구청 복지정책과',
    sites: 7,
    participants: 298,
    budget: '8,700만원',
    period: '2026-01-01 ~ 2026-12-31',
    status: '운영중',
    description: '독거 어르신 가정에 생활 관리사를 파견하여 안전 확인 및 서비스 연계.',
    target: '독거노인 (65세 이상)',
  },
  {
    id: 'PRG-005',
    name: '노인 주거 환경 개선 지원',
    category: '주거',
    operator: '강남구청 건축과',
    sites: 5,
    participants: 87,
    budget: '5,000만원',
    period: '2026-02-01 ~ 2026-07-31',
    status: '모집중',
    description: '저소득 어르신 가정의 노후 주거 환경 개선 (화장실, 안전손잡이 등).',
    target: '저소득 독거노인 가구',
  },
  {
    id: 'PRG-006',
    name: '노인 학대 예방 및 피해자 지원',
    category: '안전',
    operator: '강남구 노인보호전문기관',
    sites: 3,
    participants: 45,
    budget: '1,800만원',
    period: '2026-01-01 ~ 2026-12-31',
    status: '운영중',
    description: '노인 학대 예방 교육 및 피해 어르신 상담·지원 서비스.',
    target: '노인 학대 의심/피해자',
  },
  {
    id: 'PRG-007',
    name: '노인 교통 안전 지원 사업',
    category: '안전',
    operator: '강남구청 교통과',
    sites: 4,
    participants: 320,
    budget: '900만원',
    period: '2026-03-01 ~ 2026-11-30',
    status: '신규',
    description: '어르신 교통사고 예방 교육 및 이동 지원 서비스.',
    target: '70세 이상 구민',
  },
  {
    id: 'PRG-008',
    name: '재가 급식 지원 서비스',
    category: '영양',
    operator: '강남구 영양사협회',
    sites: 10,
    participants: 412,
    budget: '6,200만원',
    period: '2026-01-01 ~ 2026-12-31',
    status: '운영중',
    description: '거동이 불편한 어르신 가정에 주 5회 급식 배달 서비스.',
    target: '재가 어르신 (65세 이상)',
  },
];

const CATEGORY_COLORS: Record<string, string> = {
  '여가·문화': 'bg-purple-100 text-purple-800',
  '일자리': 'bg-blue-100 text-blue-800',
  '건강': 'bg-green-100 text-green-800',
  '돌봄': 'bg-orange-100 text-orange-800',
  '주거': 'bg-yellow-100 text-yellow-800',
  '안전': 'bg-red-100 text-red-800',
  '영양': 'bg-teal-100 text-teal-800',
};

const STATUS_BADGE: Record<string, string> = {
  '운영중': 'badge-green',
  '모집중': 'badge-blue',
  '신규': 'badge-yellow',
  '종료': 'badge-gray',
};

export default function ProgramsPage() {
  const categories = [...new Set(PROGRAMS.map(p => p.category))];
  const totalParticipants = PROGRAMS.reduce((sum, p) => sum + p.participants, 0);

  return (
    <GovernmentAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">구 지원 프로그램</h1>
            <p className="text-sm text-gray-500 mt-1">강남구 노인복지 프로그램 · 총 {PROGRAMS.length}개</p>
          </div>
          <div className="flex gap-2">
            <button className="btn-secondary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
              </svg>
              내보내기
            </button>
            <button className="btn-primary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
              </svg>
              프로그램 등록
            </button>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-4 gap-4">
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">전체 프로그램</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">{PROGRAMS.length}</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">총 참여자</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">{totalParticipants.toLocaleString()}</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">운영 중</p>
            <p className="text-3xl font-bold text-green-600 mt-1">{PROGRAMS.filter(p => p.status === '운영중').length}</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">신규/모집</p>
            <p className="text-3xl font-bold text-yellow-600 mt-1">{PROGRAMS.filter(p => ['신규', '모집중'].includes(p.status)).length}</p>
          </div>
        </div>

        {/* Category filter chips */}
        <div className="flex items-center gap-2 flex-wrap">
          <button className="px-3 py-1.5 rounded-full text-sm font-medium bg-indigo-600 text-white">전체</button>
          {categories.map(cat => (
            <button key={cat} className={`px-3 py-1.5 rounded-full text-sm font-medium ${CATEGORY_COLORS[cat]} hover:opacity-80 transition-opacity`}>
              {cat}
            </button>
          ))}
        </div>

        {/* Filter bar */}
        <div className="card p-4">
          <div className="flex items-center gap-3">
            <div className="flex-1">
              <div className="relative">
                <svg className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
                <input type="text" className="input pl-9" placeholder="프로그램명, 운영 기관 검색..." />
              </div>
            </div>
            <select className="input w-auto">
              <option>전체 상태</option>
              <option>운영중</option>
              <option>모집중</option>
              <option>신규</option>
              <option>종료</option>
            </select>
          </div>
        </div>

        {/* Program cards */}
        <div className="grid grid-cols-2 gap-4">
          {PROGRAMS.map((prog) => (
            <div key={prog.id} className="card p-5 hover:border-indigo-200 hover:shadow-md transition-all">
              <div className="flex items-start justify-between mb-3">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <span className={`badge text-xs ${CATEGORY_COLORS[prog.category]}`}>{prog.category}</span>
                    <span className={STATUS_BADGE[prog.status]}>{prog.status}</span>
                  </div>
                  <h3 className="font-semibold text-gray-900">{prog.name}</h3>
                  <p className="text-xs text-gray-500 mt-0.5">{prog.operator}</p>
                </div>
              </div>

              <p className="text-sm text-gray-600 mb-4 leading-relaxed">{prog.description}</p>

              <div className="grid grid-cols-3 gap-3 mb-4">
                <div className="bg-slate-50 rounded-lg p-2 text-center">
                  <p className="text-lg font-bold text-gray-900">{prog.sites}</p>
                  <p className="text-xs text-gray-500">운영 장소</p>
                </div>
                <div className="bg-slate-50 rounded-lg p-2 text-center">
                  <p className="text-lg font-bold text-gray-900">{prog.participants}</p>
                  <p className="text-xs text-gray-500">참여자</p>
                </div>
                <div className="bg-slate-50 rounded-lg p-2 text-center">
                  <p className="text-sm font-bold text-gray-900">{prog.budget}</p>
                  <p className="text-xs text-gray-500">예산</p>
                </div>
              </div>

              <div className="flex items-center justify-between text-xs text-gray-500">
                <span>대상: {prog.target}</span>
                <span>{prog.period.split(' ~ ')[0]} ~ {prog.period.split(' ~ ')[1]}</span>
              </div>

              <div className="mt-4 pt-3 border-t border-gray-100 flex gap-2">
                <button className="flex-1 py-1.5 text-xs font-medium text-indigo-600 hover:text-indigo-800 border border-indigo-200 hover:border-indigo-400 rounded-lg transition-colors">
                  상세보기
                </button>
                <button className="flex-1 py-1.5 text-xs font-medium text-gray-600 hover:text-gray-800 border border-gray-200 hover:border-gray-400 rounded-lg transition-colors">
                  수정
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* Pagination */}
        <div className="flex items-center justify-between">
          <p className="text-sm text-gray-500">총 {PROGRAMS.length}개 프로그램</p>
          <div className="flex items-center gap-1">
            <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">이전</button>
            <button className="px-3 py-1 text-sm bg-indigo-600 text-white rounded">1</button>
            <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">다음</button>
          </div>
        </div>
      </div>
    </GovernmentAppShell>
  );
}
