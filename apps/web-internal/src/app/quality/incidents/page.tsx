import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const INCIDENTS = [
  { id: 'INC-001', date: '2026-03-12', client: '이정자', caregiver: '정미영', type: '낙상', description: '욕실에서 미끄러짐, 좌측 무릎 타박상', severity: 'high', status: '처리중', reporter: '정미영' },
  { id: 'INC-002', date: '2026-03-08', client: '박순자', caregiver: '이민정', type: '건강 이상', description: '혈압 165/100, 어지러움 호소. 가족 연락 후 병원 이송', severity: 'medium', status: '처리완료', reporter: '이민정' },
  { id: 'INC-003', date: '2026-02-28', client: '김복동', caregiver: '최수진', type: '서비스 불만', description: '방문 30분 지연에 대한 이용자 불만 접수', severity: 'low', status: '처리완료', reporter: '최수진' },
  { id: 'INC-004', date: '2026-02-20', client: '강명순', caregiver: '한지영', type: '분실물', description: '지갑 분실 의심. 조사 결과 자택 내 발견', severity: 'medium', status: '처리완료', reporter: '한지영' },
  { id: 'INC-005', date: '2026-02-10', client: '윤영희', caregiver: '김은지', type: '낙상', description: '침대에서 내려오다 넘어짐. 이상 없음 확인', severity: 'medium', status: '처리완료', reporter: '김은지' },
  { id: 'INC-006', date: '2026-01-25', client: '홍길자', caregiver: '오혜진', type: '응급 이송', description: '호흡 곤란 증상으로 119 신고 및 병원 이송', severity: 'high', status: '처리완료', reporter: '오혜진' },
];

const SEVERITY_BADGE: Record<string, string> = { high: 'badge-red', medium: 'badge-yellow', low: 'badge-blue' };
const SEVERITY_LABEL: Record<string, string> = { high: '높음', medium: '보통', low: '낮음' };
const STATUS_BADGE: Record<string, string> = { '처리중': 'badge-yellow', '처리완료': 'badge-green', '미처리': 'badge-red' };

export default function IncidentsPage() {
  const openCount = INCIDENTS.filter(i => i.status === '처리중' || i.status === '미처리').length;

  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">사고 관리</h1>
            <p className="text-sm text-gray-500 mt-1">전체 {INCIDENTS.length}건 · 처리 중 {openCount}건</p>
          </div>
          <div className="flex gap-2">
            <Link href="/quality" className="btn-secondary">품질 대시보드</Link>
            <button className="btn-primary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
              </svg>
              사고 등록
            </button>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-4 gap-4">
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">이번달 발생</p>
            <p className="text-3xl font-bold text-gray-900 mt-1">2</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">처리 중</p>
            <p className="text-3xl font-bold text-yellow-600 mt-1">{openCount}</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">고위험</p>
            <p className="text-3xl font-bold text-red-600 mt-1">{INCIDENTS.filter(i => i.severity === 'high').length}</p>
          </div>
          <div className="stat-card">
            <p className="text-sm font-medium text-gray-500">이번달 완료</p>
            <p className="text-3xl font-bold text-green-600 mt-1">1</p>
          </div>
        </div>

        {/* Filter */}
        <div className="card p-4">
          <div className="flex items-center gap-3">
            <div className="flex-1 min-w-64">
              <div className="relative">
                <svg className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
                <input type="text" className="input pl-9" placeholder="이용자, 요양보호사 검색..." />
              </div>
            </div>
            <select className="input w-auto">
              <option>전체 유형</option>
              <option>낙상</option>
              <option>건강 이상</option>
              <option>응급 이송</option>
              <option>서비스 불만</option>
              <option>분실물</option>
            </select>
            <select className="input w-auto">
              <option>전체 상태</option>
              <option>처리중</option>
              <option>처리완료</option>
              <option>미처리</option>
            </select>
            <select className="input w-auto">
              <option>전체 심각도</option>
              <option>높음</option>
              <option>보통</option>
              <option>낮음</option>
            </select>
          </div>
        </div>

        {/* Table */}
        <div className="card overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50 border-b border-gray-200">
                <tr>
                  <th className="table-header">사고 번호</th>
                  <th className="table-header">발생일</th>
                  <th className="table-header">이용자</th>
                  <th className="table-header">요양보호사</th>
                  <th className="table-header">유형</th>
                  <th className="table-header">내용</th>
                  <th className="table-header">심각도</th>
                  <th className="table-header">상태</th>
                  <th className="table-header"></th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {INCIDENTS.map((inc) => (
                  <tr key={inc.id} className="hover:bg-gray-50 transition-colors">
                    <td className="table-cell font-mono text-xs text-gray-600">{inc.id}</td>
                    <td className="table-cell">{inc.date}</td>
                    <td className="table-cell font-medium">{inc.client}</td>
                    <td className="table-cell">{inc.caregiver}</td>
                    <td className="table-cell">
                      <span className="badge-gray">{inc.type}</span>
                    </td>
                    <td className="table-cell max-w-xs">
                      <p className="text-xs text-gray-600 truncate">{inc.description}</p>
                    </td>
                    <td className="table-cell">
                      <span className={SEVERITY_BADGE[inc.severity]}>{SEVERITY_LABEL[inc.severity]}</span>
                    </td>
                    <td className="table-cell">
                      <span className={STATUS_BADGE[inc.status]}>{inc.status}</span>
                    </td>
                    <td className="table-cell">
                      <Link href={`/quality/incidents/${inc.id}`} className="text-blue-600 hover:text-blue-800 text-xs font-medium">
                        상세보기
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
