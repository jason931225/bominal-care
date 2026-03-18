import Link from 'next/link';

const STEPS = [
  { label: '본인인증', href: '/apply/identity' },
  { label: '자격증', href: '/apply/credentials' },
  { label: '서비스지역', href: '/apply/service-region' },
  { label: '가능시간', href: '/apply/schedule' },
  { label: '서비스유형', href: '/apply/services' },
  { label: '추천인', href: '/apply/references' },
  { label: '검토', href: '/apply/review' },
];

interface ApplicantAppShellProps {
  children: React.ReactNode;
  currentStep: number; // 1-based, 0 = overview
  title: string;
}

export default function ApplicantAppShell({
  children,
  currentStep,
  title,
}: ApplicantAppShellProps) {
  const totalSteps = STEPS.length;
  const progressPct = currentStep > 0 ? (currentStep / totalSteps) * 100 : 0;

  return (
    <div className="min-h-screen bg-white flex flex-col">
      {/* Header */}
      <header className="sticky top-0 z-20 bg-white border-b border-slate-100">
        <div className="flex items-center px-4 h-14 top-safe">
          {currentStep > 1 ? (
            <Link
              href={STEPS[currentStep - 2]?.href ?? '/apply'}
              className="mr-3 p-2 -ml-2 rounded-full active:bg-slate-100 transition-colors"
              aria-label="뒤로"
            >
              <ChevronLeftIcon />
            </Link>
          ) : (
            <Link
              href={currentStep === 1 ? '/apply' : '/'}
              className="mr-3 p-2 -ml-2 rounded-full active:bg-slate-100 transition-colors"
              aria-label="뒤로"
            >
              <ChevronLeftIcon />
            </Link>
          )}
          <div className="flex-1">
            <p className="text-xs text-slate-400 font-medium">
              {currentStep > 0 ? `${currentStep} / ${totalSteps}단계` : '신청 안내'}
            </p>
            <h1 className="text-base font-semibold text-slate-900 leading-tight">
              {title}
            </h1>
          </div>
          <Link
            href="/apply/status"
            className="text-xs text-blue-600 font-medium px-2 py-1 rounded-lg active:bg-blue-50"
          >
            신청현황
          </Link>
        </div>

        {/* Progress bar */}
        {currentStep > 0 && (
          <div className="h-1 bg-slate-100">
            <div
              className="h-full bg-blue-600 transition-all duration-500 ease-out"
              style={{ width: `${progressPct}%` }}
            />
          </div>
        )}
      </header>

      {/* Step pills */}
      {currentStep > 0 && (
        <div className="px-4 py-3 overflow-x-auto no-scrollbar">
          <div className="flex gap-2 w-max">
            {STEPS.map((step, idx) => {
              const stepNum = idx + 1;
              const isDone = stepNum < currentStep;
              const isActive = stepNum === currentStep;
              return (
                <div
                  key={step.href}
                  className={`flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium whitespace-nowrap transition-colors ${
                    isActive
                      ? 'bg-blue-600 text-white'
                      : isDone
                      ? 'bg-blue-100 text-blue-700'
                      : 'bg-slate-100 text-slate-400'
                  }`}
                >
                  {isDone && (
                    <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path
                        fillRule="evenodd"
                        d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                        clipRule="evenodd"
                      />
                    </svg>
                  )}
                  {step.label}
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* Content */}
      <main className="flex-1 overflow-y-auto">{children}</main>
    </div>
  );
}

function ChevronLeftIcon() {
  return (
    <svg
      className="w-5 h-5 text-slate-700"
      fill="none"
      stroke="currentColor"
      strokeWidth={2.5}
      viewBox="0 0 24 24"
    >
      <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
    </svg>
  );
}
