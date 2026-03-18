// 주거 / 실버타운 — Housing / Silver Town Search
// Browse senior housing options including silver towns and care facilities

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

type HousingType = '실버타운' | '요양원' | '노인복지주택' | '공공임대';

interface HousingItem {
  id: string;
  name: string;
  type: HousingType;
  location: string;
  district: string;
  monthlyFee: number;
  deposit?: number;
  capacity: number;
  available: number;
  features: string[];
  phone: string;
  rating: number;
  certifications: string[];
}

const HOUSING_LIST: HousingItem[] = [
  {
    id: 'h-1',
    name: '서울 시니어스 타워',
    type: '실버타운',
    location: '서울 강남구 대치동',
    district: '강남구',
    monthlyFee: 1_800_000,
    deposit: 100_000_000,
    capacity: 200,
    available: 12,
    features: ['의료 서비스 제공', '수영장', '도서관', '채소 정원', '치매 예방 프로그램'],
    phone: '02-5555-1234',
    rating: 4.8,
    certifications: ['노인복지법 인가', '우수 시설 인증'],
  },
  {
    id: 'h-2',
    name: '한강뷰 시니어 빌리지',
    type: '노인복지주택',
    location: '서울 마포구 합정동',
    district: '마포구',
    monthlyFee: 1_200_000,
    deposit: 50_000_000,
    capacity: 80,
    available: 3,
    features: ['한강 조망', '밑반찬 서비스', '1인 세대 특화', '커뮤니티 라운지'],
    phone: '02-3456-7890',
    rating: 4.5,
    certifications: ['노인복지법 인가'],
  },
  {
    id: 'h-3',
    name: '행복 노인 요양원',
    type: '요양원',
    location: '경기 성남시 분당구',
    district: '성남시',
    monthlyFee: 1_500_000,
    capacity: 60,
    available: 8,
    features: ['24시간 간호 서비스', '작업치료', '물리치료', '종교 활동 지원'],
    phone: '031-7777-8888',
    rating: 4.6,
    certifications: ['요양기관 인증', '3년 연속 우수 기관'],
  },
  {
    id: 'h-4',
    name: '은빛 공공임대 아파트',
    type: '공공임대',
    location: '서울 노원구 상계동',
    district: '노원구',
    monthlyFee: 300_000,
    deposit: 5_000_000,
    capacity: 150,
    available: 0,
    features: ['저렴한 임대료', '복지 연계 서비스', '경비 24시간', '무장애 설계'],
    phone: '1600-1004',
    rating: 4.2,
    certifications: ['LH 공공임대'],
  },
];

const TYPE_COLORS: Record<HousingType, string> = {
  '실버타운': 'bg-primary-100 text-primary-700',
  '요양원': 'bg-secondary-100 text-secondary-700',
  '노인복지주택': 'bg-warning-50 text-warning-700',
  '공공임대': 'bg-success-50 text-success-700',
};

function RatingStars({ rating }: { rating: number }) {
  return (
    <span className="flex items-center gap-0.5" aria-label={`평점 ${rating}`}>
      {[1, 2, 3, 4, 5].map((star) => (
        <svg
          key={star}
          className={`w-4 h-4 ${star <= Math.round(rating) ? 'text-warning-500' : 'text-gray-300'}`}
          fill="currentColor"
          viewBox="0 0 20 20"
          aria-hidden="true"
        >
          <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
        </svg>
      ))}
      <span className="text-senior-sm font-medium text-gray-600 ml-1">{rating}</span>
    </span>
  );
}

export default function HousingPage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-2">주거 · 실버타운</h1>
        <p className="text-senior-base text-gray-500 mb-5">시니어 맞춤 주거 시설 안내</p>

        {/* Tip banner */}
        <div className="bg-info-50 border border-info-200 rounded-2xl p-4 mb-5">
          <p className="text-senior-base font-bold text-info-700 mb-1">💡 이용 안내</p>
          <p className="text-senior-sm text-info-700">장기요양 등급을 보유하고 계시면 요양원 입소 비용 지원을 받으실 수 있습니다. 케어매니저에게 문의하세요.</p>
        </div>

        {/* Housing list */}
        <div className="space-y-4">
          {HOUSING_LIST.map((item) => (
            <Link
              key={item.id}
              href={`/housing/${item.id}`}
              className="senior-card block hover:shadow-md active:scale-[0.99] transition-all"
            >
              {/* Header */}
              <div className="flex items-start justify-between mb-2">
                <div>
                  <div className="flex items-center gap-2 mb-1">
                    <span className={`${TYPE_COLORS[item.type]} text-senior-sm font-bold px-2 py-0.5 rounded-full`}>
                      {item.type}
                    </span>
                    {item.available === 0 && (
                      <span className="bg-gray-100 text-gray-500 text-senior-sm font-semibold px-2 py-0.5 rounded-full">
                        대기 필요
                      </span>
                    )}
                  </div>
                  <p className="text-senior-xl font-bold text-gray-900">{item.name}</p>
                  <p className="text-senior-sm text-gray-500">{item.location}</p>
                </div>
              </div>

              {/* Rating and availability */}
              <div className="flex items-center justify-between mb-3">
                <RatingStars rating={item.rating} />
                <span className={`text-senior-sm font-semibold ${item.available > 0 ? 'text-success-700' : 'text-gray-400'}`}>
                  {item.available > 0 ? `입소 가능 ${item.available}실` : '현재 만실'}
                </span>
              </div>

              {/* Fee */}
              <div className="bg-gray-50 rounded-xl p-3 mb-3">
                <div className="flex justify-between">
                  <span className="text-senior-sm text-gray-500">월 이용료</span>
                  <span className="text-senior-base font-bold text-gray-800">
                    {item.monthlyFee.toLocaleString('ko-KR')}원
                  </span>
                </div>
                {item.deposit !== undefined && (
                  <div className="flex justify-between mt-1">
                    <span className="text-senior-sm text-gray-500">보증금</span>
                    <span className="text-senior-sm font-semibold text-gray-700">
                      {item.deposit.toLocaleString('ko-KR')}원
                    </span>
                  </div>
                )}
              </div>

              {/* Features */}
              <div className="flex flex-wrap gap-1.5">
                {item.features.slice(0, 3).map((f) => (
                  <span key={f} className="bg-gray-100 text-gray-600 text-senior-sm px-2 py-0.5 rounded-full">
                    {f}
                  </span>
                ))}
                {item.features.length > 3 && (
                  <span className="text-senior-sm text-primary-600 font-medium">+{item.features.length - 3}개</span>
                )}
              </div>
            </Link>
          ))}
        </div>
      </div>
    </SeniorAppShell>
  );
}
