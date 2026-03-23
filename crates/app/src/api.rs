// =============================================================================
// API client — WASM-side HTTP calls to the Axum backend
// =============================================================================

use std::cell::RefCell;
use std::collections::VecDeque;

use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, de::DeserializeOwned};

/// Maximum number of cached entries.
const CACHE_MAX_ENTRIES: usize = 100;
/// Cache TTL in milliseconds (5 minutes).
const CACHE_TTL_MS: f64 = 5.0 * 60.0 * 1000.0;

struct CacheEntry {
    key: String,
    value: String,
    inserted_at: f64,
}

/// Bounded LRU cache with TTL.
struct BoundedCache {
    entries: VecDeque<CacheEntry>,
}

impl BoundedCache {
    fn new() -> Self {
        Self {
            entries: VecDeque::with_capacity(CACHE_MAX_ENTRIES),
        }
    }

    fn now_ms() -> f64 {
        js_sys::Date::now()
    }

    fn get(&mut self, key: &str) -> Option<String> {
        let now = Self::now_ms();
        // Remove expired entries
        self.entries.retain(|e| (now - e.inserted_at) < CACHE_TTL_MS);
        // Find and return (move to back for LRU)
        if let Some(pos) = self.entries.iter().position(|e| e.key == key) {
            let entry = self.entries.remove(pos)?;
            let value = entry.value.clone();
            self.entries.push_back(entry);
            Some(value)
        } else {
            None
        }
    }

    fn insert(&mut self, key: String, value: String) {
        let now = Self::now_ms();
        // Remove existing entry with same key
        self.entries.retain(|e| e.key != key);
        // Evict oldest if at capacity
        while self.entries.len() >= CACHE_MAX_ENTRIES {
            self.entries.pop_front();
        }
        self.entries.push_back(CacheEntry {
            key,
            value,
            inserted_at: now,
        });
    }

    fn invalidate_prefix(&mut self, prefix: &str) {
        self.entries.retain(|e| !e.key.starts_with(prefix));
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}

thread_local! {
    static CACHE: RefCell<BoundedCache> = RefCell::new(BoundedCache::new());
}

/// Clear cached response for a specific path prefix.
/// Called after POST/PATCH/DELETE to invalidate related caches.
fn invalidate_cache(path: &str) {
    let prefix = path.split('?').next().unwrap_or(path);
    CACHE.with(|cache| {
        cache.borrow_mut().invalidate_prefix(prefix);
    });
}

/// Clear entire cache (used on logout).
pub fn clear_cache() {
    CACHE.with(|cache| cache.borrow_mut().clear());
}

/// Base URL for API calls (same origin in production).
fn base_url() -> String {
    // In dev, the API server runs on the same origin
    String::new()
}

/// Standard API response envelope.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub meta: Option<PaginationMeta>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

/// Redirect to sign-in page (used on 401 responses).
/// Skips redirect if already on the signin page to prevent infinite loops.
fn redirect_to_signin() {
    clear_cache();
    if let Some(window) = leptos::web_sys::window() {
        let current = window.location().pathname().unwrap_or_default();
        if current != "/auth/signin" {
            let _ = window.location().set_href("/auth/signin");
        }
    }
}

/// Check response status and handle 401 redirects.
/// Returns Ok(response) for 2xx, Err for non-2xx.
/// Auth-check endpoints (like /api/auth/me) should NOT trigger redirect on 401
/// — they are expected to fail for unauthenticated users.
async fn check_response(resp: gloo_net::http::Response) -> Result<gloo_net::http::Response, String> {
    let status = resp.status();
    if status == 401 {
        redirect_to_signin();
        return Err("인증이 만료되었습니다. 다시 로그인해 주세요.".to_string());
    }
    if status < 200 || status >= 300 {
        return Err(format!("서버 오류 ({})", status));
    }
    Ok(resp)
}

/// GET request that returns the parsed response without 401 redirect.
/// Use for auth checks and other endpoints where 401 is an expected response.
pub async fn get_silent<T: DeserializeOwned>(path: &str) -> Result<ApiResponse<T>, String> {
    let url = format!("{}{}", base_url(), path);
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let text = resp.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

/// GET request to API endpoint.
pub async fn get<T: DeserializeOwned>(path: &str) -> Result<ApiResponse<T>, String> {
    // Check cache first
    let cached = CACHE.with(|cache| cache.borrow_mut().get(path));
    if let Some(json_str) = cached {
        return serde_json::from_str(&json_str).map_err(|e| e.to_string());
    }

    let url = format!("{}{}", base_url(), path);
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let resp = check_response(resp).await?;

    // Cache the raw response text
    let text = resp.text().await.map_err(|e| e.to_string())?;
    CACHE.with(|cache| {
        cache.borrow_mut().insert(path.to_string(), text.clone());
    });

    serde_json::from_str(&text).map_err(|e| e.to_string())
}

/// POST request with JSON body.
/// Does not redirect on 401 — POST endpoints may return 401 with a meaningful
/// error body (e.g. demo login "user not found") that the caller should handle.
pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
    path: &str,
    body: &B,
) -> Result<ApiResponse<T>, String> {
    let url = format!("{}{}", base_url(), path);
    let resp = gloo_net::http::Request::post(&url)
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    invalidate_cache(path);
    let text = resp.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

/// POST request without caring about the response body (e.g. logout).
pub async fn post_no_body(path: &str) -> Result<(), String> {
    let url = format!("{}{}", base_url(), path);
    let resp = gloo_net::http::Request::post(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    if status < 200 || status >= 300 {
        return Err(format!("서버 오류 ({})", status));
    }
    clear_cache();
    Ok(())
}

/// PATCH request with JSON body.
pub async fn patch<T: DeserializeOwned, B: serde::Serialize>(
    path: &str,
    body: &B,
) -> Result<ApiResponse<T>, String> {
    let url = format!("{}{}", base_url(), path);
    let resp = gloo_net::http::Request::patch(&url)
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let resp = check_response(resp).await?;
    invalidate_cache(path);
    resp.json().await.map_err(|e| e.to_string())
}

/// PUT request with JSON body.
pub async fn put<T: DeserializeOwned, B: serde::Serialize>(
    path: &str,
    body: &B,
) -> Result<ApiResponse<T>, String> {
    let url = format!("{}{}", base_url(), path);
    let resp = gloo_net::http::Request::put(&url)
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let resp = check_response(resp).await?;
    invalidate_cache(path);
    resp.json().await.map_err(|e| e.to_string())
}

/// DELETE request.
pub async fn delete<T: DeserializeOwned>(path: &str) -> Result<ApiResponse<T>, String> {
    let url = format!("{}{}", base_url(), path);
    let resp = gloo_net::http::Request::delete(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let resp = check_response(resp).await?;
    invalidate_cache(path);
    resp.json().await.map_err(|e| e.to_string())
}

// =============================================================================
// Korean formatting helpers
// =============================================================================

/// Format a UTC datetime as Korean date: "2026년 3월 18일"
pub fn format_date_kr(dt: &DateTime<Utc>) -> String {
    format!("{}년 {}월 {}일", dt.year(), dt.month(), dt.day())
}

/// Format a UTC datetime as Korean date+time: "2026년 3월 18일 14:30"
pub fn format_datetime_kr(dt: &DateTime<Utc>) -> String {
    format!(
        "{}년 {}월 {}일 {:02}:{:02}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute()
    )
}

/// Format a Korean phone number with dashes: "01012345678" → "010-1234-5678"
pub fn format_phone_kr(phone: &str) -> String {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    match digits.len() {
        11 => format!("{}-{}-{}", &digits[..3], &digits[3..7], &digits[7..]),
        10 => format!("{}-{}-{}", &digits[..3], &digits[3..6], &digits[6..]),
        _ => phone.to_string(), // return as-is if not standard length
    }
}
