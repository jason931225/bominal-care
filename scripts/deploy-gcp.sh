#!/usr/bin/env bash
# =============================================================================
# Bominal Care — GCP Deployment Script (Always Free Tier)
#
# Architecture:
#   Cloud Run (us-central1, scale-to-zero) → Direct VPC egress → Compute Engine Postgres
#   Firebase Hosting CDN → /api/ proxied to Cloud Run
#
# Free tier usage:
#   - Cloud Run: min-instances=0, 256Mi, scale to zero when idle
#   - Cloud Build: default machine (120 free min/day)
#   - Direct VPC egress: free (no VPC connector needed)
#   - Compute Engine: e2-micro (always free in us-central1)
#   - Firebase Hosting: 10GB storage, 360MB/day transfer
#   - Artifact Registry: 0.5GB free
#   - Secret Manager: 6 versions free
# =============================================================================

set -euo pipefail

PROJECT_ID="bominal"
REGION="us-central1"
ZONE="us-central1-a"

echo "=== Bominal Care GCP Deployment (Free Tier) ==="
echo "Project: $PROJECT_ID | Region: $REGION"
echo ""

# --- Step 1: Enable APIs ---
echo "[1/4] Enabling APIs..."
gcloud services enable \
  run.googleapis.com \
  artifactregistry.googleapis.com \
  cloudbuild.googleapis.com \
  firebasehosting.googleapis.com \
  secretmanager.googleapis.com \
  compute.googleapis.com \
  --quiet

# --- Step 2: Verify prerequisites ---
echo "[2/4] Checking prerequisites..."
gcloud secrets describe bominal-db-url --format="value(name)" >/dev/null 2>&1 || {
  echo "ERROR: Secret 'bominal-db-url' not found. Create it with:"
  echo "  echo -n 'postgresql://USER:PASS@10.128.0.8:5432/bominal_care' | gcloud secrets create bominal-db-url --data-file=-"
  exit 1
}
echo "  ✓ bominal-db-url secret exists"

gcloud compute instances describe bominal-deploy --zone=$ZONE --format="value(status)" 2>/dev/null | grep -q RUNNING || {
  echo "WARNING: bominal-deploy VM is not running. Starting..."
  gcloud compute instances start bominal-deploy --zone=$ZONE
  sleep 10
}
echo "  ✓ bominal-deploy VM running"

# --- Step 3: Build and deploy ---
echo "[3/4] Building and deploying (this takes ~30min on free tier machine)..."
gcloud builds submit --config=cloudbuild.yaml

# --- Step 4: Verify ---
echo "[4/4] Verifying..."
URL=$(gcloud run services describe bominal-server --region=$REGION --format="value(status.url)" 2>/dev/null)
echo ""
echo "=== Deployment Complete ==="
echo "Cloud Run:  $URL"
echo "Health:     $URL/health"
echo ""
echo "Next: Set up care.bominal.com DNS in Firebase Hosting console"
