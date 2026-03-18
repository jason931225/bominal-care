// 예약 목록 — Appointments List
// Shows upcoming and past appointments with tab navigation
// Client component — fetches data from /api/appointments

'use client';

import Link from 'next/link';
import { useState, useEffect, useCallback } from 'react';
import SeniorAppShell from '@/components/SeniorAppShell';

type AppointmentStatus = 'upcoming' | 'completed' | 'cancelled';

interface Appointment {
  id: string;
  date: string;
  time: string;
  doctor: string;
  specialty: string;
  hospital: string;
  status: AppointmentStatus;
  notes?: string;
}

const statusLabels: Record<AppointmentStatus, { label: string; color: string }> = {
  upcoming: { label: '예정', color: 'bg-primary-100 text-primary-700' },
  completed: { label: '완료', color: 'bg-success-50 text-success-700' },
  cancelled: { label: '취소', color: 'bg-gray-100 text-gray-500' },
};

// ---------------------------------------------------------------------------
// Helpers — map DB row to Appointment UI shape
// ---------------------------------------------------------------------------

function mapRowToAppointment(row: Record<string, unknown>): Appointment {
  const appointmentDate = new Date(row.appointment_date as string);
  const dateStr = `${appointmentDate.getFullYear()}년 ${appointmentDate.getMonth() + 1}월 ${appointmentDate.getDate()}일`;
  const hours = appointmentDate.getHours();
  const minutes = appointmentDate.getMinutes().toString().padStart(2, '0');
  const period = hours < 12 ? '오전' : '오후';
  const displayHours = hours <= 12 ? hours : hours - 12;
  const timeStr = `${period} ${displayHours}:${minutes}`;

  const dbStatus = row.status as string;
  let uiStatus: AppointmentStatus = 'upcoming';
  if (dbStatus === 'COMPLETED') uiStatus = 'completed';
  else if (dbStatus === 'CANCELLED') uiStatus = 'cancelled';
  else if (dbStatus === 'SCHEDULED' || dbStatus === 'CONFIRMED') uiStatus = 'upcoming';

  return {
    id: row.id as string,
    date: dateStr,
    time: timeStr,
    doctor: (row.purpose as string) ?? '',
    specialty: (row.institution_type as string) ?? '',
    hospital: (row.institution_name as string) ?? '',
    status: uiStatus,
    notes: (row.notes as string) ?? undefined,
  };
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function AppointmentCard({ appointment }: { appointment: Appointment }) {
  const status = statusLabels[appointment.status];

  return (
    <Link
      href={`/appointments/${appointment.id}`}
      className="senior-card block hover:shadow-md active:scale-[0.99] transition-all"
    >
      <div className="flex items-start justify-between mb-3">
        <div>
          <p className="text-senior-lg font-bold text-gray-900">{appointment.doctor}</p>
          <p className="text-senior-base text-gray-600">{appointment.specialty} · {appointment.hospital}</p>
        </div>
        <span className={`${status.color} text-senior-sm font-semibold px-3 py-1 rounded-full flex-shrink-0`}>
          {status.label}
        </span>
      </div>

      <div className="flex items-center gap-2 text-gray-700">
        <svg className="w-5 h-5 text-primary-500 flex-shrink-0" fill="none" stroke="currentColor" strokeWidth={2} viewBox="0 0 24 24" aria-hidden="true">
          <path strokeLinecap="round" strokeLinejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        <span className="text-senior-base font-medium">{appointment.date} {appointment.time}</span>
      </div>

      {appointment.notes && (
        <p className="mt-2 text-senior-sm text-gray-500 bg-gray-50 rounded-lg px-3 py-2">
          📝 {appointment.notes}
        </p>
      )}
    </Link>
  );
}

// ---------------------------------------------------------------------------
// Page (client component — fetches via API)
// ---------------------------------------------------------------------------

export default function AppointmentsPage() {
  const [activeTab, setActiveTab] = useState<'upcoming' | 'past'>('upcoming');
  const [upcomingAppointments, setUpcoming] = useState<Appointment[]>([]);
  const [pastAppointments, setPast] = useState<Appointment[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchAppointments = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const response = await fetch('/api/appointments?limit=50');
      if (response.status === 401) {
        window.location.href = '/auth/signin';
        return;
      }

      if (!response.ok) {
        throw new Error('Failed to fetch appointments');
      }

      const json = await response.json();
      if (!json.success) {
        throw new Error(json.error ?? 'Unknown error');
      }

      const allAppointments: Appointment[] = (json.data as Record<string, unknown>[]).map(mapRowToAppointment);

      setUpcoming(allAppointments.filter((a) => a.status === 'upcoming'));
      setPast(allAppointments.filter((a) => a.status === 'completed' || a.status === 'cancelled'));
    } catch (err) {
      console.error('[AppointmentsPage] fetch error:', err);
      setError('예약 목록을 불러오지 못했습니다. 다시 시도해 주세요.');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchAppointments();
  }, [fetchAppointments]);

  const displayList = activeTab === 'upcoming' ? upcomingAppointments : pastAppointments;

  return (
    <SeniorAppShell>
      <div className="page-content">
        {/* Page header */}
        <div className="flex items-center justify-between mb-5">
          <h1 className="text-senior-2xl font-bold text-gray-900">진료 예약</h1>
          <Link
            href="/appointments/new"
            className="senior-btn-primary px-4 py-2 text-senior-sm"
          >
            + 예약하기
          </Link>
        </div>

        {/* Tab navigation */}
        <div className="flex bg-gray-100 rounded-xl p-1 mb-5" role="tablist" aria-label="예약 탭">
          <button
            role="tab"
            aria-selected={activeTab === 'upcoming'}
            onClick={() => setActiveTab('upcoming')}
            className={`flex-1 py-2.5 rounded-lg text-senior-base font-semibold transition-all min-h-touch
              ${activeTab === 'upcoming'
                ? 'bg-white text-primary-700 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'
              }`}
          >
            예정된 예약 ({upcomingAppointments.length})
          </button>
          <button
            role="tab"
            aria-selected={activeTab === 'past'}
            onClick={() => setActiveTab('past')}
            className={`flex-1 py-2.5 rounded-lg text-senior-base font-semibold transition-all min-h-touch
              ${activeTab === 'past'
                ? 'bg-white text-primary-700 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'
              }`}
          >
            지난 예약
          </button>
        </div>

        {/* Loading state */}
        {loading && (
          <div className="text-center py-16 text-gray-400">
            <p className="text-senior-lg font-medium">불러오는 중...</p>
          </div>
        )}

        {/* Error state */}
        {!loading && error && (
          <div className="text-center py-16">
            <p className="text-senior-base text-danger-600 mb-4">{error}</p>
            <button
              onClick={fetchAppointments}
              className="senior-btn-primary px-6 py-2"
            >
              다시 시도
            </button>
          </div>
        )}

        {/* Appointment list */}
        {!loading && !error && displayList.length === 0 ? (
          <div className="text-center py-16 text-gray-400">
            <div className="text-5xl mb-4" aria-hidden="true">📅</div>
            <p className="text-senior-lg font-medium">예약이 없습니다</p>
          </div>
        ) : null}

        {!loading && !error && displayList.length > 0 && (
          <div className="space-y-3" role="tabpanel">
            {displayList.map((apt) => (
              <AppointmentCard key={apt.id} appointment={apt} />
            ))}
          </div>
        )}
      </div>
    </SeniorAppShell>
  );
}
