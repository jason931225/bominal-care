import Link from 'next/link';
import GovernmentAppShell from '@/components/GovernmentAppShell';

const CASES = [
  {
    id: 'EC-2026-001',
    applicantName: '박영희',
    age: 72,
    status: '심사중',
    submittedAt: '2026-03-14',
    reviewer: '이담당자',
    type: '장기요양 수급 신청',
    grade: '3등급 신청',
  },
  {
    id: 'EC-2026-002',
    applicantName: '이병수',
    age: 80,
    status: '접수',
    submittedAt: '2026-03-13',
    reviewer: '미배정',
    type: '수급 등급 변경',
    grade: '2등급 → 1등급',
  },
  {
    id: 'EC-2026-003',
    applicantName: '홍동수',
    age: 74,
    status: '반려',
    submittedAt: '2026-03-10',
    reviewer: '김심사관',
    type: '장기요양 수급 신청',
    grade: '4등급 신청',
  },
  {
    id: 'EC-2026-004',
    applicantName: '김정순',
    age: 68,
    status: '승인',
    submittedAt: '2026-03-08',
    reviewer: '이담당자',
    type: '장기요양 수급 신청',
    grade: '3등급 신청',
  },
  {
    id: 'EC-2026-005',
    applicantName: '최순자',
    age: 76,
    status: '심사중',
    submittedAt: '2026-03-12',
    reviewer: '김심사관',
    type: '수급 등급 변경',
    grade: '3등급 → 2등급',
  },
  {
    id: 'EC-2026-006',
    applicantName: '정만복',
    age: 82,
    status: '접수',
    submittedAt: '2026-03-15',
    reviewer: '미배정',
    type: '장기요양 수급 신청',
    grade: '2등급 신청',
  },
  {
    id: 'EC-2026-007',
    applicantName: '오영자',
    age: 70,
    status: '승인',
    submittedAt: '2026-03-05',
    reviewer: '이담당자',
    type: '장기요양 수급 신청',
    grade: '4등급 신청',
  },
  {
    id: 'EC-2026-008',
    applicantName: '한복례',
    age: 85,
    status: '심사중',
    submittedAt: '2026-03-11',
    reviewer: '이담당자',
    type: '수급 등급 변경',
    grade: '2등급 → 1등급',
  },
];

const STATUS_BADGE: Record<string, string> = {
  '접수': 'badge-gray',
  '심사중': 'badge-yellow',
  '승인': 'badge-green',
  '반려': 'badge-red',
};

export default function EligibilityPage() {
  const total = CASES.length;
  const receivedCount = CASES.filter((c) => c.status === '접수').length;
  const reviewingCount = CASES.filter((c) => c.status === '심사중').length;
  const approvedCount = CASES.filter((c) => c.status === '승인').length;
  const rejectedCount = CASES.filter((c) => c.status === '반려').length;

  return (
    <GovernmentAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">수급 자격 심사</h1>
            <p className="text-sm text-gray-500 mt-1">장기요양 수급 자격 심사 현황 · 총 {total}건</p>
          </div>
          <div className="flex gap-2">
            <button className="btn-secondary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
              </svg>
              내보내기
            </button>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-5 gap-4">
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">전체 건수</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">{total}</p>
          </div>
          <div className="stat-card border-l-4 border-l-gray-400">
            <p className="text-sm font-medium text-gray-500">접수</p>
            <p className="text-3xl font-bold text-gray-600 mt-1">{receivedCount}</p>
          </div>
          <div className="stat-card border-l-4 border-l-yellow-400">
            <p className="text-sm font-medium text-gray-500">심사중</p>
            <p className="text-3xl font-bold text-yellow-600 mt-1">{reviewingCount}</p>
          </div>
          <div className="stat-card border-l-4 border-l-green-500">
            <p className="text-sm font-medium text-gray-500">승인</p>
            <p className="text-3xl font-bold text-green-600 mt-1">{approvedCount}</p>
          </div>
          <div className="stat-card border-l-4 border-l-red-500">
            <p className="text-sm font-medium text-gray-500">반려</p>
            <p className="text-3xl font-bold text-red-600 mt-1">{rejectedCount}</p>
          </div>
        </div>

        {/* Filters */}
        <div className="card p-4">
          <div className="flex items-center gap-3 flex-wrap">
            <div className="flex-1 min-w-64">
              <div className="relative">
                <svg className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
                <input type="text" className="input pl-9" placeholder="신청자명, 사건번호 검색..." />
              </div>
            </div>
            <select className="input w-auto">
              <option>전체 상태</option>
              <option>접수</option>
              <option>심사중</option>
              <option>승인</option>
              <option>반려</option>
            </select>
            <select className="input w-auto">
              <option>전체 유형</option>
              <option>장기요양 수급 신청</option>
              <option>수급 등급 변경</option>
            </select>
            <button className="btn-secondary text-sm">초기화</button>
          </div>
        </div>

        {/* Table */}
        <div className="card overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-slate-50 border-b border-slate-200">
                <tr>
                  <th className="table-header">사건번호</th>
                  <th className="table-header">신청자</th>
                  <th className="table-header">신청 유형</th>
                  <th className="table-header">등급</th>
                  <th className="table-header">접수일</th>
                  <th className="table-header">심사관</th>
                  <th className="table-header">상태</th>
                  <th className="table-header"></th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100">
                {CASES.map((c) => (
                  <tr key={c.id} className="hover:bg-slate-50 transition-colors">
                    <td className="table-cell font-medium text-indigo-600">
                      <Link href={`/eligibility/${c.id}`} className="hover:underline">
                        {c.id}
                      </Link>
                    </td>
                    <td className="table-cell">
                      <div>
                        <p className="font-medium text-gray-900">{c.applicantName}</p>
                        <p className="text-xs text-gray-500">{c.age}세</p>
                      </div>
                    </td>
                    <td className="table-cell">{c.type}</td>
                    <td className="table-cell text-sm">{c.grade}</td>
                    <td className="table-cell text-gray-500 text-sm">{c.submittedAt}</td>
                    <td className="table-cell text-sm">{c.reviewer}</td>
                    <td className="table-cell">
                      <span className={STATUS_BADGE[c.status]}>{c.status}</span>
                    </td>
                    <td className="table-cell">
                      <Link
                        href={`/eligibility/${c.id}`}
                        className="text-indigo-600 hover:text-indigo-800 text-xs font-medium"
                      >
                        상세
                      </Link>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <div className="flex items-center justify-between px-5 py-3 border-t border-slate-100 bg-slate-50">
            <p className="text-sm text-gray-500">총 {total}건 표시 중</p>
            <div className="flex items-center gap-1">
              <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">이전</button>
              <button className="px-3 py-1 text-sm bg-indigo-600 text-white rounded">1</button>
              <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">다음</button>
            </div>
          </div>
        </div>
      </div>
    </GovernmentAppShell>
  );
}
