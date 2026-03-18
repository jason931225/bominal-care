// 약물 목록 — Medications List
// Shows all current medications with today's dosing schedule

import Link from 'next/link';
import { redirect } from 'next/navigation';
import SeniorAppShell from '@/components/SeniorAppShell';
import { auth } from '@bominal-senior/auth';
import { pool } from '@bominal-senior/db';
import { getPersonProfileByUserId, listMedications } from '@/lib/services';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface Medication {
  id: string;
  name: string;
  nameEn: string;
  dosage: string;
  frequency: string;
  times: string[];
  prescribedBy: string;
  purpose: string;
  color: string;
  shape: string;
  remaining: number;
  refillDate: string;
}

interface TodaySlot {
  timeLabel: string;
  meds: string[];
  done: boolean;
}

// ---------------------------------------------------------------------------
// Helpers — map DB rows to UI types
// ---------------------------------------------------------------------------

const FREQUENCY_LABELS: Record<string, string> = {
  ONCE_DAILY: '하루 1회',
  TWICE_DAILY: '하루 2회',
  THREE_TIMES_DAILY: '하루 3회',
  FOUR_TIMES_DAILY: '하루 4회',
  EVERY_OTHER_DAY: '격일',
  WEEKLY: '주 1회',
  AS_NEEDED: '필요 시',
};

function mapDbMedication(
  med: Record<string, unknown>,
): Medication {
  const schedules = (med.schedules ?? []) as Array<Record<string, unknown>>;
  const times = schedules.map((s) => {
    const t = s.time_of_day as string | undefined;
    return t ?? '';
  }).filter(Boolean);

  return {
    id: med.id as string,
    name: med.name as string,
    nameEn: `${med.name as string} ${med.dosage as string}`,
    dosage: med.dosage as string,
    frequency: FREQUENCY_LABELS[med.frequency as string] ?? (med.frequency as string ?? ''),
    times,
    prescribedBy: (med.prescribed_by as string) ?? '',
    purpose: (med.notes as string) ?? '',
    color: '',
    shape: (med.form as string) ?? '',
    remaining: 0,
    refillDate: '',
  };
}

function buildTodaySchedule(medications: Medication[]): TodaySlot[] {
  const slotMap = new Map<string, string[]>();

  for (const med of medications) {
    for (const time of med.times) {
      const existing = slotMap.get(time) ?? [];
      slotMap.set(time, [...existing, `${med.name} ${med.dosage}`]);
    }
  }

  return Array.from(slotMap.entries())
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([timeLabel, meds]) => ({
      timeLabel,
      meds,
      done: false,
    }));
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function MedCard({ med }: { med: Medication }) {
  const lowStock = med.remaining > 0 && med.remaining <= 10;

  return (
    <Link
      href={`/medications/${med.id}`}
      className="senior-card block hover:shadow-md active:scale-[0.99] transition-all"
    >
      <div className="flex items-start justify-between mb-2">
        <div>
          <p className="text-senior-xl font-bold text-gray-900">{med.name}</p>
          <p className="text-senior-sm text-gray-500">{med.nameEn}</p>
        </div>
        {med.purpose && (
          <span className="bg-primary-50 text-primary-700 text-senior-sm font-semibold px-2.5 py-1 rounded-lg">
            {med.purpose}
          </span>
        )}
      </div>
      <div className="flex items-center gap-4 text-senior-base text-gray-700 mb-2">
        <span>💊 {med.dosage}</span>
        <span>🕐 {med.frequency}</span>
      </div>
      <div className="flex items-center justify-between">
        <span className="text-senior-sm text-gray-500">{med.times.join(', ')}</span>
        {med.remaining > 0 && (
          <span className={`text-senior-sm font-semibold ${lowStock ? 'text-danger-600' : 'text-gray-500'}`}>
            {lowStock ? `⚠️ 잔여 ${med.remaining}정` : `잔여 ${med.remaining}정`}
          </span>
        )}
      </div>
    </Link>
  );
}

// ---------------------------------------------------------------------------
// Page (async server component)
// ---------------------------------------------------------------------------

export default async function MedicationsPage() {
  const session = await auth();
  if (!session?.user?.id) {
    redirect('/auth/signin');
  }

  let medications: Medication[] = [];
  let todaySchedule: TodaySlot[] = [];

  try {
    const personProfile = await getPersonProfileByUserId(pool, session.user.id);

    if (personProfile) {
      const dbMeds = await listMedications(pool, personProfile.id);
      medications = dbMeds.map(mapDbMedication);
      todaySchedule = buildTodaySchedule(medications);
    }
  } catch (error) {
    console.error('[MedicationsPage] Failed to fetch medications:', error);
  }

  const lowStockMeds = medications.filter((m) => m.remaining > 0 && m.remaining <= 10);

  return (
    <SeniorAppShell>
      <div className="page-content">
        <div className="flex items-center justify-between mb-5">
          <h1 className="text-senior-2xl font-bold text-gray-900">내 약물</h1>
          <Link href="/medications/log" className="text-senior-sm text-primary-600 font-medium min-h-touch flex items-center">
            복용 기록
          </Link>
        </div>

        {/* Low stock warning */}
        {lowStockMeds.length > 0 && (
          <div className="bg-danger-50 border-2 border-danger-500 rounded-2xl p-4 mb-5">
            <p className="text-senior-base font-bold text-danger-700 mb-1">⚠️ 약 재처방이 필요합니다</p>
            {lowStockMeds.map((m) => (
              <p key={m.id} className="text-senior-sm text-danger-600">
                {m.name} — 잔여 {m.remaining}정 {m.refillDate ? `(재처방일: ${m.refillDate})` : ''}
              </p>
            ))}
          </div>
        )}

        {/* Today's schedule */}
        {todaySchedule.length > 0 && (
          <section className="senior-card mb-5" aria-labelledby="today-schedule">
            <div className="flex items-center justify-between mb-3">
              <h2 id="today-schedule" className="text-senior-lg font-bold text-gray-800">오늘 복용 일정</h2>
              <Link href="/medications/log" className="text-senior-sm text-primary-600 font-medium">
                기록하기
              </Link>
            </div>
            <div className="space-y-3">
              {todaySchedule.map((slot) => (
                <div key={slot.timeLabel} className="flex items-start gap-3">
                  <div
                    className={`w-5 h-5 rounded-full border-2 flex-shrink-0 mt-0.5
                      ${slot.done ? 'bg-success-500 border-success-500' : 'border-gray-300'}`}
                    aria-label={slot.done ? '복용 완료' : '미복용'}
                  />
                  <div className="flex-1">
                    <p className={`text-senior-base font-semibold ${slot.done ? 'text-gray-400 line-through' : 'text-gray-800'}`}>
                      {slot.timeLabel}
                    </p>
                    <p className="text-senior-sm text-gray-500">{slot.meds.join(', ')}</p>
                  </div>
                  {!slot.done && (
                    <button
                      className="text-senior-sm text-white bg-success-500 hover:bg-success-700 px-3 py-1.5 rounded-lg font-medium min-h-touch"
                      aria-label={`${slot.timeLabel} 복용 완료로 표시`}
                    >
                      완료
                    </button>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {/* All medications */}
        <section aria-labelledby="all-meds-heading">
          <h2 id="all-meds-heading" className="senior-section-title">전체 약물 목록</h2>
          {medications.length === 0 ? (
            <div className="text-center py-12 text-gray-400">
              <div className="text-5xl mb-4" aria-hidden="true">💊</div>
              <p className="text-senior-lg font-medium">등록된 약물이 없습니다</p>
            </div>
          ) : (
            <div className="space-y-3">
              {medications.map((med) => (
                <MedCard key={med.id} med={med} />
              ))}
            </div>
          )}
        </section>
      </div>
    </SeniorAppShell>
  );
}
