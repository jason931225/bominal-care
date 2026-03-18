'use client';
import { type ReactNode } from 'react';
import { cn } from '../../lib/utils';

export interface WizardStep {
  label: string;
  description?: string;
}

interface StepWizardProps {
  steps: WizardStep[];
  currentStep: number;
  onStepChange?: (step: number) => void;
  children: ReactNode;
  className?: string;
}

export function StepWizard({ steps, currentStep, onStepChange, children, className }: StepWizardProps) {
  return (
    <div className={cn('flex flex-col gap-6', className)}>
      <nav aria-label="단계 진행" className="flex items-start gap-0">
        {steps.map((step, idx) => {
          const isDone = idx < currentStep;
          const isCurrent = idx === currentStep;
          return (
            <div key={idx} className="flex flex-1 flex-col items-center gap-1 relative">
              {idx < steps.length - 1 && (
                <div
                  className={cn(
                    'absolute top-4 left-1/2 right-0 h-0.5 -translate-y-1/2',
                    isDone ? 'bg-primary-600' : 'bg-gray-200',
                  )}
                  aria-hidden="true"
                />
              )}
              <button
                type="button"
                onClick={() => onStepChange?.(idx)}
                disabled={!onStepChange || idx > currentStep}
                className={cn(
                  'relative z-10 flex h-8 w-8 items-center justify-center rounded-full text-sm font-semibold transition-colors',
                  isDone && 'bg-primary-600 text-white',
                  isCurrent && 'bg-primary-600 text-white ring-4 ring-primary-100',
                  !isDone && !isCurrent && 'bg-gray-200 text-gray-500',
                  onStepChange && idx <= currentStep && 'cursor-pointer hover:bg-primary-700',
                )}
                aria-current={isCurrent ? 'step' : undefined}
              >
                {isDone ? '✓' : idx + 1}
              </button>
              <span className={cn('text-xs text-center', isCurrent ? 'text-primary-700 font-medium' : 'text-gray-500')}>
                {step.label}
              </span>
            </div>
          );
        })}
      </nav>
      <div>{children}</div>
    </div>
  );
}
