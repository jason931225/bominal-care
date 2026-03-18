// 파트너 서비스 디렉토리 — Partner Services Directory
// Lists partner agencies offering health, welfare, and daily life services

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

type PartnerCategory = '의료' | '복지' | '법률' | '금융' | '여가' | '일자리';

interface Partner {
  id: string;
  name: string;
  category: PartnerCategory;
  description: string;
  services: string[];
  phone: string;
  address: string;
  hours: string;
  subsidy: boolean;
}

const PARTNERS: Partner[] = [
  {
    id: 'p-1',
    name: '서울 노인 복지관',
    category: '복지',
    description: '종합 사회복지 서비스 제공 — 방문상담, 프로그램 운영',
    services: ['상담 서비스', '건강 프로그램', '문화·여가', '식사 제공'],
    phone: '02-1234-6000',
    address: '서울시 중구 을지로 200',
    hours: '월~금 오전 9시 ~ 오후 6시',
    subsidy: true,
  },
  {
    id: 'p-2',
    name: '한국 노인 법률지원센터',
    category: '법률',
    description: '상속, 부동산, 재산 관리 등 법률 무료 상담',
    services: ['유언·상속', '부동산 상담', '소비자 피해', '금융 사기 예방'],
    phone: '1670-3500',
    address: '서울시 서초구 법원로 15',
    hours: '월~금 오전 10시 ~ 오후 5시',
    subsidy: true,
  },
  {
    id: 'p-3',
    name: '시니어 건강 검진 센터',
    category: '의료',
    description: '노인 맞춤 종합 건강 검진 (할인 적용)',
    services: ['혈액 검사', '심전도', '골밀도 측정', '암 검진 패키지'],
    phone: '1600-7788',
    address: '서울시 강남구 테헤란로 50',
    hours: '월~토 오전 8시 ~ 오후 5시',
    subsidy: false,
  },
  {
    id: 'p-4',
    name: '시니어 금융 도우미',
    category: '금융',
    description: '연금, 복지 급여, 재산 관리 상담 지원',
    services: ['국민연금 상담', '기초연금 신청', '금융 사기 신고', '재산 목록 정리'],
    phone: '1588-2100',
    address: '전화 상담 가능',
    hours: '월~금 오전 9시 ~ 오후 6시',
    subsidy: true,
  },
  {
    id: 'p-5',
    name: '시니어 문화 여가 센터',
    category: '여가',
    description: '취미, 여행, 문화 활동 프로그램 운영',
    services: ['서예·그림', '원예치료', '영화 관람', '국내 여행 패키지'],
    phone: '02-9876-5432',
    address: '서울시 마포구 성미산로 100',
    hours: '화~일 오전 10시 ~ 오후 8시',
    subsidy: false,
  },
];

const CATEGORY_COLORS: Record<PartnerCategory, string> = {
  '의료': 'bg-danger-50 text-danger-700',
  '복지': 'bg-secondary-100 text-secondary-700',
  '법률': 'bg-primary-100 text-primary-700',
  '금융': 'bg-warning-50 text-warning-700',
  '여가': 'bg-success-50 text-success-700',
  '일자리': 'bg-info-50 text-info-700',
};

const CATEGORY_ICONS: Record<PartnerCategory, string> = {
  '의료': '🏥',
  '복지': '🏛',
  '법률': '⚖️',
  '금융': '💰',
  '여가': '🎨',
  '일자리': '💼',
};

export default function PartnersPage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        <Link
          href="/services"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          서비스로
        </Link>

        <h1 className="text-senior-2xl font-bold text-gray-900 mb-2">파트너 서비스 🤝</h1>
        <p className="text-senior-base text-gray-500 mb-5">신뢰할 수 있는 협력 기관 안내</p>

        {/* Category quick filter row */}
        <div className="flex gap-2 mb-5 overflow-x-auto pb-1" aria-label="파트너 카테고리">
          {(Object.entries(CATEGORY_ICONS) as [PartnerCategory, string][]).map(([cat, icon]) => (
            <span key={cat} className={`${CATEGORY_COLORS[cat]} flex-shrink-0 text-senior-sm font-semibold px-3 py-1.5 rounded-full`}>
              {icon} {cat}
            </span>
          ))}
        </div>

        {/* Partner cards */}
        <div className="space-y-4">
          {PARTNERS.map((partner) => {
            const catColor = CATEGORY_COLORS[partner.category];
            const catIcon = CATEGORY_ICONS[partner.category];

            return (
              <div key={partner.id} className="senior-card">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <span className="text-2xl" aria-hidden="true">{catIcon}</span>
                    <div>
                      <p className="text-senior-xl font-bold text-gray-900">{partner.name}</p>
                      <span className={`${catColor} text-senior-sm font-semibold px-2 py-0.5 rounded-full`}>
                        {partner.category}
                      </span>
                    </div>
                  </div>
                  {partner.subsidy && (
                    <span className="bg-success-50 text-success-700 text-senior-sm font-bold px-2 py-1 rounded-full flex-shrink-0 ml-2">
                      무료·지원
                    </span>
                  )}
                </div>

                <p className="text-senior-base text-gray-700 mb-3">{partner.description}</p>

                {/* Services list */}
                <div className="flex flex-wrap gap-1.5 mb-3">
                  {partner.services.map((svc) => (
                    <span key={svc} className="bg-gray-100 text-gray-600 text-senior-sm px-2.5 py-0.5 rounded-full">
                      {svc}
                    </span>
                  ))}
                </div>

                {/* Contact info */}
                <div className="space-y-1.5 text-senior-sm text-gray-600 border-t border-gray-100 pt-3">
                  <div className="flex items-center gap-2">
                    <span aria-hidden="true">📍</span>
                    <span>{partner.address}</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span aria-hidden="true">🕐</span>
                    <span>{partner.hours}</span>
                  </div>
                  <a
                    href={`tel:${partner.phone}`}
                    className="flex items-center gap-2 text-primary-600 font-semibold min-h-touch"
                  >
                    <span aria-hidden="true">📞</span>
                    <span>{partner.phone}</span>
                  </a>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </SeniorAppShell>
  );
}
