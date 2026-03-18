// =============================================================================
// Integration Providers — external service abstractions + mock implementations
// Ported from packages/integrations/src/
// =============================================================================

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use chrono::Utc;
use serde::{Deserialize, Serialize};

// =============================================================================
// Identity Provider
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationRequest {
    pub name: String,
    pub date_of_birth: String,
    pub phone: String,
    pub identity_document_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KycLevel {
    PhoneVerified,
    IdentityVerified,
    FullVerified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationResult {
    pub verified: bool,
    pub kyc_level: KycLevel,
    pub verified_at: chrono::DateTime<Utc>,
    pub provider: String,
}

pub trait IdentityProvider: Send + Sync {
    fn verify(
        &self,
        request: IdentityVerificationRequest,
    ) -> impl std::future::Future<Output = Result<IdentityVerificationResult, anyhow::Error>> + Send;
}

// =============================================================================
// Messaging Provider
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageChannel {
    Sms,
    Kakao,
    Push,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    pub to: String,
    pub channel: MessageChannel,
    pub title: Option<String>,
    pub body: String,
    pub template_id: Option<String>,
    pub template_vars: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageStatus {
    Sent,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResult {
    pub message_id: String,
    pub sent_at: chrono::DateTime<Utc>,
    pub channel: MessageChannel,
    pub status: MessageStatus,
}

pub trait MessagingProvider: Send + Sync {
    fn send(
        &self,
        request: MessageRequest,
    ) -> impl std::future::Future<Output = Result<MessageResult, anyhow::Error>> + Send;

    fn send_bulk(
        &self,
        requests: Vec<MessageRequest>,
    ) -> impl std::future::Future<Output = Result<Vec<MessageResult>, anyhow::Error>> + Send;
}

// =============================================================================
// Map Provider
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeocodingResult {
    pub latitude: f64,
    pub longitude: f64,
    pub address: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceResult {
    pub distance_km: f64,
    pub duration_minutes: u32,
}

pub trait MapProvider: Send + Sync {
    fn geocode(
        &self,
        address: &str,
    ) -> impl std::future::Future<Output = Result<GeocodingResult, anyhow::Error>> + Send;

    fn reverse_geocode(
        &self,
        lat: f64,
        lng: f64,
    ) -> impl std::future::Future<Output = Result<GeocodingResult, anyhow::Error>> + Send;

    fn calculate_distance(
        &self,
        from: (f64, f64),
        to: (f64, f64),
    ) -> impl std::future::Future<Output = Result<DistanceResult, anyhow::Error>> + Send;
}

// =============================================================================
// Payment Provider
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub customer_name: String,
    pub customer_email: Option<String>,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Completed,
    Failed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResult {
    pub payment_id: String,
    pub status: PaymentStatus,
    pub amount: f64,
    pub currency: String,
    pub paid_at: Option<chrono::DateTime<Utc>>,
}

pub trait PaymentProvider: Send + Sync {
    fn create_payment(
        &self,
        request: PaymentRequest,
    ) -> impl std::future::Future<Output = Result<PaymentResult, anyhow::Error>> + Send;

    fn get_payment(
        &self,
        payment_id: &str,
    ) -> impl std::future::Future<Output = Result<PaymentResult, anyhow::Error>> + Send;

    fn refund(
        &self,
        payment_id: &str,
        amount: Option<f64>,
    ) -> impl std::future::Future<Output = Result<PaymentResult, anyhow::Error>> + Send;
}

// =============================================================================
// Document Provider
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureRequest {
    pub document_url: String,
    pub signer_name: String,
    pub signer_email: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureStatus {
    Pending,
    Signed,
    Declined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureResult {
    pub signature_id: String,
    pub status: SignatureStatus,
    pub signed_at: Option<chrono::DateTime<Utc>>,
    pub signed_document_url: Option<String>,
}

pub trait DocumentProvider: Send + Sync {
    fn request_signature(
        &self,
        request: SignatureRequest,
    ) -> impl std::future::Future<Output = Result<SignatureResult, anyhow::Error>> + Send;

    fn get_signature_status(
        &self,
        signature_id: &str,
    ) -> impl std::future::Future<Output = Result<SignatureResult, anyhow::Error>> + Send;
}

// =============================================================================
// Medical Provider
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub code: String,
    pub name: String,
    pub diagnosed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medication {
    pub name: String,
    pub dosage: String,
    pub prescribed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthRecord {
    pub patient_id: String,
    pub conditions: Vec<Condition>,
    pub medications: Vec<Medication>,
    pub last_visit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationSync {
    pub name: String,
    pub dosage: String,
}

pub trait MedicalProvider: Send + Sync {
    fn fetch_health_record(
        &self,
        patient_id: &str,
        consent_token: &str,
    ) -> impl std::future::Future<Output = Result<HealthRecord, anyhow::Error>> + Send;

    fn sync_medications(
        &self,
        patient_id: &str,
        consent_token: &str,
    ) -> impl std::future::Future<Output = Result<Vec<MedicationSync>, anyhow::Error>> + Send;
}

// =============================================================================
// Mock Implementations
// =============================================================================

// -- Mock Identity Provider ---------------------------------------------------

pub struct MockIdentityProvider;

impl IdentityProvider for MockIdentityProvider {
    async fn verify(
        &self,
        request: IdentityVerificationRequest,
    ) -> Result<IdentityVerificationResult, anyhow::Error> {
        let kyc_level = if request.identity_document_url.is_some() {
            KycLevel::FullVerified
        } else if !request.date_of_birth.is_empty() {
            KycLevel::IdentityVerified
        } else {
            KycLevel::PhoneVerified
        };

        Ok(IdentityVerificationResult {
            verified: true,
            kyc_level,
            verified_at: Utc::now(),
            provider: "MockIdentityProvider".to_string(),
        })
    }
}

// -- Mock Messaging Provider --------------------------------------------------

pub struct MockMessagingProvider {
    counter: AtomicU64,
}

impl MockMessagingProvider {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }

    fn next_message_id(&self) -> String {
        let n = self.counter.fetch_add(1, Ordering::Relaxed);
        format!("mock-msg-{}-{n}", Utc::now().timestamp_millis())
    }
}

impl Default for MockMessagingProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl MessagingProvider for MockMessagingProvider {
    async fn send(&self, request: MessageRequest) -> Result<MessageResult, anyhow::Error> {
        let message_id = self.next_message_id();
        let sent_at = Utc::now();

        tracing::info!(
            message_id = %message_id,
            to = %request.to,
            channel = ?request.channel,
            title = ?request.title,
            body = %request.body,
            "[MockMessaging] Sending message",
        );

        Ok(MessageResult {
            message_id,
            sent_at,
            channel: request.channel,
            status: MessageStatus::Sent,
        })
    }

    async fn send_bulk(
        &self,
        requests: Vec<MessageRequest>,
    ) -> Result<Vec<MessageResult>, anyhow::Error> {
        tracing::info!(count = requests.len(), "[MockMessaging] Sending bulk messages");
        let mut results = Vec::with_capacity(requests.len());
        for req in requests {
            results.push(self.send(req).await?);
        }
        Ok(results)
    }
}

// -- Mock Map Provider --------------------------------------------------------

/// Well-known Seoul district coordinates for geocoding mock responses.
struct DistrictGeocode {
    name: &'static str,
    result: GeocodingResult,
}

const SEOUL_DEFAULT_LAT: f64 = 37.5665;
const SEOUL_DEFAULT_LNG: f64 = 126.978;

fn seoul_default_geocode() -> GeocodingResult {
    GeocodingResult {
        latitude: SEOUL_DEFAULT_LAT,
        longitude: SEOUL_DEFAULT_LNG,
        address: "서울특별시 중구 태평로1가 31".to_string(),
        display_name: "서울시청, 중구, 서울특별시".to_string(),
    }
}

fn district_geocodes() -> Vec<DistrictGeocode> {
    vec![
        DistrictGeocode {
            name: "강남구",
            result: GeocodingResult {
                latitude: 37.5172,
                longitude: 127.0473,
                address: "서울특별시 강남구".to_string(),
                display_name: "강남구, 서울특별시".to_string(),
            },
        },
        DistrictGeocode {
            name: "종로구",
            result: GeocodingResult {
                latitude: 37.5735,
                longitude: 126.979,
                address: "서울특별시 종로구".to_string(),
                display_name: "종로구, 서울특별시".to_string(),
            },
        },
        DistrictGeocode {
            name: "마포구",
            result: GeocodingResult {
                latitude: 37.5663,
                longitude: 126.9019,
                address: "서울특별시 마포구".to_string(),
                display_name: "마포구, 서울특별시".to_string(),
            },
        },
        DistrictGeocode {
            name: "송파구",
            result: GeocodingResult {
                latitude: 37.5145,
                longitude: 127.1059,
                address: "서울특별시 송파구".to_string(),
                display_name: "송파구, 서울특별시".to_string(),
            },
        },
        DistrictGeocode {
            name: "서대문구",
            result: GeocodingResult {
                latitude: 37.5791,
                longitude: 126.9368,
                address: "서울특별시 서대문구".to_string(),
                display_name: "서대문구, 서울특별시".to_string(),
            },
        },
    ]
}

pub struct MockMapProvider;

impl MapProvider for MockMapProvider {
    async fn geocode(&self, address: &str) -> Result<GeocodingResult, anyhow::Error> {
        for dg in district_geocodes() {
            if address.contains(dg.name) {
                return Ok(dg.result);
            }
        }
        Ok(GeocodingResult {
            address: address.to_string(),
            display_name: address.to_string(),
            ..seoul_default_geocode()
        })
    }

    async fn reverse_geocode(&self, lat: f64, lng: f64) -> Result<GeocodingResult, anyhow::Error> {
        let mut nearest = seoul_default_geocode();
        let mut min_dist = f64::INFINITY;

        for dg in district_geocodes() {
            let dist = euclidean_distance(lat, lng, dg.result.latitude, dg.result.longitude);
            if dist < min_dist {
                min_dist = dist;
                nearest = dg.result;
            }
        }

        Ok(nearest)
    }

    async fn calculate_distance(
        &self,
        from: (f64, f64),
        to: (f64, f64),
    ) -> Result<DistanceResult, anyhow::Error> {
        let straight_line_km = haversine_km(from.0, from.1, to.0, to.1);

        // Apply a road-network factor of ~1.3 for urban Seoul
        let distance_km = (straight_line_km * 1.3 * 100.0).round() / 100.0;

        // Estimate driving time: average 30 km/h in urban Seoul
        let duration_minutes = ((distance_km / 30.0) * 60.0).round() as u32;

        Ok(DistanceResult {
            distance_km,
            duration_minutes,
        })
    }
}

fn euclidean_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    ((lat1 - lat2).powi(2) + (lng1 - lng2).powi(2)).sqrt()
}

fn haversine_km(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let r = 6371.0_f64; // Earth radius in km
    let d_lat = (lat2 - lat1).to_radians();
    let d_lng = (lng2 - lng1).to_radians();
    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lng / 2.0).sin().powi(2);
    r * 2.0 * a.sqrt().atan2((1.0 - a).sqrt())
}

// -- Mock Payment Provider ----------------------------------------------------

pub struct MockPaymentProvider {
    counter: AtomicU64,
    store: Mutex<HashMap<String, PaymentResult>>,
}

impl MockPaymentProvider {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            store: Mutex::new(HashMap::new()),
        }
    }

    fn next_payment_id(&self) -> String {
        let n = self.counter.fetch_add(1, Ordering::Relaxed);
        format!("mock-pay-{}-{n}", Utc::now().timestamp_millis())
    }
}

impl Default for MockPaymentProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl PaymentProvider for MockPaymentProvider {
    async fn create_payment(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, anyhow::Error> {
        let payment_id = self.next_payment_id();
        let paid_at = Utc::now();

        let result = PaymentResult {
            payment_id: payment_id.clone(),
            status: PaymentStatus::Completed,
            amount: request.amount,
            currency: request.currency,
            paid_at: Some(paid_at),
        };

        self.store
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?
            .insert(payment_id.clone(), result.clone());

        tracing::info!(
            payment_id = %payment_id,
            order_id = %request.order_id,
            customer_name = %request.customer_name,
            amount = request.amount,
            "[MockPayment] Payment created",
        );

        Ok(result)
    }

    async fn get_payment(&self, payment_id: &str) -> Result<PaymentResult, anyhow::Error> {
        let store = self
            .store
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;

        if let Some(stored) = store.get(payment_id) {
            return Ok(stored.clone());
        }

        Ok(PaymentResult {
            payment_id: payment_id.to_string(),
            status: PaymentStatus::Completed,
            amount: 0.0,
            currency: "KRW".to_string(),
            paid_at: Some(Utc::now()),
        })
    }

    async fn refund(
        &self,
        payment_id: &str,
        amount: Option<f64>,
    ) -> Result<PaymentResult, anyhow::Error> {
        let mut store = self
            .store
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;

        let original = store.get(payment_id);
        let refund_amount = amount.unwrap_or_else(|| original.map_or(0.0, |o| o.amount));
        let currency = original
            .map_or_else(|| "KRW".to_string(), |o| o.currency.clone());

        let result = PaymentResult {
            payment_id: payment_id.to_string(),
            status: PaymentStatus::Completed,
            amount: refund_amount,
            currency,
            paid_at: Some(Utc::now()),
        };

        store.insert(payment_id.to_string(), result.clone());

        tracing::info!(
            payment_id = %payment_id,
            refund_amount = refund_amount,
            "[MockPayment] Refund processed",
        );

        Ok(result)
    }
}

// -- Mock Document Provider ---------------------------------------------------

pub struct MockDocumentProvider {
    counter: AtomicU64,
    store: Mutex<HashMap<String, SignatureResult>>,
}

impl MockDocumentProvider {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            store: Mutex::new(HashMap::new()),
        }
    }

    fn next_signature_id(&self) -> String {
        let n = self.counter.fetch_add(1, Ordering::Relaxed);
        format!("mock-sig-{}-{n}", Utc::now().timestamp_millis())
    }
}

impl Default for MockDocumentProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentProvider for MockDocumentProvider {
    async fn request_signature(
        &self,
        request: SignatureRequest,
    ) -> Result<SignatureResult, anyhow::Error> {
        let signature_id = self.next_signature_id();
        let signed_at = Utc::now();
        let signed_document_url =
            format!("{}?signed=true&id={signature_id}", request.document_url);

        let result = SignatureResult {
            signature_id: signature_id.clone(),
            status: SignatureStatus::Signed,
            signed_at: Some(signed_at),
            signed_document_url: Some(signed_document_url),
        };

        self.store
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?
            .insert(signature_id.clone(), result.clone());

        tracing::info!(
            signature_id = %signature_id,
            signer_name = %request.signer_name,
            signer_email = %request.signer_email,
            purpose = %request.purpose,
            "[MockDocument] Signature requested and auto-signed",
        );

        Ok(result)
    }

    async fn get_signature_status(
        &self,
        signature_id: &str,
    ) -> Result<SignatureResult, anyhow::Error> {
        let store = self
            .store
            .lock()
            .map_err(|e| anyhow::anyhow!("lock poisoned: {e}"))?;

        if let Some(stored) = store.get(signature_id) {
            return Ok(stored.clone());
        }

        Ok(SignatureResult {
            signature_id: signature_id.to_string(),
            status: SignatureStatus::Pending,
            signed_at: None,
            signed_document_url: None,
        })
    }
}

// -- Mock Medical Provider ----------------------------------------------------

/// Sample Korean medical data representative of common conditions in elderly patients.
fn sample_conditions() -> Vec<Condition> {
    vec![
        Condition {
            code: "I10".to_string(),
            name: "본태성 고혈압".to_string(),
            diagnosed_at: "2018-03-15".to_string(),
        },
        Condition {
            code: "E11".to_string(),
            name: "2형 당뇨병".to_string(),
            diagnosed_at: "2020-07-22".to_string(),
        },
        Condition {
            code: "M81.0".to_string(),
            name: "폐경 후 골다공증".to_string(),
            diagnosed_at: "2021-11-05".to_string(),
        },
    ]
}

fn sample_medications() -> Vec<Medication> {
    vec![
        Medication {
            name: "암로디핀".to_string(),
            dosage: "5mg 1일 1회".to_string(),
            prescribed_at: "2018-04-01".to_string(),
        },
        Medication {
            name: "메트포르민".to_string(),
            dosage: "500mg 1일 2회".to_string(),
            prescribed_at: "2020-08-10".to_string(),
        },
        Medication {
            name: "알렌드론산".to_string(),
            dosage: "70mg 주 1회".to_string(),
            prescribed_at: "2021-12-01".to_string(),
        },
        Medication {
            name: "아스피린".to_string(),
            dosage: "100mg 1일 1회".to_string(),
            prescribed_at: "2019-01-20".to_string(),
        },
    ]
}

pub struct MockMedicalProvider;

impl MedicalProvider for MockMedicalProvider {
    async fn fetch_health_record(
        &self,
        patient_id: &str,
        _consent_token: &str,
    ) -> Result<HealthRecord, anyhow::Error> {
        tracing::info!(patient_id = %patient_id, "[MockMedical] Fetching health record");

        Ok(HealthRecord {
            patient_id: patient_id.to_string(),
            conditions: sample_conditions(),
            medications: sample_medications(),
            last_visit: Some("2026-01-10".to_string()),
        })
    }

    async fn sync_medications(
        &self,
        patient_id: &str,
        _consent_token: &str,
    ) -> Result<Vec<MedicationSync>, anyhow::Error> {
        tracing::info!(patient_id = %patient_id, "[MockMedical] Syncing medications");

        Ok(sample_medications()
            .into_iter()
            .map(|m| MedicationSync {
                name: m.name,
                dosage: m.dosage,
            })
            .collect())
    }
}

// =============================================================================
// Mock Providers bundle
// =============================================================================

/// Holds a complete set of mock provider instances for development and testing.
pub struct MockProviders {
    pub identity: MockIdentityProvider,
    pub messaging: MockMessagingProvider,
    pub maps: MockMapProvider,
    pub payments: MockPaymentProvider,
    pub documents: MockDocumentProvider,
    pub medical: MockMedicalProvider,
}

impl MockProviders {
    /// Returns a fresh set of all mock provider instances.
    ///
    /// Use this in tests or local development to wire up all external service
    /// stubs without touching real APIs.
    pub fn new() -> Self {
        Self {
            identity: MockIdentityProvider,
            messaging: MockMessagingProvider::new(),
            maps: MockMapProvider,
            payments: MockPaymentProvider::new(),
            documents: MockDocumentProvider::new(),
            medical: MockMedicalProvider,
        }
    }
}

impl Default for MockProviders {
    fn default() -> Self {
        Self::new()
    }
}
