// 서비스 디렉토리 — Services Directory
// Hub page linking to rides, meals, and partner services

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const SERVICE_CATEGORIES = [
  {
    href: '/services/rides',
    title: '교통 서비스',
    subtitle: '병원·외출 이동 지원',
    icon: '🚐',
    bg: 'bg-primary-50',
    border: 'border-primary-200',
    text: 'text-primary-700',
    badge: '이용 가능',
    badgeColor: 'bg-primary-100 text-primary-700',
  },
  {
    href: '/services/meals',
    title: '식사 배달',
    subtitle: '건강 도시락 및 밑반찬',
    icon: '🍱',
    bg: 'bg-secondary-50',
    border: 'border-secondary-200',
    text: 'text-secondary-700',
    badge: '주문 가능',
    badgeColor: 'bg-secondary-100 text-secondary-700',
  },
  {
    href: '/services/partners',
    title: '파트너 서비스',
    subtitle: '전문 협력 기관 안내',
    icon: '🤝',
    bg: 'bg-warning-50',
    border: 'border-warning-200',
    text: 'text-warning-700',
    badge: '신규 파트너',
    badgeColor: 'bg-warning-100 text-warning-700',
  },
];

const RECENT_BOOKINGS = [
  {
    id: 'b-1',
    type: '교통',
    icon: '🚐',
    title: '병원 이동 — 서울 중앙 의원',
    date: '3월 15일 오전 10:00',
    status: '예정',
    statusColor: 'bg-primary-100 text-primary-700',
  },
  {
    id: 'b-2',
    type: '식사',
    icon: '🍱',
    title: '한식 건강 도시락 (3일분)',
    date: '3월 14일 오전 11:30',
    status: '배달 완료',
    statusColor: 'bg-success-50 text-success-700',
  },
];

const SERVICE_NOTICES = [
  '🎉 3월 15일부터 병원 셔틀 노선이 확대됩니다.',
  '🍽 주말 특선 도시락 메뉴가 업데이트됐습니다.',
];

export default function ServicesPage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-5">서비스</h1>

        {/* Service notices */}
        <div className="senior-card mb-5 space-y-2">
          <p className="text-senior-sm font-bold text-gray-600 mb-1">📢 공지사항</p>
          {SERVICE_NOTICES.map((notice, i) => (
            <p key={i} className="text-senior-sm text-gray-700 leading-relaxed">{notice}</p>
          ))}
        </div>

        {/* Category cards */}
        <section aria-labelledby="categories-heading" className="mb-6">
          <h2 id="categories-heading" className="senior-section-title">서비스 카테고리</h2>
          <div className="space-y-3">
            {SERVICE_CATEGORIES.map((cat) => (
              <Link
                key={cat.href}
                href={cat.href}
                className={`${cat.bg} ${cat.border} border-2 rounded-2xl p-5 flex items-center gap-4 hover:shadow-md active:scale-[0.99] transition-all block`}
              >
                <span className="text-4xl flex-shrink-0" aria-hidden="true">{cat.icon}</span>
                <div className="flex-1">
                  <p className={`text-senior-xl font-bold ${cat.text}`}>{cat.title}</p>
                  <p className="text-senior-sm text-gray-600 mt-0.5">{cat.subtitle}</p>
                </div>
                <div className="flex flex-col items-end gap-2">
                  <span className={`${cat.badgeColor} text-xs font-semibold px-2 py-1 rounded-full`}>
                    {cat.badge}
                  </span>
                  <svg className="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                  </svg>
                </div>
              </Link>
            ))}
          </div>
        </section>

        {/* Recent bookings */}
        <section aria-labelledby="recent-bookings-heading">
          <h2 id="recent-bookings-heading" className="senior-section-title">최근 이용 내역</h2>
          <div className="space-y-3">
            {RECENT_BOOKINGS.map((booking) => (
              <div key={booking.id} className="senior-card flex items-center gap-4">
                <span className="text-3xl flex-shrink-0" aria-hidden="true">{booking.icon}</span>
                <div className="flex-1">
                  <p className="text-senior-base font-bold text-gray-800">{booking.title}</p>
                  <p className="text-senior-sm text-gray-500">{booking.date}</p>
                </div>
                <span className={`${booking.statusColor} text-senior-sm font-semibold px-2.5 py-1 rounded-full flex-shrink-0`}>
                  {booking.status}
                </span>
              </div>
            ))}
          </div>
        </section>
      </div>
    </SeniorAppShell>
  );
}
