import Link from 'next/link';
import ApplicantAppShell from '@/components/ApplicantAppShell';

const STATUS_STEPS = [
  {
    id: 'submitted',
    label: '신청서 제출',
    date: '2026.03.10',
    time: '14:32',
    status: 'done',
    detail: '신청서가 정상적으로 접수되었습니다.',
  },
  {
    id: 'review',
    label: '서류 검토',
    date: '2026.03.12',
    time: '09:15',
    status: 'done',
    detail: '제출하신 서류를 검토하였습니다. 모든 서류가 확인되었습니다.',
  },
  {
    id: 'interview',
    label: '면접 일정 조율',
    date: '2026.03.14',
    time: '11:00',
    status: 'active',
    detail: '면접 일정을 조율 중입니다. 담당자가 곧 연락드릴 예정입니다.',
  },
  {
    id: 'training',
    label: '오리엔테이션',
    date: null,
    time: null,
    status: 'pending',
    detail: '면접 완료 후 진행됩니다.',
  },
  {
    id: 'approval',
    label: '최종 승인',
    date: null,
    time: null,
    status: 'pending',
    detail: '모든 절차 완료 후 활동을 시작할 수 있습니다.',
  },
];

const SUBMISSION_INFO = {
  applicationId: 'CGV-2026-03102',
  submittedAt: '2026년 3월 10일 14:32',
  estimatedDays: '5~7 영업일',
  contact: '02-1234-5678',
};

export default function ApplicationStatusPage() {
  // Track current step for UI indication (currently showing 'active' status inline)
  void STATUS_STEPS.findIndex((s) => s.status === 'active');

  return (
    <ApplicantAppShell currentStep={0} title="신청 현황">
      <div className="px-4 py-6 space-y-6">
        {/* Status Summary Card */}
        <div className="bg-gradient-to-br from-blue-600 to-indigo-600 rounded-2xl p-5 text-white">
          <div className="flex items-start justify-between">
            <div>
              <p className="text-blue-200 text-xs font-medium mb-1">신청번호</p>
              <p className="font-bold text-lg">{SUBMISSION_INFO.applicationId}</p>
            </div>
            <span className="bg-amber-400 text-amber-900 text-xs font-bold px-3 py-1.5 rounded-full">
              검토 중
            </span>
          </div>
          <div className="mt-4 pt-4 border-t border-blue-500 grid grid-cols-2 gap-4">
            <div>
              <p className="text-blue-200 text-xs">제출일</p>
              <p className="text-sm font-medium mt-0.5">{SUBMISSION_INFO.submittedAt}</p>
            </div>
            <div>
              <p className="text-blue-200 text-xs">예상 소요 기간</p>
              <p className="text-sm font-medium mt-0.5">{SUBMISSION_INFO.estimatedDays}</p>
            </div>
          </div>
        </div>

        {/* Progress Timeline */}
        <div>
          <h3 className="section-title">진행 단계</h3>
          <div className="card">
            <div className="space-y-0">
              {STATUS_STEPS.map((step, idx) => {
                const isLast = idx === STATUS_STEPS.length - 1;
                return (
                  <div key={step.id} className="flex gap-4">
                    {/* Timeline */}
                    <div className="flex flex-col items-center">
                      <div className={`w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0 z-10 ${
                        step.status === 'done'
                          ? 'bg-green-500'
                          : step.status === 'active'
                          ? 'bg-blue-600 ring-4 ring-blue-100'
                          : 'bg-slate-200'
                      }`}>
                        {step.status === 'done' ? (
                          <svg className="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 20 20">
                            <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                          </svg>
                        ) : step.status === 'active' ? (
                          <div className="w-2.5 h-2.5 bg-white rounded-full" />
                        ) : (
                          <div className="w-2.5 h-2.5 bg-slate-300 rounded-full" />
                        )}
                      </div>
                      {!isLast && (
                        <div className={`w-0.5 h-full min-h-8 my-1 ${
                          step.status === 'done' ? 'bg-green-300' : 'bg-slate-200'
                        }`} />
                      )}
                    </div>

                    {/* Content */}
                    <div className={`flex-1 pb-5 ${isLast ? 'pb-0' : ''}`}>
                      <div className="flex items-center justify-between">
                        <p className={`text-sm font-semibold ${
                          step.status === 'pending' ? 'text-slate-400' : 'text-slate-800'
                        }`}>{step.label}</p>
                        {step.date && (
                          <p className="text-xs text-slate-400">{step.date} {step.time}</p>
                        )}
                      </div>
                      <p className={`text-xs mt-1 leading-relaxed ${
                        step.status === 'pending' ? 'text-slate-400' : 'text-slate-500'
                      }`}>{step.detail}</p>
                      {step.status === 'active' && (
                        <span className="inline-block mt-1.5 text-xs font-medium text-blue-600 bg-blue-50 px-2 py-0.5 rounded-full">
                          진행 중
                        </span>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </div>

        {/* Actions */}
        <div>
          <h3 className="section-title">다음 단계</h3>
          <div className="card space-y-3">
            <div className="flex items-start gap-3">
              <div className="w-9 h-9 bg-blue-100 rounded-xl flex items-center justify-center flex-shrink-0">
                <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                </svg>
              </div>
              <div>
                <p className="text-sm font-semibold text-slate-800">담당자 연락 대기</p>
                <p className="text-xs text-slate-500 mt-0.5">면접 일정 관련하여 {SUBMISSION_INFO.contact}으로 연락드립니다.</p>
              </div>
            </div>
          </div>
        </div>

        {/* Contact */}
        <div className="card">
          <h3 className="text-sm font-semibold text-slate-800 mb-3">문의하기</h3>
          <a
            href={`tel:${SUBMISSION_INFO.contact}`}
            className="flex items-center gap-3 py-3 border-b border-slate-100"
          >
            <div className="w-9 h-9 bg-slate-100 rounded-xl flex items-center justify-center">
              <svg className="w-5 h-5 text-slate-600" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
              </svg>
            </div>
            <div>
              <p className="text-sm font-medium text-slate-800">전화 문의</p>
              <p className="text-xs text-slate-400">{SUBMISSION_INFO.contact} · 평일 09:00–18:00</p>
            </div>
          </a>
          <Link
            href="/"
            className="flex items-center gap-3 py-3"
          >
            <div className="w-9 h-9 bg-slate-100 rounded-xl flex items-center justify-center">
              <svg className="w-5 h-5 text-slate-600" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
              </svg>
            </div>
            <div>
              <p className="text-sm font-medium text-slate-800">카카오톡 문의</p>
              <p className="text-xs text-slate-400">@시니어케어 채널로 문의</p>
            </div>
          </Link>
        </div>

        <div className="pb-6">
          <Link href="/apply" className="btn-secondary block text-center">
            신청 정보 수정
          </Link>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
