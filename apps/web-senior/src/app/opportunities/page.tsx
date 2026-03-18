// 일자리 · 봉사 · 커뮤니티 — Jobs, Volunteering & Community Opportunities

import Link from 'next/link';
import SeniorAppShell from '@/components/SeniorAppShell';

type OpportunityType = '일자리' | '봉사' | '교육' | '커뮤니티';

interface Opportunity {
  id: string;
  type: OpportunityType;
  title: string;
  organization: string;
  location: string;
  schedule: string;
  description: string;
  pay?: string;
  deadline?: string;
  ageLimit?: string;
  spots: number;
}

const OPPORTUNITIES: Opportunity[] = [
  {
    id: 'opp-1',
    type: '일자리',
    title: '노인복지관 안내 도우미',
    organization: '서울 노인 복지관',
    location: '서울 중구',
    schedule: '주 3일 (월·수·금) 오전 9시~1시',
    description: '복지관 방문객 안내 및 프로그램 보조 업무. 편안한 근무 환경.',
    pay: '시간당 12,000원',
    deadline: '2026년 3월 31일',
    ageLimit: '60세 이상',
    spots: 3,
  },
  {
    id: 'opp-2',
    type: '일자리',
    title: '학교 급식 보조원',
    organization: '서울시 교육청',
    location: '서울 강남구',
    schedule: '주 5일 오전 10시~2시',
    description: '초등학교 급식실 배식 보조. 정규 채용 가능.',
    pay: '월 1,200,000원',
    deadline: '2026년 4월 15일',
    ageLimit: '65세 이하',
    spots: 5,
  },
  {
    id: 'opp-3',
    type: '봉사',
    title: '독거노인 말벗 봉사',
    organization: '사랑나눔 복지재단',
    location: '서울 전 지역',
    schedule: '주 1회 이상 자유 참여',
    description: '혼자 사시는 어르신과 전화 통화 및 방문 말벗 활동.',
    spots: 20,
  },
  {
    id: 'opp-4',
    type: '교육',
    title: '스마트폰 활용 교실',
    organization: '디지털 새싹 센터',
    location: '서울 마포구',
    schedule: '매주 화·목 오후 2시~4시 (8주 과정)',
    description: '카카오톡, 유튜브, 온라인 쇼핑 등 실생활 스마트폰 활용법 교육.',
    spots: 15,
  },
  {
    id: 'opp-5',
    type: '커뮤니티',
    title: '시니어 합창단 단원 모집',
    organization: '행복 시니어 문화센터',
    location: '서울 종로구',
    schedule: '매주 목요일 오후 3시~5시',
    description: '노래를 좋아하시는 60세 이상 어르신. 정기 공연 및 친목 활동.',
    spots: 8,
  },
  {
    id: 'opp-6',
    type: '봉사',
    title: '급식 봉사 (월 2회)',
    organization: '희망 무료 급식소',
    location: '서울 동대문구',
    schedule: '매월 둘째·넷째 토요일 오전 10시~2시',
    description: '취약계층 어르신들께 따뜻한 식사를 대접하는 봉사 활동.',
    spots: 10,
  },
];

const TYPE_CONFIG: Record<OpportunityType, { color: string; icon: string }> = {
  '일자리': { color: 'bg-primary-100 text-primary-700', icon: '💼' },
  '봉사': { color: 'bg-success-50 text-success-700', icon: '🙌' },
  '교육': { color: 'bg-info-50 text-info-700', icon: '📖' },
  '커뮤니티': { color: 'bg-warning-50 text-warning-700', icon: '🎵' },
};

export default function OpportunitiesPage() {
  return (
    <SeniorAppShell>
      <div className="page-content">
        <h1 className="text-senior-2xl font-bold text-gray-900 mb-2">일자리 · 봉사 · 참여</h1>
        <p className="text-senior-base text-gray-500 mb-5">활기찬 노후를 위한 다양한 기회</p>

        {/* Type filter badges */}
        <div className="flex gap-2 flex-wrap mb-5">
          {(Object.entries(TYPE_CONFIG) as [OpportunityType, { color: string; icon: string }][]).map(([type, config]) => (
            <span key={type} className={`${config.color} text-senior-sm font-semibold px-3 py-1.5 rounded-full`}>
              {config.icon} {type}
            </span>
          ))}
        </div>

        {/* Opportunity cards */}
        <div className="space-y-4">
          {OPPORTUNITIES.map((opp) => {
            const config = TYPE_CONFIG[opp.type];
            return (
              <Link
                key={opp.id}
                href={`/opportunities/${opp.id}`}
                className="senior-card block hover:shadow-md active:scale-[0.99] transition-all"
              >
                <div className="flex items-start justify-between mb-2">
                  <div className="flex items-center gap-2">
                    <span className="text-2xl" aria-hidden="true">{config.icon}</span>
                    <span className={`${config.color} text-senior-sm font-bold px-2 py-0.5 rounded-full`}>
                      {opp.type}
                    </span>
                  </div>
                  {opp.deadline && (
                    <span className="text-senior-sm text-danger-600 font-medium flex-shrink-0 ml-2">
                      마감: {opp.deadline}
                    </span>
                  )}
                </div>

                <p className="text-senior-xl font-bold text-gray-900 mb-1">{opp.title}</p>
                <p className="text-senior-sm text-gray-500 mb-2">{opp.organization} · {opp.location}</p>

                <p className="text-senior-base text-gray-700 mb-3 line-clamp-2">{opp.description}</p>

                <div className="flex flex-wrap gap-3 text-senior-sm">
                  <span className="flex items-center gap-1 text-gray-600">
                    🕐 {opp.schedule}
                  </span>
                  {opp.pay && (
                    <span className="flex items-center gap-1 text-success-700 font-semibold">
                      💰 {opp.pay}
                    </span>
                  )}
                  {opp.ageLimit && (
                    <span className="flex items-center gap-1 text-gray-500">
                      👤 {opp.ageLimit}
                    </span>
                  )}
                  <span className="flex items-center gap-1 text-primary-600 font-semibold ml-auto">
                    잔여 {opp.spots}자리
                  </span>
                </div>
              </Link>
            );
          })}
        </div>
      </div>
    </SeniorAppShell>
  );
}
