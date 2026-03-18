import type { HealthRecord, MedicalProvider } from './types.js';

// Sample Korean medical data representative of common conditions in elderly patients
const SAMPLE_CONDITIONS: HealthRecord['conditions'] = [
  { code: 'I10', name: '본태성 고혈압', diagnosedAt: '2018-03-15' },
  { code: 'E11', name: '2형 당뇨병', diagnosedAt: '2020-07-22' },
  { code: 'M81.0', name: '폐경 후 골다공증', diagnosedAt: '2021-11-05' },
];

const SAMPLE_MEDICATIONS: HealthRecord['medications'] = [
  { name: '암로디핀', dosage: '5mg 1일 1회', prescribedAt: '2018-04-01' },
  { name: '메트포르민', dosage: '500mg 1일 2회', prescribedAt: '2020-08-10' },
  { name: '알렌드론산', dosage: '70mg 주 1회', prescribedAt: '2021-12-01' },
  { name: '아스피린', dosage: '100mg 1일 1회', prescribedAt: '2019-01-20' },
];

/**
 * Mock medical provider (My Healthway / 나의건강기록 adapter) for development and testing.
 * Returns realistic Korean health record data for elderly patients.
 * The consentToken parameter is accepted but not validated in mock mode.
 */
export class MockMedicalProvider implements MedicalProvider {
  async fetchHealthRecord(patientId: string, _consentToken: string): Promise<HealthRecord> {
    console.log(`[MockMedical] Fetching health record`, { patientId });

    return {
      patientId,
      conditions: SAMPLE_CONDITIONS,
      medications: SAMPLE_MEDICATIONS,
      lastVisit: '2026-01-10',
    };
  }

  async syncMedications(
    patientId: string,
    _consentToken: string,
  ): Promise<readonly { name: string; dosage: string }[]> {
    console.log(`[MockMedical] Syncing medications`, { patientId });

    return SAMPLE_MEDICATIONS.map(({ name, dosage }) => ({ name, dosage }));
  }
}
