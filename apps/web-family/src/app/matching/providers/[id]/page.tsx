import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const PROVIDER = {
  id: 'prov-01',
  name: '김도현',
  photo: '👨‍⚕️',
  credential: '물리치료사 1급 면허 보유',
  licenseNo: 'PT-2018-041283',
  experience: '경력 8년',
  rating: 4.9,
  reviewCount: 127,
  distance: '2.3km',
  location: '서울 강남구',
  availability: '즉시 가능',
  specialties: ['낙상 후 재활', '파킨슨병', '뇌졸중 재활', '고령자 근력 강화', '보행 훈련'],
  price: '회당 45,000원',
  bio: '8년간 노인 재활 전문 물리치료사로 활동하며 200명 이상의 어르신을 담당하였습니다. 낙상 후 재활 및 보행 능력 회복에 특화되어 있으며, 환자 및 가족과의 소통을 중요시합니다.',
  workHours: '월~토 오전 9시 ~ 오후 6시',
  languages: ['한국어'],
  certifications: [
    { name: '노인 재활 전문가 과정 수료', year: '2021' },
    { name: '낙상 예방 프로그램 지도사', year: '2020' },
    { name: '신경계 재활 전문 교육 이수', year: '2019' },
  ],
  reviews: [
    {
      id: 1,
      author: '김**',
      rating: 5,
      date: '2026-02-20',
      text: '어머니 낙상 후 재활 치료를 받았는데 정말 친절하고 전문적이었습니다. 3개월 만에 보행 능력이 많이 회복되었어요.',
    },
    {
      id: 2,
      author: '이**',
      rating: 5,
      date: '2026-01-15',
      text: '설명을 자세히 해주시고, 가족에게도 가정 운동법을 가르쳐 주셨습니다. 매우 만족합니다.',
    },
    {
      id: 3,
      author: '박**',
      rating: 4,
      date: '2025-12-30',
      text: '전문성과 친절함이 좋았습니다. 시간 약속을 잘 지켜주셔서 신뢰가 갑니다.',
    },
  ],
};

export default function ProviderDetailPage({
  params: _params,
}: {
  params: Promise<{ id: string }>;
}) {
  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Breadcrumb */}
        <nav className="flex items-center gap-2 text-sm text-gray-500 mb-4 flex-wrap">
          <Link href="/matching" className="hover:text-blue-600">매칭 요청</Link>
          <span>/</span>
          <Link href="/matching/req-002" className="hover:text-blue-600">요청 #002</Link>
          <span>/</span>
          <span className="text-gray-900 font-medium">{PROVIDER.name}</span>
        </nav>

        {/* Profile Header */}
        <div className="bg-white border border-gray-200 rounded-xl p-6 mb-4">
          <div className="flex items-start gap-5">
            <span className="text-6xl">{PROVIDER.photo}</span>
            <div className="flex-1">
              <h1 className="text-2xl font-bold text-gray-900">{PROVIDER.name}</h1>
              <p className="text-sm text-gray-600 mt-0.5">{PROVIDER.credential}</p>
              <p className="text-xs text-gray-400">면허번호: {PROVIDER.licenseNo}</p>

              <div className="flex items-center gap-3 mt-3 flex-wrap">
                <span className="text-base font-bold text-yellow-500">★ {PROVIDER.rating}</span>
                <span className="text-sm text-gray-500">({PROVIDER.reviewCount}개 리뷰)</span>
                <span className="text-sm text-gray-500">📍 {PROVIDER.location}</span>
                <span className="text-sm text-green-600 font-medium bg-green-50 px-2 py-0.5 rounded-full border border-green-200">
                  {PROVIDER.availability}
                </span>
              </div>

              <div className="flex flex-wrap gap-1.5 mt-3">
                {PROVIDER.specialties.map((s) => (
                  <span key={s} className="text-xs px-2 py-0.5 bg-blue-50 text-blue-700 rounded-full border border-blue-200">
                    {s}
                  </span>
                ))}
              </div>
            </div>
          </div>

          <div className="grid sm:grid-cols-3 gap-3 mt-5 pt-4 border-t border-gray-100">
            <div className="text-center">
              <p className="text-lg font-bold text-gray-900">{PROVIDER.experience}</p>
              <p className="text-xs text-gray-500">경력</p>
            </div>
            <div className="text-center">
              <p className="text-lg font-bold text-blue-600">{PROVIDER.price}</p>
              <p className="text-xs text-gray-500">서비스 비용</p>
            </div>
            <div className="text-center">
              <p className="text-lg font-bold text-gray-900">{PROVIDER.distance}</p>
              <p className="text-xs text-gray-500">거리</p>
            </div>
          </div>
        </div>

        {/* Bio */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-2">소개</h2>
          <p className="text-sm text-gray-600 leading-relaxed">{PROVIDER.bio}</p>
        </div>

        {/* Schedule & Info */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-3">근무 정보</h2>
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-gray-500">근무 시간</span>
              <span className="font-medium text-gray-800">{PROVIDER.workHours}</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-gray-500">사용 언어</span>
              <span className="font-medium text-gray-800">{PROVIDER.languages.join(', ')}</span>
            </div>
          </div>
        </div>

        {/* Certifications */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-4">
          <h2 className="font-bold text-gray-900 mb-3">자격 및 교육</h2>
          <ul className="space-y-2">
            {PROVIDER.certifications.map((cert) => (
              <li key={cert.name} className="flex items-start gap-2 text-sm">
                <span className="text-green-500 flex-shrink-0 mt-0.5">✓</span>
                <span className="text-gray-700">{cert.name}</span>
                <span className="text-gray-400 ml-auto flex-shrink-0">{cert.year}</span>
              </li>
            ))}
          </ul>
        </div>

        {/* Reviews */}
        <div className="bg-white border border-gray-200 rounded-xl p-5 mb-6">
          <h2 className="font-bold text-gray-900 mb-3">이용자 리뷰</h2>
          <div className="space-y-4">
            {PROVIDER.reviews.map((review) => (
              <div key={review.id} className="pb-4 border-b border-gray-100 last:border-0 last:pb-0">
                <div className="flex items-center justify-between mb-1.5">
                  <span className="font-medium text-gray-800 text-sm">{review.author}</span>
                  <div className="flex items-center gap-2">
                    <span className="text-sm text-yellow-500">{'★'.repeat(review.rating)}</span>
                    <span className="text-xs text-gray-400">{review.date}</span>
                  </div>
                </div>
                <p className="text-sm text-gray-600">{review.text}</p>
              </div>
            ))}
          </div>
        </div>

        {/* Action Buttons */}
        <div className="flex gap-3">
          <Link
            href="/matching/req-002"
            className="flex-1 py-3 border border-gray-300 text-gray-700 text-sm font-semibold rounded-xl text-center hover:bg-gray-50 transition-colors"
          >
            돌아가기
          </Link>
          <button className="flex-1 py-3 bg-blue-600 text-white text-sm font-semibold rounded-xl hover:bg-blue-700 transition-colors">
            이 제공자 선택
          </button>
        </div>
      </div>
    </FamilyAppShell>
  );
}
