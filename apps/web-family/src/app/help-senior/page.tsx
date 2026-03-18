import Link from 'next/link';
import FamilyAppShell from '@/components/FamilyAppShell';

const DELEGATED_ACTIONS = [
  {
    id: 'book',
    icon: '📅',
    title: '서비스 예약 대리',
    description: '어르신을 대신하여 방문 요양, 병원 동행 등 서비스를 예약합니다',
    href: '/help-senior/book',
    color: 'blue',
    cta: '예약하기',
  },
  {
    id: 'medication',
    icon: '💊',
    title: '복약 관리',
    description: '어르신의 복약 일정을 조회하고 복약 알림을 설정합니다',
    href: '/medications',
    color: 'green',
    cta: '복약 현황 보기',
  },
  {
    id: 'emergency',
    icon: '🚨',
    title: '긴급 연락',
    description: '담당 케어매니저 또는 요양보호사에게 긴급 연락을 보냅니다',
    href: '/help-senior/emergency',
    color: 'red',
    cta: '긴급 연락',
  },
  {
    id: 'report',
    icon: '📋',
    title: '가족 보고서 요청',
    description: '케어매니저에게 어르신 상태 보고서를 요청합니다',
    href: '/help-senior/report',
    color: 'purple',
    cta: '보고서 요청',
  },
];

const RECENT_ACTIVITIES = [
  {
    id: 1,
    icon: '📅',
    title: '병원 동행 예약 완료',
    desc: '2026-03-18 오전 9시 · 강남성모병원 내과',
    time: '2일 전',
    by: '김가족',
  },
  {
    id: 2,
    icon: '💊',
    title: '복약 알림 설정',
    desc: '아침 8시 / 저녁 7시 알림 활성화',
    time: '1주일 전',
    by: '김가족',
  },
  {
    id: 3,
    icon: '📋',
    title: '2월 케어 보고서 요청',
    desc: '케어매니저 최지원에게 전달됨',
    time: '3월 2일',
    by: '김가족',
  },
];

const colorStyle: Record<string, string> = {
  blue: 'bg-blue-50 border-blue-200 hover:border-blue-400',
  green: 'bg-green-50 border-green-200 hover:border-green-400',
  red: 'bg-red-50 border-red-200 hover:border-red-400',
  purple: 'bg-purple-50 border-purple-200 hover:border-purple-400',
};

const ctaStyle: Record<string, string> = {
  blue: 'bg-blue-600 hover:bg-blue-700 text-white',
  green: 'bg-green-600 hover:bg-green-700 text-white',
  red: 'bg-red-600 hover:bg-red-700 text-white',
  purple: 'bg-purple-600 hover:bg-purple-700 text-white',
};

export default function HelpSeniorPage() {
  return (
    <FamilyAppShell>
      <div className="max-w-3xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">대리 서비스</h1>
          <p className="text-sm text-gray-500 mt-1">
            김복순 어머님을 대신하여 서비스를 관리합니다
          </p>
        </div>

        {/* Permission Notice */}
        <div className="bg-blue-50 border border-blue-200 rounded-xl p-4 flex items-start gap-3 mb-6">
          <span className="text-xl flex-shrink-0">ℹ️</span>
          <div>
            <p className="text-sm font-semibold text-blue-800">대리 서비스 권한 안내</p>
            <p className="text-sm text-blue-700 mt-0.5">
              현재 <strong>김가족</strong>님은 김복순 어머님의 <strong>법정 대리인</strong>으로 등록되어 있습니다.
              모든 대리 활동은 기록되며 어르신께 알림이 전송됩니다.
            </p>
          </div>
        </div>

        {/* Action Cards */}
        <div className="grid sm:grid-cols-2 gap-4 mb-8">
          {DELEGATED_ACTIONS.map((action) => (
            <div
              key={action.id}
              className={`border rounded-xl p-5 transition-colors ${colorStyle[action.color]}`}
            >
              <div className="flex items-center gap-3 mb-3">
                <span className="text-3xl">{action.icon}</span>
                <h3 className="font-bold text-gray-900">{action.title}</h3>
              </div>
              <p className="text-sm text-gray-600 mb-4 leading-relaxed">{action.description}</p>
              <Link
                href={action.href}
                className={`inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-semibold transition-colors ${ctaStyle[action.color]}`}
              >
                {action.cta} →
              </Link>
            </div>
          ))}
        </div>

        {/* Recent Activity */}
        <div>
          <h2 className="text-lg font-bold text-gray-900 mb-4">최근 대리 활동</h2>
          <div className="bg-white border border-gray-200 rounded-xl divide-y divide-gray-100">
            {RECENT_ACTIVITIES.map((activity) => (
              <div key={activity.id} className="flex items-start gap-3 p-4">
                <span className="text-xl flex-shrink-0">{activity.icon}</span>
                <div className="flex-1 min-w-0">
                  <p className="font-semibold text-gray-900 text-sm">{activity.title}</p>
                  <p className="text-xs text-gray-500 mt-0.5">{activity.desc}</p>
                  <div className="flex items-center gap-2 mt-1">
                    <span className="text-xs text-gray-400">{activity.by} 처리</span>
                    <span className="text-xs text-gray-300">·</span>
                    <span className="text-xs text-gray-400">{activity.time}</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </FamilyAppShell>
  );
}
