import type { DocumentProvider, SignatureRequest, SignatureResult } from './types.js';

let signatureCounter = 0;

function generateSignatureId(): string {
  signatureCounter += 1;
  return `mock-sig-${Date.now()}-${signatureCounter}`;
}

// In-memory store of pending/signed documents
const signatureStore = new Map<string, SignatureResult>();

/**
 * Mock document signature provider for development and testing.
 * All signature requests are immediately auto-signed with a mock signed document URL.
 */
export class MockDocumentProvider implements DocumentProvider {
  async requestSignature(request: SignatureRequest): Promise<SignatureResult> {
    const signatureId = generateSignatureId();
    const signedAt = new Date();

    // Derive a mock signed document URL by appending a signed marker
    const signedDocumentUrl = `${request.documentUrl}?signed=true&id=${signatureId}`;

    const result: SignatureResult = {
      signatureId,
      status: 'signed',
      signedAt,
      signedDocumentUrl,
    };

    signatureStore.set(signatureId, result);

    console.log(`[MockDocument] Signature requested and auto-signed`, {
      signatureId,
      signerName: request.signerName,
      signerEmail: request.signerEmail,
      purpose: request.purpose,
      signedAt: signedAt.toISOString(),
    });

    return result;
  }

  async getSignatureStatus(signatureId: string): Promise<SignatureResult> {
    const stored = signatureStore.get(signatureId);

    if (stored) {
      return stored;
    }

    // Return a pending status for unknown signature IDs
    return {
      signatureId,
      status: 'pending',
    };
  }
}
