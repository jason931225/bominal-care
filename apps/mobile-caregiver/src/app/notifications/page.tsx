'use client';

import { useState } from 'react';
import CaregiverAppShell from '@/components/CaregiverAppShell';
import Link from 'next/link';

interface Notification {
  id: string;
  type: 'urgent' | 'schedule' | 'medication' | 'task' | 'system' | 'message';
  title: string;
  body: string;
  time: string;
  date: string;
  read: boolean;
  actionHref?: string;
  actionLabel?: string;
}

const INITIAL_NOTIFICATIONS: Notification[] = [
  {
    id: 'n001',
    type: 'urgent',
    title: '긴급: 이순자 어르신 혈압 이상',
    body: '방금 체크한 혈압이 168/105로 높습니다. 즉시 확인이 필요합니다.',
    time: '09:45',
    date: '오늘',
    read: false,
    actionHref: '/clients/c001',
    actionLabel: '이용자 정보 보기',
  },
  {
    id: 'n002',
    type: 'schedule',
    title: '일정 변경 알림',
    body: '박영철 어르신 3월 17일(화) 방문이 14:30에서 16:00으로 변경되었습니다.',
    time: '08:30',
    date: '오늘',
    read: false,
    actionHref: '/schedule',
    actionLabel: '일정 확인',
  },
  {
    id: 'n003',
    type: 'medication',
    title: '투약 확인 요청',
    body: '이순자 어르신 오전 10시 투약이 아직 기록되지 않았습니다.',
    time: '10:15',
    date: '오늘',
    read: false,
    actionHref: '/clients/c001/medications',
    actionLabel: '투약 기록',
  },
  {
    id: 'n004',
    type: 'task',
    title: '케어일지 미작성 안내',
    body: '어제 이순자 어르신 방문 케어일지가 아직 제출되지 않았습니다.',
    time: '08:00',
    date: '오늘',
    read: true,
    actionHref: '/notes/new',
    actionLabel: '작성하기',
  },
  {
    id: 'n005',
    type: 'message',
    title: '관리자 메시지',
    body: '3월 월례 교육이 3월 20일(금) 오후 2시에 있습니다. 참석 여부를 확인해 주세요.',
    time: '17:00',
    date: '어제',
    read: true,
  },
  {
    id: 'n006',
    type: 'system',
    title: '급여 명세서 발행',
    body: '2월분 급여 명세서가 발행되었습니다. 앱에서 확인하실 수 있습니다.',
    time: '09:00',
    date: '3월 10일',
    read: true,
  },
  {
    id: 'n007',
    type: 'schedule',
    title: '새 이용자 배정',
    body: '최말순 어르신이 새로 배정되었습니다. 케어플랜을 확인해 주세요.',
    time: '14:00',
    date: '3월 9일',
    read: true,
    actionHref: '/clients/c003',
    actionLabel: '이용자 보기',
  },
];

const TYPE_CONFIG: Record<Notification['type'], { icon: string; bg: string; label: string }> = {
  urgent: { icon: '🚨', bg: 'bg-red-100', label: '긴급' },
  schedule: { icon: '📅', bg: 'bg-blue-100', label: '일정' },
  medication: { icon: '💊', bg: 'bg-purple-100', label: '투약' },
  task: { icon: '✅', bg: 'bg-green-100', label: '업무' },
  system: { icon: '⚙️', bg: 'bg-slate-100', label: '시스템' },
  message: { icon: '💬', bg: 'bg-amber-100', label: '메시지' },
};

export default function NotificationsPage() {
  const [notifications, setNotifications] = useState<Notification[]>(INITIAL_NOTIFICATIONS);
  const [filter, setFilter] = useState<'all' | 'unread'>('all');

  const unreadCount = notifications.filter((n) => !n.read).length;

  const markAllRead = () => {
    setNotifications((prev) => prev.map((n) => ({ ...n, read: true })));
  };

  const markRead = (id: string) => {
    setNotifications((prev) =>
      prev.map((n) => (n.id === id ? { ...n, read: true } : n))
    );
  };

  const deleteNotification = (id: string) => {
    setNotifications((prev) => prev.filter((n) => n.id !== id));
  };

  const displayed = notifications.filter((n) => filter === 'all' || !n.read);

  const grouped = displayed.reduce<Record<string, Notification[]>>((acc, n) => {
    const key = n.date;
    return { ...acc, [key]: [...(acc[key] ?? []), n] };
  }, {});

  return (
    <CaregiverAppShell activeTab="notifications" title="알림">
      <div className="px-4 py-4 space-y-4">
        {/* Filter & Actions */}
        <div className="flex items-center justify-between">
          <div className="flex gap-2">
            {(['all', 'unread'] as const).map((f) => (
              <button
                key={f}
                type="button"
                onClick={() => setFilter(f)}
                className={`px-3 py-1.5 rounded-full text-sm font-medium transition-colors ${
                  filter === f ? 'bg-blue-600 text-white' : 'bg-slate-100 text-slate-600'
                }`}
              >
                {f === 'all' ? '전체' : `읽지 않음 ${unreadCount > 0 ? `(${unreadCount})` : ''}`}
              </button>
            ))}
          </div>
          {unreadCount > 0 && (
            <button
              type="button"
              onClick={markAllRead}
              className="text-xs text-blue-600 font-medium"
            >
              모두 읽음
            </button>
          )}
        </div>

        {/* Notification Groups */}
        {Object.keys(grouped).length === 0 ? (
          <div className="text-center py-16">
            <div className="text-5xl mb-4">🔕</div>
            <p className="text-slate-500 text-sm font-medium">알림이 없습니다.</p>
          </div>
        ) : (
          Object.entries(grouped).map(([date, items]) => (
            <div key={date}>
              <h3 className="text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2">{date}</h3>
              <div className="space-y-2">
                {items.map((notif) => {
                  const conf = TYPE_CONFIG[notif.type];
                  return (
                    <div
                      key={notif.id}
                      className={`card relative ${notif.read ? 'opacity-70' : 'border-blue-100'}`}
                      onClick={() => markRead(notif.id)}
                    >
                      {/* Unread dot */}
                      {!notif.read && (
                        <div className="absolute top-4 right-4 w-2.5 h-2.5 bg-blue-600 rounded-full" />
                      )}

                      <div className="flex items-start gap-3">
                        <div className={`w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 ${conf.bg}`}>
                          <span className="text-xl">{conf.icon}</span>
                        </div>
                        <div className="flex-1 min-w-0 pr-4">
                          <div className="flex items-center gap-2 mb-0.5">
                            <span className={`text-xs font-medium px-1.5 py-0.5 rounded ${conf.bg} text-slate-600`}>
                              {conf.label}
                            </span>
                            <span className="text-xs text-slate-400">{notif.time}</span>
                          </div>
                          <p className={`text-sm font-semibold leading-snug ${
                            notif.type === 'urgent' ? 'text-red-700' : 'text-slate-800'
                          }`}>
                            {notif.title}
                          </p>
                          <p className="text-xs text-slate-500 mt-1 leading-relaxed">{notif.body}</p>
                          {notif.actionHref && notif.actionLabel && (
                            <Link
                              href={notif.actionHref}
                              className="inline-flex items-center gap-1 mt-2 text-xs font-semibold text-blue-600"
                              onClick={(e) => e.stopPropagation()}
                            >
                              {notif.actionLabel}
                              <svg className="w-3 h-3" fill="none" stroke="currentColor" strokeWidth={2.5} viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
                              </svg>
                            </Link>
                          )}
                        </div>
                      </div>

                      {/* Swipe to delete hint */}
                      <div className="flex justify-end mt-3 pt-2 border-t border-slate-100">
                        <button
                          type="button"
                          onClick={(e) => {
                            e.stopPropagation();
                            deleteNotification(notif.id);
                          }}
                          className="text-xs text-slate-400 font-medium px-2 py-1 rounded-lg active:bg-red-50 active:text-red-500 transition-colors"
                        >
                          삭제
                        </button>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          ))
        )}

        {/* Notification Settings Link */}
        <div className="card flex items-center gap-3 active:scale-98 transition-transform">
          <span className="text-xl">⚙️</span>
          <div className="flex-1">
            <p className="text-sm font-medium text-slate-800">알림 설정</p>
            <p className="text-xs text-slate-400">알림 유형 및 시간 설정</p>
          </div>
          <svg className="w-4 h-4 text-slate-300" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
          </svg>
        </div>

        <div className="pb-2" />
      </div>
    </CaregiverAppShell>
  );
}
