import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const MOCK_APPLICATION = {
  id: 'APP-001',
  name: '김현숙',
  age: 42,
  gender: '여',
  phone: '010-1122-3344',
  email: 'hyunsuk.kim@email.com',
  address: '서울 서초구 반포동',
  cert: '요양보호사 1급',
  certNo: '제2019-서초-0234호',
  certIssuedAt: '2019-06-15',
  experience: '3년 2개월',
  availability: '평일 오전 (09:00 - 13:00)',
  source: '온라인 지원',
  appliedAt: '2026-03-14',
  status: '검토중',
  coverLetter: '저는 노인 요양 분야에서 3년 이상의 경험을 가지고 있으며, 어르신들의 삶의 질 향상을 위해 최선을 다하겠습니다. 치매 환자 케어 경험이 있으며, 가족들과의 소통도 중요시 여깁니다.',
  workHistory: [
    { org: '서울 노인복지센터', role: '방문요양보호사', period: '2023.01 - 2026.02', note: '치매 어르신 전담' },
    { org: '강남 재가센터', role: '방문요양보호사', period: '2021.06 - 2022.12', note: '방문요양 / 목욕' },
  ],
  training: ['치매 전문 교육 16시간 이수 (2024)', '노인 응급처치 교육 8시간 (2023)', '인지활동형 프로그램 교육 (2022)'],
  reviewer: '',
  reviewNotes: '',
};

export default async function ApplicationReviewPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  const app = { ...MOCK_APPLICATION, id };

  return (
    <InternalAppShell>
      <div className="space-y-6">
        {/* Breadcrumb */}
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <Link href="/caregivers" className="hover:text-blue-600">요양보호사 관리</Link>
          <span>/</span>
          <Link href="/caregivers/applications" className="hover:text-blue-600">채용 신청</Link>
          <span>/</span>
          <span className="text-gray-900">{app.name}</span>
        </div>

        <div className="flex items-start justify-between">
          <div className="flex items-center gap-4">
            <div className="w-14 h-14 rounded-full bg-purple-100 flex items-center justify-center">
              <span className="text-xl font-semibold text-purple-700">{app.name[0]}</span>
            </div>
            <div>
              <div className="flex items-center gap-3">
                <h1 className="page-title">{app.name}</h1>
                <span className="badge-yellow">{app.status}</span>
              </div>
              <p className="text-sm text-gray-500 mt-1">{app.age}세 · {app.cert} · 지원일 {app.appliedAt}</p>
            </div>
          </div>
          <div className="flex gap-2">
            <button className="btn-danger">불합격 처리</button>
            <button className="btn-secondary">면접 일정 지정</button>
            <button className="btn-primary">최종 합격 처리</button>
          </div>
        </div>

        <div className="grid grid-cols-3 gap-6">
          {/* Left: profile */}
          <div className="col-span-1 space-y-4">
            <div className="card p-5">
              <h2 className="section-title mb-4">기본 정보</h2>
              <dl className="space-y-3">
                {[
                  { label: '연락처', value: app.phone },
                  { label: '이메일', value: app.email },
                  { label: '주소', value: app.address },
                  { label: '자격증', value: `${app.cert} (${app.certNo})` },
                  { label: '자격 취득일', value: app.certIssuedAt },
                  { label: '총 경력', value: app.experience },
                  { label: '가능 시간', value: app.availability },
                  { label: '지원 경로', value: app.source },
                ].map(({ label, value }) => (
                  <div key={label}>
                    <dt className="text-xs font-medium text-gray-400 uppercase tracking-wide">{label}</dt>
                    <dd className="text-sm text-gray-900 mt-0.5">{value}</dd>
                  </div>
                ))}
              </dl>
            </div>

            <div className="card p-5">
              <h2 className="section-title mb-3">교육 이수</h2>
              <ul className="space-y-1.5">
                {app.training.map((item, idx) => (
                  <li key={idx} className="flex items-start gap-2 text-sm text-gray-700">
                    <svg className="w-3.5 h-3.5 text-green-500 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                      <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                    </svg>
                    {item}
                  </li>
                ))}
              </ul>
            </div>
          </div>

          {/* Right: details */}
          <div className="col-span-2 space-y-6">
            {/* Work history */}
            <div className="card p-5">
              <h2 className="section-title mb-4">경력 사항</h2>
              <div className="space-y-4">
                {app.workHistory.map((job, idx) => (
                  <div key={idx} className="flex items-start gap-4 p-4 bg-gray-50 rounded-xl">
                    <div className="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center flex-shrink-0">
                      <svg className="w-5 h-5 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                        <path strokeLinecap="round" strokeLinejoin="round" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                      </svg>
                    </div>
                    <div>
                      <p className="font-semibold text-gray-900">{job.org}</p>
                      <p className="text-sm text-gray-600">{job.role} · {job.period}</p>
                      <p className="text-sm text-gray-500 mt-1">{job.note}</p>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Cover letter */}
            <div className="card p-5">
              <h2 className="section-title mb-3">자기소개서</h2>
              <p className="text-sm text-gray-700 leading-relaxed bg-gray-50 rounded-xl p-4">
                {app.coverLetter}
              </p>
            </div>

            {/* Review area */}
            <div className="card p-5">
              <h2 className="section-title mb-4">검토 의견</h2>
              <div className="space-y-4">
                <div>
                  <label className="label">면접 일시</label>
                  <input type="datetime-local" className="input" />
                </div>
                <div>
                  <label className="label">검토 내용</label>
                  <textarea
                    className="input w-full h-24 resize-none"
                    placeholder="검토 의견을 입력하세요..."
                  />
                </div>
                <div className="flex gap-3 justify-end">
                  <button className="btn-danger">불합격</button>
                  <button className="btn-secondary">면접 일정 저장</button>
                  <button className="btn-primary">최종 합격</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
