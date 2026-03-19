// =============================================================================
// ValidatedJson — Axum extractor that validates input on deserialization
// =============================================================================

use axum::{
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

/// Extracts and validates a JSON body.
/// Replaces `Json<T>` on POST/PATCH handlers to ensure `.validate()` is called.
pub struct ValidatedJson<T>(pub T);

#[derive(Debug)]
pub enum ValidatedJsonError {
    JsonError(JsonRejection),
    ValidationError(validator::ValidationErrors),
}

impl IntoResponse for ValidatedJsonError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::JsonError(e) => (
                StatusCode::BAD_REQUEST,
                format!("잘못된 요청 형식입니다: {e}"),
            ),
            Self::ValidationError(e) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("입력값 검증 실패: {e}"),
            ),
        };
        let body = serde_json::json!({
            "success": false,
            "data": null,
            "error": message,
        });
        (status, axum::Json(body)).into_response()
    }
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidatedJsonError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Json(value) = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(ValidatedJsonError::JsonError)?;

        value.validate().map_err(ValidatedJsonError::ValidationError)?;

        Ok(ValidatedJson(value))
    }
}
