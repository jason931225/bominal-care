import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const CAREGIVERS = [
  { id: 'G001', name: '이민정', age: 45, cert: '요양보호사 1급', hire: '2021-03-01', clients: 5, todayVisits: 3, phone: '010-1111-2222', status: '근무중', rating: 4.9 },
  { id: 'G002', name: '최수진', age: 38, cert: '요양보호사 1급', hire: '2022-06-15', clients: 4, todayVisits: 2, phone: '010-2222-3333', status: '근무중', rating: 4.7 },
  { id: 'G003', name: '정미영', age: 52, cert: '요양보호사 1급', hire: '2020-01-10', clients: 6, todayVisits: 4, phone: '010-3333-4444', status: '근무중', rating: 4.8 },
  { id: 'G004', name: '한지영', age: 41, cert: '요양보호사 2급', hire: '2023-02-20', clients: 3, todayVisits: 2, phone: '010-4444-5555', status: '외출', rating: 4.5 },
  { id: 'G005', name: '김은지', age: 35, cert: '요양보호사 1급', hire: '2021-09-01', clients: 4, todayVisits: 3, phone: '010-5555-6666', status: '근무중', rating: 4.6 },
  { id: 'G006', name: '오혜진', age: 48, cert: '요양보호사 1급', hire: '2019-11-15', clients: 5, todayVisits: 3, phone: '010-6666-7777', status: '근무중', rating: 4.9 },
  { id: 'G007', name: '박지수', age: 29, cert: '요양보호사 2급', hire: '2024-01-08', clients: 3, todayVisits: 2, phone: '010-7777-8888', status: '휴가', rating: 4.3 },
  { id: 'G008', name: '윤소영', age: 44, cert: '요양보호사 1급', hire: '2022-04-01', clients: 4, todayVisits: 0, phone: '010-8888-9999', status: '대기', rating: 4.4 },
];

const STATUS_CONFIG: Record<string, { badge: string; dot: string }> = {
  '근무중': { badge: 'badge-green', dot: 'bg-green-500' },
  '외출': { badge: 'badge-yellow', dot: 'bg-yellow-500' },
  '휴가': { badge: 'badge-blue', dot: 'bg-blue-400' },
  '대기': { badge: 'badge-gray', dot: 'bg-gray-400' },
};

export default function CaregiversPage() {
  const activeCount = CAREGIVERS.filter(g => g.status === '근무중').length;

  return (
    <InternalAppShell>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="page-title">요양보호사 관리</h1>
            <p className="text-sm text-gray-500 mt-1">총 {CAREGIVERS.length}명 · 현재 근무 {activeCount}명</p>
          </div>
          <div className="flex gap-2">
            <Link href="/caregivers/applications" className="btn-secondary">
              채용 신청 보기
            </Link>
            <button className="btn-primary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
              </svg>
              요양보호사 등록
            </button>
          </div>
        </div>

        {/* Status summary */}
        <div className="grid grid-cols-4 gap-4">
          {[
            { label: '근무중', count: CAREGIVERS.filter(g => g.status === '근무중').length, color: 'text-green-700 bg-green-50' },
            { label: '외출', count: CAREGIVERS.filter(g => g.status === '외출').length, color: 'text-yellow-700 bg-yellow-50' },
            { label: '휴가', count: CAREGIVERS.filter(g => g.status === '휴가').length, color: 'text-blue-700 bg-blue-50' },
            { label: '대기', count: CAREGIVERS.filter(g => g.status === '대기').length, color: 'text-gray-700 bg-gray-100' },
          ].map((s) => (
            <div key={s.label} className="stat-card">
              <p className="text-sm font-medium text-gray-500">{s.label}</p>
              <p className="text-3xl font-bold text-gray-900 mt-1">{s.count}</p>
              <span className={`text-xs font-medium px-2 py-0.5 rounded-full mt-2 inline-block ${s.color}`}>명</span>
            </div>
          ))}
        </div>

        {/* Filter bar */}
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
              <option>근무중</option>
              <option>외출</option>
              <option>휴가</option>
              <option>대기</option>
            </select>
            <select className="input w-auto">
              <option>전체 자격증</option>
              <option>1급</option>
              <option>2급</option>
            </select>
          </div>
        </div>

        {/* Grid view */}
        <div className="grid grid-cols-2 xl:grid-cols-3 gap-4">
          {CAREGIVERS.map((caregiver) => (
            <div key={caregiver.id} className="card p-5 hover:border-blue-200 hover:shadow-md transition-all">
              <div className="flex items-start justify-between mb-4">
                <div className="flex items-center gap-3">
                  <div className="relative">
                    <div className="w-11 h-11 rounded-full bg-blue-100 flex items-center justify-center">
                      <span className="text-base font-semibold text-blue-700">{caregiver.name[0]}</span>
                    </div>
                    <span className={`absolute bottom-0 right-0 w-3 h-3 rounded-full border-2 border-white ${STATUS_CONFIG[caregiver.status].dot}`}></span>
                  </div>
                  <div>
                    <p className="font-semibold text-gray-900">{caregiver.name}</p>
                    <p className="text-xs text-gray-500">{caregiver.cert}</p>
                  </div>
                </div>
                <span className={STATUS_CONFIG[caregiver.status].badge}>{caregiver.status}</span>
              </div>

              <div className="grid grid-cols-3 gap-2 mb-4 text-center">
                <div className="bg-gray-50 rounded-lg p-2">
                  <p className="text-lg font-bold text-gray-900">{caregiver.clients}</p>
                  <p className="text-xs text-gray-500">담당 이용자</p>
                </div>
                <div className="bg-gray-50 rounded-lg p-2">
                  <p className="text-lg font-bold text-gray-900">{caregiver.todayVisits}</p>
                  <p className="text-xs text-gray-500">오늘 방문</p>
                </div>
                <div className="bg-gray-50 rounded-lg p-2">
                  <p className="text-lg font-bold text-gray-900">{caregiver.rating}</p>
                  <p className="text-xs text-gray-500">평균 평점</p>
                </div>
              </div>

              <div className="flex items-center justify-between text-xs text-gray-500 mb-4">
                <span>입사: {caregiver.hire}</span>
                <span>{caregiver.phone}</span>
              </div>

              <Link
                href={`/caregivers/${caregiver.id}`}
                className="block w-full text-center py-2 text-sm font-medium text-blue-600 hover:text-blue-800 border border-blue-200 hover:border-blue-400 rounded-lg transition-colors"
              >
                상세보기
              </Link>
            </div>
          ))}
        </div>
      </div>
    </InternalAppShell>
  );
}
