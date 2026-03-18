'use client';
import { cn } from '../../lib/utils';

type MedStatus = 'pending' | 'taken' | 'missed';

interface MedicationCardProps {
  name: string;
  dosage: string;
  time: string;
  status?: MedStatus;
  onTaken?: () => void;
  onMissed?: () => void;
  className?: string;
}

const statusLabels: Record<MedStatus, string> = {
  pending: '대기',
  taken: '복용 완료',
  missed: '복용 안 함',
};

const statusStyles: Record<MedStatus, string> = {
  pending: 'bg-gray-100 text-gray-700',
  taken: 'bg-success-50 text-success-700',
  missed: 'bg-danger-50 text-danger-700',
};

export function MedicationCard({ name, dosage, time, status = 'pending', onTaken, onMissed, className }: MedicationCardProps) {
  return (
    <div className={cn('rounded-xl border border-gray-200 bg-white p-4', className)}>
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className="text-base font-bold text-gray-900">{name}</p>
          <p className="text-sm text-gray-500">{dosage}</p>
          <p className="mt-1 text-sm text-gray-400">{time}</p>
        </div>
        <span className={cn('rounded-full px-2.5 py-1 text-xs font-medium', statusStyles[status])}>
          {statusLabels[status]}
        </span>
      </div>
      {status === 'pending' && (onTaken || onMissed) && (
        <div className="mt-4 flex gap-3">
          {onTaken && (
            <button
              type="button"
              onClick={onTaken}
              className="flex-1 rounded-lg bg-success-500 py-2.5 text-sm font-semibold text-white hover:bg-success-600 transition-colors min-h-[44px]"
            >
              복용 완료
            </button>
          )}
          {onMissed && (
            <button
              type="button"
              onClick={onMissed}
              className="flex-1 rounded-lg border border-danger-300 py-2.5 text-sm font-semibold text-danger-600 hover:bg-danger-50 transition-colors min-h-[44px]"
            >
              복용 안 함
            </button>
          )}
        </div>
      )}
    </div>
  );
}
