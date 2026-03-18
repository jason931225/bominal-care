import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const MOCK_CLIENT = {
  id: 'C001',
  name: '박순자',
  age: 72,
  birth: '1954-08-23',
  gender: '여',
  grade: '2등급',
  phone: '010-1234-5678',
  guardian: '박민수 (아들)',
  guardianPhone: '010-9876-5432',
  address: '서울특별시 강남구 역삼동 123-45',
  registeredAt: '2024-01-15',
  caregiver: '이민정',
  service: '방문요양',
  serviceHours: '주 5회, 회당 3시간',
  status: '활성',
  notes: '고혈압, 당뇨 복약 관리 필요. 이동 시 보행보조기 사용.',
};

const CARE_HISTORY = [
  { date: '2026-03-15', caregiver: '이민정', service: '방문요양', hours: '09:00 - 12:00', note: '정상 수행', status: '완료' },
  { date: '2026-03-13', caregiver: '이민정', service: '방문요양', hours: '09:00 - 12:00', note: '혈압 측정 (140/90)', status: '완료' },
  { date: '2026-03-11', caregiver: '이민정', service: '방문요양', hours: '09:00 - 12:00', note: '정상 수행', status: '완료' },
  { date: '2026-03-09', caregiver: '이민정', service: '방문목욕', hours: '10:00 - 12:00', note: '정상 수행', status: '완료' },
  { date: '2026-03-07', caregiver: '이민정', service: '방문요양', hours: '09:00 - 12:00', note: '가족 동석 요청', status: '완료' },
];

export default async function ClientDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  const client = { ...MOCK_CLIENT, id };

  return (
    <InternalAppShell>
      <div className="space-y-6">
        {/* Breadcrumb */}
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <Link href="/clients" className="hover:text-blue-600">이용자 관리</Link>
          <span>/</span>
          <span className="text-gray-900">{client.name}</span>
        </div>

        {/* Header */}
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-4">
            <div className="w-16 h-16 rounded-full bg-blue-100 flex items-center justify-center">
              <span className="text-2xl font-semibold text-blue-700">{client.name[0]}</span>
            </div>
            <div>
              <div className="flex items-center gap-3">
                <h1 className="page-title">{client.name}</h1>
                <span className="badge-green">활성</span>
                <span className="badge bg-orange-50 text-orange-700">{client.grade}</span>
              </div>
              <p className="text-sm text-gray-500 mt-1">{client.age}세 · {client.gender} · {client.service} · 등록일 {client.registeredAt}</p>
            </div>
          </div>
          <div className="flex gap-2">
            <Link href={`/clients/${client.id}/care-plan`} className="btn-secondary">
              케어 플랜 보기
            </Link>
            <button className="btn-primary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
              정보 수정
            </button>
          </div>
        </div>

        {/* Two column layout */}
        <div className="grid grid-cols-3 gap-6">
          {/* Left: profile details */}
          <div className="col-span-1 space-y-4">
            <div className="card p-5">
              <h2 className="section-title mb-4">기본 정보</h2>
              <dl className="space-y-3">
                {[
                  { label: '생년월일', value: client.birth },
                  { label: '연락처', value: client.phone },
                  { label: '주소', value: client.address },
                  { label: '보호자', value: client.guardian },
                  { label: '보호자 연락처', value: client.guardianPhone },
                ].map(({ label, value }) => (
                  <div key={label} className="flex flex-col gap-0.5">
                    <dt className="text-xs font-medium text-gray-400 uppercase tracking-wide">{label}</dt>
                    <dd className="text-sm text-gray-900">{value}</dd>
                  </div>
                ))}
              </dl>
            </div>

            <div className="card p-5">
              <h2 className="section-title mb-4">서비스 정보</h2>
              <dl className="space-y-3">
                {[
                  { label: '서비스 종류', value: client.service },
                  { label: '이용 시간', value: client.serviceHours },
                  { label: '담당 요양보호사', value: client.caregiver },
                ].map(({ label, value }) => (
                  <div key={label} className="flex flex-col gap-0.5">
                    <dt className="text-xs font-medium text-gray-400 uppercase tracking-wide">{label}</dt>
                    <dd className="text-sm text-gray-900">{value}</dd>
                  </div>
                ))}
              </dl>
            </div>

            <div className="card p-5">
              <h2 className="section-title mb-3">특이 사항</h2>
              <p className="text-sm text-gray-700 leading-relaxed">{client.notes}</p>
            </div>
          </div>

          {/* Right: care history */}
          <div className="col-span-2 card">
            <div className="flex items-center justify-between px-5 py-4 border-b border-gray-100">
              <h2 className="section-title">방문 이력</h2>
              <div className="flex items-center gap-2">
                <select className="input text-xs w-auto py-1">
                  <option>최근 1개월</option>
                  <option>최근 3개월</option>
                  <option>전체</option>
                </select>
              </div>
            </div>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="table-header">날짜</th>
                    <th className="table-header">요양보호사</th>
                    <th className="table-header">서비스</th>
                    <th className="table-header">시간</th>
                    <th className="table-header">특이사항</th>
                    <th className="table-header">상태</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {CARE_HISTORY.map((visit, idx) => (
                    <tr key={idx} className="hover:bg-gray-50 transition-colors">
                      <td className="table-cell font-medium">{visit.date}</td>
                      <td className="table-cell">{visit.caregiver}</td>
                      <td className="table-cell">{visit.service}</td>
                      <td className="table-cell text-gray-500">{visit.hours}</td>
                      <td className="table-cell text-gray-600">{visit.note}</td>
                      <td className="table-cell">
                        <span className="badge-green">{visit.status}</span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            {/* Pagination */}
            <div className="flex items-center justify-between px-5 py-3 border-t border-gray-100 bg-gray-50">
              <p className="text-sm text-gray-500">총 42건의 방문 기록</p>
              <div className="flex items-center gap-1">
                <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">이전</button>
                <button className="px-3 py-1 text-sm bg-blue-600 text-white rounded">1</button>
                <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">2</button>
                <button className="px-3 py-1 text-sm text-gray-500 hover:bg-gray-200 rounded">다음</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
