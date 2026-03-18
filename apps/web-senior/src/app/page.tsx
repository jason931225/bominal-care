// 홈 대시보드 — Today Dashboard
// Shows personalised greeting, today's schedule, medication reminders, and quick actions

import Link from 'next/link';
import { redirect } from 'next/navigation';
import SeniorAppShell from '@/components/SeniorAppShell';
import { auth } from '@bominal-senior/auth';
import { pool } from '@bominal-senior/db';
import {
  getPersonProfileByUserId,
  listMedications,
  getUpcomingAppointments,
} from '@/lib/services';

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
// Static quick actions (no DB needed)
// ---------------------------------------------------------------------------

const quickActions = [
  { href: '/appointments/new', label: '진료 예약', emoji: '📅', bg: 'bg-primary-50', text: 'text-primary-700', border: 'border-primary-200' },
  { href: '/medications', label: '약 복용', emoji: '💊', bg: 'bg-warning-50', text: 'text-warning-700', border: 'border-warning-200' },
  { href: '/emergency', label: '긴급 연락', emoji: '🚨', bg: 'bg-danger-50', text: 'text-danger-700', border: 'border-danger-200' },
  { href: '/services', label: '서비스', emoji: '🛎️', bg: 'bg-secondary-50', text: 'text-secondary-700', border: 'border-secondary-200' },
];

// ---------------------------------------------------------------------------
// Schedule item type coloring helper
// ---------------------------------------------------------------------------

interface ScheduleItem {
  id: string;
  time: string;
  title: string;
  type: 'medication' | 'appointment' | 'care';
  done: boolean;
  color: string;
  textColor: string;
  badgeColor: string;
}

function getScheduleColors(type: string) {
  switch (type) {
    case 'medication':
      return { color: 'bg-warning-50 border-warning-500', textColor: 'text-warning-700', badgeColor: 'bg-warning-500' };
    case 'appointment':
      return { color: 'bg-primary-50 border-primary-400', textColor: 'text-primary-700', badgeColor: 'bg-primary-500' };
    case 'care':
      return { color: 'bg-secondary-50 border-secondary-500', textColor: 'text-secondary-700', badgeColor: 'bg-secondary-500' };
    default:
      return { color: 'bg-gray-50 border-gray-500', textColor: 'text-gray-700', badgeColor: 'bg-gray-500' };
  }
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function GreetingBanner({ userName, todayDate }: { userName: string; todayDate: string }) {
  return (
    <div className="bg-gradient-to-br from-primary-600 to-primary-700 rounded-2xl p-5 text-white mb-5">
      <p className="text-senior-sm text-primary-200 mb-1">{todayDate}</p>
      <h1 className="text-senior-2xl font-bold mb-1">
        안녕하세요, {userName}님! 👋
      </h1>
      <p className="text-senior-base text-primary-100">
        오늘도 건강하고 활기찬 하루 되세요.
      </p>
    </div>
  );
}

function MedicationAlertBanner({ untakenMeds }: { untakenMeds: Array<{ name: string; dosage: string }> }) {
  if (untakenMeds.length === 0) return null;

  return (
    <div className="bg-warning-50 border-2 border-warning-500 rounded-2xl p-4 mb-5 flex items-center gap-3">
      <div className="text-3xl" aria-hidden="true">💊</div>
      <div className="flex-1">
        <p className="text-senior-base font-bold text-warning-700">복용하지 않은 약이 있습니다</p>
        <p className="text-senior-sm text-warning-600">약 {untakenMeds.length}가지</p>
      </div>
      <Link
        href="/medications"
        className="bg-warning-500 hover:bg-warning-700 text-white font-bold text-senior-sm px-4 py-2 rounded-xl min-h-touch flex items-center"
      >
        확인
      </Link>
    </div>
  );
}

function TodaySchedule({ schedule }: { schedule: ScheduleItem[] }) {
  return (
    <section className="mb-5" aria-labelledby="schedule-heading">
      <div className="flex items-center justify-between mb-3">
        <h2 id="schedule-heading" className="senior-section-title mb-0">오늘 일정</h2>
        <Link href="/appointments" className="text-senior-sm text-primary-600 font-medium min-h-touch flex items-center">
          전체 보기
        </Link>
      </div>
      {schedule.length === 0 ? (
        <div className="text-center py-8 text-gray-400">
          <p className="text-senior-base">오늘 예정된 일정이 없습니다</p>
        </div>
      ) : (
        <div className="space-y-2">
          {schedule.map((item) => (
            <div
              key={item.id}
              className={`${item.color} border-l-4 rounded-r-xl rounded-l-sm p-4 flex items-center gap-3`}
            >
              <div className="text-center min-w-[52px]">
                <span className={`text-senior-sm font-bold ${item.textColor}`}>{item.time}</span>
              </div>
              <div className="flex-1">
                <p className={`text-senior-base font-semibold ${item.textColor} ${item.done ? 'line-through opacity-60' : ''}`}>
                  {item.title}
                </p>
              </div>
              {item.done && (
                <span className="text-success-500" aria-label="완료">
                  <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                  </svg>
                </span>
              )}
            </div>
          ))}
        </div>
      )}
    </section>
  );
}

function QuickActions() {
  return (
    <section className="mb-5" aria-labelledby="quick-actions-heading">
      <h2 id="quick-actions-heading" className="senior-section-title">빠른 실행</h2>
      <div className="grid grid-cols-2 gap-3">
        {quickActions.map(({ href, label, emoji, bg, text, border }) => (
          <Link
            key={href}
            href={href}
            className={`${bg} ${border} border-2 rounded-2xl p-4 flex flex-col items-center gap-2 min-h-touch-senior justify-center hover:opacity-80 active:scale-95 transition-all`}
          >
            <span className="text-3xl" aria-hidden="true">{emoji}</span>
            <span className={`text-senior-base font-bold ${text}`}>{label}</span>
          </Link>
        ))}
      </div>
    </section>
  );
}

function MedicationReminders({ medications }: { medications: Array<{ id: string; name: string; dosage: string }> }) {
  return (
    <section className="mb-5" aria-labelledby="med-reminders-heading">
      <div className="flex items-center justify-between mb-3">
        <h2 id="med-reminders-heading" className="senior-section-title mb-0">약 복용 목록</h2>
        <Link href="/medications/log" className="text-senior-sm text-primary-600 font-medium min-h-touch flex items-center">
          기록 보기
        </Link>
      </div>
      {medications.length === 0 ? (
        <div className="senior-card text-center py-6 text-gray-400">
          <p className="text-senior-base">등록된 약물이 없습니다</p>
        </div>
      ) : (
        <div className="senior-card space-y-3">
          {medications.map((med) => (
            <div key={med.id} className="flex items-center gap-3 py-1">
              <div className="w-5 h-5 rounded-full border-2 flex-shrink-0 border-gray-300" aria-hidden="true" />
              <div className="flex-1">
                <p className="text-senior-base font-semibold text-gray-800">{med.name}</p>
                <p className="text-senior-sm text-gray-500">{med.dosage}</p>
              </div>
            </div>
          ))}
          <Link
            href="/medications"
            className="senior-btn-primary w-full mt-2 text-center"
          >
            복용 완료 기록하기
          </Link>
        </div>
      )}
    </section>
  );
}

// ---------------------------------------------------------------------------
// Page (async server component)
// ---------------------------------------------------------------------------

export default async function HomePage() {
  const session = await auth();
  if (!session?.user?.id) {
    redirect('/auth/signin');
  }

  const today = new Date();
  const todayDate = formatKoreanDate(today);

  let userName = session.user.name ?? '사용자';
  let scheduleItems: ScheduleItem[] = [];
  let medicationList: Array<{ id: string; name: string; dosage: string }> = [];

  try {
    const personProfile = await getPersonProfileByUserId(pool, session.user.id);

    if (personProfile) {
      userName = personProfile.firstName || session.user.name || '사용자';

      // Fetch appointments and medications in parallel
      const [appointments, medications] = await Promise.all([
        getUpcomingAppointments(pool, personProfile.id, 5),
        listMedications(pool, personProfile.id),
      ]);

      // Map appointments to schedule items
      const appointmentSchedule: ScheduleItem[] = appointments.map((apt) => {
        const aptDate = new Date(apt.appointment_date as string);
        const hours = aptDate.getHours().toString().padStart(2, '0');
        const minutes = aptDate.getMinutes().toString().padStart(2, '0');
        const colors = getScheduleColors('appointment');
        return {
          id: apt.id as string,
          time: `${hours}:${minutes}`,
          title: `${apt.purpose ?? '진료'} — ${apt.institution_name as string}`,
          type: 'appointment' as const,
          done: false,
          ...colors,
        };
      });

      // Map medications to schedule items and reminders
      medicationList = medications.map((med) => ({
        id: med.id as string,
        name: med.name as string,
        dosage: med.dosage as string,
      }));

      const medicationSchedule: ScheduleItem[] = medications.map((med) => {
        const colors = getScheduleColors('medication');
        const schedules = med.schedules as Array<Record<string, unknown>>;
        const firstTime = schedules.length > 0 ? (schedules[0]?.time_of_day as string ?? '08:00') : '08:00';
        return {
          id: `med-${med.id as string}`,
          time: firstTime,
          title: `${med.name as string} ${med.dosage as string}`,
          type: 'medication' as const,
          done: false,
          ...colors,
        };
      });

      // Combine and sort by time
      scheduleItems = [...appointmentSchedule, ...medicationSchedule].sort((a, b) =>
        a.time.localeCompare(b.time),
      );
    }
  } catch (error) {
    console.error('[HomePage] Failed to fetch data:', error);
    // Continue with defaults — UI will show empty states
  }

  return (
    <SeniorAppShell>
      <div className="page-content">
        <GreetingBanner userName={userName} todayDate={todayDate} />
        <MedicationAlertBanner untakenMeds={medicationList} />
        <QuickActions />
        <TodaySchedule schedule={scheduleItems} />
        <MedicationReminders medications={medicationList} />
      </div>
    </SeniorAppShell>
  );
}
