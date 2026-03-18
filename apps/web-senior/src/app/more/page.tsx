// 더보기 메뉴 — More / Navigation Hub
// Secondary navigation for all features not in the bottom nav bar

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const MORE_SECTIONS = [
  {
    title: '건강 관리',
    items: [
      { href: '/medical-history', icon: '📋', label: '건강 기록', description: '진단, 검사, 처치 이력' },
      { href: '/medications/log', icon: '📊', label: '복용 기록', description: '약 복용 이행 현황' },
    ],
  },
  {
    title: '생활 서비스',
    items: [
      { href: '/services/rides', icon: '🚐', label: '교통 서비스', description: '병원·외출 이동 지원' },
      { href: '/services/meals', icon: '🍱', label: '식사 배달', description: '건강 도시락 주문' },
      { href: '/services/partners', icon: '🤝', label: '파트너 서비스', description: '협력 기관 안내' },
      { href: '/housing', icon: '🏠', label: '주거·실버타운', description: '노인 주거 시설 탐색' },
      { href: '/opportunities', icon: '💼', label: '일자리·봉사', description: '취업·봉사·커뮤니티' },
    ],
  },
  {
    title: '계정 및 설정',
    items: [
      { href: '/profile', icon: '👤', label: '내 프로필', description: '개인정보 및 건강 기본값' },
      { href: '/consent', icon: '🔒', label: '동의 관리', description: '개인정보 동의 설정' },
      { href: '/notifications', icon: '🔔', label: '알림', description: '알림 목록 및 설정' },
      { href: '/settings', icon: '⚙️', label: '앱 설정', description: '글자 크기, 고대비, 언어' },
    ],
  },
];

export default function MorePage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-5">더보기</h1>

        {/* Profile quick card */}
        <Link href="/profile" className="senior-card flex items-center gap-4 mb-5 hover:shadow-md transition-shadow">
          <div className="w-16 h-16 rounded-full bg-primary-100 flex items-center justify-center text-3xl flex-shrink-0" aria-hidden="true">
            👵
          </div>
          <div>
            <p className="text-senior-xl font-bold text-gray-900">김영자 님</p>
            <p className="text-senior-sm text-gray-500">75세 · 장기요양 3등급</p>
            <p className="text-senior-sm text-primary-600 font-medium mt-0.5">프로필 보기 →</p>
          </div>
        </Link>

        {/* Emergency button — always prominent */}
        <Link
          href="/emergency"
          className="w-full bg-danger-500 hover:bg-danger-700 text-white rounded-2xl px-5 py-4 flex items-center gap-4 mb-5 shadow-md transition-colors"
          aria-label="긴급 연락 페이지"
        >
          <span className="text-3xl" aria-hidden="true">🚨</span>
          <div>
            <p className="text-senior-xl font-black">긴급 연락</p>
            <p className="text-senior-sm text-red-100">119, 112, 보호자에게 바로 연결</p>
          </div>
          <svg className="w-6 h-6 ml-auto opacity-80" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
          </svg>
        </Link>

        {/* Sectioned navigation */}
        {MORE_SECTIONS.map((section) => (
          <section key={section.title} className="mb-5" aria-labelledby={`section-${section.title}`}>
            <h2 id={`section-${section.title}`} className="senior-section-title">{section.title}</h2>
            <div className="space-y-2">
              {section.items.map((item) => (
                <Link
                  key={item.href}
                  href={item.href}
                  className="senior-card flex items-center gap-4 hover:shadow-md active:scale-[0.99] transition-all"
                >
                  <span className="text-2xl flex-shrink-0" aria-hidden="true">{item.icon}</span>
                  <div className="flex-1">
                    <p className="text-senior-base font-bold text-gray-800">{item.label}</p>
                    <p className="text-senior-sm text-gray-500">{item.description}</p>
                  </div>
                  <svg className="w-5 h-5 text-gray-400 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                  </svg>
                </Link>
              ))}
            </div>
          </section>
        ))}

        {/* App version */}
        <p className="text-center text-senior-sm text-gray-400 pb-4">
          시니어 포털 v1.0.0
        </p>
      </div>
    </SeniorAppShell>
  );
}
