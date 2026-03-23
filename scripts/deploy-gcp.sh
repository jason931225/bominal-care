#!/usr/bin/env bash
# =============================================================================
# Bominal Care — GCP Deployment Script
# Architecture: Cloud Run (us-central1) + Compute Engine Postgres (us-central1-a)
# Frontend: Firebase Hosting CDN → /api/ proxied to Cloud Run
# =============================================================================

set -euo pipefail

PROJECT_ID="bominal"
REGION="us-central1"
ZONE="us-central1-a"
REPO="bominal"
SERVICE="bominal-server"
DB_INSTANCE="bominal-deploy"
DB_INTERNAL_IP="10.128.0.8"
VPC_CONNECTOR="bominal-vpc"

echo "=== Bominal Care GCP Deployment ==="
echo "Project: $PROJECT_ID"
echo "Region:  $REGION"
echo ""

# --- Step 0: Check prerequisites ---
echo "[0/7] Checking prerequisites..."
gcloud config set project "$PROJECT_ID"
command -v docker >/dev/null || { echo "ERROR: docker not found"; exit 1; }

# --- Step 1: Enable required APIs ---
echo "[1/7] Enabling APIs..."
gcloud services enable \
  run.googleapis.com \
  artifactregistry.googleapis.com \
  cloudbuild.googleapis.com \
  firebasehosting.googleapis.com \
  vpcaccess.googleapis.com \
  compute.googleapis.com \
  --quiet

# --- Step 2: Create Artifact Registry (if not exists) ---
echo "[2/7] Setting up Artifact Registry..."
gcloud artifacts repositories describe "$REPO" \
  --location="$REGION" --format="value(name)" 2>/dev/null || \
gcloud artifacts repositories create "$REPO" \
  --repository-format=docker \
  --location="$REGION" \
  --description="Bominal Care container images"

# --- Step 3: Create VPC Connector (Cloud Run → Compute Engine internal network) ---
echo "[3/7] Setting up VPC connector..."
gcloud compute networks vpc-access connectors describe "$VPC_CONNECTOR" \
  --region="$REGION" --format="value(name)" 2>/dev/null || \
gcloud compute networks vpc-access connectors create "$VPC_CONNECTOR" \
  --region="$REGION" \
  --range="10.8.0.0/28" \
  --min-instances=2 \
  --max-instances=3

# --- Step 4: Ensure firewall allows Cloud Run → Postgres ---
echo "[4/7] Configuring firewall..."
gcloud compute firewall-rules describe allow-postgres-from-vpc-connector \
  --format="value(name)" 2>/dev/null || \
gcloud compute firewall-rules create allow-postgres-from-vpc-connector \
  --direction=INGRESS \
  --priority=1000 \
  --network=default \
  --action=ALLOW \
  --rules=tcp:5432 \
  --source-ranges="10.8.0.0/28" \
  --target-tags=postgres-server

echo ""
echo "=== IMPORTANT: Ensure your Compute Engine VM has the 'postgres-server' network tag ==="
echo "Run: gcloud compute instances add-tags $DB_INSTANCE --zone=$ZONE --tags=postgres-server"
echo ""

# --- Step 5: Set DATABASE_URL as Cloud Run secret ---
echo "[5/7] Setting DATABASE_URL..."
echo ""
echo "You need to set the DATABASE_URL secret. Run:"
echo "  echo -n 'postgresql://bominal_care:PASSWORD@${DB_INTERNAL_IP}:5432/bominal_care' | \\"
echo "    gcloud secrets create bominal-db-url --data-file=- --replication-policy=automatic"
echo ""
echo "Then grant Cloud Run access:"
echo "  gcloud secrets add-iam-policy-binding bominal-db-url \\"
echo "    --member='serviceAccount:PROJECT_NUMBER-compute@developer.gserviceaccount.com' \\"
echo "    --role='roles/secretmanager.secretAccessor'"
echo ""
read -p "Press Enter after setting up the secret (or Ctrl+C to abort)..."

# --- Step 6: Build and deploy ---
echo "[6/7] Building and deploying..."
gcloud builds submit --config=cloudbuild.yaml

# --- Step 7: Verify ---
echo "[7/7] Verifying deployment..."
URL=$(gcloud run services describe "$SERVICE" --region="$REGION" --format="value(status.url)")
echo ""
echo "=== Deployment Complete ==="
echo "Cloud Run URL: $URL"
echo "Firebase URL:  https://bominal.care (after DNS setup)"
echo ""
echo "Test: curl $URL/health"
curl -s "$URL/health" && echo ""
