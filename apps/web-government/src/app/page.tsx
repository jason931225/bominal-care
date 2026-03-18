import GovernmentAppShell from '@/components/GovernmentAppShell';

const STATS = [
  { label: '등록 제공 기관', value: '142', sub: '강남구 내 전체', color: 'indigo' },
  { label: '활성 이용자', value: '4,821', sub: '장기요양 수급자', color: 'blue' },
  { label: '운영 프로그램', value: '38', sub: '구 지원 프로그램', color: 'teal' },
  { label: '처리 대기 건', value: '23', sub: '신청 검토 필요', color: 'orange' },
];

const PROVIDER_ALERTS = [
  { org: '행복노인복지센터', issue: '현지조사 불이행', severity: 'high', date: '2026-03-10' },
  { org: '강남노인복지관', issue: '서류 미제출', severity: 'medium', date: '2026-03-12' },
  { org: '역삼돌봄센터', issue: '요양보호사 자격 만료', severity: 'high', date: '2026-03-08' },
  { org: '삼성재가센터', issue: '이용자 불만 접수', severity: 'low', date: '2026-03-14' },
];

const PENDING_CASES = [
  { type: '신규 기관 지정 신청', applicant: '강남케어센터 (주)', appliedAt: '2026-03-13', status: '검토중' },
  { type: '장기요양 수급자 신청', applicant: '박영희 (72세)', appliedAt: '2026-03-14', status: '서류 검토' },
  { type: '수급 등급 변경 신청', applicant: '이병수 (80세)', appliedAt: '2026-03-12', status: '방문 조사 예정' },
  { type: '기관 변경 신청', applicant: '김정순 → 강남노인복지관', appliedAt: '2026-03-15', status: '대기' },
  { type: '이의 신청', applicant: '홍동수 (74세)', appliedAt: '2026-03-11', status: '검토중' },
];

const PROGRAM_SUMMARY = [
  { name: '경로당 프로그램', count: 12, participants: 482, status: '운영중' },
  { name: '노인 일자리 사업', count: 8, participants: 210, status: '운영중' },
  { name: '치매 예방 프로그램', count: 6, participants: 143, status: '운영중' },
  { name: '독거노인 돌봄 서비스', count: 7, participants: 298, status: '운영중' },
  { name: '노인 주거 환경 개선', count: 5, participants: 87, status: '모집중' },
];

const SEVERITY_BADGE: Record<string, string> = {
  high: 'badge-red',
  medium: 'badge-yellow',
  low: 'badge-blue',
};

const STATUS_BADGE: Record<string, string> = {
  '검토중': 'badge-yellow',
  '서류 검토': 'badge-blue',
  '방문 조사 예정': 'badge-blue',
  '대기': 'badge-gray',
};

const STAT_COLORS: Record<string, string> = {
  indigo: 'bg-indigo-50 text-indigo-700',
  blue: 'bg-blue-50 text-blue-700',
  teal: 'bg-teal-50 text-teal-700',
  orange: 'bg-orange-50 text-orange-700',
};

export default function GovernmentDashboardPage() {
  return (
    <GovernmentAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">정부 포털 대시보드</h1>
            <p className="text-sm text-gray-500 mt-1">서울특별시 강남구 · 2026년 3월 15일 기준</p>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-4 gap-4">
          {STATS.map((stat) => (
            <div key={stat.label} className="stat-card">
              <p className="text-sm font-medium text-gray-500">{stat.label}</p>
              <p className="text-3xl font-bold text-gray-900 mt-1">{stat.value}</p>
              <p className={`text-xs mt-2 font-medium px-2 py-0.5 rounded-full inline-block ${STAT_COLORS[stat.color]}`}>
                {stat.sub}
              </p>
            </div>
          ))}
        </div>

        <div className="grid grid-cols-3 gap-6">
          {/* Provider alerts */}
          <div className="col-span-2 card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-slate-100">
              <h2 className="section-title">기관 이상 알림</h2>
              <span className="text-xs text-gray-500">미처리 {PROVIDER_ALERTS.filter(a => a.severity !== 'low').length}건</span>
            </div>
            <div className="divide-y divide-slate-100">
              {PROVIDER_ALERTS.map((alert, idx) => (
                <div key={idx} className="flex items-center gap-4 px-5 py-3 hover:bg-slate-50 transition-colors">
                  <div className={`w-2 h-2 rounded-full flex-shrink-0 ${
                    alert.severity === 'high' ? 'bg-red-500' :
                    alert.severity === 'medium' ? 'bg-yellow-500' :
                    'bg-blue-400'
                  }`} />
                  <div className="flex-1">
                    <p className="text-sm font-medium text-gray-900">{alert.org}</p>
                    <p className="text-xs text-gray-500">{alert.issue}</p>
                  </div>
                  <span className="text-xs text-gray-400">{alert.date}</span>
                  <span className={SEVERITY_BADGE[alert.severity]}>
                    {alert.severity === 'high' ? '높음' : alert.severity === 'medium' ? '보통' : '낮음'}
                  </span>
                  <button className="text-indigo-600 hover:text-indigo-800 text-xs font-medium">처리</button>
                </div>
              ))}
            </div>
          </div>

          {/* Program summary */}
          <div className="card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-slate-100">
              <h2 className="section-title">프로그램 현황</h2>
              <a href="/programs" className="text-xs text-indigo-600 hover:text-indigo-800 font-medium">전체보기</a>
            </div>
            <ul className="divide-y divide-slate-100">
              {PROGRAM_SUMMARY.map((prog, idx) => (
                <li key={idx} className="px-5 py-3">
                  <div className="flex items-center justify-between">
                    <div>
                      <p className="text-sm font-medium text-gray-900">{prog.name}</p>
                      <p className="text-xs text-gray-500">{prog.count}개 · {prog.participants}명</p>
                    </div>
                    <span className={prog.status === '운영중' ? 'badge-green' : 'badge-yellow'}>
                      {prog.status}
                    </span>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* Pending cases */}
        <div className="card">
          <div className="flex items-center justify-between px-5 py-4 border-b border-slate-100">
            <h2 className="section-title">처리 대기 업무</h2>
            <span className="text-xs text-gray-500">총 {PENDING_CASES.length}건</span>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-slate-50">
                <tr>
                  <th className="table-header">업무 유형</th>
                  <th className="table-header">신청자/대상</th>
                  <th className="table-header">신청일</th>
                  <th className="table-header">상태</th>
                  <th className="table-header"></th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100">
                {PENDING_CASES.map((c, idx) => (
                  <tr key={idx} className="hover:bg-slate-50 transition-colors">
                    <td className="table-cell font-medium">{c.type}</td>
                    <td className="table-cell">{c.applicant}</td>
                    <td className="table-cell text-gray-500">{c.appliedAt}</td>
                    <td className="table-cell">
                      <span className={STATUS_BADGE[c.status] || 'badge-gray'}>{c.status}</span>
                    </td>
                    <td className="table-cell">
                      <button className="text-indigo-600 hover:text-indigo-800 text-xs font-medium">처리</button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Quick actions */}
        <div className="grid grid-cols-3 gap-4">
          {[
            { href: '/providers', label: '기관 디렉토리', desc: '기관 검색 및 조회', emoji: '🏢' },
            { href: '/programs', label: '프로그램 관리', desc: '구 지원 프로그램 현황', emoji: '📋' },
            { href: '#', label: '통계 보고서', desc: '월별 현황 보고서', emoji: '📊' },
          ].map((link) => (
            <a key={link.href} href={link.href} className="card p-4 hover:border-indigo-300 hover:shadow-md transition-all group">
              <div className="text-2xl mb-2">{link.emoji}</div>
              <p className="text-sm font-semibold text-gray-900 group-hover:text-indigo-700">{link.label}</p>
              <p className="text-xs text-gray-500 mt-0.5">{link.desc}</p>
            </a>
          ))}
        </div>
      </div>
    </GovernmentAppShell>
  );
}
