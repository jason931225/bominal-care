// 가족 대시보드 — Family Dashboard
// Shows care timeline, status cards, and quick actions for the family member

import Link from 'next/link';
import { redirect } from 'next/navigation';
import FamilyAppShell from '@/components/FamilyAppShell';
import { auth } from '@bominal-senior/auth';
import { pool } from '@bominal-senior/db';
import { getPersonProfileByUserId, listSignals } from '@/lib/services';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface TimelineEvent {
  id: number;
  time: string;
  type: string;
  icon: string;
  color: string;
  title: string;
  desc: string;
  actor: string;
}

interface StatusCard {
  label: string;
  value: string;
  icon: string;
  color: string;
  sub: string;
}

// ---------------------------------------------------------------------------
// Color mappings
// ---------------------------------------------------------------------------

const colorBadge: Record<string, string> = {
  blue: 'bg-blue-50 text-blue-700 border border-blue-200',
  green: 'bg-green-50 text-green-700 border border-green-200',
  yellow: 'bg-yellow-50 text-yellow-700 border border-yellow-200',
  purple: 'bg-purple-50 text-purple-700 border border-purple-200',
  red: 'bg-red-50 text-red-700 border border-red-200',
};

const dotColor: Record<string, string> = {
  blue: 'bg-blue-400',
  green: 'bg-green-400',
  yellow: 'bg-yellow-400',
  purple: 'bg-purple-400',
  red: 'bg-red-400',
};

// ---------------------------------------------------------------------------
// Helpers — map DB signals to timeline events
// ---------------------------------------------------------------------------

const EVENT_TYPE_CONFIG: Record<string, { icon: string; color: string; type: string }> = {
  MEDICATION_TAKEN: { icon: '💊', color: 'blue', type: '복약' },
  MEDICATION_MISSED: { icon: '💊', color: 'red', type: '복약 누락' },
  VISIT_CHECK_IN: { icon: '🏥', color: 'green', type: '방문' },
  VISIT_CHECK_OUT: { icon: '🏥', color: 'green', type: '방문 종료' },
  OBSERVATION_RECORDED: { icon: '📝', color: 'yellow', type: '관찰' },
  CARE_PLAN_UPDATED: { icon: '📋', color: 'purple', type: '케어플랜' },
  APPOINTMENT_CREATED: { icon: '📅', color: 'blue', type: '예약' },
  APPOINTMENT_COMPLETED: { icon: '✅', color: 'green', type: '진료 완료' },
  CONSENT_GRANTED: { icon: '✅', color: 'purple', type: '승인' },
  REFERRAL_CREATED: { icon: '📄', color: 'yellow', type: '의뢰' },
};

function formatRelativeTime(date: Date): string {
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60_000);

  if (diffMins < 60) {
    return `${diffMins}분 전`;
  }

  const diffHours = Math.floor(diffMins / 60);
  if (diffHours < 24) {
    const period = date.getHours() < 12 ? '오전' : '오후';
    const displayHours = date.getHours() <= 12 ? date.getHours() : date.getHours() - 12;
    const minutes = date.getMinutes().toString().padStart(2, '0');
    return `${period} ${displayHours}:${minutes}`;
  }

  const diffDays = Math.floor(diffHours / 24);
  if (diffDays === 1) {
    const period = date.getHours() < 12 ? '오전' : '오후';
    const displayHours = date.getHours() <= 12 ? date.getHours() : date.getHours() - 12;
    const minutes = date.getMinutes().toString().padStart(2, '0');
    return `어제 ${period} ${displayHours}:${minutes}`;
  }

  return `${diffDays}일 전`;
}

function extractMetadataDesc(metadata: unknown): string {
  if (!metadata || typeof metadata !== 'object') return '';
  const meta = metadata as Record<string, unknown>;
  const parts: string[] = [];
  if (meta.medicationName) parts.push(String(meta.medicationName));
  if (meta.scheduledTime) parts.push(`${String(meta.scheduledTime)} 예정`);
  if (meta.missedDate) parts.push(`${String(meta.missedDate)} 누락`);
  if (meta.description) parts.push(String(meta.description));
  if (meta.reason) parts.push(String(meta.reason));
  return parts.length > 0 ? parts.join(' · ') : '';
}

function mapSignalToEvent(signal: Record<string, unknown>, index: number): TimelineEvent {
  const eventType = signal.event_type as string;
  const config = EVENT_TYPE_CONFIG[eventType] ?? { icon: '📌', color: 'blue', type: eventType };
  const createdAt = new Date(signal.created_at as string);

  return {
    id: index + 1,
    time: formatRelativeTime(createdAt),
    type: config.type,
    icon: config.icon,
    color: config.color,
    title: (signal.message as string) ?? '',
    desc: extractMetadataDesc(signal.metadata),
    actor: (signal.actor_user_id as string) ?? '',
  };
}

// ---------------------------------------------------------------------------
// Korean date formatting
// ---------------------------------------------------------------------------

function formatKoreanDate(date: Date): string {
  const dayNames = ['일요일', '월요일', '화요일', '수요일', '목요일', '금요일', '토요일'];
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  const day = date.getDate();
  const dayOfWeek = dayNames[date.getDay()] ?? '';
  return `${year}년 ${month}월 ${day}일 ${dayOfWeek}`;
}

// ---------------------------------------------------------------------------
// Page (async server component)
// ---------------------------------------------------------------------------

export default async function FamilyDashboard() {
  const session = await auth();
  if (!session?.user?.id) {
    redirect('/auth/signin');
  }

  const today = new Date();
  const todayDate = formatKoreanDate(today);

  let personName = '어르신';
  let timelineEvents: TimelineEvent[] = [];

  // Default status cards — will be enriched with real data when available
  let statusCards: StatusCard[] = [
    { label: '오늘 방문', value: '-', icon: '✅', color: 'green', sub: '정보 없음' },
    { label: '복약 현황', value: '-', icon: '💊', color: 'blue', sub: '정보 없음' },
    { label: '알림', value: '0건', icon: '🔔', color: 'red', sub: '새 알림 없음' },
    { label: '승인 대기', value: '-', icon: '⏳', color: 'yellow', sub: '' },
  ];

  try {
    const personProfile = await getPersonProfileByUserId(pool, session.user.id);

    if (personProfile) {
      personName = `${personProfile.lastName}${personProfile.firstName}`;

      // Fetch observability signals for the person
      const signalsResult = await listSignals(
        pool,
        { subjectPersonId: personProfile.id },
        { page: 1, limit: 10 },
      );

      timelineEvents = signalsResult.data.map(mapSignalToEvent);

      // Derive status card values from signals
      const medEvents = signalsResult.data.filter(
        (s) => (s.event_type as string) === 'MEDICATION_TAKEN',
      );
      const visitEvents = signalsResult.data.filter(
        (s) => (s.event_type as string) === 'VISIT_CHECK_IN' || (s.event_type as string) === 'VISIT_CHECK_OUT',
      );

      statusCards = [
        {
          label: '오늘 방문',
          value: visitEvents.length > 0 ? '완료' : '예정 없음',
          icon: '✅',
          color: 'green',
          sub: visitEvents.length > 0 ? '방문 기록 있음' : '',
        },
        {
          label: '복약 현황',
          value: medEvents.length > 0 ? `${medEvents.length}건` : '-',
          icon: '💊',
          color: 'blue',
          sub: medEvents.length > 0 ? '복약 기록 확인' : '기록 없음',
        },
        {
          label: '알림',
          value: `${signalsResult.total}건`,
          icon: '🔔',
          color: 'red',
          sub: signalsResult.total > 0 ? '최근 이벤트' : '새 알림 없음',
        },
        {
          label: '승인 대기',
          value: '-',
          icon: '⏳',
          color: 'yellow',
          sub: '',
        },
      ];
    }
  } catch (error) {
    console.error('[FamilyDashboard] Failed to fetch data:', error);
    // Continue with defaults
  }

  return (
    <FamilyAppShell>
      <div className="max-w-5xl mx-auto px-4 py-6">
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-gray-900">대시보드</h1>
          <p className="text-sm text-gray-500 mt-1">
            {personName}님의 오늘 케어 현황입니다 — {todayDate}
          </p>
        </div>

        {/* Status Cards */}
        <div className="grid grid-cols-2 lg:grid-cols-4 gap-3 mb-8">
          {statusCards.map((card) => (
            <div key={card.label} className="bg-white rounded-xl border border-gray-200 p-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-2xl">{card.icon}</span>
                <span className={`text-xs font-semibold px-2 py-0.5 rounded-full ${colorBadge[card.color] ?? ''}`}>
                  {card.value}
                </span>
              </div>
              <p className="text-sm font-semibold text-gray-800">{card.label}</p>
              <p className="text-xs text-gray-500 mt-0.5">{card.sub}</p>
            </div>
          ))}
        </div>

        <div className="grid lg:grid-cols-3 gap-6">
          {/* Timeline column */}
          <div className="lg:col-span-2">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-bold text-gray-900">최근 케어 이벤트</h2>
              <Link href="/timeline" className="text-sm text-blue-600 hover:underline">전체 보기 →</Link>
            </div>
            {timelineEvents.length === 0 ? (
              <div className="bg-white rounded-xl border border-gray-200 p-8 text-center text-gray-400">
                <p className="text-sm">최근 이벤트가 없습니다</p>
              </div>
            ) : (
              <div className="bg-white rounded-xl border border-gray-200 divide-y divide-gray-100">
                {timelineEvents.map((event) => (
                  <div key={event.id} className="flex gap-4 p-4">
                    <div className="flex flex-col items-center pt-1">
                      <span className="text-xl">{event.icon}</span>
                      <div className={`w-0.5 flex-1 mt-2 ${dotColor[event.color] ?? 'bg-gray-400'} opacity-30`} />
                    </div>
                    <div className="flex-1 min-w-0 pb-1">
                      <div className="flex items-center gap-2 mb-1">
                        <span className={`text-xs font-semibold px-2 py-0.5 rounded-full ${colorBadge[event.color] ?? ''}`}>
                          {event.type}
                        </span>
                        <span className="text-xs text-gray-400">{event.time}</span>
                      </div>
                      <p className="font-semibold text-gray-900 text-sm">{event.title}</p>
                      {event.desc && <p className="text-sm text-gray-500 mt-0.5">{event.desc}</p>}
                      {event.actor && <p className="text-xs text-gray-400 mt-1">{event.actor}</p>}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Right column */}
          <div className="flex flex-col gap-4">
            {/* Quick Actions */}
            <div>
              <h2 className="text-base font-bold text-gray-900 mb-3">빠른 실행</h2>
              <div className="grid grid-cols-2 gap-2">
                {[
                  { href: '/matching', icon: '🔍', label: '매칭 요청' },
                  { href: '/approvals', icon: '✅', label: '승인 처리' },
                  { href: '/help-senior/book', icon: '📅', label: '예약 대리' },
                  { href: '/payments', icon: '💳', label: '결제 내역' },
                ].map((action) => (
                  <Link
                    key={action.href}
                    href={action.href}
                    className="flex flex-col items-center justify-center gap-1.5 bg-white border border-gray-200 rounded-xl p-3 hover:border-blue-300 hover:bg-blue-50 transition-colors"
                  >
                    <span className="text-2xl">{action.icon}</span>
                    <span className="text-xs font-medium text-gray-700">{action.label}</span>
                  </Link>
                ))}
              </div>
            </div>

            {/* Alert Box */}
            <div className="bg-amber-50 border border-amber-200 rounded-xl p-4">
              <div className="flex items-center gap-2 mb-3">
                <span>⚠️</span>
                <h3 className="font-semibold text-amber-800 text-sm">주의 알림</h3>
              </div>
              <ul className="space-y-2 text-sm text-amber-700">
                <li className="flex gap-2"><span>•</span><span>최근 케어 이벤트를 확인해 주세요</span></li>
              </ul>
            </div>

            {/* Care Plan Summary */}
            <div className="bg-white border border-gray-200 rounded-xl p-4">
              <div className="flex items-center justify-between mb-3">
                <h3 className="font-semibold text-gray-800 text-sm">케어 플랜 요약</h3>
                <Link href="/care" className="text-xs text-blue-600 hover:underline">자세히</Link>
              </div>
              <div className="space-y-2">
                {[
                  { label: '방문 요양', value: '주 5회', active: true },
                  { label: '물리치료', value: '주 2회', active: false },
                  { label: '인지 프로그램', value: '주 3회', active: true },
                ].map((item) => (
                  <div key={item.label} className="flex items-center justify-between text-sm">
                    <span className="text-gray-600">{item.label}</span>
                    <div className="flex items-center gap-1.5">
                      <span className={`w-2 h-2 rounded-full ${item.active ? 'bg-green-500' : 'bg-yellow-400'}`} />
                      <span className="text-gray-700 font-medium">{item.value}</span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </FamilyAppShell>
  );
}
