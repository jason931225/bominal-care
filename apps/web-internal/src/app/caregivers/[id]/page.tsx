import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const MOCK_CAREGIVER = {
  id: 'G001',
  name: '이민정',
  age: 45,
  gender: '여',
  phone: '010-1111-2222',
  address: '서울 강남구 역삼동',
  cert: '요양보호사 1급',
  certNo: '제2021-강남-001호',
  hire: '2021-03-01',
  status: '근무중',
  rating: 4.9,
  totalVisits: 312,
  thisMonthVisits: 18,
  clients: ['박순자', '이정자'],
  education: '간호조무사 자격 보유',
  specialNotes: '치매 전문 교육 이수. 야간 근무 가능.',
};

const SCHEDULE = [
  { date: '월', times: ['09:00-12:00 박순자 (방문요양)', '14:00-16:00 이정자 (방문요양)'] },
  { date: '화', times: ['09:00-12:00 박순자 (방문요양)', '14:00-16:00 이정자 (방문요양)'] },
  { date: '수', times: ['09:00-12:00 박순자 (방문목욕)'] },
  { date: '목', times: ['09:00-12:00 박순자 (방문요양)', '14:00-16:00 이정자 (방문요양)'] },
  { date: '금', times: ['09:00-12:00 박순자 (방문요양)'] },
  { date: '토', times: [] },
  { date: '일', times: [] },
];

const RECENT_RECORDS = [
  { date: '2026-03-15', client: '박순자', service: '방문요양', hours: '09:00-12:00', status: '완료', note: '정상' },
  { date: '2026-03-15', client: '이정자', service: '방문요양', hours: '14:00-16:00', status: '완료', note: '정상' },
  { date: '2026-03-13', client: '박순자', service: '방문요양', hours: '09:00-12:00', status: '완료', note: '정상' },
  { date: '2026-03-11', client: '박순자', service: '방문목욕', hours: '09:00-11:00', status: '완료', note: '정상' },
  { date: '2026-03-10', client: '이정자', service: '방문요양', hours: '14:00-16:00', status: '완료', note: '정상' },
];

export default async function CaregiverDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  const caregiver = { ...MOCK_CAREGIVER, id };

  return (
    <InternalAppShell>
      <div className="space-y-6">
        {/* Breadcrumb */}
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <Link href="/caregivers" className="hover:text-blue-600">요양보호사 관리</Link>
          <span>/</span>
          <span className="text-gray-900">{caregiver.name}</span>
        </div>

        {/* Header */}
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-4">
            <div className="relative">
              <div className="w-16 h-16 rounded-full bg-blue-100 flex items-center justify-center">
                <span className="text-2xl font-semibold text-blue-700">{caregiver.name[0]}</span>
              </div>
              <span className="absolute bottom-0 right-0 w-4 h-4 bg-green-500 rounded-full border-2 border-white"></span>
            </div>
            <div>
              <div className="flex items-center gap-3">
                <h1 className="page-title">{caregiver.name}</h1>
                <span className="badge-green">{caregiver.status}</span>
              </div>
              <p className="text-sm text-gray-500 mt-1">
                {caregiver.age}세 · {caregiver.gender} · {caregiver.cert} · 입사 {caregiver.hire}
              </p>
            </div>
          </div>
          <div className="flex gap-2">
            <button className="btn-secondary">메시지 전송</button>
            <button className="btn-primary">정보 수정</button>
          </div>
        </div>

        <div className="grid grid-cols-3 gap-6">
          {/* Left: profile */}
          <div className="col-span-1 space-y-4">
            {/* Stats */}
            <div className="grid grid-cols-2 gap-3">
              <div className="stat-card text-center">
                <p className="text-2xl font-bold text-gray-900">{caregiver.rating}</p>
                <p className="text-xs text-gray-500 mt-1">평균 평점</p>
              </div>
              <div className="stat-card text-center">
                <p className="text-2xl font-bold text-gray-900">{caregiver.totalVisits}</p>
                <p className="text-xs text-gray-500 mt-1">총 방문 수</p>
              </div>
              <div className="stat-card text-center">
                <p className="text-2xl font-bold text-gray-900">{caregiver.thisMonthVisits}</p>
                <p className="text-xs text-gray-500 mt-1">이번달 방문</p>
              </div>
              <div className="stat-card text-center">
                <p className="text-2xl font-bold text-gray-900">{caregiver.clients.length}</p>
                <p className="text-xs text-gray-500 mt-1">담당 이용자</p>
              </div>
            </div>

            {/* Profile details */}
            <div className="card p-5">
              <h2 className="section-title mb-4">기본 정보</h2>
              <dl className="space-y-3">
                {[
                  { label: '연락처', value: caregiver.phone },
                  { label: '주소', value: caregiver.address },
                  { label: '자격증 번호', value: caregiver.certNo },
                  { label: '학력/경력', value: caregiver.education },
                ].map(({ label, value }) => (
                  <div key={label}>
                    <dt className="text-xs font-medium text-gray-400 uppercase tracking-wide">{label}</dt>
                    <dd className="text-sm text-gray-900 mt-0.5">{value}</dd>
                  </div>
                ))}
              </dl>
            </div>

            {/* Assigned clients */}
            <div className="card p-5">
              <h2 className="section-title mb-3">담당 이용자</h2>
              <ul className="space-y-2">
                {caregiver.clients.map((client, idx) => (
                  <li key={idx} className="flex items-center gap-2 text-sm">
                    <div className="w-7 h-7 rounded-full bg-green-100 flex items-center justify-center">
                      <span className="text-xs font-semibold text-green-700">{client[0]}</span>
                    </div>
                    <span className="text-gray-800">{client}</span>
                  </li>
                ))}
              </ul>
            </div>

            <div className="card p-5">
              <h2 className="section-title mb-3">특이 사항</h2>
              <p className="text-sm text-gray-700 leading-relaxed">{caregiver.specialNotes}</p>
            </div>
          </div>

          {/* Right: schedule + records */}
          <div className="col-span-2 space-y-6">
            {/* Weekly schedule */}
            <div className="card">
              <div className="px-5 py-4 border-b border-gray-100">
                <h2 className="section-title">이번 주 일정</h2>
              </div>
              <div className="p-4">
                <div className="grid grid-cols-7 gap-2">
                  {SCHEDULE.map((day) => (
                    <div key={day.date} className="text-center">
                      <div className="text-xs font-semibold text-gray-500 mb-2">{day.date}</div>
                      <div className="space-y-1">
                        {day.times.length > 0 ? (
                          day.times.map((t, i) => (
                            <div key={i} className="text-xs bg-blue-50 text-blue-700 rounded p-1 text-left leading-tight">
                              {t.split(' ')[0]}
                              <br />
                              <span className="text-blue-500">{t.split(' ').slice(1).join(' ')}</span>
                            </div>
                          ))
                        ) : (
                          <div className="h-8 bg-gray-50 rounded text-xs text-gray-400 flex items-center justify-center">
                            휴무
                          </div>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>

            {/* Recent records */}
            <div className="card">
              <div className="px-5 py-4 border-b border-gray-100">
                <h2 className="section-title">최근 방문 기록</h2>
              </div>
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead className="bg-gray-50">
                    <tr>
                      <th className="table-header">날짜</th>
                      <th className="table-header">이용자</th>
                      <th className="table-header">서비스</th>
                      <th className="table-header">시간</th>
                      <th className="table-header">비고</th>
                      <th className="table-header">상태</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-100">
                    {RECENT_RECORDS.map((rec, idx) => (
                      <tr key={idx} className="hover:bg-gray-50">
                        <td className="table-cell font-medium">{rec.date}</td>
                        <td className="table-cell">{rec.client}</td>
                        <td className="table-cell">{rec.service}</td>
                        <td className="table-cell text-gray-500">{rec.hours}</td>
                        <td className="table-cell text-gray-500">{rec.note}</td>
                        <td className="table-cell"><span className="badge-green">{rec.status}</span></td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
