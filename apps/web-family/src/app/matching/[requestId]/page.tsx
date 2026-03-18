import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const REQUEST_DATA = {
  id: 'req-002',
  title: '물리치료 서비스 요청 #002',
  status: 'reviewing',
  statusLabel: '추천 검토 중',
  serviceType: '물리치료',
  schedule: '주 4회 (월/화/목/금) 오후 2시',
  duration: '60분/회',
  notes: '낙상 후 재활 목적. 하지 근력 강화 및 보행 훈련 필요. 고혈압 주의.',
  createdAt: '2026-03-13',
  budget: '월 300,000원 이내',
};

const PROVIDERS = [
  {
    id: 'prov-01',
    name: '김도현',
    credential: '물리치료사 면허 보유',
    experience: '경력 8년',
    rating: 4.9,
    reviewCount: 127,
    distance: '2.3km',
    availability: '즉시 가능',
    specialties: ['낙상 후 재활', '파킨슨병', '뇌졸중 재활'],
    price: '회당 45,000원',
    matchScore: 97,
    photo: '👨‍⚕️',
  },
  {
    id: 'prov-02',
    name: '이혜진',
    credential: '물리치료사 면허 보유',
    experience: '경력 5년',
    rating: 4.7,
    reviewCount: 84,
    distance: '1.1km',
    availability: '3월 18일부터',
    specialties: ['고령자 재활', '근골격계', '통증 관리'],
    price: '회당 42,000원',
    matchScore: 91,
    photo: '👩‍⚕️',
  },
  {
    id: 'prov-03',
    name: '박성민',
    credential: '물리치료사 면허 보유',
    experience: '경력 12년',
    rating: 4.8,
    reviewCount: 203,
    distance: '4.0km',
    availability: '즉시 가능',
    specialties: ['노인 재활', '낙상 예방', '보행 훈련', '신경계 재활'],
    price: '회당 50,000원',
    matchScore: 88,
    photo: '👨‍⚕️',
  },
  {
    id: 'prov-04',
    name: '최유나',
    credential: '물리치료사 면허 보유',
    experience: '경력 3년',
    rating: 4.5,
    reviewCount: 41,
    distance: '0.8km',
    availability: '즉시 가능',
    specialties: ['고령자 재활', '하지 근력'],
    price: '회당 38,000원',
    matchScore: 82,
    photo: '👩‍⚕️',
  },
];

export default function MatchRequestDetailPage({
  params: _params,
}: {
  params: Promise<{ requestId: string }>;
}) {
  return (
    <FamilyAppShell>
      <div className="max-w-4xl mx-auto px-4 py-6">
        {/* Breadcrumb */}
        <nav className="flex items-center gap-2 text-sm text-gray-500 mb-4">
          <Link href="/matching" className="hover:text-blue-600">매칭 요청</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">{REQUEST_DATA.title}</span>
        </nav>

        {/* Request Summary */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-6">
          <div className="flex items-start justify-between gap-3 mb-4">
            <h1 className="text-xl font-bold text-gray-900">{REQUEST_DATA.title}</h1>
            <span className="flex-shrink-0 text-sm font-semibold px-3 py-1 bg-blue-50 text-blue-700 border border-blue-200 rounded-full">
              {REQUEST_DATA.statusLabel}
            </span>
          </div>
          <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-3">
            {[
              { label: '서비스 유형', value: REQUEST_DATA.serviceType },
              { label: '스케줄', value: REQUEST_DATA.schedule },
              { label: '소요 시간', value: REQUEST_DATA.duration },
              { label: '예산', value: REQUEST_DATA.budget },
              { label: '요청일', value: REQUEST_DATA.createdAt },
            ].map((item) => (
              <div key={item.label} className="bg-gray-50 rounded-lg p-3">
                <p className="text-xs text-gray-400">{item.label}</p>
                <p className="text-sm font-medium text-gray-800 mt-0.5">{item.value}</p>
              </div>
            ))}
          </div>
          {REQUEST_DATA.notes && (
            <div className="mt-3 p-3 bg-amber-50 border border-amber-200 rounded-lg">
              <p className="text-xs font-semibold text-amber-700 mb-1">특이사항</p>
              <p className="text-sm text-amber-800">{REQUEST_DATA.notes}</p>
            </div>
          )}
        </div>

        {/* Recommendations */}
        <div className="mb-4">
          <h2 className="text-lg font-bold text-gray-900 mb-1">
            추천 제공자 <span className="text-blue-600">{PROVIDERS.length}명</span>
          </h2>
          <p className="text-sm text-gray-500">AI 매칭 점수 순으로 정렬되었습니다</p>
        </div>

        <div className="space-y-4">
          {PROVIDERS.map((provider, idx) => (
            <div
              key={provider.id}
              className={`bg-white border rounded-xl p-5 ${idx === 0 ? 'border-blue-300 shadow-md' : 'border-gray-200'}`}
            >
              {idx === 0 && (
                <div className="flex items-center gap-1.5 mb-3">
                  <span className="text-yellow-400">⭐</span>
                  <span className="text-xs font-bold text-yellow-700 bg-yellow-50 px-2 py-0.5 rounded-full border border-yellow-200">
                    최고 추천
                  </span>
                </div>
              )}
              <div className="flex items-start gap-4">
                <span className="text-4xl">{provider.photo}</span>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center justify-between gap-2 flex-wrap">
                    <div>
                      <h3 className="font-bold text-gray-900 text-base">{provider.name}</h3>
                      <p className="text-xs text-gray-500">{provider.credential} · {provider.experience}</p>
                    </div>
                    <div className="text-right flex-shrink-0">
                      <div className="text-lg font-bold text-blue-600">{provider.matchScore}점</div>
                      <div className="text-xs text-gray-400">매칭 점수</div>
                    </div>
                  </div>

                  <div className="flex items-center gap-3 mt-2 flex-wrap">
                    <span className="text-sm font-semibold text-yellow-600">★ {provider.rating}</span>
                    <span className="text-xs text-gray-400">({provider.reviewCount}개 리뷰)</span>
                    <span className="text-xs text-gray-500">📍 {provider.distance}</span>
                    <span className={`text-xs font-medium px-2 py-0.5 rounded-full ${
                      provider.availability === '즉시 가능'
                        ? 'bg-green-50 text-green-700 border border-green-200'
                        : 'bg-gray-100 text-gray-600 border border-gray-200'
                    }`}>
                      {provider.availability}
                    </span>
                  </div>

                  <div className="flex flex-wrap gap-1.5 mt-2">
                    {provider.specialties.map((s) => (
                      <span key={s} className="text-xs px-2 py-0.5 bg-blue-50 text-blue-700 rounded-full border border-blue-200">
                        {s}
                      </span>
                    ))}
                  </div>

                  <div className="flex items-center justify-between mt-3 gap-3">
                    <span className="text-sm font-bold text-gray-900">{provider.price}</span>
                    <div className="flex gap-2">
                      <Link
                        href={`/matching/providers/${provider.id}`}
                        className="px-3 py-1.5 border border-gray-300 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-50 transition-colors"
                      >
                        프로필 보기
                      </Link>
                      <button className="px-3 py-1.5 bg-blue-600 text-white text-sm font-semibold rounded-lg hover:bg-blue-700 transition-colors">
                        선택
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </FamilyAppShell>
  );
}
