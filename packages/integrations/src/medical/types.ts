export interface HealthRecord {
  readonly patientId: string;
  readonly conditions: readonly { code: string; name: string; diagnosedAt: string }[];
  readonly medications: readonly { name: string; dosage: string; prescribedAt: string }[];
  readonly lastVisit?: string;
}

export interface MedicalProvider {
  fetchHealthRecord(patientId: string, consentToken: string): Promise<HealthRecord>;
  syncMedications(
    patientId: string,
    consentToken: string,
  ): Promise<readonly { name: string; dosage: string }[]>;
}
