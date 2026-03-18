import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const CLIENTS = [
  { id: 'C001', name: '박순자', age: 72, grade: '2등급', address: '서울 강남구 역삼동', caregiver: '이민정', service: '방문요양', status: '활성', lastVisit: '2026-03-15', phone: '010-1234-5678' },
  { id: 'C002', name: '김복동', age: 68, grade: '3등급', address: '서울 강남구 삼성동', caregiver: '최수진', service: '방문목욕', status: '활성', lastVisit: '2026-03-14', phone: '010-2345-6789' },
  { id: 'C003', name: '이정자', age: 81, grade: '1등급', address: '서울 강남구 도곡동', caregiver: '정미영', service: '방문요양', status: '주의', lastVisit: '2026-03-15', phone: '010-3456-7890' },
  { id: 'C004', name: '조길동', age: 75, grade: '4등급', address: '서울 강남구 개포동', caregiver: '-', service: '방문요양', status: '대기', lastVisit: '-', phone: '010-4567-8901' },
  { id: 'C005', name: '강명순', age: 69, grade: '3등급', address: '서울 강남구 수서동', caregiver: '한지영', service: '방문요양', status: '활성', lastVisit: '2026-03-13', phone: '010-5678-9012' },
  { id: 'C006', name: '윤영희', age: 77, grade: '2등급', address: '서울 강남구 일원동', caregiver: '김은지', service: '방문목욕', status: '활성', lastVisit: '2026-03-12', phone: '010-6789-0123' },
  { id: 'C007', name: '홍길자', age: 84, grade: '1등급', address: '서울 강남구 대치동', caregiver: '오혜진', service: '방문요양', status: '활성', lastVisit: '2026-03-15', phone: '010-7890-1234' },
  { id: 'C008', name: '장미숙', age: 71, grade: '3등급', address: '서울 강남구 논현동', caregiver: '박지수', service: '방문목욕', status: '활성', lastVisit: '2026-03-11', phone: '010-8901-2345' },
  { id: 'C009', name: '이철수', age: 73, grade: '2등급', address: '서울 강남구 청담동', caregiver: '정미영', service: '방문요양', status: '활성', lastVisit: '2026-03-15', phone: '010-9012-3456' },
  { id: 'C010', name: '김영희', age: 66, grade: '4등급', address: '서울 강남구 압구정동', caregiver: '최수진', service: '방문요양', status: '활성', lastVisit: '2026-03-14', phone: '010-0123-4567' },
];

const STATUS_BADGE: Record<string, string> = {
  '활성': 'badge-green',
  '주의': 'badge-yellow',
  '대기': 'badge-gray',
  '종결': 'badge-red',
};

const GRADE_COLORS: Record<string, string> = {
  '1등급': 'text-red-700 bg-red-50',
  '2등급': 'text-orange-700 bg-orange-50',
  '3등급': 'text-yellow-700 bg-yellow-50',
  '4등급': 'text-green-700 bg-green-50',
  '5등급': 'text-blue-700 bg-blue-50',
};

export default function ClientsPage() {
  return (
    <InternalAppShell>
      <div className="space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">이용자 관리</h1>
            <p className="text-sm text-gray-500 mt-1">총 {CLIENTS.length}명 등록</p>
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
              이용자 등록
            </button>
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
                <input type="text" className="input pl-9" placeholder="이름, 전화번호 검색..." />
              </div>
            </div>
            <select className="input w-auto">
              <option>전체 상태</option>
              <option>활성</option>
              <option>주의</option>
              <option>대기</option>
              <option>종결</option>
            </select>
            <select className="input w-auto">
              <option>전체 등급</option>
              <option>1등급</option>
              <option>2등급</option>
              <option>3등급</option>
              <option>4등급</option>
              <option>5등급</option>
            </select>
            <select className="input w-auto">
              <option>전체 서비스</option>
              <option>방문요양</option>
              <option>방문목욕</option>
              <option>방문간호</option>
            </select>
            <button className="btn-secondary text-sm">초기화</button>
          </div>
        </div>

        {/* Table */}
        <div className="card overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50 border-b border-gray-200">
                <tr>
                  <th className="table-header">이용자</th>
                  <th className="table-header">나이</th>
                  <th className="table-header">등급</th>
                  <th className="table-header">서비스</th>
                  <th className="table-header">담당 요양보호사</th>
                  <th className="table-header">최근 방문</th>
                  <th className="table-header">연락처</th>
                  <th className="table-header">상태</th>
                  <th className="table-header"></th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {CLIENTS.map((client) => (
                  <tr key={client.id} className="hover:bg-gray-50 transition-colors">
                    <td className="table-cell">
                      <div className="flex items-center gap-3">
                        <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
                          <span className="text-xs font-semibold text-blue-700">{client.name[0]}</span>
                        </div>
                        <div>
                          <p className="font-medium text-gray-900">{client.name}</p>
                          <p className="text-xs text-gray-500">{client.address}</p>
                        </div>
                      </div>
                    </td>
                    <td className="table-cell text-gray-600">{client.age}세</td>
                    <td className="table-cell">
                      <span className={`badge ${GRADE_COLORS[client.grade]}`}>{client.grade}</span>
                    </td>
                    <td className="table-cell">{client.service}</td>
                    <td className="table-cell">{client.caregiver}</td>
                    <td className="table-cell text-gray-500">{client.lastVisit}</td>
                    <td className="table-cell text-gray-500">{client.phone}</td>
                    <td className="table-cell">
                      <span className={STATUS_BADGE[client.status]}>{client.status}</span>
                    </td>
                    <td className="table-cell">
                      <Link href={`/clients/${client.id}`} className="text-blue-600 hover:text-blue-800 text-xs font-medium">
                        상세보기
                      </Link>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <div className="flex items-center justify-between px-5 py-3 border-t border-gray-100 bg-gray-50">
            <p className="text-sm text-gray-500">총 {CLIENTS.length}명 중 1-10명 표시</p>
            <div className="flex items-center gap-1">
              <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">이전</button>
              <button className="px-3 py-1 text-sm bg-blue-600 text-white rounded">1</button>
              <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">2</button>
              <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">다음</button>
            </div>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
