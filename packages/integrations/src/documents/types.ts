export interface SignatureRequest {
  readonly documentUrl: string;
  readonly signerName: string;
  readonly signerEmail: string;
  readonly purpose: string;
}

export interface SignatureResult {
  readonly signatureId: string;
  readonly status: 'pending' | 'signed' | 'declined';
  readonly signedAt?: Date;
  readonly signedDocumentUrl?: string;
}

export interface DocumentProvider {
  requestSignature(request: SignatureRequest): Promise<SignatureResult>;
  getSignatureStatus(signatureId: string): Promise<SignatureResult>;
}
