import FamilyAppShell from '@/components/FamilyAppShell';

const NOTIFICATIONS = [
  {
    id: 1,
    read: false,
    urgent: true,
    icon: '🚨',
    color: 'red',
    title: '긴급: 혈압 이상 감지',
    body: '김복순 어머님의 혈압이 정상 범위를 초과했습니다. 수축기 148 mmHg 측정.',
    time: '방금 전',
    category: '건강 경보',
  },
  {
    id: 2,
    read: false,
    urgent: false,
    icon: '✅',
    color: 'purple',
    title: '승인 요청 도착',
    body: '물리치료 주 4회 증가 요청이 접수되었습니다. 48시간 내 승인이 필요합니다.',
    time: '1시간 전',
    category: '승인 요청',
  },
  {
    id: 3,
    read: false,
    urgent: false,
    icon: '💊',
    color: 'blue',
    title: '복약 완료 알림',
    body: '오전 복약(혈압약, 당뇨약, 혈전예방제)이 정상적으로 완료되었습니다.',
    time: '2시간 전',
    category: '복약',
  },
  {
    id: 4,
    read: true,
    urgent: false,
    icon: '📄',
    color: 'gray',
    title: '계약 갱신 안내',
    body: '방문 요양 서비스 계약이 30일 후 만료됩니다. 갱신을 위해 담당 케어매니저에게 연락하세요.',
    time: '어제',
    category: '계약',
  },
  {
    id: 5,
    read: true,
    urgent: false,
    icon: '🏥',
    color: 'green',
    title: '방문 케어 완료',
    body: '어제 오전 케어 방문이 정상 완료되었습니다. 이상 없음.',
    time: '어제 오후 1:00',
    category: '방문',
  },
  {
    id: 6,
    read: true,
    urgent: false,
    icon: '💳',
    color: 'indigo',
    title: '결제 처리 완료',
    body: '3월 방문 요양 서비스 비용 148,000원이 등록 계좌에서 자동 결제되었습니다.',
    time: '3월 10일',
    category: '결제',
  },
  {
    id: 7,
    read: true,
    urgent: false,
    icon: '🔍',
    color: 'teal',
    title: '매칭 추천 도착',
    body: '신규 요양보호사 3명이 매칭 조건에 부합하여 추천 목록에 추가되었습니다.',
    time: '3월 8일',
    category: '매칭',
  },
];

const colorBadge: Record<string, string> = {
  red: 'bg-red-50 text-red-700 border border-red-200',
  purple: 'bg-purple-50 text-purple-700 border border-purple-200',
  blue: 'bg-blue-50 text-blue-700 border border-blue-200',
  gray: 'bg-gray-100 text-gray-600 border border-gray-200',
  green: 'bg-green-50 text-green-700 border border-green-200',
  indigo: 'bg-indigo-50 text-indigo-700 border border-indigo-200',
  teal: 'bg-teal-50 text-teal-700 border border-teal-200',
};

export default function NotificationsPage() {
  const unreadCount = NOTIFICATIONS.filter((n) => !n.read).length;

  return (
    <FamilyAppShell>
      <div className="max-w-2xl mx-auto px-4 py-6">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">알림 센터</h1>
            <p className="text-sm text-gray-500 mt-1">
              미읽은 알림 <span className="font-semibold text-blue-600">{unreadCount}건</span>
            </p>
          </div>
          <button className="text-sm text-blue-600 hover:underline font-medium">
            모두 읽음 처리
          </button>
        </div>

        {/* Unread */}
        {NOTIFICATIONS.filter((n) => !n.read).length > 0 && (
          <section className="mb-6">
            <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-3">
              새 알림
            </h2>
            <div className="space-y-2">
              {NOTIFICATIONS.filter((n) => !n.read).map((notif) => (
                <div
                  key={notif.id}
                  className={`bg-white border rounded-xl p-4 flex gap-3 shadow-sm ${
                    notif.urgent ? 'border-red-300' : 'border-blue-200'
                  }`}
                >
                  <span className="text-2xl flex-shrink-0 mt-0.5">{notif.icon}</span>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 flex-wrap mb-1">
                      <span className={`text-xs font-semibold px-2 py-0.5 rounded-full ${colorBadge[notif.color]}`}>
                        {notif.category}
                      </span>
                      {notif.urgent && (
                        <span className="text-xs font-bold text-red-600 bg-red-50 px-2 py-0.5 rounded-full border border-red-200">
                          긴급
                        </span>
                      )}
                      <span className="text-xs text-gray-400 ml-auto">{notif.time}</span>
                    </div>
                    <p className="font-semibold text-gray-900 text-sm">{notif.title}</p>
                    <p className="text-sm text-gray-600 mt-0.5">{notif.body}</p>
                  </div>
                  <div className="w-2 h-2 rounded-full bg-blue-500 flex-shrink-0 mt-2" />
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Read */}
        <section>
          <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-3">
            이전 알림
          </h2>
          <div className="space-y-2">
            {NOTIFICATIONS.filter((n) => n.read).map((notif) => (
              <div
                key={notif.id}
                className="bg-gray-50 border border-gray-200 rounded-xl p-4 flex gap-3"
              >
                <span className="text-xl flex-shrink-0 mt-0.5 opacity-60">{notif.icon}</span>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 flex-wrap mb-1">
                    <span className="text-xs text-gray-400">{notif.category}</span>
                    <span className="text-xs text-gray-400 ml-auto">{notif.time}</span>
                  </div>
                  <p className="font-medium text-gray-600 text-sm">{notif.title}</p>
                  <p className="text-sm text-gray-500 mt-0.5">{notif.body}</p>
                </div>
              </div>
            ))}
          </div>
        </section>
      </div>
    </FamilyAppShell>
  );
}
