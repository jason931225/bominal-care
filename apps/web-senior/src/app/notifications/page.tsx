'use client';

// 알림 — Notifications list for senior app
// Unread/all tabs, mark as read functionality

import { useState } from 'react';
import SeniorAppShell from '@/components/SeniorAppShell';

type NotificationType = 'medication' | 'visit' | 'alert' | 'system';

interface Notification {
  id: string;
  type: NotificationType;
  title: string;
  body: string;
  time: string;
  read: boolean;
}

const INITIAL_NOTIFICATIONS: Notification[] = [
  {
    id: 'n-1',
    type: 'medication',
    title: '약 복용 시간입니다',
    body: '점심 식후 아스피린 100mg, 메트포르민 500mg을 복용해 주세요.',
    time: '오늘 12:00',
    read: false,
  },
  {
    id: 'n-2',
    type: 'visit',
    title: '방문 요양 예정 알림',
    body: '오늘 오후 2시에 최지원 요양보호사가 방문 예정입니다.',
    time: '오늘 09:30',
    read: false,
  },
  {
    id: 'n-3',
    type: 'alert',
    title: '약 재처방 필요',
    body: '오메프라졸 잔여량이 8정입니다. 3월 18일까지 재처방이 필요합니다.',
    time: '오늘 08:00',
    read: false,
  },
  {
    id: 'n-4',
    type: 'medication',
    title: '아침 약 복용 완료',
    body: '아침 식후 암로디핀 5mg, 메트포르민 500mg 복용이 기록되었습니다.',
    time: '오늘 08:15',
    read: true,
  },
  {
    id: 'n-5',
    type: 'visit',
    title: '방문 요양 완료',
    body: '어제 오후 2시 최지원 요양보호사 방문이 완료되었습니다. 케어일지를 확인하세요.',
    time: '어제 16:00',
    read: true,
  },
  {
    id: 'n-6',
    type: 'system',
    title: '보호자 메시지',
    body: '딸 김영미님이 메시지를 남겼습니다: "어머니, 오늘 저녁에 전화할게요 😊"',
    time: '어제 10:00',
    read: true,
  },
];

const TYPE_CONFIG: Record<NotificationType, { icon: string; bg: string; text: string }> = {
  medication: { icon: '💊', bg: 'bg-purple-50', text: 'text-purple-700' },
  visit: { icon: '🏠', bg: 'bg-blue-50', text: 'text-blue-700' },
  alert: { icon: '⚠️', bg: 'bg-red-50', text: 'text-red-700' },
  system: { icon: '📢', bg: 'bg-green-50', text: 'text-green-700' },
};

export default function NotificationsPage() {
  const [notifications, setNotifications] = useState<Notification[]>(INITIAL_NOTIFICATIONS);
  const [tab, setTab] = useState<'unread' | 'all'>('unread');

  const unreadCount = notifications.filter((n) => !n.read).length;

  const displayed = tab === 'unread'
    ? notifications.filter((n) => !n.read)
    : notifications;

  const markRead = (id: string) => {
    setNotifications((prev) =>
      prev.map((n) => (n.id === id ? { ...n, read: true } : n))
    );
  };

  const markAllRead = () => {
    setNotifications((prev) => prev.map((n) => ({ ...n, read: true })));
  };

  return (
    <SeniorAppShell>
      <div className="page-content">
        <div className="flex items-center justify-between mb-5">
          <h1 className="text-senior-2xl font-bold text-gray-900">알림</h1>
          {unreadCount > 0 && (
            <button
              onClick={markAllRead}
              className="text-senior-sm text-primary-600 font-medium min-h-touch flex items-center"
              aria-label="모든 알림을 읽음으로 표시"
            >
              모두 읽음
            </button>
          )}
        </div>

        {/* Tabs */}
        <div className="flex gap-2 mb-5 bg-gray-100 p-1 rounded-xl" role="tablist">
          <button
            role="tab"
            aria-selected={tab === 'unread'}
            onClick={() => setTab('unread')}
            className={`flex-1 py-2.5 rounded-lg text-senior-base font-semibold transition-colors ${
              tab === 'unread'
                ? 'bg-white text-gray-900 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'
            }`}
          >
            읽지 않음
            {unreadCount > 0 && (
              <span className="ml-2 bg-danger-500 text-white text-xs font-bold px-1.5 py-0.5 rounded-full">
                {unreadCount}
              </span>
            )}
          </button>
          <button
            role="tab"
            aria-selected={tab === 'all'}
            onClick={() => setTab('all')}
            className={`flex-1 py-2.5 rounded-lg text-senior-base font-semibold transition-colors ${
              tab === 'all'
                ? 'bg-white text-gray-900 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'
            }`}
          >
            전체
          </button>
        </div>

        {/* Notification List */}
        {displayed.length === 0 ? (
          <div className="text-center py-16">
            <p className="text-5xl mb-4">🔔</p>
            <p className="text-senior-lg font-semibold text-gray-500">새 알림이 없습니다</p>
          </div>
        ) : (
          <div className="space-y-3" role="list">
            {displayed.map((notification) => {
              const config = TYPE_CONFIG[notification.type];
              return (
                <div
                  key={notification.id}
                  role="listitem"
                  className={`senior-card transition-all ${
                    !notification.read ? 'border-l-4 border-l-primary-500' : ''
                  }`}
                >
                  <div className="flex items-start gap-4">
                    <div className={`w-12 h-12 rounded-full flex items-center justify-center flex-shrink-0 ${config.bg}`}>
                      <span className="text-2xl">{config.icon}</span>
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-2">
                        <p className={`text-senior-base font-bold ${!notification.read ? 'text-gray-900' : 'text-gray-600'}`}>
                          {notification.title}
                        </p>
                        {!notification.read && (
                          <span className="w-2.5 h-2.5 bg-primary-500 rounded-full flex-shrink-0 mt-1.5" aria-label="읽지 않은 알림" />
                        )}
                      </div>
                      <p className="text-senior-sm text-gray-500 mt-1 leading-relaxed">{notification.body}</p>
                      <div className="flex items-center justify-between mt-2">
                        <span className="text-senior-xs text-gray-400">{notification.time}</span>
                        {!notification.read && (
                          <button
                            onClick={() => markRead(notification.id)}
                            className="text-senior-xs text-primary-600 font-medium hover:text-primary-800"
                            aria-label={`${notification.title} 알림 읽음으로 표시`}
                          >
                            읽음 표시
                          </button>
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </SeniorAppShell>
  );
}
