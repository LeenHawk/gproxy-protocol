use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::gemini::types::{GeminiApiErrorResponse, GeminiModelInfo, GeminiResponseHeaders};

/// Successful body for Gemini `models.get` endpoint.
pub type ResponseBody = GeminiModelInfo;

/// Full HTTP response for Gemini `models.get` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiModelGetResponse {
    Success {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: GeminiResponseHeaders,
        /// Successful body.
        body: ResponseBody,
    },
    Error {
        /// HTTP status code returned by server (typically non-2xx).
        #[serde(with = "crate::gemini::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: GeminiResponseHeaders,
        /// Error body.
        body: GeminiApiErrorResponse,
    },
}
