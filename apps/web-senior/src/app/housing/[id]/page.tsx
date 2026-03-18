// 주거 시설 상세 — Housing Detail Page

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

const MOCK_HOUSING = {
  id: 'h-1',
  name: '서울 시니어스 타워',
  type: '실버타운',
  location: '서울 강남구 대치동',
  fullAddress: '서울특별시 강남구 대치동 1234-5 시니어스 타워 3~15층',
  phone: '02-5555-1234',
  website: 'https://example.com',
  monthlyFee: 1_800_000,
  deposit: 100_000_000,
  capacity: 200,
  available: 12,
  rating: 4.8,
  description: '강남 중심부에 위치한 프리미엄 실버타운입니다. 전문 의료진이 상주하며 24시간 건강 관리 서비스를 제공합니다. 독립적인 생활이 가능한 어르신부터 돌봄이 필요한 어르신까지 맞춤형 서비스를 제공합니다.',
  roomTypes: [
    { type: '1인실 (33㎡)', fee: 1_800_000, available: 5 },
    { type: '1.5인실 (49㎡)', fee: 2_400_000, available: 4 },
    { type: '2인실 (66㎡)', fee: 3_200_000, available: 3 },
  ],
  features: [
    { icon: '🏥', name: '의료 서비스', detail: '내과·재활의학과 전문의 주 3회 방문' },
    { icon: '🏊', name: '수영장', detail: '온수 수영장 — 수중 운동 프로그램 포함' },
    { icon: '📚', name: '도서관', detail: '1만권 이상 소장, 전자책 열람 가능' },
    { icon: '🌿', name: '채소 정원', detail: '옥상 텃밭 — 직접 재배 및 요리 체험' },
    { icon: '🧠', name: '인지 프로그램', detail: '치매 예방 프로그램 주 5회 운영' },
    { icon: '🍽', name: '식당', detail: '1일 3식 제공 (영양사 설계 식단)' },
  ],
  certifications: ['노인복지법 인가', '보건복지부 우수 시설 인증 (2025)', 'ISO 9001 품질경영인증'],
  nearbyFacilities: ['강남 세브란스 병원 (700m)', '대치동 마트 (300m)', '강남역 지하철 (1.2km)'],
};

interface PageProps {
  params: Promise<{ id: string }>;
}

export default async function HousingDetailPage({ params: _params }: PageProps) {
  const housing = MOCK_HOUSING;

  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Back */}
        <Link
          href="/housing"
          className="inline-flex items-center gap-2 text-primary-600 font-medium text-senior-base mb-5 min-h-touch"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
          주거 목록으로
        </Link>

        {/* Header */}
        <div className="senior-card mb-4">
          <div className="flex items-start justify-between mb-3">
            <div>
              <span className="bg-primary-100 text-primary-700 text-senior-sm font-bold px-2 py-0.5 rounded-full">
                {housing.type}
              </span>
              <h1 className="text-senior-2xl font-bold text-gray-900 mt-2">{housing.name}</h1>
              <p className="text-senior-base text-gray-600">{housing.location}</p>
            </div>
            <div className="text-right flex-shrink-0 ml-3">
              <div className="flex items-center gap-1 justify-end mb-1">
                <svg className="w-4 h-4 text-warning-500" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                </svg>
                <span className="text-senior-base font-bold text-gray-800">{housing.rating}</span>
              </div>
              <span className="bg-success-50 text-success-700 text-senior-sm font-bold px-2.5 py-1 rounded-full">
                입소 가능 {housing.available}실
              </span>
            </div>
          </div>
          <p className="text-senior-base text-gray-700 leading-relaxed">{housing.description}</p>
        </div>

        {/* Room types and fees */}
        <section className="senior-card mb-4" aria-labelledby="room-types-heading">
          <h2 id="room-types-heading" className="text-senior-lg font-bold text-gray-800 mb-3">객실 유형 및 요금</h2>
          <div className="space-y-2">
            {housing.roomTypes.map((room, i) => (
              <div key={i} className="flex items-center justify-between p-3 bg-gray-50 rounded-xl">
                <div>
                  <p className="text-senior-base font-semibold text-gray-800">{room.type}</p>
                  <p className={`text-senior-sm ${room.available > 0 ? 'text-success-700' : 'text-gray-400'}`}>
                    {room.available > 0 ? `${room.available}실 가능` : '대기'}
                  </p>
                </div>
                <p className="text-senior-lg font-bold text-primary-700">
                  {room.fee.toLocaleString('ko-KR')}원/월
                </p>
              </div>
            ))}
          </div>
          {housing.deposit && (
            <p className="mt-2 text-senior-sm text-gray-500">
              * 보증금: {housing.deposit.toLocaleString('ko-KR')}원 별도
            </p>
          )}
        </section>

        {/* Features */}
        <section className="senior-card mb-4" aria-labelledby="features-heading">
          <h2 id="features-heading" className="text-senior-lg font-bold text-gray-800 mb-3">시설 · 서비스</h2>
          <div className="space-y-3">
            {housing.features.map((feature, i) => (
              <div key={i} className="flex items-start gap-3">
                <span className="text-2xl flex-shrink-0" aria-hidden="true">{feature.icon}</span>
                <div>
                  <p className="text-senior-base font-semibold text-gray-800">{feature.name}</p>
                  <p className="text-senior-sm text-gray-500">{feature.detail}</p>
                </div>
              </div>
            ))}
          </div>
        </section>

        {/* Certifications */}
        <section className="senior-card mb-4" aria-labelledby="certs-heading">
          <h2 id="certs-heading" className="text-senior-lg font-bold text-gray-800 mb-2">인증 · 허가</h2>
          <div className="flex flex-wrap gap-2">
            {housing.certifications.map((cert, i) => (
              <span key={i} className="bg-success-50 text-success-700 text-senior-sm font-medium px-3 py-1 rounded-full">
                ✓ {cert}
              </span>
            ))}
          </div>
        </section>

        {/* Nearby */}
        <section className="senior-card mb-5" aria-labelledby="nearby-heading">
          <h2 id="nearby-heading" className="text-senior-lg font-bold text-gray-800 mb-2">주변 편의시설</h2>
          <ul className="space-y-1">
            {housing.nearbyFacilities.map((f, i) => (
              <li key={i} className="flex items-center gap-2 text-senior-base text-gray-700">
                <span aria-hidden="true">📍</span>
                {f}
              </li>
            ))}
          </ul>
        </section>

        {/* CTA */}
        <div className="space-y-3">
          <a href={`tel:${housing.phone}`} className="senior-btn-primary w-full">
            📞 {housing.phone} 상담 전화
          </a>
          <Link href="/housing" className="senior-btn-secondary w-full">
            목록으로 돌아가기
          </Link>
        </div>
      </div>
    </SeniorAppShell>
  );
}
