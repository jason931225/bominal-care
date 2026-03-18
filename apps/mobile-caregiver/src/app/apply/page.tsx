import Link from 'next/link';
import ApplicantAppShell from '@/components/ApplicantAppShell';

const REQUIREMENTS = [
  { icon: '📋', label: '요양보호사 자격증 사본' },
  { icon: '🪪', label: '신분증 (주민등록증 또는 운전면허증)' },
  { icon: '📍', label: '서비스 가능 지역 정보' },
  { icon: '🕐', label: '근무 가능 시간 정보' },
  { icon: '📞', label: '추천인 연락처 (선택사항)' },
];

const BENEFITS = [
  { value: '주 40시간', label: '최대 근무시간' },
  { value: '15,230원', label: '시간당 기본 수당' },
  { value: '4대 보험', label: '전액 지원' },
  { value: '월 2회', label: '교육·역량강화' },
];

export default function ApplyOverviewPage() {
  return (
    <ApplicantAppShell currentStep={0} title="요양보호사 신청">
      <div className="px-4 py-6 space-y-6">
        {/* Hero */}
        <div className="bg-gradient-to-br from-blue-600 to-blue-700 rounded-2xl p-6 text-white">
          <div className="text-3xl mb-3">🤝</div>
          <h2 className="text-xl font-bold mb-2">함께 돌봄을 시작해요</h2>
          <p className="text-blue-100 text-sm leading-relaxed">
            어르신들의 소중한 일상을 함께하는 요양보호사로 활동해 보세요.
            안정적인 수입과 보람 있는 일을 경험할 수 있습니다.
          </p>
        </div>

        {/* Benefits */}
        <div>
          <h3 className="section-title">활동 혜택</h3>
          <div className="grid grid-cols-2 gap-3">
            {BENEFITS.map((b) => (
              <div key={b.label} className="card text-center py-5">
                <p className="text-xl font-bold text-blue-600">{b.value}</p>
                <p className="text-xs text-slate-500 mt-1">{b.label}</p>
              </div>
            ))}
          </div>
        </div>

        {/* Process */}
        <div>
          <h3 className="section-title">신청 절차</h3>
          <div className="card space-y-4">
            {[
              { step: '01', title: '온라인 신청', desc: '기본 정보 및 자격 입력' },
              { step: '02', title: '서류 검토', desc: '영업일 기준 3-5일 소요' },
              { step: '03', title: '면접·교육', desc: '방문 또는 화상 면접' },
              { step: '04', title: '활동 시작', desc: '이용자 매칭 및 배정' },
            ].map((item, idx) => (
              <div key={item.step} className="flex items-start gap-4">
                <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0 mt-0.5">
                  <span className="text-xs font-bold text-blue-600">{item.step}</span>
                </div>
                <div className="flex-1">
                  <p className="font-medium text-slate-800 text-sm">{item.title}</p>
                  <p className="text-xs text-slate-500 mt-0.5">{item.desc}</p>
                </div>
                {idx < 3 && (
                  <div className="absolute ml-4 mt-9 w-0.5 h-4 bg-blue-100" />
                )}
              </div>
            ))}
          </div>
        </div>

        {/* Requirements */}
        <div>
          <h3 className="section-title">준비 서류</h3>
          <div className="card space-y-3">
            {REQUIREMENTS.map((req) => (
              <div key={req.label} className="flex items-center gap-3">
                <span className="text-lg">{req.icon}</span>
                <span className="text-sm text-slate-700">{req.label}</span>
              </div>
            ))}
          </div>
        </div>

        {/* CTA */}
        <div className="space-y-3 pb-6">
          <Link href="/apply/identity" className="btn-primary block text-center">
            신청 시작하기
          </Link>
          <Link href="/apply/status" className="btn-secondary block text-center">
            기존 신청 현황 확인
          </Link>
        </div>
      </div>
    </ApplicantAppShell>
  );
}
