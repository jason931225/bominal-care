import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const APPLICATIONS = [
  {
    id: 'APP-001',
    name: '김현숙',
    age: 42,
    phone: '010-1122-3344',
    cert: '요양보호사 1급',
    experience: '3년 2개월',
    appliedAt: '2026-03-14',
    status: '검토중',
    availability: '평일 오전',
    source: '온라인 지원',
  },
  {
    id: 'APP-002',
    name: '박정아',
    age: 37,
    phone: '010-2233-4455',
    cert: '요양보호사 1급',
    experience: '1년 6개월',
    appliedAt: '2026-03-13',
    status: '면접예정',
    availability: '전일',
    source: '워크넷',
  },
  {
    id: 'APP-003',
    name: '최미화',
    age: 51,
    phone: '010-3344-5566',
    cert: '요양보호사 2급',
    experience: '8개월',
    appliedAt: '2026-03-12',
    status: '대기',
    availability: '평일 오후',
    source: '직접 방문',
  },
  {
    id: 'APP-004',
    name: '이지영',
    age: 33,
    phone: '010-4455-6677',
    cert: '요양보호사 1급',
    experience: '4년 1개월',
    appliedAt: '2026-03-10',
    status: '최종합격',
    availability: '전일',
    source: '온라인 지원',
  },
  {
    id: 'APP-005',
    name: '강수연',
    age: 46,
    phone: '010-5566-7788',
    cert: '요양보호사 1급',
    experience: '2년 3개월',
    appliedAt: '2026-03-08',
    status: '불합격',
    availability: '주말 포함',
    source: '지인 추천',
  },
];

const STATUS_BADGE: Record<string, string> = {
  '검토중': 'badge-yellow',
  '면접예정': 'badge-blue',
  '대기': 'badge-gray',
  '최종합격': 'badge-green',
  '불합격': 'badge-red',
};

export default function ApplicationsPage() {
  const pendingCount = APPLICATIONS.filter(a => ['검토중', '면접예정', '대기'].includes(a.status)).length;

  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">채용 신청 관리</h1>
            <p className="text-sm text-gray-500 mt-1">전체 {APPLICATIONS.length}건 · 검토 필요 {pendingCount}건</p>
          </div>
          <button className="btn-primary">
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
            </svg>
            채용 공고 등록
          </button>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-5 gap-3">
          {['검토중', '면접예정', '대기', '최종합격', '불합격'].map((status) => (
            <div key={status} className="stat-card text-center">
              <p className="text-2xl font-bold text-gray-900">
                {APPLICATIONS.filter(a => a.status === status).length}
              </p>
              <p className="text-xs text-gray-500 mt-1">{status}</p>
            </div>
          ))}
        </div>

        {/* Filter */}
        <div className="card p-4">
          <div className="flex items-center gap-3">
            <div className="flex-1 min-w-64">
              <div className="relative">
                <svg className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
                <input type="text" className="input pl-9" placeholder="이름, 연락처 검색..." />
              </div>
            </div>
            <select className="input w-auto">
              <option>전체 상태</option>
              <option>검토중</option>
              <option>면접예정</option>
              <option>대기</option>
              <option>최종합격</option>
              <option>불합격</option>
            </select>
            <select className="input w-auto">
              <option>전체 자격증</option>
              <option>1급</option>
              <option>2급</option>
            </select>
          </div>
        </div>

        {/* Table */}
        <div className="card overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50 border-b border-gray-200">
                <tr>
                  <th className="table-header">지원자</th>
                  <th className="table-header">자격증</th>
                  <th className="table-header">경력</th>
                  <th className="table-header">가능 시간</th>
                  <th className="table-header">지원 경로</th>
                  <th className="table-header">지원일</th>
                  <th className="table-header">상태</th>
                  <th className="table-header"></th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {APPLICATIONS.map((app) => (
                  <tr key={app.id} className="hover:bg-gray-50 transition-colors">
                    <td className="table-cell">
                      <div className="flex items-center gap-3">
                        <div className="w-8 h-8 rounded-full bg-purple-100 flex items-center justify-center flex-shrink-0">
                          <span className="text-xs font-semibold text-purple-700">{app.name[0]}</span>
                        </div>
                        <div>
                          <p className="font-medium text-gray-900">{app.name}</p>
                          <p className="text-xs text-gray-500">{app.age}세 · {app.phone}</p>
                        </div>
                      </div>
                    </td>
                    <td className="table-cell">{app.cert}</td>
                    <td className="table-cell">{app.experience}</td>
                    <td className="table-cell">{app.availability}</td>
                    <td className="table-cell text-gray-500">{app.source}</td>
                    <td className="table-cell text-gray-500">{app.appliedAt}</td>
                    <td className="table-cell">
                      <span className={STATUS_BADGE[app.status]}>{app.status}</span>
                    </td>
                    <td className="table-cell">
                      <Link href={`/caregivers/applications/${app.id}`} className="text-blue-600 hover:text-blue-800 text-xs font-medium">
                        검토
                      </Link>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
