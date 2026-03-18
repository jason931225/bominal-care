import InternalAppShell from '@/components/InternalAppShell';
import Link from 'next/link';

const MOCK_INCIDENT = {
  id: 'INC-001',
  date: '2026-03-12',
  reportedAt: '2026-03-12 11:45',
  client: '이정자',
  caregiver: '정미영',
  type: '낙상',
  description: '욕실에서 미끄러짐, 좌측 무릎 타박상 발생. 요양보호사 정미영이 즉시 발견하여 조치.',
  severity: 'high',
  status: '처리중',
  reporter: '정미영',
  location: '이용자 자택 욕실',
  witness: '없음',
  injuryDetails: '좌측 무릎 타박상. 찰과상 있음. 걷기 불편 호소.',
  immediateAction: '상처 소독 및 보호대 적용. 가족(딸)에게 즉시 연락. 병원 방문 안내.',
  followUp: '익일 병원 방문 결과 확인 예정.',
  resolution: '',
  timeline: [
    { time: '11:30', event: '낙상 사고 발생' },
    { time: '11:32', event: '요양보호사 발견 및 응급처치' },
    { time: '11:35', event: '가족(딸) 연락' },
    { time: '11:45', event: '기관 담당자에게 사고 보고' },
    { time: '12:30', event: '가족 도착 확인' },
    { time: '14:00', event: '근처 정형외과 방문 (타박상 진단)' },
  ],
};

const SEVERITY_BADGE: Record<string, string> = { high: 'badge-red', medium: 'badge-yellow', low: 'badge-blue' };
const SEVERITY_LABEL: Record<string, string> = { high: '높음', medium: '보통', low: '낮음' };

export default async function IncidentDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  const incident = { ...MOCK_INCIDENT, id };

  return (
    <InternalAppShell>
      <div className="space-y-6">
        {/* Breadcrumb */}
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <Link href="/quality" className="hover:text-blue-600">품질 관리</Link>
          <span>/</span>
          <Link href="/quality/incidents" className="hover:text-blue-600">사고 관리</Link>
          <span>/</span>
          <span className="text-gray-900">{incident.id}</span>
        </div>

        {/* Header */}
        <div className="flex items-start justify-between">
          <div>
            <div className="flex items-center gap-3">
              <h1 className="page-title">사고 상세 — {incident.id}</h1>
              <span className={SEVERITY_BADGE[incident.severity]}>{SEVERITY_LABEL[incident.severity]}</span>
              <span className="badge-yellow">{incident.status}</span>
            </div>
            <p className="text-sm text-gray-500 mt-1">
              {incident.date} · {incident.client} · {incident.type}
            </p>
          </div>
          <div className="flex gap-2">
            <button className="btn-secondary">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" />
              </svg>
              보고서 출력
            </button>
            <button className="btn-primary">처리 완료</button>
          </div>
        </div>

        <div className="grid grid-cols-3 gap-6">
          {/* Left */}
          <div className="col-span-1 space-y-4">
            <div className="card p-5">
              <h2 className="section-title mb-4">사고 기본 정보</h2>
              <dl className="space-y-3">
                {[
                  { label: '사고 번호', value: incident.id },
                  { label: '발생 일시', value: incident.date },
                  { label: '보고 일시', value: incident.reportedAt },
                  { label: '이용자', value: incident.client },
                  { label: '담당 요양보호사', value: incident.caregiver },
                  { label: '보고자', value: incident.reporter },
                  { label: '사고 유형', value: incident.type },
                  { label: '발생 장소', value: incident.location },
                  { label: '목격자', value: incident.witness },
                ].map(({ label, value }) => (
                  <div key={label}>
                    <dt className="text-xs font-medium text-gray-400 uppercase tracking-wide">{label}</dt>
                    <dd className="text-sm text-gray-900 mt-0.5">{value}</dd>
                  </div>
                ))}
              </dl>
            </div>

            {/* Timeline */}
            <div className="card p-5">
              <h2 className="section-title mb-4">사고 타임라인</h2>
              <div className="space-y-3">
                {incident.timeline.map((item, idx) => (
                  <div key={idx} className="flex items-start gap-3">
                    <div className="flex flex-col items-center">
                      <div className="w-2 h-2 rounded-full bg-blue-500 flex-shrink-0 mt-1.5"></div>
                      {idx < incident.timeline.length - 1 && (
                        <div className="w-0.5 h-6 bg-blue-200 mt-1"></div>
                      )}
                    </div>
                    <div>
                      <p className="text-xs font-semibold text-blue-600">{item.time}</p>
                      <p className="text-sm text-gray-700">{item.event}</p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Right */}
          <div className="col-span-2 space-y-5">
            <div className="card p-5">
              <h2 className="section-title mb-3">사고 내용</h2>
              <p className="text-sm text-gray-700 bg-gray-50 rounded-xl p-4 leading-relaxed">
                {incident.description}
              </p>
            </div>

            <div className="card p-5">
              <h2 className="section-title mb-3">부상/피해 내용</h2>
              <p className="text-sm text-gray-700 leading-relaxed">{incident.injuryDetails}</p>
            </div>

            <div className="card p-5">
              <h2 className="section-title mb-3">즉각 조치 사항</h2>
              <p className="text-sm text-gray-700 leading-relaxed">{incident.immediateAction}</p>
            </div>

            <div className="card p-5">
              <h2 className="section-title mb-3">후속 조치 계획</h2>
              <p className="text-sm text-gray-700 leading-relaxed">{incident.followUp}</p>
            </div>

            {/* Resolution form */}
            <div className="card p-5">
              <h2 className="section-title mb-4">처리 결과 입력</h2>
              <div className="space-y-4">
                <div>
                  <label className="label">처리 결과</label>
                  <textarea
                    className="input w-full h-28 resize-none"
                    placeholder="사고 처리 결과를 입력하세요..."
                    defaultValue={incident.resolution}
                  />
                </div>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="label">처리 완료일</label>
                    <input type="date" className="input" />
                  </div>
                  <div>
                    <label className="label">재발 방지 대책</label>
                    <input type="text" className="input" placeholder="재발 방지 대책 요약..." />
                  </div>
                </div>
                <div className="flex justify-end gap-2">
                  <button className="btn-secondary">임시 저장</button>
                  <button className="btn-primary">처리 완료 확정</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </InternalAppShell>
  );
}
