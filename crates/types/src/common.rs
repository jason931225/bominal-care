// =============================================================================
// Common types — ApiResponse, PaginationMeta, DateRange
// Ported from packages/types/src/common.ts + api-helpers.ts
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// PaginationMeta
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

impl PaginationMeta {
    pub fn new(total: i64, page: i64, limit: i64) -> Self {
        let total_pages = if limit > 0 {
            (total + limit - 1) / limit
        } else {
            0
        };
        Self {
            total,
            page,
            limit,
            total_pages,
        }
    }
}

// ---------------------------------------------------------------------------
// PaginationParams
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PaginationParams {
    #[serde(deserialize_with = "deserialize_i64_from_str")]
    pub page: i64,
    #[serde(deserialize_with = "deserialize_i64_from_str")]
    pub limit: i64,
}

/// Accept both `123` (number) and `"123"` (string) for query-param i64 fields.
fn deserialize_i64_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    struct I64OrStr;
    impl<'de> de::Visitor<'de> for I64OrStr {
        type Value = i64;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("i64 or string-encoded i64")
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<i64, E> { Ok(v) }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<i64, E> { Ok(v as i64) }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<i64, E> {
            v.parse().map_err(de::Error::custom)
        }
    }
    deserializer.deserialize_any(I64OrStr)
}

impl PaginationParams {
    pub fn new(page: i64, limit: i64) -> Self {
        let page = page.max(1);
        let limit = limit.clamp(1, 100);
        Self { page, limit }
    }

    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.limit
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self { page: 1, limit: 20 }
    }
}

// ---------------------------------------------------------------------------
// ApiResponse<T>
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: None,
        }
    }

    pub fn success_with_meta(data: T, meta: PaginationMeta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(meta),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
            meta: None,
        }
    }
}

// ---------------------------------------------------------------------------
// DateRange
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
